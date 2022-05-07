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

with open('contract_address.json') as f:
    contract_address = json.load(f)

terra = LCDClient(
    url="https://bombay-lcd.terra.dev/",
    chain_id="bombay-12",
    gas_prices={ 'uluna': live_gas_prices['uluna'] },
    gas_adjustment="1.5")
test1 = terra.wallet(MnemonicKey(mnemonic=walletMnemonics["mango_validator"]['mnemonic']))

UST_balance = terra.bank.balance(address=test1.key.acc_address)
print(int(UST_balance[0]['uusd'].amount))

result = terra.wasm.contract_query(contract_address, {"get_u_s_t_balance": { "account_addr": test1.key.acc_address }})
print(result)