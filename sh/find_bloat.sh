#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

cargo bloat --release --bin rser2tcp s-n 200 
#| rg "eframe "

cargo llvm-lines --bin rser2tcp | rg eframe | head -30
