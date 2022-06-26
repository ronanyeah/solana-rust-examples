use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
}

#[derive(serde::Deserialize)]
struct Parsed {
    info: SplToken,
}

#[derive(serde::Deserialize)]
struct SplToken {
    mint: String,
    #[serde(rename(deserialize = "tokenAmount"))]
    token_amount: Amount,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct Amount {
    amount: String,
    #[serde(rename(deserialize = "uiAmountString"))]
    ui_amount_string: String,
    #[serde(rename(deserialize = "uiAmount"))]
    ui_amount: f64,
    decimals: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let wallet: Pubkey = env.wallet_pubkey.parse()?;

    let tokens = get_token_accounts(&client, &wallet)?;

    let accounts: Vec<_> = tokens
        .iter()
        .filter_map(|x| {
            if let solana_account_decoder::UiAccountData::Json(d) = &x.account.data {
                Some(d)
            } else {
                None
            }
        })
        .collect();

    let parsed: Vec<_> = accounts
        .iter()
        .filter_map(|x| serde_json::from_value::<Parsed>(x.parsed.clone()).ok())
        .filter(|x| x.info.token_amount.decimals == 0 && x.info.token_amount.ui_amount == 1.0)
        .map(|x| x.info.mint)
        .collect();

    println!("NFTs owned by {}:", wallet.to_string());
    println!("{:#?}", parsed);

    Ok(())
}

fn get_token_accounts(
    rpc: &RpcClient,
    owner: &Pubkey,
) -> Result<Vec<solana_client::rpc_response::RpcKeyedAccount>, Box<dyn std::error::Error>> {
    rpc.get_token_accounts_by_owner(
        &owner,
        solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::ID),
    )
    .map_err(|e| e.into())
}
