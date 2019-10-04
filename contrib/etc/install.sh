#!/bin/bash

set -ex

### Install rustup (includes cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
chmod 744 rustup.sh
./rustup.sh -y

source $HOME/.cargo/env

pwd
pushd /opt/app-root/src
cargo build
popd
