#!/bin/bash
echo ""
cd ./panorama_s
clear
# RUSTFLAGS="-A unused" cargo build -p panorama_s  # 屏蔽警告
cargo build -p panorama_s
cd ..
echo ""
