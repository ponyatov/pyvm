# .mk files
MK += Makefile
MK += $(wildcard   mk/*.mk)

# ini
S += $(wildcard lib/*.ini) $(wildcard lib/*.f)

# Rust
R += $(wildcard      ./src/*.rs)      ./Cargo.toml

# F#
F += $(wildcard lib/*.fs*)
