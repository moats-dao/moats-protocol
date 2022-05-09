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

    def placeBid(self, premium, bid_count):
        executeMsg = {
            "submit_bid": {
                "collateral_token": BLUNA_CONTRACT,
                "premium_slot": premium
            }
        }
        msg = MsgExecuteContract(self.account_address, self.contract_address, execute_msg=executeMsg,
                                 coins={"uusd": bid_count})

        executeTx = self.wallet.create_and_sign_tx(CreateTxOptions(msgs=[msg], memo="place bid"))
        executeTxResult = self.terra.tx.broadcast(executeTx)
        print("submitBidHash:")
        print(executeTxResult.txhash)
        return executeTxResult.txhash

    def getTxID(self, hash):
        reqURL = "https://lcd.terra.dev/txs"
        txInfo = requests.get(reqURL+'/'+hash)
        txInfoJSON = txInfo.json()
        if txInfoJSON["logs"][0]["events"][3]["attributes"][2]["key"] == "bid_idx":
            return txInfoJSON["logs"][0]["events"][3]["attributes"][2]["value"]

    def getBidInfo(self, ID):
        ID = str(ID)
        bidInfo = self.terra.wasm.contract_query(self.contract_address, {"get_bid_info": {"bid_idx": ID}})
        return bidInfo

    def getBidsByUser(self):
        msg = {
            "get_bids_by_user": {
                "collateral_token": BLUNA_CONTRACT,
                "bidder": self.contract_address,
                "start_after": "123",
                "limit": 30
            }
        }
        bidsByUser = self.terra.wasm.contract_query(self.contract_address, msg)
        IDs = []
        for bid in bidsByUser["bids"]:
            IDs.append(bid["idx"])
        return IDs

    def getTokenInfo(self, bidInfo):
        contract = bidInfo["collateral_token"]
        contractInfo = self.terra.wasm.contract_info(contract)
        return contractInfo["init_msg"]["symbol"], contract

    def activateBid(self, ID, adress):
        executeMsg = {
            "activate_bids": {
                "collateral_token": adress,
                "bids_idx": [ID]
            }
        }
        msg = MsgExecuteContract(self.account_address, self.contract_address, execute_msg=executeMsg)
        print("attempting to activate bid")
        executeTx = self.wallet.create_and_sign_tx(CreateTxOptions(msgs=[msg], memo="activate bid"))
        executeTxResult = self.terra.tx.broadcast(executeTx)
        print("activated bid, txhash:")
        print(executeTxResult.txhash)

    def claimLiq(self, symbolAdress):
        executeMsg = {
            "claim_liquidations": {
                "collateral_token": symbolAdress
            }
        }
        print(f"attempting to claim/withdraw bids")
        msg = MsgExecuteContract(self.account_address, self.contract_address, execute_msg=executeMsg)
        executeTx = self.wallet.create_and_sign_tx(CreateTxOptions(msgs=[msg]))
        executeTxResult = self.terra.tx.broadcast(executeTx)
        print(f"withdrawal tx hash: {executeTxResult.txhash}")

if __name__ == "__main__":
    m_liquidation_module = liquidation_module()
    premium = 1

    # 엥커 bLuna 리퀴데이션 큐에 Submit Bid on
    # Submit Bid on 이상 없으면 Activate Bids on
    USTBalance = m_liquidation_module.get_UST()
    if USTBalance > 5 * MILLION:
        print("placing bid")
        m_liquidation_module.placeBid(premium, 5 * MILLION)

    # 지켜보고 있다가 bLuna가 해당 프리미엄에 받아지면 Claim
    while True:
        currentBids = m_liquidation_module.getBidsByUser()
        for bid in currentBids:
            currentBidInfo = m_liquidation_module.getBidInfo(bid)
            currentBidToken, currentBidTokenAdress = m_liquidation_module.getTokenInfo(currentBidInfo)

            if currentBidInfo["wait_end"] is None:
                print(f"bid {bid} is active")
                # if there is collateral to be withdrawn
                if int(currentBidInfo["pending_liquidated_collateral"]) > 10000:
                    print(f"withdrawal of {float(currentBidInfo['pending_liquidated_collateral']) / MILLION} {currentBidToken} pending")
                    m_liquidation_module.claimLiq(currentBidTokenAdress)
                    break
                else:
                    print(f"waiting to be filled {str(round(int(currentBidInfo['amount']) / MILLION, 2))} USD remaining in the {currentBidInfo['premium_slot']} % pool")
            elif datetime.utcfromtimestamp(currentBidInfo["wait_end"] + 30) < datetime.utcnow():
                print("ready to activate")
                m_liquidation_module.activateBid(bid, currentBidTokenAdress)
            else:
                print(f"not ready, wait_end: {datetime.utcfromtimestamp(currentBidInfo['wait_end'])} UTC, current time {datetime.utcnow()}")
        sleep(5)
