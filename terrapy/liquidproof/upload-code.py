import base64
import json, urllib
from urllib.request import Request

from terra_sdk.client.lcd.api.tx import CreateTxOptions
#from terra_sdk.client.localterra import LocalTerra
from terra_sdk.client.lcd import LCDClient
from terra_sdk.core.wasm import MsgStoreCode, MsgInstantiateContract, MsgExecuteContract
from terra_sdk.core.fee import Fee
from terra_sdk.key.mnemonic import MnemonicKey

if __name__ == "__main__":
    url_request = Request("https://fcd.terra.dev/v1/txs/gas_prices", headers=    {"User-Agent": "Mozilla/5.0"})
    live_gas_prices = json.loads(urllib.request.urlopen(url_request).read().decode())

    # Opening JSON file
    with open("config.json", "r", encoding="utf-8") as read_file:
        config_dic = json.load(read_file)

    if config_dic['network'] == "MAINNET":
        CHAIN_ID = "columbus-5"
        CHAIN_URL = "https://lcd.terra.dev"
        ANC_LIQ_QUE_CONTRACT = "terra1e25zllgag7j9xsun3me4stnye2pcg66234je3u"
        BLUNA_CONTRACT = "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp"
        ASTROPORT_ROUTER = "terra16t7dpwwgx9n3lq6l6te3753lsjqwhxwpday9zx"
    elif config_dic['network'] == "TESTNET":
        CHAIN_ID = "bombay-12"
        CHAIN_URL = "https://bombay-lcd.terra.dev"
        ANC_LIQ_QUE_CONTRACT = "terra18j0wd0f62afcugw2rx5y8e6j5qjxd7d6qsc87r"
        BLUNA_CONTRACT = "terra1u0t35drzyy0mujj8rkdyzhe264uls4ug3wdp3x"
        ASTROPORT_ROUTER = "terra13wf295fj9u209nknz2cgqmmna7ry3d3j5kv7t4"
    else:
        CHAIN_ID = "localterra"

    terra = LCDClient(
        chain_id=CHAIN_ID, url=CHAIN_URL,
        gas_prices={ 'uluna': live_gas_prices['uluna'] },
        gas_adjustment="1.5")
    test1 = terra.wallet(MnemonicKey(mnemonic=config_dic['seed']))
    contract_file = open("./artifacts/liquidproof.wasm", "rb")
    file_bytes = base64.b64encode(contract_file.read()).decode()
    store_code = MsgStoreCode(test1.key.acc_address, file_bytes)
    store_code_tx = test1.create_and_sign_tx(CreateTxOptions(msgs=[store_code], fee=Fee(3100000, "60000uluna")))
    store_code_tx_result = terra.tx.broadcast(store_code_tx)
    print(store_code_tx_result)

    code_id = store_code_tx_result.logs[0].events_by_type["store_code"]["code_id"][0]

    with open('code_id.json', 'w') as f:
        json.dump(code_id, f)
