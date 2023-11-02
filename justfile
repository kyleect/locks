default:
    @just --list

build:
    cargo build --release

build-docker:
    docker build -t kyleect/locks:1.0.0 .

run-docker-repl:
    docker run --rm -it kyleect/locks:1.0.0 locks repl

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
    cargo nextest run --features='gc-stress,gc-trace,vm-trace' --workspace {{args}}

test-miri *args:
    MIRIFLAGS='-Zmiri-disable-isolation' cargo +nightly miri nextest run \
        --features='gc-stress,gc-trace,vm-trace' --no-default-features \
        --workspace {{args}}
