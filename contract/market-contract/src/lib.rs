use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey,
    CryptoHash, Gas, PanicOnDefault, Promise,
};
use std::collections::HashMap;

use crate::external::*;
use crate::ft::*;
use crate::internal::*;
use crate::nft_callbacks::*;
use crate::sale::*;
use near_sdk::env::STORAGE_PRICE_PER_BYTE;

mod external;
mod ft;
mod internal;
mod nft_callbacks;
mod sale;
mod sale_views;

//GAS constants to attach to calls
const GAS_FOR_RESOLVE_PURCHASE: Gas = Gas(115_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);

//the minimum storage to have a sale on the contract.
const STORAGE_PER_SALE: u128 = 1000 * STORAGE_PRICE_PER_BYTE;

//every sale will have a unique ID which is `CONTRACT + DELIMITER + TOKEN_ID`
static DELIMETER: &str = ".";

//Creating custom types to use within the contract. This makes things more readable.
pub type SalePriceInYoctoNear = U128;
pub type TokenId = String;
pub type FungibleTokenId = AccountId;
pub type ContractAndTokenId = String;
//defines the payout type we'll be parsing from the NFT contract as a part of the royalty standard.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

//main contract struct to store all the information
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //keep track of the owner of the contract
    pub owner_id: AccountId,

    //which fungible token can be used to purchase NFTs
    pub ft_infos: LookupMap<String, Balance>,

    /*
        to keep track of the sales, we map the ContractAndTokenId to a Sale.
        the ContractAndTokenId is the unique identifier for every sale. It is made
        up of the `contract ID + DELIMITER + token ID`
    */
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,

    //keep track of all the Sale IDs for every account ID
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>,

    //keep track of all the token IDs for sale for a given contract
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keep track of the storage that accounts have payed
    pub storage_deposits: LookupMap<AccountId, Balance>,

    //keep track of how many FTs each account has deposited in order to purchase NFTs with
    pub ft_deposits: UnorderedMap<TokenId, UnorderedMap<AccountId, Balance>>,

    // Whitelist for transactions
    pub whitelist: UnorderedSet<AccountId>,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByOwnerIdInner { account_id_hash: CryptoHash },
    ByNFTContractId,
    ByNFTContractIdInner { account_id_hash: CryptoHash },
    ByNFTTokenType,
    ByNFTTokenTypeInner { token_type_hash: CryptoHash },
    FTTokenIds,
    StorageDeposits,
    Accounts,
    Metadata,
    FTDeposits,
    Whitelist,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default data and the owner ID
        that's passed in
    */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            //set the owner_id field equal to the passed in owner_id.
            owner_id,

            //set the FT ID equal to the passed in ft_id.
            ft_infos: LookupMap::new(StorageKey::FTTokenIds),

            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            sales: UnorderedMap::new(StorageKey::Sales),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId),
            by_nft_contract_id: LookupMap::new(StorageKey::ByNFTContractId),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            ft_deposits: UnorderedMap::new(StorageKey::FTDeposits),
            whitelist: UnorderedSet::new(StorageKey::Whitelist),
        };

        //return the Contract object
        this
    }

    //Allows users to deposit storage. This is to cover the cost of storing sale objects on the contract
    //Optional account ID is to users can pay for storage for other people.
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        //get the account ID to pay for storage for
        let storage_account_id = account_id
            //convert the valid account ID into an account ID
            .map(|a| a.into())
            //if we didn't specify an account ID, we simply use the caller of the function
            .unwrap_or_else(env::predecessor_account_id);

        //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //make sure the deposit is greater than or equal to the minimum storage for a sale
        assert!(
            deposit >= STORAGE_PER_SALE,
            "Requires minimum deposit of {}",
            STORAGE_PER_SALE
        );

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;
        //insert the balance back into the map for that account ID
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    //Allows users to withdraw any excess storage that they're not using. Say Bob pays 0.01N for 1 sale
    //Alice then buys Bob's token. This means bob has paid 0.01N for a sale that's no longer on the marketplace
    //Bob could then withdraw this 0.01N back into his account.
    #[payable]
    pub fn storage_withdraw(&mut self) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key).
        assert_one_yocto();

        //the account to withdraw storage to is always the function caller
        let owner_id = env::predecessor_account_id();
        //get the amount that the user has by removing them from the map. If they're not in the map, default to 0
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);

        //how many sales is that user taking up currently. This returns a set
        let sales = self.by_owner_id.get(&owner_id);
        //get the length of that set.
        let len = sales.map(|s| s.len()).unwrap_or_default();
        //how much NEAR is being used up for all the current sales on the account
        let diff = u128::from(len) * STORAGE_PER_SALE;

        //the excess to withdraw is the total storage paid - storage being used up.
        amount -= diff;

        //if that excess to withdraw is > 0, we transfer the amount to the user.
        if amount > 0 {
            Promise::new(owner_id.clone()).transfer(amount);
        }
        //we need to add back the storage being used up into the map if it's greater than 0.
        //this is so that if the user had 500 sales on the market, we insert that value here so
        //if those sales get taken down, the user can then go and withdraw 500 sales worth of storage.
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    // 화이트리스트에 추가
    pub fn make_it_white(&mut self, account_id: AccountId) {
        assert_one_yocto();
        self.whitelist.insert(&account_id);
    }

    /// views
    //return the minimum storage for 1 sale
    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_PER_SALE)
    }

    //return how much storage an account has paid for
    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.storage_deposits.get(&account_id).unwrap_or(0))
    }

    // 토큰의 가격 정보 반환
    pub fn price_per_ft(&self, token_id: String) -> U128 {
        U128(self.ft_infos.get(&token_id).unwrap())
    }

    // 가지고 있는 토큰 반환
    // pub fn ft_balance_of(&self, token_id: String, account_id: AccountId) -> U128 {
    //     U128(
    //         self.ft_deposits
    //             .get(&token_id)
    //             .expect("No such token")
    //             .get(&account_id)
    //             .unwrap_or(0),
    //     )
    // }

    // 화이트리스트에 포함 여부 확인
    pub fn is_white(&self, account_id: AccountId) -> bool {
        self.whitelist.contains(&account_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::Balance;

    const NEAR: u128 = 1000000000000000000000000;
    const centiNEAR: u128 = 10000000000000000000000;

    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

    fn set_context_id(predecessor: AccountId, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

    fn set_context_n_sign(predecessor: &str, amount: Balance, signer: &str) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);
        builder.signer_account_id(signer.parse().unwrap());

        testing_env!(builder.build());
    }

    fn set_predecessor(predecessor: &str) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());

        testing_env!(builder.build());
    }

    fn set_deposit(amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

    #[test]
    fn check_nft_on_approve() {
        // initialize Contract
        let mut contract = Contract::new("contract_account".parse().unwrap());

        // set context
        set_context_n_sign("nft_contract_id", 10 * NEAR, "owner_id");

        let owner: AccountId = "owner_id".parse().unwrap();

        contract.storage_deposit(Some(owner.clone()));

        // call nft_on_approve()
        // arguments: {
            // &mut self,
            // token_id: TokenId,
            // owner_id: AccountId,
            // approval_id: u64,
            // msg: String,
            // ft_amounts: String,
            // ft_price: String,
        // }
        contract.nft_on_approve(
            "token_id".to_string(),
            owner.clone(),
            0u64,
            NEAR.to_string(),
            10u64.to_string(),
            (10*centiNEAR).to_string(),
        );

        assert_eq!(contract.get_supply_sales(), U64(1));
        assert_eq!(contract.get_supply_by_owner_id(owner.clone()), U64(1));
        assert_eq!(contract.get_supply_by_owner_id("stranger".parse().unwrap()), U64(0));
        // println!("{:#?}", contract.get_sales_by_owner_id(owner.clone(), None, Some(1)));

        assert_eq!(contract.get_supply_by_nft_contract_id("nft_contract_id".parse().unwrap()), U64(1));
    }

    #[test]
    fn check_ft_transactions() {
        // initialize contract
        let mut contract = Contract::new("contract_account".parse().unwrap());

        // update ft information
        // a has 10 * token1, whose price is 1 NEAR per token
        // b has 100 * token2, whose price is 0.1 NEAR per token
        let token1: TokenId = "token1".to_string();
        let token2: TokenId = "token2".to_string();

        let a: AccountId = "account_a".parse().unwrap();    // length restriction for AccountId
        let b: AccountId = "account_b".parse().unwrap();

        contract.ft_infos.insert(&token1, &(1 * NEAR));
        contract.ft_infos.insert(&token2, &(10 * centiNEAR));

        assert_eq!(contract.price_per_ft(token1.clone()), U128(1 * NEAR));
        assert_eq!(contract.price_per_ft(token2.clone()), U128(10 * centiNEAR));

        let mut map_token1: UnorderedMap<AccountId, Balance> = UnorderedMap::new(b'a');
        map_token1.insert(&a, &(10 * NEAR));
        contract.ft_deposits.insert(&token1, &map_token1);

        let mut map_token2: UnorderedMap<AccountId, Balance> = UnorderedMap::new(b'a');
        map_token1.insert(&b, &(10 * NEAR));
        contract.ft_deposits.insert(&token2, &map_token2);
        
        // assert_eq!(contract.ft_balance_of(token1.clone(), a.clone()), U128(10 * NEAR));
        // assert_eq!(contract.ft_balance_of(token2.clone(), b.clone()), U128(10 * NEAR));
        // assert_eq!(contract.ft_balance_of(token1.clone(), b.clone()), U128(0));
        
        // register on whitelist; require 1 yoctoNEAR
        set_deposit(1);
        contract.make_it_white(a.clone());
        set_deposit(1);
        contract.make_it_white(b.clone());

        // check the update for whitelist
        assert!(contract.is_white(a.clone()));
        assert!(contract.is_white(b.clone()));

        // transactions
        // set_context("account_a", 10 * NEAR);
        // contract.buy_token(token1.clone(), 5);
        // refund is not working here...
        // contract.buy_token(token2.clone(), 10);
        // set_deposit(1);
        // contract.sell_token(token1.clone(), 15);

        // set_context_id(b.clone(), 5);
        // contract.sell_token(token2.clone(), 10);
        // contract.buy_token(token1.clone(), 5);
    }

    #[test]
    #[should_panic]
    /// should disapprove transaction of non-white
    fn racism() {
        // initialize contract
        let mut contract = Contract::new("contract_account".parse().unwrap());

        // setting for tests
        let token = "Token".to_string();
        contract.ft_infos.insert(&token, &(1 * NEAR));
        set_context("stranger", 10);

        // test1: stranger tries to buy token
        // comment out next line for test2
        contract.buy_token(token.clone(), 5);
                                                                
        // additional setting for test2
        let someone: AccountId = "tanned".parse().unwrap();
        let mut map_token: UnorderedMap<AccountId, Balance> = UnorderedMap::new(b'a');
        map_token.insert(&someone, &(10 * NEAR));
        contract.ft_deposits.insert(&token, &map_token);
        set_predecessor("tanned");
        
        // test2: got tanned and tries to sell token
        contract.sell_token(token.clone(), 5);
    }
}
