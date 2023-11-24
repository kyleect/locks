default:
    @just --list

install: build
    mv target/release/locks.exe ~/.cargo/bin

install-debug: build-debug
    mv target/debug/locks.exe ~/.cargo/bin

build:
    cargo build --release

build-debug:
    cargo build

build-docker:
    docker build -t kyleect/locks:1.0.0 .

build-vsc:
    cd vsc && just build

run-repl-docker:
    docker run --rm -it kyleect/locks:1.0.0 locks repl

run-lsp-docker:
    docker run --rm -it kyleect/locks:1.0.0 locks lsp

build-all: build
    cd vsc/ && just build
    cd playground/ && just build

clean:
    cargo clean

clean-all: clean
    cd vsc/ && just clean
    cd playground/ && just clean

fmt:
    cargo +nightly fmt --all

fmt-all: fmt
    cd vsc && just fmt
    cd playground/ && just fmt

lint:
    cargo +nightly fmt --all -- --check
    cargo clippy --all-targets --no-deps --workspace

lint-all: lint
    cd vsc/ && just lint
    cd playground/ && just lint

run-playground:
    cd playground/ && just run

run-pprof *args:
    cargo run --features='pprof' --no-default-features --profile='pprof' -- {{args}}

run-trace *args:
    cargo run --features='gc-stress,gc-trace,vm-trace' -- {{args}}

test *args:
    cargo nextest run --features='gc-stress,gc-trace' --workspace {{args}}

test-miri *args:
    MIRIFLAGS='-Zmiri-disable-isolation' cargo +nightly miri nextest run \
        --features='gc-stress,gc-trace' --no-default-features \
        --workspace {{args}}
