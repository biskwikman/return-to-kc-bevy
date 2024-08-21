#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/return-to-kc-bevy.exe . &&
exec ./return-to-kc-bevy.exe "@"
