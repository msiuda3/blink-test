#!/bin/bash
cargo +esp build --target xtensa-esp32-espidf -Z build-std=std,panic_abort
