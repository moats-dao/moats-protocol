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

# qry_url = r"https://fcd.terra.dev/v1/txs/gas_prices"
# qry_url = r"https://little-icy-sun.terra-mainnet.quiknode.pro/v1/txs/gas_prices"
qry_url = r"https://fcd.terra.dev/v1/txs?account=terra1tmnqgvg567ypvsvk6rwsga3srp7e3lg6u0elp8"
# qry_url = r"https://little-icy-sun.terra-mainnet.quiknode.pro/txs?account=terra1tmnqgvg567ypvsvk6rwsga3srp7e3lg6u0elp8"

url_request = Request(qry_url, headers=    {"User-Agent": "Mozilla/5.0"})
res = json.loads(urllib.request.urlopen(url_request).read().decode())

#pp.pprint(res)
df = pd.json_normalize(res)
print(df)
