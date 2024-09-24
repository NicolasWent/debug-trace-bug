# Debug Trace Bug

When we make an ETH call to get the ERC20 balance of a token for a wallet, normally, there is a storage
slot that should be accessed.

This storage slot is useful to get because afterwards, we can for example modify this storage slot to set
a wallet balance.

## Project goal

The goal of this project is to reproduce a bug:
When I do debug-trace-call on an ERC20 balanceOf call, I actually get 0 storage slots read or written to
and even no SLOAD operations.

This shouldn't happen as normally when we do balanceOf we should get at least one read operation to read the
balance storage slot.

This is a minimal reproduction of this bug.

## How to run

- Create a .env file with an `HTTP_RPC` environment variable inside containing your http rpc.

> cargo build
> cd target/debug/
> ./debug-trace-bug

