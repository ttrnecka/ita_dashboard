set shell := ["powershell.exe", "-c"]

# build for release
build:
    cargo build --release

# run the application
run:
    cargo run

# check the code
check:
    cargo check