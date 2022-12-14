# Stellarust v0.5

### Heavily inspired by [Stellaru](https://github.com/benreid24/Stellaru). This is a learn-rust-the-hard-way project, and I chose this project because I play stellaris a LOT, and stellaru is quite slow. 

Prereqs:

- Install [Rustup](https://rustup.rs/);
- run `cargo install  cargo-watch systemfd`;
- Install npm;
- `cargo build --all --release`<br />`cargo build --all`
- `cd frontend/`<br />`npm install`
- `cd production_data`<br />`./prepare_data.sh;`

If you want to use HTTPS, point `$STELLARUST_CERT` and `$STELLARUST_KEY` to the cert and key files, respectively. I used [mkcert](https://github.com/FiloSottile/mkcert) to generate the keys and make firefox accept them.


Then run the following VSCode Task:
- run `Deploy Stellarust In Production Mode With Automagic Redeployement`
- navigate to `localhost:8000`


If you are debugging the HTTP(S) frontend, run the following VSCode Task:
- Debug HTTP(S) Client
