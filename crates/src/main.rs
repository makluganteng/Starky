use listener::Listener;
use tokio;

#[tokio::main]
async fn main() {
    let listener =
        Listener::new("https://starknet-goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")
            .unwrap();
    listener.run().await;
}
