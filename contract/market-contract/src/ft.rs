use crate::*;

#[near_bindgen]
impl Contract {
    /// 토큰 구매
    /// 토큰 이름과 얼마나 구매할 지 입력
    #[payable]
    pub fn buy_token(&mut self, token_id: String, trading_amounts: u64) {
        assert_one_yocto();

        let token_id: String = token_id.into();
        assert!(self.ft_infos.contains_key(&token_id), "Must have the token");
        let price_per_ft: Balance = self.ft_infos.get(&token_id).unwrap().into();

        let buyer_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        assert!(self.whitelist.contains(&buyer_id), "Customers must enroll in the whitelist");

        let trading_amounts: u128 = trading_amounts.into();

        let value = price_per_ft * trading_amounts;

        assert!(
            deposit >= value,
            "Attached deposit must be greater than or equal to the current price: {:?}",
            value
        );

        // 구매자가 가지고 있는 FT 읽고 구매할 만큼 추가
        let mut ft_map = self
            .ft_deposits
            .get(&token_id)
            .unwrap_or_else(|| UnorderedMap::new(b'a'));
        let mut cur_bal: Balance = ft_map.get(&buyer_id).unwrap_or(0);
        cur_bal += value;
        ft_map.insert(&buyer_id, &cur_bal);

        self.ft_deposits.insert(&token_id, &ft_map);
    }

    /// 토큰 판매
    /// 토큰 이름과 얼마나 판매할 지 입력
    #[payable]
    pub fn sell_token(&mut self, token_id: AccountId, trading_amounts: u64) {
        assert_one_yocto();

        let token_id: String = token_id.into();
        assert!(self.ft_infos.contains_key(&token_id), "Must have the token");
        let price_per_ft: Balance = self.ft_infos.get(&token_id).unwrap().into();

        let seller_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        assert!(self.whitelist.contains(&seller_id), "Customers must enroll in the whitelist");

        let trading_amounts: u128 = trading_amounts.into();

        let value = price_per_ft * trading_amounts;

        // 판매자가 가지고 있는 FT 읽고 판매할 만큼 감소
        let mut ft_map = self.ft_deposits.get(&token_id).unwrap();
        let mut cur_bal: Balance = ft_map.get(&seller_id).unwrap_or(0);
        assert!(
            cur_bal >= value,
            "Not enough tokens to sell"
        );

        cur_bal -= value;
        if cur_bal == 0 {
            ft_map.remove(&seller_id);
            if ft_map.is_empty() {
                self.ft_deposits.remove(&token_id);
            }
        } else {
            ft_map.insert(&seller_id, &cur_bal);
            self.ft_deposits.insert(&token_id, &ft_map);
        }

        Promise::new(seller_id).transfer(trading_amounts * price_per_ft);
    }
}
