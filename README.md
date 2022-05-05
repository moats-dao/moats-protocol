# Moats Protocol

## Setup environment

### Frontend (React)

* Shell commands
```bash
npm install
```

### Backend (Cosmwasm)

* Shell commands
```bash
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
npm install @terra-money/terrain
```

* Test
```bash
cd contracts
cargo test
cargo build
```

### Backend (Terra.js)

* Shell commands
```bash
npm install terrajs/
```

### Backend (Terra.py)

* Shell commands
```bash
cd terrapy
pipenv install --skip-lock --pre
pipenv shell
```