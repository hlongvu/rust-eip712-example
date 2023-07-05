const { ethers } = require('ethers');

const domain = {
  name: "Ether Mail",
  version: "1",
  chainId: 1,
  verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
};

// The named list of all type definitions

const types = {
  Person: [
    { name: "name", type: 'string' },
    { name: 'wallet', type: 'address' },
  ],
  Mail: [
    { name: 'from', type: 'Person' },
    { name: 'to', type: 'Person' },
    { name: 'contents', type: 'string' }
  ]
}



let main = async function() {
  const wallet = new ethers.Wallet(ethers.keccak256(ethers.toUtf8Bytes("cow")));

  const value = {
    from: {
        name: "Cow",
        wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"
    },
    to: {
        name: "Bob",
        wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
    },
    contents: "Hello, Bob!"
  }
  let signature = await wallet.signTypedData(domain, types, value)
  console.log(signature)
};

main();
