import React, { useContext } from "react";
import styled from "styled-components";
import { Link } from "react-router-dom";

import Authorize from "./Authorize";

const Container = styled.div`
  width: 100%;
  height: 70px;
  display: flex;
  align-items: center;
  justify-content: center;
  position: fixed;
  top: 0px;
  z-index: 998;
  background-color: blue;
`;

const HeaderWrapper = styled.div`
  width: 1500px;
  height: 70px;
  padding: 20px 80px 20px 80px;
  display: flex;
  align-items: center;
  justify-content: space-between;
`;

const LogoWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 55px;
  height: 55px;
  font-size: 1.5rem;
  font-weight: 800;
`;
const Logo = styled.img`
  width: 100%;
  height: 100%;
`;

const TitleWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 400px;
  height: 50px;
  margin-right: 124px;
  color: ${({ theme }) => theme.colors.basicRed};
  font-size: 45px;
  line-height: 1.33;
`;

const StyledLink = styled(Link)`
  text-decoration: none;
  color: inherit;
`;

const NavContainer = styled.div`
  width: 550px;
  height: 70px;
  padding: 0px 10px;
  display: flex;
  align-items: center;
  justify-content: space-around;
`;

const NameWrapper = styled.div`
  font-size: 1.2rem;
  font-weight: 700;
  cursor: pointer;
`;

export default function Header({ isSignedIn, wallet }) {
  return (
    <Container>
      <HeaderWrapper>
        <LogoWrapper>TrusT</LogoWrapper>
        <NavContainer>
          <StyledLink to="/">
            <NameWrapper>AssetRegister</NameWrapper>
          </StyledLink>
          <StyledLink to="/STMarket">
            <NameWrapper>STMarket</NameWrapper>
          </StyledLink>
          <StyledLink to="/MyPage">
            <NameWrapper>MYPage</NameWrapper>
          </StyledLink>
        </NavContainer>
        <Authorize isSignedIn={isSignedIn} wallet={wallet} />
      </HeaderWrapper>
    </Container>
  );
}
