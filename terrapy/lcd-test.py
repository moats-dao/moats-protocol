import pprint as pp
import base64
import json, urllib
from urllib.request import Request

import pandas as pd

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

terra.
#pp.pprint(res)
df = pd.json_normalize(res)
print(df)
