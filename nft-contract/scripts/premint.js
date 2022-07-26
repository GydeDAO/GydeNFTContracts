const { keyStores, connect, Contract } = require("near-api-js");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID_MINT = "test2gyde.testnet";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
    keyStore,
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
};
// const config = {
//   keyStore,
//   networkId: "mainnet",
//   nodeUrl: "https://rpc.mainnet.near.org",
//   walletUrl: "https://wallet.mainnet.near.org",
//   helperUrl: "https://helper.mainnet.near.org",
//   explorerUrl: "https://explorer.mainnet.near.org",
// };

deployContract(ACCOUNT_ID_MINT);

async function deployContract(accountIdMint) {
    let owner_id = accountIdMint;

    const near = await connect(config).catch((error) => {
        console.log(error);
    });

    // Take accounts
    const accountMint = await near.account(accountIdMint);

    // Load contracts
    const mintContract = new Contract(
        accountMint, // the account object that is connecting
        accountIdMint,
        {
            // name of contract you're connecting to
            viewMethods: [], // view methods do not change state but usually return a value
            changeMethods: ["nft_mint"], // change methods modify state
            sender: accountMint, // account object to initialize and sign transactions.
        }
    );


    for (let i = 0; i < 20; i++) {
        await mintContract
            .nft_mint({
                args: {
                    token_id: `user-${i}`,
                    metadata: {
                        title: `User Role ${i}`,
                        media: ``,
                        user_type: 'user',
                        organization: i < 10 ? 'gyde' : `gyde-${i}`
                    },
                    receiver_id: owner_id,
                },
                gas: "300000000000000",
                amount: "1000000000000000000000000",
            })
            .catch((error) => console.log(error));
    }


    for (let i = 0; i < 2; i++) {
        await mintContract
            .nft_mint({
                args: {
                    token_id: `super-user-${i}`,
                    metadata: {
                        title: `Super User Role ${i}`,
                        media: ``,
                        user_type: 'super_user',
                        organization: 'gyde'
                    },
                    receiver_id: owner_id,
                },
                gas: "300000000000000",
                amount: "1000000000000000000000000",
            })
            .catch((error) => console.log(error));
    }

    for (let i = 0; i < 1; i++) {
        await mintContract
            .nft_mint({
                args: {
                    token_id: `admin-${i}`,
                    metadata: {
                        title: `Admin Role ${i}`,
                        media: ``,
                        user_type: 'admin',
                        organization: 'gyde'
                    },
                    receiver_id: owner_id,
                },
                gas: "300000000000000",
                amount: "1000000000000000000000000",
            })
            .catch((error) => console.log(error));
    }
}