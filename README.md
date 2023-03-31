# NCD Demo Project  
## STO Project T.rusT

![image](https://user-images.githubusercontent.com/69336797/229003603-77b5cb4f-a3ee-470f-bdbe-190e30abeeeb.png)
This project is for the NCD program.

# 0. Near Certified Developer Course Goals

 1. **perceiving** web 3 in the context of human history

 2. **reading** contract code in Rust and JavaScript

 3. **writing** contracts using Rust and JavaScript

 4. **testing** contracts using unit and simulation tests

 5. **deploying** contracts and a sensible user interface




# 1. **perceiving** web 3 in the context of human history

We have two aspects of our objectives.

![제목 없음-2022-11-03-2052](https://user-images.githubusercontent.com/113986828/229123739-e002d7af-d798-432d-8072-b42ce4ad7e31.png)
One is completing the offering and exchange part of the STO process.
We divided the STO process into four parts. 

1. oracle 

2. offering 

3. Trust 

4. exchange

We implemented 2, 4 of those 4 process. 



#  2. **reading** contract code in Rust and JavaScript

- https://github.com/near-examples/ft-tutorial/tree/main/market-contract

- https://github.com/near-examples/nft-tutorial/tree/8.marketplace/market-contract

- https://docs.near.org/tutorials/nfts/marketplace

# 3. **writing** contracts using Rust and JavaScript

## NFT Contract
- fn nft_mint => Register assets to NFT and input information.

<img width="438" alt="image" src="https://user-images.githubusercontent.com/69336797/229028889-cf736a4c-6820-4aa8-b938-07619c08a7bb.png">

- fn nft_approve => Register assets to the market and input the number of tokens to divide the assets and their price.

<img width="1059" alt="image" src="https://user-images.githubusercontent.com/69336797/229029068-d9e75674-a211-4aee-b1ee-131f02ac6354.png">


## Market Contract 
- fn nft_on_approve => Register assets to the market and input the number of tokens to divide the assets and their price. called by NFT contract

<img width="734" alt="image" src="https://user-images.githubusercontent.com/69336797/229029506-9d97f764-eae0-4ea3-a42d-a1958bd8b7e7.png">

- fn buy_token  => Purchase of assets that have been divided into each token.
<img width="752" alt="image" src="https://user-images.githubusercontent.com/69336797/229029374-e4db992a-5c76-4474-810b-ce41be3c1546.png">

- fn sell_token => Sell of assets that have been divided into each token.

<img width="734" alt="image" src="https://user-images.githubusercontent.com/69336797/229029409-d5166e95-ab0f-469c-8be1-399ef2581697.png">

# 4. **testing** contracts using unit and simulation tests

Unit test to confirm NFT registration on the market.

![image](https://user-images.githubusercontent.com/69336797/229070762-52db0258-1b53-4dca-a03a-9b983a388933.png)



# 5. **deploying** contracts and a sensible user interface


# How to use

1. contract build & deploy
    - nft contract deploy && fn new_default_meta call
    - market contract deploy && fn new call
2. mint NFT to register a asset
3. approve NFT to market
4. now, you can buy asset tokens and sell asset tokens

# Next Step

1. use Oracle 

    [Offering part]    
    - to allow only whitelisted accounts to trade.
    - for the part that links the real value of the real thing.

    [Market part]
    - to allow price fluctuations triggered by Oracle to make it possible to change the price of assets.

2. add Trust part

    - to keep the token, the result of the STO's issuance 
    to protect the trustless. 
    There is a question of what is the right price.
    When the issuer and distributor have the token, the trust company will be implemented to solve the trust problem 







