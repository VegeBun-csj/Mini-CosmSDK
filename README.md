# Mini-CosmSDK

This is minimal cosmos sdk implemented by rust
> The final goal is to implement the full function of cosmos sdk

## Architecture
* mini-cosm: Start command cli

* modules: Base app and some cosmos modules


> Mini-CosmSDK uses the [tendermint-abci](https://crates.io/crates/tendermint-abci) crate to communicate with a consensus instance which runs as a separate process. Tendermint must be installed and run separately (see instructions below).

## Requirements

- **Rust compiler**

The minimum supported Rust version is 1.67.1. Follow the [installation instructions](https://doc.rust-lang.org/book/ch01-01-installation.html).

- **Tendermint**

we uses Tendermint version v0.34.21. After cloning the [Tendermint repo](https://github.com/tendermint/tendermint) checkout v0.34.21 then follow the [installation instructions](https://github.com/tendermint/tendermint/blob/v0.34.21/docs/introduction/install.md).


## Running a local chain

1. Clone this repo:

```console
git clone https://github.com/VegeBun-csj/Mini-CosmSDK.git
cd Mini-CosmSDK
```

2. Initialize a new chain:

```console
make init
```

3. Build and start the application:

```console
make run
```

The application will listen for connections on `tcp://127.0.0.1:26658`.

4. From a different terminal window start Tendermint:
> you should have installed the tendermint and set it as the global command.

```console
make tendermint-start
```

Tendermint will connect to the application and bind it's RPC server to `127.0.0.1:26657`.

The chain (consisting of one node) is now up and running.


## Querying the chain

> use mini-cosm as a client
1. Install mini-cosm:

```console
cargo run --release
```

> set the mini-cosm as the global command.

2. Query a balance:

```console
mini-cosm query bank balances cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux
```

Which returns:

```json
{
  "balances": [
    {
      "denom": "uatom",
      "amount": "34"
    }
  ],
  "pagination": null
}
```

The balance of this address was set to 34 in the genesis file.

3. Import the key corresponding to the above address into the mini-cosm key store:

```console
mini-cosm keys add alice
```
and import with `race draft rival universe maid cheese steel logic crowd fork comic easy truth drift tomorrow eye buddy head time cash swing swift midnight borrow`.

4. Send tokens:

```console
mini-cosm tx bank send alice cosmos180tr8wmsk8ugt32yynj8efqwg3yglmpwp22rut 10uatom --fee 1uatom
```

5. Query the address balance and observe that it has decreased by 11uatom which is the sum of the amount transferred and the fee:

```console
mini-cosm query bank balances cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux
```

Which returns:

```json
{
  "balances": [
    {
      "denom": "uatom",
      "amount": "23"
    }
  ],
  "pagination": null
}

```