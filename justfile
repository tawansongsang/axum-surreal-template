alias bb := backend_build
alias bc := backend_check
alias br := backend_run
alias bd := backend_dev
alias bt := backend_test
alias btdb := backend_test_surrealdb
alias btr := backend_test_rpc
alias btd := backend_test_derive
alias si := surreal_import

backend_build:
  cargo build --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_check:
  cargo check --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_run:
  cargo run --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_dev:
  cargo run --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml --example quick_dev

backend_test:
  cargo test --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml

backend_test_surrealdb:
  cargo test --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml -p lib-surrealdb

backend_test_rpc:
  cargo test --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml -p lib-rpc

backend_test_derive:
  cargo test --manifest-path backend/Cargo.toml --config backend/.cargo/config.toml -p lib-derive
  
surreal_import PATH:
  surreal import --conn http://localhost:8000 --user root --pass root --ns ns_tempalte --db db_template {{PATH}}
