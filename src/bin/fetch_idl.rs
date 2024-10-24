use flate2::read::ZlibDecoder;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::io::Read;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    program_id: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let client = RpcClient::new(env.rpc_url.to_string());
    let program_id: Pubkey = env.program_id.parse()?;

    let (base, _) = Pubkey::find_program_address(&[], &program_id);

    let idl_address = Pubkey::create_with_seed(&base, "anchor:idl", &program_id)?;

    let idl_account_data = client.get_account_data(&idl_address)?;

    let len = u32::from_le_bytes(idl_account_data[40..44].try_into()?);

    let mut decoder = ZlibDecoder::new(&idl_account_data[44..44 + len as usize]);
    let mut s = String::new();
    decoder.read_to_string(&mut s)?;

    let idl: serde_json::Value = serde_json::from_str(&s)?;

    println!("{:#?}", idl);

    Ok(())
}
