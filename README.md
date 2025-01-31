# ChaCha20 and integration with Poly1305

Please check the latest version on [GitHub](github.com/leo-leesco/Crypto-TD4).

## Build

`cargo build` produces `x25519` in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`x25519` expects :
- `m` as a 64-hexadecimal string
- (optional) `u` as a 64-hexadecimal string; if it is not provided, will be set to the default `u=9`
