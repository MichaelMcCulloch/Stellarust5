# Stellarust5

Prereqs:

- Install [Rustup](https://rustup.rs/);
- run `cargo install miniserve cargo-watch systemfd`;
- Install npm;
- `cd production_data;`<br />`./prepare_data.sh;`

If you want to use HTTPS, point `$STELLARUST_CERT` and `$STELLARUST_KEY` to the cert and key files, respectively. I used [mkcert](https://github.com/FiloSottile/mkcert) to generate the keys and make firefox accept them.


Then run any of the defined vscode tasks
