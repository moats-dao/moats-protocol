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
    with open('code_id.json') as f:
        code_id = json.load(f)

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

    instantiate = MsgInstantiateContract(
        sender=test1.key.acc_address,
        admin=test1.key.acc_address,
        code_id=int(code_id),
        init_msg={
            "admin": test1.key.acc_address,
            "anc_liq_que_contract": ANC_LIQ_QUE_CONTRACT,
            "bluna_contract": BLUNA_CONTRACT,
            "astroport_router": ASTROPORT_ROUTER,
        },    # InitMsg
        init_coins={}  # init coins
        #init_coins={"uluna": 1000000, "uusd": 1000000}  # init coins
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