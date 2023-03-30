use crate::*;
use near_sdk::{ext_contract};

pub trait NonFungibleTokenCore {
    //approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, sale_conditions: U128, ft_amounts: u64, ft_price: Balance);

    //check if the passed in account has access to approve the token ID
	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    //revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    //revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}
// ft_price가 최초 등록할때 정한 시세
#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        sale_conditions: U128,
        ft_amounts: u64,
        ft_price: Balance,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {

    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, sale_conditions: Option<String>, ft_amounts: u64, ft_price: Balance) {
        /*
            assert at least one yocto for security reasons - this will cause a redirect to the NEAR wallet.
            The user needs to attach enough to pay for storage on the contract
        */
        assert_at_least_one_yocto();

        let approval_id = 0u64;

        // if some message was passed into the function, we initiate a cross contract call on the
        // account we're giving access to. 
        if let Some(sale_conditions) = sale_conditions {
            // Defaulting GAS weight to 1, no attached deposit, and no static GAS to attach.
            ext_non_fungible_approval_receiver::ext(account_id)
                .nft_on_approve(
                    token_id, 
                    token.owner_id, 
                    approval_id, 
                    sale_conditions,
                    ft_amounts,
                    ft_price
                ).as_return();
        }
    }

    //check if the passed in account has access to approve the token ID
	fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        //get the token object from the token_id
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        //get the approval number for the passed in account ID
		let approval = token.approved_account_ids.get(&approved_account_id);

        //if there was some approval ID found for the account ID
        if let Some(approval) = approval {
            //if a specific approval_id was passed into the function
			if let Some(approval_id) = approval_id {
                //return if the approval ID passed in matches the actual approval ID for the account
				approval_id == *approval
            //if there was no approval_id passed into the function, we simply return true
			} else {
				true
			}
        //if there was no approval ID found for the account ID, we simply return false
		} else {
			false
		}
    }

    //revoke a specific account from transferring the token on your behalf 
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        //assert that the user attached exactly 1 yoctoNEAR for security reasons
        assert_one_yocto();
        //get the token object using the passed in token_id
        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        //get the caller of the function and assert that they are the owner of the token
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        //if the account ID was in the token's approval, we remove it and the if statement logic executes
        if token
            .approved_account_ids
            .remove(&account_id)
            .is_some()
        {
            //refund the funds released by removing the approved_account_id to the caller of the function
            refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

            //insert the token back into the tokens_by_id collection with the account_id removed from the approval list
            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        //assert that the caller attached exactly 1 yoctoNEAR for security
        assert_one_yocto();

        //get the token object from the passed in token ID
        let mut token = self.tokens_by_id.get(&token_id).expect("No token");
        //get the caller and make sure they are the owner of the tokens
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        //only revoke if the approved account IDs for the token is not empty
        if !token.approved_account_ids.is_empty() {
            //refund the approved account IDs to the caller of the function
            refund_approved_account_ids(predecessor_account_id, &token.approved_account_ids);
            //clear the approved account IDs
            token.approved_account_ids.clear();
            //insert the token back into the tokens_by_id collection with the approved account IDs cleared
            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}