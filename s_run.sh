#!/bin/bash
echo ""
cd ./panorama_s
clear
# cargo run --quiet -p panorama_s
# cargo run -p panorama_s -- --auth-key xxx
cargo run -p panorama_s 
cd ..
echo ""
