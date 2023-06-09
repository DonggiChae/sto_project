import "regenerator-runtime/runtime";
import React from "react";
import { Route, Routes } from "react-router-dom";

import Header from "./src/components/Header/Header";
import AssetRegister from "./src/components/Pages/AssetRegister";
import MyPage from "./src/components/Pages/MyPage";
import STMarket from "./src/components/Pages/STMarket";
import GlobalStyle from "./src/GlobalStyle";
import STBuy from "./src/components/Pages/STBuy";

export default function App({ isSignedIn, wallet }) {
  return (
    <React.StrictMode>
      <GlobalStyle />
      <Header isSignedIn={isSignedIn} wallet={wallet} />
      <Routes>
        <Route
          path="/"
          element={<AssetRegister isSignedIn={isSignedIn} wallet={wallet} />}
        />
        <Route
          path="/STMarket"
          element={<STMarket isSignedIn={isSignedIn} wallet={wallet} />}
        />
        <Route
          path="/STMarket/buy/:tokenId"
          element={<STBuy isSignedIn={isSignedIn} wallet={wallet} />}
        />
        <Route
          path="/MyPage"
          element={<MyPage isSignedIn={isSignedIn} wallet={wallet} />}
        />
      </Routes>
    </React.StrictMode>
  );
}
