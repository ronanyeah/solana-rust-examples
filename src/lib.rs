use solana_account_decoder::{
    parse_account_data::ParseAccountError,
    parse_token_extension::{parse_extension, UiExtension, UiTokenMetadata},
};
use solana_client::nonblocking::rpc_client::RpcClient;
use spl_token_2022::{
    extension::{ExtensionType::TokenMetadata, StateWithExtensions},
    solana_program::pubkey::Pubkey,
    state::Mint,
};

pub async fn fetch_token_2022_metadata(
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
