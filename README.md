# Moats Protocol

## Setup environment

### Frontend (React)

* Setup
```bash
npm install
```

### Backend Smart Contract (Rust Cosmwasm)

* Setup
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

* Deploy
```bash
cargo schema
./build_optimized_wasm.sh
pipenv shell
./terrapy/upload-code.py
./terrapy/create-contract.py
```

### Backend Test (Terra.js)

* keys.terrain.js file needed

* Setup
```bash
npm install terrajs/
```

### Backend (Terra.py)

* config.json & keys.json file needed

* Setup
```bash
cd terrapy
pipenv install --skip-lock --pre
pipenv shell
```