use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    wallet_pubkey: String,
    mint_account_pubkey: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let target: Pubkey = env.wallet_pubkey.parse()?;
    let mint_id: Pubkey = env.mint_account_pubkey.parse()?;

    let rpc = RpcClient::new(env.rpc_url.to_string());

    let addr = spl_associated_token_account::get_associated_token_address(&target, &mint_id);

    let balance = rpc.get_token_account_balance(&addr)?;

    println!("Wallet pubkey: {}", target.to_string());
    println!("Mint account: {}", mint_id.to_string());
    println!("Associated token account: {}", addr.to_string());
    println!("Amount: {}", balance.ui_amount_string);
    println!("Decimals: {}", balance.decimals);

    Ok(())
}
