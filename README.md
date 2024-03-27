# Solana Scripts

This is a library of Rust scripts for specific purposes:

- [Generate a new wallet](#new-wallet)
- [Create an SPL token](#create-an-spl-token)
- [Mint an SPL token](#mint-spl-tokens)
- [Get an associated token balance](#get-token-balance)
- [Get the creation date of an account](#get-account-creation-timestamp)
- [Get the owner of an NFT](#get-nft-owner)
- [List NFTs in wallet](#list-nfts-in-wallet)
- [Subscribe to events](#subscribe-to-events)

The scripts are found in the `src/bin` folder. They can be configured using environment variables, which are documented below.

[Open an issue](https://github.com/ronanyeah/solana-rust-examples/issues) if you have any requests or suggestions.

---

## New wallet

`cargo run --bin new_wallet` | [Code](./src/bin/new_wallet.rs)

Generates a new wallet and prints the pubkey, Base58 private key, and JSON private key.

---

## Create an SPL token

`cargo run --bin create_spl` | [Code](./src/bin/create_spl.rs)

Creates a new [SPL token](https://spl.solana.com/token) mint account.

| Environment Variable | Note                                                       |
| -------------------- | ---------------------------------------------------------- |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`                 |
| SIGNER_KEYPAIR       | Base58 encoded keypair, to pay for the transaction.        |
| MINT_KEYPAIR         | Base58 encoded keypair, representing the new mint account. |

---

## Mint SPL tokens

`cargo run --bin mint_spl` | [Code](./src/bin/mint_spl.rs)

Mints 10,000 SPL tokens from a specified mint to the [associated token account](https://spl.solana.com/associated-token-account) of a specified wallet.

| Environment Variable | Note                                                               |
| -------------------- | ------------------------------------------------------------------ |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`                         |
| SIGNER_KEYPAIR       | Base58 encoded keypair, which has mint authority.                  |
| MINT_ACCOUNT_PUBKEY  | The pubkey address of the SPL Token mint account.                  |
| RECEIVER_PUBKEY      | The pubkey address of the wallet you want to fund with the tokens. |

---

## Get token balance

`cargo run --bin associated_token_balance` | [Code](./src/bin/associated_token_balance.rs)

Prints the balance of an [associated token account](https://spl.solana.com/associated-token-account), for a specified wallet and mint.

| Environment Variable | Note                                                   |
| -------------------- | ------------------------------------------------------ |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`             |
| WALLET_PUBKEY        | The pubkey address of the wallet that owns the tokens. |
| MINT_ACCOUNT_PUBKEY  | The pubkey address of the SPL Token mint account.      |

---

## Get account creation timestamp

`cargo run --bin creation_date` | [Code](./src/bin/creation_date.rs)

Fetches and prints the creation timestamp of a specified [account](https://docs.solana.com/developing/programming-model/accounts).

| Environment Variable | Note                                                      |
| -------------------- | --------------------------------------------------------- |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`                |
| ACCOUNT_PUBKEY       | The pubkey address of the account you want to introspect. |

---

## Get NFT owner

`cargo run --bin nft_owner` | [Code](./src/bin/nft_owner.rs)

Prints the wallet address that owns a specified NFT.

| Environment Variable | Note                                              |
| -------------------- | ------------------------------------------------- |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`        |
| MINT_ACCOUNT_PUBKEY  | The pubkey address of the SPL Token mint account. |

---

## List NFTs in wallet

`cargo run --bin list_nfts` | [Code](./src/bin/list_nfts.rs)

Fetches and prints the mint pubkeys of every NFT in the specified wallet.

| Environment Variable | Note                                              |
| -------------------- | ------------------------------------------------- |
| RPC_URL              | e.g. `https://api.mainnet-beta.solana.com`        |
| WALLET_PUBKEY        | The pubkey address of the wallet that owns the NFTs. |

---

## Subscribe to events

`cargo run --bin pubsub` | [Code](./src/bin/pubsub.rs)

Listens to events from [`slotSubscribe`](https://solana.com/docs/rpc/websocket/slotsubscribe).

| Environment Variable | Note                                              |
| -------------------- | ------------------------------------------------- |
| WS_URL              | e.g. `wss://api.mainnet-beta.solana.com`        |