# smanage-rs

My website, [kmdw.dev](https://kmdw.dev), is a bunch of separate webservers all proxied behind an nginx instance. A while back, I wrote a tool which I called `smanage` in regular Python. It took a JSON configuration file and outputted an nginx configuration file along with a Bash script to start every server. However, as extensions piled up (run configurations, weird subdomain stuff, etc.) it became really unmaintainable. So, I rewrote it in Rust -- which is what you're seeing now!

I have little experience with Rust, so the code is probably pretty bad. I'd appreciate any feedback.

## Development

It's a pretty basic Rust project. Just `cargo run`, `cargo build`, the whole shebang.

It uses `serde` and `serde_json` for reading the configuration file. It then uses a small helper struct to manage indentation so that it doesn't have to have `\n\t`s everywhere. I tried to make it a lot more modular than the old Python version, and adding features has been a lot easier.
