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
  cargo build 

backend_check:
  cargo check 

backend_run:
  cargo run 

backend_dev:
  cargo run  --example quick_dev

backend_test:
  cargo test 

backend_test_surrealdb:
  cargo test  -p lib-surrealdb

backend_test_rpc:
  cargo test  -p lib-rpc

backend_test_derive:
  cargo test  -p lib-derive
  
surreal_import PATH:
  surreal import --conn http://localhost:8000 --user root --pass root --ns ns_tempalte --db db_template {{PATH}}
