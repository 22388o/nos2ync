use serde::Deserialize;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let config = load_config_from_file(&path);

    println!("private key: {}", config.nostr_private_key);
    println!("last n tweets: {}", config.last_n_tweets);
    println!("twitter username: {}", config.twitter_username);
    println!("consumer key: {}", config.consumer_key);
    println!("consumer secret: {}", config.consumer_secret);
}

#[derive(Debug, Deserialize)]
struct Config {
    nostr_private_key: String,
    last_n_tweets: u16,
    twitter_username: String,
    consumer_key: String,
    consumer_secret: String,
}

fn load_config_from_file(path: &str) -> Config {
    let contents = std::fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let config: Config = ron::from_str(&contents).unwrap();
    config
}
