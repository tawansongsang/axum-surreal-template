alias bb := backend_build
alias br := backend_run
alias bt := backend_test
alias bd := backend_dev

backend_build:
  cargo build --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_run:
  cargo run --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_test:
  cargo test --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_dev:
  cargo run --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml --example quick_dev
