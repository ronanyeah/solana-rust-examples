use solana_account_decoder::{parse_token::UiTokenAccount, UiAccountData};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let wallet: Pubkey = env.wallet_pubkey.parse()?;

    let token_accounts = client
        .get_token_accounts_by_owner(
            &wallet,
            solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::ID),
        )
        .await?;

    let parsed_accounts = token_accounts
        .clone()
        .iter()
        .map(|token_account| {
            let UiAccountData::Json(json_data) = &token_account.account.data else {
                return Err("non-JSON token account data returned")?;
            };
            let info = json_data.parsed.get("info").ok_or("missing 'info' field")?;
            let data = serde_json::from_value::<UiTokenAccount>(info.clone())?;
            Ok::<_, Box<dyn std::error::Error>>(data)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let nfts: Vec<_> = parsed_accounts
        .into_iter()
        .filter(|acct| acct.token_amount.decimals == 0 && acct.token_amount.ui_amount == Some(1.0))
        .map(|acct| acct.mint)
        .collect();

    println!("NFTs owned by {}:", wallet.to_string());
    println!("{:#?}", nfts);

    Ok(())
}
