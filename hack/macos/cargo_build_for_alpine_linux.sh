# https://github.com/messense/homebrew-macos-cross-toolchains

brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-musl

export CC_AARCH64_UNKNOWN_LINUX_MUSL=aarch64-unknown-linux-musl-gcc
export CXX_AARCH64_UNKNOWN_LINUX_MUSL=aarch64-unknown-linux-musl-g++
export AR_AARCH64_UNKNOWN_LINUX_MUSL=aarch64-unknown-linux-musl-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-unknown-linux-musl-gcc

rustup target add aarch64-unknown-linux-musl

cargo build --target aarch64-unknown-linux-musl