import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const NftModule = buildModule("NftModule", (m) => {
  const adminOwner = m.getAccount(0);

  const nft = m.contract("KakuseiNFT", [adminOwner]);

  return { nft };
});

export default NftModule;