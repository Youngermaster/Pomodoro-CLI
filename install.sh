#!/bin/bash
# Make sure to install Rust
cargo build --release
# Make sure to run this command with sudo.
mkdir /usr/share/pomodoro-cli
cp -r target/release/pomodoro_cli /usr/share/pomodoro-cli
cp -r Audio/ /usr/share/pomodoro-cli

chmod +x pomodoro_cli
cp pomodoro_cli /usr/bin/
