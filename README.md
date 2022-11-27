rustup toolchain install nightly

rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

rustup component add llvm-tools-preview

cargo install bootimage



cargo build --target x86_64-rust_os.json

qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
