use futures_util::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;

#[derive(serde::Deserialize)]
struct Env {
    ws_url: url::Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;

    let ps_client = PubsubClient::new(&env.ws_url.to_string()).await?;

    let (mut accounts, unsubscriber) = ps_client.slot_subscribe().await?;

    let mut count = 0;
    while let Some(response) = accounts.next().await {
        println!("{:?}", response);
        count += 1;
        if count >= 5 {
            break;
        }
    }

    unsubscriber().await;

    Ok(())
}
