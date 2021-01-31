cargo publish --no-verify --manifest-path ..\Cargo.toml
timeout 8
cargo publish --no-verify --manifest-path ..\..\Nature-Demo\Cargo.toml
cargo publish --no-verify --manifest-path ..\..\Nature-Integrate-Test-Executor\Cargo.toml
