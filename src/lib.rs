use solana_account_decoder::{
    parse_account_data::ParseAccountError,
    parse_token_extension::{parse_extension, UiExtension, UiTokenMetadata},
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{
    extension::{ExtensionType::TokenMetadata, StateWithExtensions},
    state::Mint,
};

pub async fn fetch_token_2022_nft_metadata(
    rpc: &RpcClient,
    mint: &Pubkey,
) -> Result<UiTokenMetadata, Box<dyn std::error::Error>> {
    let acct = rpc.get_account(mint).await?;
    let mint_acct = StateWithExtensions::<Mint>::unpack(&acct.data[..])?;
    let extension = parse_extension(&TokenMetadata, &mint_acct);
    if let UiExtension::TokenMetadata(data) = extension {
        Ok(data)
    } else {
        Err(ParseAccountError::AdditionalDataMissing(
            "Metadata not found".to_string(),
        ))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nft_22() {
        let mainnet_rpc = std::env::var("RPC_URL").unwrap();
        let client = RpcClient::new(mainnet_rpc);
        let token_22_nft = "FT6sLdVn6zPYiXeJJSkbrtEw6o2z1mQA74ZMUJA4f77V";
        let acct = token_22_nft.parse().unwrap();
        let res = fetch_token_2022_nft_metadata(&client, &acct).await.unwrap();
        assert!(res.mint == token_22_nft);
    }
}
