import React from "react";
import styled from "styled-components";

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin-top: 100px;
`;

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin-top: 100px;
`;

export default function STMarket({ wallet }) {
  const handleGetSale = () =>
    wallet.viewMethod({
      contractId: MARKET_CONTRACT_ID,
      method: "get_sale",
      args: {
        nft_contract_id: NFT_CONTRACT_NAME,
        token_id: "1",
      },
    });
    
  const handleAddSale = () => {
    wallet.callMethod({
      contractId: MARKET_CONTRACT_ID,
      method: "add_sale",
      args: {
        nft_contract_id: NFT_CONTRACT_NAME,
        token_id: "1",
        price: "100000000000000000000000",
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });
  }

  const  handleDeleteSale = () => {
    wallet.callMethod({
      contractId: MARKET_CONTRACT_ID,
      method: "delete_sale",
      args: {
        nft_contract_id: NFT_CONTRACT_NAME,
        token_id: "1",
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });


  const handleOffer = () => {
    wallet.callMethod({
      contractId: MARKET_CONTRACT_ID,
      method: "offer",
      args: {
        nft_contract_id: NFT_CONTRACT_NAME,
        token_id: "1",
        ft_amount: "100000000000000000000000",
        amount: "100000000000000000000000",
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });
  }


  const handleAcceptOffer = () => {
    wallet.callMethod({
      contractId: MARKET_CONTRACT_ID,
      method: "accept_offer",
      args: {
        nft_contract_id: NFT_CONTRACT_NAME,
        token_id: "1",
        offer_id: "1",
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",

    });
  }

  return <Container>STMarket</Container>;
}

const marketContract = {
  get_sale: async (nft_contract_id, token_id) => {
    return await CONTRACT.query({
      contractId: MARKET_CONTRACT_ID,
      methodName: "get_sale",
      args: {
        nft_contract_id,
        token_id,
      },
    });
  },

  add_sale: async (nft_contract_id, token_id, price) => {
    return await CONTRACT.functionCall({
      contractId: MARKET_CONTRACT_ID,
      methodName: "add_sale",
      args: {
        nft_contract_id,
        token_id,
        price,
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });
  },

  delete_sale: async (nft_contract_id, token_id) => {
    return await CONTRACT.functionCall({
      contractId: MARKET_CONTRACT_ID,
      methodName: "delete_sale",
      args: {
        nft_contract_id,
        token_id,
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });
  },

  offer: async (nft_contract_id, token_id, ft_amount, amount) => {
    return await CONTRACT.functionCall({
      contractId: MARKET_CONTRACT_ID,
      methodName: "offer",
      args: {
        nft_contract_id,
        token_id,
        ft_amount,
        amount,
      },
      gas: "100000000000000",
      attachedDeposit: "1000000000000000000000",
    });
  },

  get_account_sales: async (account_id, from_index, limit) => {
    return await CONTRACT.query({
      contractId: MARKET_CONTRACT_ID,
      methodName: "get_account_sales",
      args: {
        account_id,
        from_index,
        limit,
      },
    });
  },

  get_supply_sales: async () => {
    return await CONTRACT.query({
      contractId: MARKET_CONTRACT_ID,
      methodName: "get_supply_sales",
      args: {},
    });
  },
};
