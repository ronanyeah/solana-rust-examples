use borsh::de::BorshDeserialize;
use mpl_token_metadata::state::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    mint_account_pubkey: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let mint: Pubkey = env.mint_account_pubkey.parse()?;

    let metadata = get_metadata(&client, &mint)?;

    println!("{} metadata:\n{:#?}", mint.to_string(), metadata);

    Ok(())
}

fn get_metadata(
    rpc: &RpcClient,
    mint_address: &Pubkey,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let (meta_addr, _) = mpl_token_metadata::pda::find_metadata_account(&mint_address);
    let metadata_account = rpc.get_account(&meta_addr)?;
    let acct = &mut &metadata_account.data[..];
    Metadata::deserialize(acct).map_err(|e| e.into())
}
