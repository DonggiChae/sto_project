import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import { Route, Routes } from "react-router-dom";

import Header from "./src/components/Header/Header";
import AssetRegister from "./src/components/Pages/AssetRegister";
import MyPage from "./src/components/Pages/MyPage";
import STMarket from "./src/components/Pages/STMarket";

import "./assets/global.css";

export default function App({ isSignedIn, contractId, wallet }) {
  return (
    <>
      <Header isSignedIn={isSignedIn} contractId={contractId} wallet={wallet} />
      <Routes>
        <Route path="/" element={<AssetRegister />} />
        <Route path="/stMarket" element={<STMarket />} />
        <Route path="/MyPage" element={<MyPage />} />
      </Routes>
    </>
  );
}
