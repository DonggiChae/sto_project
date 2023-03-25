import React, { useState, useEffect } from "react";
import styled from "styled-components";
import dotenv from "dotenv";
import { useParams } from "react-router-dom";
import STBuy from "./STBuy";

dotenv.config();
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

export default function STBuy({ wallet }) {
  const { id } = useParams();
  // const [nft, setNft] = useState(null);
  const [quantity, setQuantity] = useState(1);

  // useEffect(() => {
  //   const fetchNft = async () => {
  //     const nftContract = await getNftContract();
  //     const result = await nftContract.nft_token({ token_id: id });
  //     setNft(result);
  //   };
  //   fetchNft();
  // }, [id]);

  // useEffect(() => {
  //   const fetchMarketPrice = async () => {
  //     const marketContract = await getMarketContract();
  //     const result = await marketContract.get_price({ nft_contract_token: id });
  //     setMarketPrice(result);
  //   };
  //   fetchMarketPrice();
  // }, [id]);

  const handlePurchase = async () => {
    // const marketContract = await getMarketContract();
    // const amount = parseNearAmount((marketPrice * quantity).toString());
    // await marketContract.nft_purchase({
    //   nft_contract_token: id,
    //   deposit: amount,
    // });
  };

  const metadata = {
    title: "test-title",
    description: "test-description",
    media: "",
  };

  const price = 0.1;

  return (
    <Container>
      <Image src="https://via.placeholder.com/300x300" alt={metadata.title} />

      <Title>{metadata.title}</Title>
      <Description>{metadata.description}</Description>
      <Price>Price: {price} NEAR</Price>
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
        <BuyButton onClick={handlePurchase()}>Buy</BuyButton>
      </form>
    </Container>
  );
}
