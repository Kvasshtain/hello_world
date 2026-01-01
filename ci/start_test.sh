#!/bin/bash

sleep 10

/opt/bin/solana-keygen new --no-bip39-passphrase -s -o /opt/ci/test.json --force

/opt/bin/solana config set --url http://solana:8899

/opt/bin/solana airdrop 1000 /opt/ci/test.json

echo !!!!!!!!!!!!!

/opt/bin/solana address -k /opt/ci/test.json

/opt/bin/hw