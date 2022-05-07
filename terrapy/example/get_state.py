import asyncio
import time
from terra_sdk.client.lcd import AsyncLCDClient
from terra_sdk.core.wasm import MsgExecuteContract
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.client.lcd.api.tx import CreateTxOptions
from terra_sdk.client.lcd import LCDClient

"""
uusd
```
contract_address
terra1kk96quk4mz39zprfgmp77tzq7h7u45dm77mq7t
aterra
terra1sccezpctw4y3596cd73ezd2zm9s24sad0cymcm
```
uluna
```
contract_address
terra1277yy06avcepnsr9uzsrjgn5n626lhurna9w9k
aterra
terra1edldvy2gz9pje584vvpz5y5at5k24ec3wx4kpc
```
"""
contract_address = "terra1lnlffms8sm02szrqzd9zeh6q3rg7uvc5m5p2z0"
atoken_address = "terra1sccezpctw4y3596cd73ezd2zm9s24sad0cymcm"


async def get_config(terra: AsyncLCDClient):
    resp = await terra.wasm.contract_query(
        contract_address=contract_address,
        query={"config": {}},
    )
    print('get_config')
    print(resp)


async def get_atokens(terra: AsyncLCDClient):
    resp = await terra.wasm.contract_query(
        contract_address=atoken_address,
        query={"balance": {"address": "terra1sagh7hjz89mwrvg54s068rvl2wtlcj7thdazcw"}},
    )
    print('get_atokens')
    print(resp)


async def get_state(terra: AsyncLCDClient):
    resp = await terra.wasm.contract_query(
        contract_address=contract_address,
        query={"state": {}},
    )
    print('get_state')
    print(resp)


async def get_depositor(terra: AsyncLCDClient):
    resp = await terra.wasm.contract_query(
        contract_address=contract_address,
        query={
            "ident": {
                "address": "terra1sagh7hjz89mwrvg54s068rvl2wtlcj7thdazcw",
                "epoch": int(time.time()),
            }
        },
    )
    print('get_depositor')
    print(resp)


async def get_tvl(terra: AsyncLCDClient):
    print('get_tvl')
    print("---------")
    resp = await terra.wasm.contract_query(
        contract_address=contract_address,
        query={"tvl": {"indice": -1}},
    )
    print(resp)
    for i in range(0, 6):
        print("---------")
        resp = await terra.wasm.contract_query(
            contract_address=contract_address,
            query={"tvl": {"indice": i}},
        )

        print(i, resp)
    print("---------")


def exec_DepositStable(terra):
    executeMsg = {
        "deposit": {}
    }
    wallet = terra.wallet(MnemonicKey(mnemonic="puzzle figure hub weekend expose sauce distance clutch talent moon worry robust fortune layer arrow raccoon way pause concert race flip extend pepper flee"))
    account_address = wallet.key.acc_address

    msg = MsgExecuteContract(account_address, contract_address, execute_msg=executeMsg, coins={"uusd": 5 * 1000000})

    executeTx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[msg]))
    executeTxResult = terra.tx.broadcast(executeTx)
    print("submitBidHash:")
    print(executeTxResult.txhash)
    return executeTxResult.txhash


async def main():
    if 1:
      terra = LCDClient(chain_id="bombay-12", url="https://bombay-lcd.terra.dev")
      exec_DepositStable(terra)
    else:
      terra = AsyncLCDClient("https://bombay-lcd.terra.dev", "bombay-12")
      await get_config(terra)
      await get_state(terra)
      await get_tvl(terra)
      await get_depositor(terra)
      await get_atokens(terra)
    await terra.session.close()  # you must close the session


if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(main())

