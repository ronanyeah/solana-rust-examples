use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::account::ReadableAccount;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    mint_account_pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let mint: Pubkey = env.mint_account_pubkey.parse()?;

    let account = fetch_nft_account(&client, &mint).await?;

    let token_account = spl_token::state::Account::unpack(&mut account.data())?;

    println!("{} owner:\n{}", mint.to_string(), token_account.owner);

    Ok(())
}

async fn fetch_nft_account(
    client: &RpcClient,
    mint: &Pubkey,
) -> Result<solana_sdk::account::Account, Box<dyn std::error::Error>> {
    let filters = Some(vec![
        // account size
        RpcFilterType::DataSize(165),
        // mint bytes
        RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Base58(mint.to_string()))),
        // amount bytes = 1
        RpcFilterType::Memcmp(Memcmp::new(
            64,
            // Bytes filter seems to be failing:
            //MemcmpEncodedBytes::Bytes(
            //1u64.to_le_bytes().to_vec(), // Little-endian encoding of 1
            //),
            MemcmpEncodedBytes::Base64("AQAAAAAAAA==".to_string()),
        )),
    ]);

    let accounts = client
        .get_program_accounts_with_config(
            &spl_token::ID,
            RpcProgramAccountsConfig {
                filters,
                account_config: RpcAccountInfoConfig {
                    encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                    ..Default::default()
                },
                with_context: None,
                sort_results: None,
            },
        )
        .await?;

    match accounts.len() {
        0 => Err("Account not found")?,
        1 => Ok(accounts[0].1.clone()),
        _ => Err("Multiple NFT accounts found")?,
    }
}
