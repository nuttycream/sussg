all: build

init:
    cargo run -- init

build:
    cargo run -- build

serve:
    cargo run -- serve
