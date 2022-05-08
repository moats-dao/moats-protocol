from glob import glob
from pickle import NONE
from time import sleep
from terra_sdk.client.lcd import LCDClient
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.client.lcd.api.tx import CreateTxOptions
from terra_sdk.core.wasm import MsgExecuteContract
from terra_sdk.core.market import MsgSwap
from terra_sdk.core.coins import Coins
from terra_sdk.core.coins import Coin
from datetime import datetime
import requests
import json
import base64


MILLION = 1000000
CHAIN_ID = None
CHAIN_URL = None
ANC_LIQ_QUE_CONTRACT = None
BLUNA_CONTRACT = None
ASTROPORT_ROUTER = None

class liquidation_module:
    def __init__(self):
        global CHAIN_ID
        global ANC_LIQ_QUE_CONTRACT
        global BLUNA_CONTRACT
        global ASTROPORT_ROUTER

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

        with open('contract_address.json') as f:
            self.contract_address = json.load(f)

        self.terra = LCDClient(chain_id=CHAIN_ID, url=CHAIN_URL)

        seed = config_dic['seed']

        self.wallet = self.terra.wallet(MnemonicKey(mnemonic=seed))
        self.account_address = self.wallet.key.acc_address

    def get_UST(self):
        UST_balance = self.terra.wasm.contract_query(self.contract_address, {"get_ust_balance": { "account_addr": self.account_address }})
        return int(UST_balance)


if __name__ == "__main__":
    m_liquidation_module = liquidation_module()
    premium = 1

    # 엥커 bLuna 리퀴데이션 큐에 Submit Bid on
    # Submit Bid on 이상 없으면 Activate Bids on
    USTBalance = m_liquidation_module.get_UST()
    if USTBalance > 5 * MILLION:
        print("ok")
        #m_liquidation_module.placeBid(premium, 5 * MILLION)