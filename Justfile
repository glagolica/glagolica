build-release: build compress

build:
    RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" --release

compress:
    upx --best --lzma target/release/glag
    upx --best --lzma target/release/server
