import React, { useEffect, useState } from "react";
import styled from "styled-components";
import dotenv from "dotenv";

import Input from "../Input";
import Button from "../Button";
import { async } from "regenerator-runtime";

dotenv.config();
const NFT_CONTRACT_NAME = process.env.NFT_CONTRACT_NAME;

const THIRTY_TGAS = "30000000000000";
const NO_DEPOSIT = "0";

const Container = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  margin-top: 120px;
  background-color: grey;
`;
const NFTFormWrapper = styled.div``;
const NFTForm = styled.form``;

export default function AssetRegister({ wallet }) {
  const [tokenId, setTokenId] = useState("");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [media, setMedia] = useState("");
  const [account, setAccount] = useState("");
  let mintNFT = {
    receiverId: NFT_CONTRACT_NAME,
    actions: [
      {
        type: "nft_mint",
        params: {
          methodName: "add_message",
          args: {
            token_id: tokenId,
            metadata: { title: title, description: description },
            receiver_id: account,
          },
          gas: THIRTY_TGAS,
          deposit: NO_DEPOSIT,
        },
      },
    ],
  };

  const handleTokenIdChange = (e) => {
    setTokenId(e.target.value);
  };

  const handleTitleChange = (e) => {
    setTitle(e.target.value);
  };

  const handleDescriptionChange = (e) => {
    setDescription(e.target.value);
  };

  const handleMediaChange = (e) => {
    setMedia(e.target.value);
  };

  const handleMint = (e) => {
    e.preventDefault();
    console.log(NFT_CONTRACT_NAME);
    wallet.callMethod({
      contractId: NFT_CONTRACT_NAME,
      method: "nft_mint",
      args: {
        token_id: tokenId,
        metadata: { title: title, description: description },
        receiver_id: account,
      },
      deposit: "100000000000000000000000",
    });
    e.target.reset();
  };
  useEffect(() => {
    setAccount(wallet.accountId);
  }, []);

  return (
    <Container>
      STMarket
      <NFTFormWrapper>
        Asset Registeration
        <NFTForm onSubmit={handleMint}>
          <Input
            name="TokenID"
            onChange={handleTokenIdChange}
            placeholder={"TokenID"}
            required
          />
          <Input
            name="Title"
            onChange={handleTitleChange}
            placeholder={"Title"}
            required
          />
          <Input
            name="Description"
            onChange={handleDescriptionChange}
            placeholder={"Description"}
            required
          />
          <Input
            name="Media"
            onChange={handleMediaChange}
            placeholder={"Media"}
            required
          />
          <Button type="submit" title="mint" />
        </NFTForm>
      </NFTFormWrapper>
    </Container>
  );
}
