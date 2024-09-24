use std::env;

use alloy_primitives::{address, Address, Bytes, B256};
use alloy_provider::{ext::DebugApi, ProviderBuilder};
use alloy_rpc_types::{
    trace::geth::{
        GethDebugTracerConfig, GethDebugTracingCallOptions, GethDebugTracingOptions,
        GethDefaultTracingOptions, GethTrace,
    },
    BlockId, TransactionRequest,
};
use alloy_sol_types::{sol, SolCall};
use dotenv::dotenv;
use eyre::Result;

// WETH token address
const WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
// The address of titan builder (just a random address to get the balance from)
const TITAN_BUILDER: Address = address!("4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97");

sol!(
    #[sol(rpc)]
    IErc20,
    "src/erc20_abi.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let provider = ProviderBuilder::new().on_http(
        env::var("HTTP_RPC")
            .expect("Missing HTTP_RPC environment variable")
            .parse()
            .expect("HTTP_RPC must be a valid HTTP RPC url"),
    );

    let debug_trace = provider
        .debug_trace_call(
            TransactionRequest::default().to(WETH).input(
                Bytes::from(
                    IErc20::balanceOfCall {
                        _owner: TITAN_BUILDER,
                    }
                    .abi_encode(),
                )
                .into(),
            ),
            BlockId::latest(),
            GethDebugTracingCallOptions {
                tracing_options: GethDebugTracingOptions {
                    config: GethDefaultTracingOptions {
                        enable_memory: Some(true),
                        disable_memory: None,
                        disable_stack: Some(false),
                        disable_storage: Some(false),
                        enable_return_data: Some(true),
                        disable_return_data: None,
                        debug: Some(true),
                        limit: Some(0),
                    },
                    tracer: None,
                    tracer_config: GethDebugTracerConfig::default(),
                    timeout: None,
                },
                state_overrides: None,
                block_overrides: None,
            },
        )
        .await?;

    if let GethTrace::Default(default_trace) = debug_trace {
        let mut ret: Option<B256> = None;
        for struct_log in default_trace.struct_logs {
            if !struct_log.storage.is_none() {
                let storage = struct_log.storage.unwrap();
                let mut keys = storage.keys();
                while let Some(key) = keys.next() {
                    ret = Some(*key);
                }
            }
        }

        if ret.is_none() {
            panic!("balanceOfCall didn't gave us any right storage slot for token {WETH:?} and wallet {TITAN_BUILDER:?}");
        } else {
            println!("Storage slot is: {}", ret.unwrap());
        }
    } else {
        panic!("Trace should be default, got {debug_trace:#?}");
    }

    Ok(())
}
