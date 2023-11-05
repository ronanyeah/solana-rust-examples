use solana_client::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    mint_account_pubkey: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let mint: Pubkey = env.mint_account_pubkey.parse()?;

    let res = get_token_largest_accounts(&client, &mint)?;

    let addr: Pubkey = res
        .value
        .first()
        .ok_or("No token accounts found!")?
        .address
        .parse()?;

    let account = client.get_account(&addr)?;

    let token = spl_token::state::Account::unpack(&mut account.data())?;

    println!("{} owner:\n{}", mint.to_string(), token.owner);

    Ok(())
}

#[derive(serde::Deserialize)]
struct RpcTokenAccounts {
    address: String,
    //amount: String,
    //decimals: u8,
}

fn get_token_largest_accounts(
    rpc: &RpcClient,
    mint_address: &Pubkey,
) -> Result<solana_client::rpc_response::Response<Vec<RpcTokenAccounts>>, Box<dyn std::error::Error>>
{
    let method = "getTokenLargestAccounts";
    let request = solana_client::rpc_request::RpcRequest::Custom { method };
    let params = serde_json::json!([mint_address.to_string()]);

    rpc.send(request, params).map_err(|e| e.into())
}
