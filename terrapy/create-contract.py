import base64
import json, urllib
from urllib.request import Request

from terra_sdk.client.lcd.api.tx import CreateTxOptions
#from terra_sdk.client.localterra import LocalTerra
from terra_sdk.client.lcd import LCDClient
from terra_sdk.core.wasm import MsgStoreCode, MsgInstantiateContract, MsgExecuteContract
from terra_sdk.core.fee import Fee
from terra_sdk.key.mnemonic import MnemonicKey

url_request = Request("https://fcd.terra.dev/v1/txs/gas_prices", headers=    {"User-Agent": "Mozilla/5.0"})
live_gas_prices = json.loads(urllib.request.urlopen(url_request).read().decode())

# Opening JSON file
with open('keys.json') as f:
    # returns JSON object as a dictionary
    walletMnemonics = json.load(f)

with open('code_id.json') as f:
    code_id = json.load(f)

terra = LCDClient(
    url="https://bombay-lcd.terra.dev/",
    chain_id="bombay-12",
    gas_prices={ 'uluna': live_gas_prices['uluna'] },
    gas_adjustment="1.5")
test1 = terra.wallet(MnemonicKey(mnemonic=walletMnemonics["testnetyk"]['mnemonic']))

instantiate = MsgInstantiateContract(
    sender=test1.key.acc_address,
    admin=test1.key.acc_address,
    code_id=int(code_id),
    init_msg={ "price": "23" },    # InitMsg
    #{},    # InitMsg
    init_coins={}  # init coins
    #{"uluna": 1000000, "uusd": 1000000}  # init coins
)
instantiate_tx = test1.create_and_sign_tx(CreateTxOptions(msgs=[instantiate]))
instantiate_tx_result = terra.tx.broadcast(instantiate_tx)
print(instantiate_tx_result)

contract_address = instantiate_tx_result.logs[0].events_by_type[
    "instantiate_contract"
]["contract_address"][0]

with open('contract_address.json', 'w') as f:
    json.dump(contract_address, f)

import sys
sys.exit()

execute = MsgExecuteContract(
    test1.key.acc_address,
    test1.key.acc_address,
    contract_address,
    {"increment": {}},
    {"uluna": 100000},
)

execute_tx = test1.create_and_sign_tx(
    CreateTxOptions(msgs=[execute], fee=Fee(1000000, Coins(uluna=1000000)))
)

execute_tx_result = terra.tx.broadcast(execute_tx)
print(execute_tx_result)

result = terra.wasm.contract_query(contract_address, {"get_count": {}})
print(result)