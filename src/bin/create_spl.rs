use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_token::state::Mint;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair: String,
    mint_keypair: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let signer_wallet = Keypair::from_base58_string(&env.signer_keypair);
    let mint_account = Keypair::from_base58_string(&env.mint_keypair);
    let client = RpcClient::new(env.rpc_url.to_string());

    let decimals = 9;

    let minimum_balance_for_rent_exemption =
        client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &signer_wallet.pubkey(),
        &mint_account.pubkey(),
        minimum_balance_for_rent_exemption,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize_mint_instruction: Instruction = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint_account.pubkey(),
        &signer_wallet.pubkey(),
        None,
        decimals,
    )?;

    let recent_blockhash = client.get_latest_blockhash()?;

    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&signer_wallet.pubkey()),
        &[&mint_account, &signer_wallet],
        recent_blockhash,
    );

    client.send_and_confirm_transaction_with_spinner(&transaction)?;

    println!(
        "SPL Token mint account with {} decimals created successfully:\n{}",
        decimals,
        mint_account.pubkey().to_string()
    );

    Ok(())
}
