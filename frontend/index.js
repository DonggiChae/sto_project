// React
import React from "react";
import { createRoot, useState, useEffect } from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
// NEAR
import { Wallet } from "./src/near-wallet";

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
// const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS });
const wallet = new Wallet({});

// Setup on page load
window.onload = async () => {
  const isSignedIn = await wallet.startUp();
  createRoot(document.getElementById("root")).render(
    <React.StrictMode>
      <BrowserRouter>
        <App isSignedIn={isSignedIn} wallet={wallet} />
      </BrowserRouter>
    </React.StrictMode>
  );
};
