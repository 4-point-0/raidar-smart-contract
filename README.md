# Raidar NEAR Smart Contract

<p align="middle">
   <a href="https://near.org/" target="blank"><img src="./logos/near_logo.png" width="200" alt="Near Logo" /></a>
</p>
<p>
    &nbsp;
    &nbsp;
</p>

<p align="middle">
   <a href="" target="blank"><img src="./logos/logo.png" width="200" alt="Berklee Logo"/></a>
</p>

Raidar is a platform that allows artists and users buy and sell rights to music in a digital collectibles (NFTs) format.

# Quick Start

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. See `package.json` for a full list of `scripts` you can run with `yarn`

---

Install dependencies:

    yarn install

Build contract:

    yarn build

Build and deploy your contract to TestNet with a temporary dev account:

    yarn deploy

Test your contract:

    yarn test

# Exploring The Code

1. The smart contract code lives in the `/contract` folder.
2. Test your contract: `yarn test`, this will run the tests in `integration-tests` directory.

# Deploy

Every smart contract in NEAR has its [own associated account][near accounts]. When you run `yarn deploy`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.

## Step 0: Install near-cli (optional)

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

## Step 1: Create an account for the contract

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-account.testnet`, you can deploy your contract to `some-alias.your-account.testnet`.

Assuming you've already created an account on [NEAR Wallet], here's how to create `some-alias.your-account.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

   near login

2. Create a subaccount (replace `some-alias` below with an actual account name):

   near create-account some-alias.your-account.testnet --masterAccount your-account.testnet

## Step 2: deploy the contract

Use the CLI to deploy the contract to TestNet with your account ID.
Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.

    near deploy --accountId some-alias.your-account.testnet --wasmFile ./out/main.wasm

# Troubleshooting

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.

# Development deployment

```bash
yarn build
near deploy --accountId raidar-dev.testnet --wasmFile ./out/main.wasm
```

# Staging deployment

```bash
yarn build
near deploy --accountId raidar-staging.testnet --wasmFile ./out/main.wasm
```

# Production deployment

```bash
yarn build
near deploy --accountId raidar.near --wasmFile ./out/main.wasm
```
