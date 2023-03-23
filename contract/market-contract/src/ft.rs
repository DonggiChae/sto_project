use crate::*;

#[near_bindgen]
impl Contract {

    /// 토큰 구매
    /// 토큰 이름과 얼마나(금액) 구매할 지 입력
    #[payable]
    pub fn buy_token(&mut self, token_id: AccountId, price: Balance) {
        assert_one_yocto();

        let token_id: String = token_id.into();
        assert_eq!(self.ft_id, token_id, "Must be the same token");

        let buyer_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        assert!(deposit >= price, "Attached deposit must be greater than or equal to the current price: {:?}", price);
        let price: Balance = price.into();

        // 구매자가 가지고 있는 FT 읽고 구매할 만큼 추가
        let cur_bal = self.ft_deposits.get(&buyer_id).unwrap_or(0);
        self.ft_deposits.insert(&buyer_id, &(cur_bal + price));

        if price > 0 {
            // Perform the cross contract call to transfer the FTs to the seller
            ext_ft_contract::ext(self.ft_id.clone())
                // Attach 1 yoctoNEAR with static GAS equal to the GAS for nft transfer. Also attach an unused GAS weight of 1 by default.
                .with_attached_deposit(1)
                .ft_transfer(
                    self.owner_id, //seller to transfer the FTs to
                    U128(price), //amount to transfer
                    Some("Token sold".to_string()), //memo (to include some context)
                );

        // If the promise was not successful, we won't transfer any FTs and instead refund the buyer
        } else {
            // Get the buyer's current balance and increment it
            let cur_bal = self.ft_deposits.get(&buyer_id).unwrap();
            self.ft_deposits.insert(&buyer_id, &(cur_bal - price));
        }
    }

    /// 토큰 판매
    /// 토큰 이름과 얼마나(금액) 판매할 지 입력
    #[payable]
    pub fn sell_token(&mut self, token_id: AccountId, price: Balance) {
        assert_one_yocto();

        let token_id: String = token_id.into();
        assert_eq!(self.ft_id, token_id, "Must be the same token");

        let seller_id = env::predecessor_account_id();
        let price: Balance = price.into();

        // 판매자가 가지고 있는 FT 읽고 판매할 만큼 제거
        let cur_bal = self.ft_deposits.get(&seller_id).expect("No token");

        assert!(cur_bal >= price, "Current balance must be greater than or equal to the current price: {:?}", price);
        self.ft_deposits.insert(&seller_id, &(cur_bal - price));

        if price > 0 {
            // Perform the cross contract call to transfer the FTs to the seller
            Promise::new(seller_id).transfer(price);

        // If the promise was not successful, we won't transfer any FTs and instead refund the buyer
        } else {
            // Get the buyer's current balance and increment it
            let cur_bal = self.ft_deposits.get(&buyer_id).unwrap();
            self.ft_deposits.insert(&buyer_id, &(cur_bal + price));
        }
    }
}