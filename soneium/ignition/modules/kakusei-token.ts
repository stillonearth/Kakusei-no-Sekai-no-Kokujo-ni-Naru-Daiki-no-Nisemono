// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const TokenModule = buildModule("TokenModule", (m) => {
  const adminOwner = m.getAccount(0);

  const token = m.contract("KAKUSEIToken");

  return { token };
});

export default TokenModule;