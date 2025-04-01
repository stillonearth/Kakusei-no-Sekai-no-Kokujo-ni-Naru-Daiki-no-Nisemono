const { ethers } = require("ethers");
const { Command } = require("commander");
var fs = require("fs");

// abi json
let abiPath =
  "./deployments/chain-1946/artifacts/NftModule#KakuseiNFT.json";
let abi = JSON.parse(fs.readFileSync(abiPath, "utf8"));

// contract deployments json
let deployedAddressesFilePath =
  "./deployments/chain-1946/deployed_addresses.json";
let deployedAddresses = JSON.parse(
  fs.readFileSync(deployedAddressesFilePath, "utf8")
);

const CONTRACT_ABI = abi.abi;
const CONTRACT_ADDRESS = deployedAddresses["NftModule#KakuseiNFT"];

const provider = new ethers.JsonRpcProvider("https://rpc.minato.soneium.org");

let walletWithProvider = new ethers.Wallet(
  fs.readFileSync("./private_key", "utf8"),
  provider
);

const nftContract = new ethers.Contract(
  CONTRACT_ADDRESS,
  CONTRACT_ABI,
  walletWithProvider
);

async function mintNft(owner) {
  let txResponse = await nftContract.safeMint(owner);

  const txReceipt = await txResponse.wait();
  let nftId = txReceipt.logs[0].args[2];

  return parseInt(nftId);
}

async function setBaseUri(baseUri) {
  let txResponse = await nftContract.setBaseUri(baseUri);

  await txResponse.wait();
}

async function tokenUri(tokenId) {
  let result = await nftContract.tokenURI(tokenId);

  return result;
}


const program = new Command();

program
  .name("kakusei-cli")
  .description("CLI to mint kakusei-no-sekai NFTs.")
  .version("0.0.1");

program
  .command("mint")
  .description("mint kakusei-no-sekai nft")
  .argument("<owner>", "owner of nft")
  .action(async (owner) => {
    console.log(await mintNft(owner));
    process.exit();
  });

program
  .command("set-base-uri")
  .description("mint kakusei-no-sekai nft")
  .argument("<base_uri>", "owner of nft")
  .action(async (base_uri) => {
    console.log(await setBaseUri(base_uri));
    process.exit();
  });

program
  .command("token-uri")
  .description("get uri for token id")
  .argument("<id>", "id of token")
  .action(async (id) => {
    console.log(await tokenUri(id));
    process.exit();
  });

program.parse();
