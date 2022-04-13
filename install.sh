#!/bin/bash
# Make sure to install Rust
cargo build --release
# Make sure to run this command with sudo.
sudo mkdir /usr/share/pomodoro-cli
sudo cp -r target/release/pomodoro_cli /usr/share/pomodoro-cli
sudo cp -r Audio/ /usr/share/pomodoro-cli

chmod +x pomodoro_cli
sudo cp pomodoro_cli /usr/bin/
