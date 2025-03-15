const { ethers } = require('ethers');
const { Command } = require('commander');
var fs = require('fs');

// abi json
let abiPath = "../blockchain/ignition/deployments/chain-1946/artifacts/NftModule#KakuseiNFT.json";
let abi = JSON.parse(fs.readFileSync(abiPath, 'utf8'));

// contract deployments json
let deployedAddressesFilePath = "../blockchain/ignition/deployments/chain-1946/deployed_addresses.json";
let deployedAddresses = JSON.parse(fs.readFileSync(deployedAddressesFilePath, 'utf8'));

const CONTRACT_ABI = abi.abi;
const CONTRACT_ADDRESS = deployedAddresses["NftModule#KakuseiNFT"];

const provider = new ethers.JsonRpcProvider("https://rpc.minato.soneium.org");

let walletWithProvider = new ethers.Wallet(
    fs.readFileSync("./private_key", 'utf8'),
    provider
);

const nftContract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, walletWithProvider);

async function mintNft(owner, uri){

    let txResponse = await nftContract.safeMint(
        owner, 
        uri,
    );

    const txReceipt = await txResponse.wait();

    console.log(txReceipt)
}

const program = new Command();

program
  .name('kakusei-cli')
  .description('CLI to mint kakusei-no-sekai NFTs.')
  .version('0.0.1');

program.command('mint')
  .description('mint kakusei-no-sekai nft')
  .argument('<owner>', 'owner of key')
  .argument('<uri>', 'owner uri')
  .action(async (owner, uri) => {
    await mintNft(owner, uri);
    process.exit();
  });

program.parse();