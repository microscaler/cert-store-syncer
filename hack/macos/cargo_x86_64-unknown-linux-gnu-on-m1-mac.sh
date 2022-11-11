# https://github.com/messense/homebrew-macos-cross-toolchains

brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-gnu

export CC_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-gcc
export CXX_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-g++
export AR_X86_64_UNKNOWN_LINUX_GNU=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

rustup target add x86_64-unknown-linux-gnu

cargo build --target x86_64-unknown-linux-gnu