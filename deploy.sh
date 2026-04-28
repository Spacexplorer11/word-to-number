#!/bin/bash
git pull
sudo systemctl stop word-to-number
cargo build --release || exit 1
sudo systemctl start word-to-number