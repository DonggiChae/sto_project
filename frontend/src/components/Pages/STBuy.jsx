import React, { useState, useEffect } from "react";
import styled from "styled-components";
import dotenv from "dotenv";
import { useParams } from "react-router-dom";
import STBuy from "./STBuy";
import { async } from "regenerator-runtime";

dotenv.config();
const NFT_CONTRACT_NAME = process.env.NFT_CONTRACT_NAME;
const MARKET_CONTRACT_NAME = process.env.MARKET_CONTRACT_NAME;

const Container = styled.div`
  margin-top: 120px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
`;

const Title = styled.h1`
  margin-top: 20px;
  font-size: 36px;
  font-weight: bold;
  margin-bottom: 10px;
`;

const Description = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 10px;
`;

const Price = styled.p`
  font-size: 24px;
  margin-bottom: 20px;
`;

const Quantity = styled.p`
  font-size: 18px;
  margin-bottom: 10px;
  display: flex;
  align-items: center;
  margin-bottom: 20px;
`;

const BuyButton = styled.button`
  font-size: 24px;
  padding: 10px 50px;
  margin-top: 20px;
  display: flex;
  align-items: center;
`;

const Image = styled.img`
  width: 300px;
  height: 300px;
  object-fit: cover;
`;

export default function STBuy({ wallet, tokenId }) {
  const [nft, setNft] = useState(null);
  const [quantity, setQuantity] = useState(1);
  const [marketPrice, setMarketPrice] = useState();

  // nft.core의 nft_token 에서 nft 정보 가져옴
  const nftResult = async () => {
    await wallet.viewMethod({
      contractId: NFT_CONTRACT_NAME,
      method: "nft_token",
      args: {
        token_id: tokenId,
      },
    });
  };

  // 토큰 가격 정보 lib.rs에서 가져옴
  const priceResult = async () => {
    await wallet.viewMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "price_per_ft",
      args: {
        token_id: tokenId,
      },
    });
  };

  useEffect(() => {
    setNft(nftResult);
  }, [tokenId]);

  useEffect(() => {
    setMarketPrice(priceResult);
  }, [tokenId]);

  const handleBuy = async (quantity) => {
    await wallet.callMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "buy_token",
      args: {
        token_id: tokenId,
        trading_amounts: quantity,
      },
    });
  };

  // 화면 테스트 용
  // const metadata = {
  //   title: "test-title",
  //   description: "test-description",
  //   media: "",
  // };

  // const price = 1;

  return (
    <Container>
      <Image
        src="https://via.placeholder.com/300x300"
        alt={nft.metadata.title}
      />

      <Title>{nft.metadata.title}</Title>
      <Description>{nft.metadata.description}</Description>
      <Price>Price: {marketPrice} NEAR</Price>
      <form>
        <Quantity>
          <label>Quantity : </label>
          <input
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(e.target.value)}
          />
        </Quantity>
        <div>Total Price : {price * quantity} NEAR</div>
        <BuyButton onClick={handleBuy()}>Buy</BuyButton>
      </form>
    </Container>
  );
}
