import React from "react";
import styled from "styled-components";

const Auth = styled.div`
  grid-column: 3;
  grid-row: 1 / 3;
  z-index: 2;
  justify-self: right;
  display: flex;
  align-items: center;
`;

const UserName = styled.div`
  font-weight: 500;
  font-size: 14px;
  margin-right: 12px;
  cursor: pointer;
`;

const LogOut = styled.div`
  cursor: pointer;
  width: 34px;
  height: 34px;
  border-radius: 27px;
  background: #2c3139;
  background-position: center;
  background-repeat: no-repeat;
  background-size: 14px 10px;
`;

export default function Authorize({ isSignedIn, wallet }) {
  const handleSignIn = async () => {
    wallet.signIn();
  };

  const handleSignOut = async () => {
    wallet.signOut();
  };

  if (isSignedIn) {
    return (
      <Auth>
        <UserName onClick={handleSignOut}>SignOut</UserName>
      </Auth>
    );
  }

  return (
    <Auth>
      <UserName onClick={handleSignIn}>Sign In</UserName>
    </Auth>
  );
}
