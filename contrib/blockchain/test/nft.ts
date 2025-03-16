import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

describe("nft", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deployFixture() {
    const [owner, otherAccount] = await hre.ethers.getSigners();
    const Nft = await hre.ethers.getContractFactory("KakuseiNFT");
    const nft = await Nft.deploy(owner);
    return { nft, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      const { nft, owner } = await loadFixture(deployFixture);
      let owner_ = await nft.owner();
      expect(owner_).to.equal(owner.address);
    });

    it("Should mint", async function () {
      const { nft, owner } = await loadFixture(deployFixture);
      let result = await nft.safeMint(owner);

      let txResult = await result.wait();
      let tokenId = txResult.logs[0].args[2];
      expect(tokenId).to.equal(0);

      await nft.setBaseUri("https://test.rs/");

      let tokenURI = await nft.tokenURI(0);
      console.log(tokenURI);
      expect(tokenURI).to.equal("https://test.rs/0");
    });
  });
});