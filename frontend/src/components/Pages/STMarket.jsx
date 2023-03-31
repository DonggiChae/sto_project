import React, { useState, useEffect } from "react";
import styled from "styled-components";
import dotenv from "dotenv";
import { Route, Routes, Link, useParams } from "react-router-dom";
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

const ListingsTable = styled.table`
  margin-top: 50px;
  border-collapse: collapse;
  width: 80%;
  th,
  td {
    padding: 8px;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }
  th {
    background-color: #f2f2f2;
  }
  tr:hover {
    background-color: #f5f5f5;
  }
  tbody {
    color: #ffffff;
  }
`;

const Button = styled.button`
  background-color: lightblue;
  border: none;
  color: white;
  padding: 12px 24px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;
  border-radius: 5px;
`;

export default function STMarket({ wallet }) {
  const handleGetSale = () => {
    wallet.viewMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "get_sale",
      args: {
        nft_contract_1d,
        token_id,
      },
    });
  };

  const handleAddSale = async () => {
    await wallet.callMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "add_sale",
      args: {
        nft_contract_id,
        token_id,
        price,
      },
      deposit: "1000000000000000000000",
    });
  };

  const handleDeleteSale = async () => {
    await wallet.callMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "delete_sale",
      args: {
        nft_contract_id,
        token_id,
      },
      deposit: "1000000000000000000000",
    });
  };

  const handleOffer = async () => {
    await wallet.callMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "offer",
      args: {
        nft_contract_id,
        token_id,
        ft_amount,
        amount,
      },
      deposit: "1000000000000000000000",
    });
  };

  const handleGetAccountSales = () => {
    wallet.viewMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "get_account_sales",
      args: {
        account_id,
        from_index,
        limit,
      },
    });
  };

  const handleGetSupplySales = () => {
    wallet.viewMethod({
      contractId: MARKET_CONTRACT_NAME,
      method: "get_supply_sales",
      args: {},
    });
  };

  const [listings, setListings] = useState([]);

  useEffect(() => {
    const getSales = async () => {
      const salesCount = await handleGetSupplySales();
      const sales = [];
      console.log(salesCount);

      for (let i = 0; i < salesCount; i++) {
        const sale = await handleGetSale(i);
        sales.push(sale);
      }

      setListings(sales);
    };
    getSales();
  }, []);

  // const { index } = useParams();
  // const index = 1;
  // const tokenId = "test_token_id";

  return (
    <Container>
      <h1>STMarket</h1>
      <ListingsTable>
        <thead>
          <tr>
            <th>Sale ID</th>
            <th>NFT Contract</th>
            <th>Token ID</th>
            <th>Owner</th>
            <th>Sale Price</th>
            <th>Buy</th>
          </tr>
        </thead>
        <tbody>
          {/* <td>test</td> */}
          {/* 실제로는 이 부분 주석을 풀어야 함 */}
          {listings.map((sale, index) => (
            <tr key={index}>
              <td>{sale.sale_id}</td>
              <td>{sale.nft_contract_id}</td>
              <td>{sale.token_id}</td>
              <td>{sale.owner_id}</td>
              <td>{sale.sale_conditions}</td>
              <td>
                <Button>
                <Link
                  to={`/STMarket/buy/${tokenId}`}
                  key={tokenId}
                  tokenId={tokenId}
                >
                  BUY
                </Link>
              </Button>
              </td>
            </tr>
          ))}
          {/* -----------------------------------------
          {/* 이 아래 부분은 화면 테스트 용, 실제로는 삭제되어야 함 */}
          {/* <tr key={index}>
            <td>test_id</td>
            <td>test_nft_contract_id</td>
            <td>test_token_id</td>
            <td>test_owner_id</td>
            <td>test_sale_conditions</td>
            <td>
              <Button>
                <Link
                  to={`/STMarket/buy/${tokenId}`}
                  key={tokenId}
                  tokenId={tokenId}
                >
                  BUY
                </Link>
              </Button>
            </td>
          </tr> */}
          {/* ----------------------------------------- */}
        </tbody>
      </ListingsTable>
    </Container>
  );
}
