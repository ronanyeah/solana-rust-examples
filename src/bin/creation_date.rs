use chrono::prelude::*;
use solana_client::{
    rpc_client::RpcClient, rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::{
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    account_pubkey: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let rpc = RpcClient::new(env.rpc_url.to_string());

    let addr: Pubkey = env.account_pubkey.parse()?;

    let datetime = get_account_creation_date(&rpc, &addr)?;

    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("{} creation date:", addr.to_string());
    println!("UTC - {}", timestamp_str);

    Ok(())
}

fn get_account_creation_date(
    rpc: &RpcClient,
    addr: &Pubkey,
) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    fn fetch(
        rpc: &RpcClient,
        addr: &Pubkey,
        before: Option<Signature>,
    ) -> Result<RpcConfirmedTransactionStatusWithSignature, Box<dyn std::error::Error>> {
        let mut sigs = rpc.get_signatures_for_address_with_config(
            &addr,
            solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config {
                before,
                ..Default::default()
            },
        )?;

        sigs.sort_by_key(|sig| sig.block_time);

        let earliest = sigs.first().ok_or("Empty signature list!")?;

        if sigs.len() < 1000 {
            Ok(earliest.clone())
        } else {
            let sig = Signature::from_str(&earliest.signature)?;
            fetch(&rpc, &addr, Some(sig))
        }
    }

    let status = fetch(&rpc, &addr, None)?;

    let d = UNIX_EPOCH
        + Duration::from_secs(status.block_time.ok_or("Missing block time!")?.try_into()?);

    Ok(DateTime::<Utc>::from(d))
}
