use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::{
    extension::{BaseStateWithExtensions, StateWithExtensions},
    state::{Account, Mint},
};
use spl_token_metadata_interface::state::TokenMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenAccountType {
    NotExists,
    NotTokenAccount { owner: Pubkey },
    Token22Acct,
    TokenAcct,
}

pub async fn fetch_token_2022_nft_metadata(
    rpc: &RpcClient,
    mint: &Pubkey,
) -> Result<TokenMetadata, Box<dyn std::error::Error>> {
    let acct = rpc.get_account(mint).await?;
    let mint_acct = StateWithExtensions::<Mint>::unpack(&acct.data[..])?;
    let metadata = mint_acct.get_variable_len_extension::<TokenMetadata>()?;
    Ok(metadata)
}

pub async fn fetch_token_2022_account(
    rpc: &RpcClient,
    mint: &Pubkey,
) -> Result<Account, Box<dyn std::error::Error>> {
    let acct = rpc.get_account(mint).await?;
    let token_acct = StateWithExtensions::<Account>::unpack(&acct.data[..])?;
    Ok(token_acct.base)
}

pub async fn check_token_account(
    rpc: &RpcClient,
    token_account: &Pubkey,
) -> anyhow::Result<TokenAccountType> {
    let token_program = spl_token_2022_interface::ID;
    let legacy_token_program = spl_token_interface::ID;
    let res = rpc
        .get_account_with_commitment(token_account, Default::default())
        .await?;
    let result = res.value.map_or(TokenAccountType::NotExists, |acct| {
        if acct.owner == token_program {
            TokenAccountType::Token22Acct
        } else if acct.owner == legacy_token_program {
            TokenAccountType::TokenAcct
        } else {
            TokenAccountType::NotTokenAccount { owner: acct.owner }
        }
    });

    Ok(result)
}

pub async fn check_mint_account(
    rpc: &RpcClient,
    mint_account: &Pubkey,
) -> anyhow::Result<TokenAccountType> {
    let token_program = spl_token_2022_interface::ID;
    let legacy_token_program = spl_token_interface::ID;
    let res = rpc
        .get_account_with_commitment(mint_account, Default::default())
        .await?;
    let result = res.value.map_or(TokenAccountType::NotExists, |acct| {
        if acct.owner == token_program {
            TokenAccountType::Token22Acct
        } else if acct.owner == legacy_token_program {
            TokenAccountType::TokenAcct
        } else {
            TokenAccountType::NotTokenAccount { owner: acct.owner }
        }
    });
    Ok(result)
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
        assert!(res.mint.to_string() == token_22_nft);
    }

    #[tokio::test]
    async fn test_token_22() {
        let mainnet_rpc = std::env::var("RPC_URL").unwrap();
        let client = RpcClient::new(mainnet_rpc);
        let token_addr = "AebBgJ8wRMnTRL83SqK2VcdMiRGhb4NzhQUamCctN1ny";
        let acct = token_addr.parse().unwrap();
        let _res = fetch_token_2022_account(&client, &acct).await.unwrap();
    }

    #[tokio::test]
    async fn test_mint_check() {
        let mainnet_rpc = std::env::var("RPC_URL").unwrap();
        let client = RpcClient::new(mainnet_rpc);
        println!(
            "token22: {:?}",
            check_mint_account(
                &client,
                &"E2gLkTXSbbTMmJM19xkquawun2ShJSi7G59A8c2PtbFa"
                    .parse()
                    .unwrap()
            )
            .await
            .unwrap()
        );
        println!(
            "legacy: {:?}",
            check_mint_account(
                &client,
                &"DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
                    .parse()
                    .unwrap()
            )
            .await
            .unwrap()
        );
    }
}
