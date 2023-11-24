<img alt="nos2ync" src="./.img/logo.png" width="150" />

[![Discord](https://badgen.net/badge/icon/discord?icon=discord&label)](https://discord.gg/EvYB9ZgYvV)

### Nos2ync

Nostr sync is a tool that helps you to sync your last `n` tweets with your Nostr account.
Generally it helps you to use Nostr without losing your tweets.

## Install & Usage

Install:
```
cargo install nos2ync
```

Usage:
```
nos2ync ./nos2ync_config.ron
```

## Config

Config.ron is a file that contains information that nos2ync needs to do it job.

The file is in `ron (rust object notation)`.

Example config:

```ron
Config(
    nostr_private_key: "value",
    last_n_tweets: 10,
    twitter_username: "value",
    consumer_key: "value",
    consumer_secret: "value",
)
```

## Contribution

Contributions are welcomed!

## License

Nos2ync is under [MIT](./LICENSE) license
