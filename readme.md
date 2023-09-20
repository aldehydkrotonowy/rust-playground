### add new workspace

cargo new some_name [--lib]

### add entry cargo.toml

add new member to members array

A. watch and rebuild in workspace
cargo watch -x "run --bin rocket_web_server"
cargo watch -x "run -p rocket_web_server" --ignore "\*.sqlite"
