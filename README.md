# Moats Protocol

## Setup environment

### Backend (Terra.py)

* config.json & keys.json file needed

* Setup
```bash
pip install --upgrade --pre pipenv
cd terrapy
pipenv install --skip-lock --pre
pipenv shell
```

* Liquidation
> check config.json
```bash
cd terrapy
pipenv shell
cd ..
python ./terrapy/liquidproof/liquidation_sdk.py
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

* Test (mocking)
```bash
cd contracts
cargo test
cargo build
```

* Deploy
> check config.json
```bash
cargo schema
./build_optimized_wasm.sh
cd terrapy
pipenv shell
cd ..
python ./terrapy/liquidproof/upload-code.py
python ./terrapy/liquidproof/create-contract.py
```

* Test (network)
> check config.json
```bash
python ./terrapy/liquidproof/liquidation_contract.py
```

<strike>
### Backend Test (Terra.js)

* keys.terrain.js file needed

* Setup
```bash
npm install terrajs/
```

### Frontend (React)

* Setup
```bash
npm install
```
</strike>