all: build serve

init:
    cargo run -- init

build:
    cargo run -- build

serve:
    cargo run -- serve

clean:
    rm -rf ./public/
    cargo clean
