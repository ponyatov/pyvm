RTARGET = x86_64-unknown-linux-gnu
# RTARGET = aarch64-unknown-linux-gnu
# RTARGET = i686-pc-windows-gnu

$(RUSTUP) $(CARGO):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup target add $(RTARGET)
	rustup component add rust-analyzer rustfmt
	cargo install cargo-watch cargo-binutils
# rustup target add x86_64-unknown-linux-gnu
# rustup target add aarch64-unknown-linux-gnu
# rustup target add i686-pc-windows-gnu

.PHONY: watch
watch: $(R)
	cargo watch -x "run --target x86_64-unknown-linux-gnu -- $(S)"
