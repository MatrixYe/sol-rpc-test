/// @Name methods
///
/// @Date 2024/8/13 下午3:15
///
/// @Author Matrix.Ye
///
/// @Description:
///
///
///

use serde_json::Value;
use strum_macros::EnumIter;

pub trait JsonRpcInterface {
    fn method_name(&self) -> &str;
    fn request_body(&self) -> Value;
}
#[derive(Debug, EnumIter)]
pub enum RpcMethod {
    GetAccountInfo,
    GetBalance,
    GetSlot,
    GetBlock,
    GetBlockCommitment,
    GetBlockHeight,
    GetBlockProduction,
    GetBlockTime,
    GetBlocks,
    GetBlocksWithLimit,
    GetClusterNodes,
    GetEpochInfo,
    GetEpochSchedule,
    GetFeeForMessage,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
    GetIdentity,
    GetInflationGovernor,
    GetInflationRate,
    GetInflationReward,
    GetLargestAccounts,
    GetLeaderSchedule,
    GetMaxRetransmitSlot,
    GetMaxShredInsertSlot,
    GetMinimumBalanceForRentExemption,
    GetMultipleAccounts,
    GetProgramAccounts,
    GetRecentPerformanceSamples,
    GetRecentPrioritizationFees,
    GetSignatureStatuses,
    GetSignaturesForAddress,
    GetSlotLeader,
    GetSlotLeaders,
    GetStakeActivation,
    GetStakeMinimumDelegation,
    GetSupply,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenLargestAccounts,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    IsBlockhashValid,
    MinimumLedgerSlot,
}

impl JsonRpcInterface for RpcMethod {
    fn method_name(&self) -> &str {
        match self {
            RpcMethod::GetAccountInfo => "getAccountInfo",
            RpcMethod::GetBalance => "getBalance",
            RpcMethod::GetBlock => "getBlock",
            RpcMethod::GetSlot => "getSlot",
            RpcMethod::GetBlockCommitment => "getBlockCommitment",
            RpcMethod::GetBlockHeight => "getBlockHeight",
            RpcMethod::GetBlockProduction => "getBlockProduction",
            RpcMethod::GetBlockTime => "getBlockTime",
            RpcMethod::GetBlocks => "getBlocks",
            RpcMethod::GetBlocksWithLimit => "getBlocksWithLimit",
            RpcMethod::GetClusterNodes => "getClusterNodes",
            RpcMethod::GetEpochInfo => "getEpochInfo",
            RpcMethod::GetEpochSchedule => "getEpochSchedule",
            RpcMethod::GetFeeForMessage => "getFeeForMessage",
            RpcMethod::GetFirstAvailableBlock => "getFirstAvailableBlock",
            RpcMethod::GetGenesisHash => "getGenesisHash",
            RpcMethod::GetHealth => "getHealth",
            RpcMethod::GetIdentity => "getIdentity",
            RpcMethod::GetInflationGovernor => "getInflationGovernor",
            RpcMethod::GetInflationRate => "getInflationRate",
            RpcMethod::GetInflationReward => "getInflationReward",
            RpcMethod::GetLargestAccounts => "getLargestAccounts",
            RpcMethod::GetLeaderSchedule => "getLeaderSchedule",
            RpcMethod::GetMaxRetransmitSlot => "getMaxRetransmitSlot",
            RpcMethod::GetMaxShredInsertSlot => "getMaxShredInsertSlot",
            RpcMethod::GetMinimumBalanceForRentExemption => "getMinimumBalanceForRentExemption",
            RpcMethod::GetMultipleAccounts => "getMultipleAccounts",
            RpcMethod::GetProgramAccounts => "getProgramAccounts",
            RpcMethod::GetRecentPerformanceSamples => "getRecentPerformanceSamples",
            RpcMethod::GetRecentPrioritizationFees => "getRecentPrioritizationFees",
            RpcMethod::GetSignatureStatuses => "getSignatureStatuses",
            RpcMethod::GetSignaturesForAddress => "getSignaturesForAddress",
            RpcMethod::GetSlotLeader => "getSlotLeader",
            RpcMethod::GetSlotLeaders => "getSlotLeaders",
            RpcMethod::GetStakeActivation => "getStakeActivation",
            RpcMethod::GetStakeMinimumDelegation => "getStakeMinimumDelegation",
            RpcMethod::GetSupply => "getSupply",
            RpcMethod::GetTokenAccountBalance => "getTokenAccountBalance",
            RpcMethod::GetTokenAccountsByDelegate => "getTokenAccountsByDelegate",
            RpcMethod::GetTokenAccountsByOwner => "getTokenAccountsByOwner",
            RpcMethod::GetTokenLargestAccounts => "getTokenLargestAccounts",
            RpcMethod::GetTokenSupply => "getTokenSupply",
            RpcMethod::GetTransaction => "getTransaction",
            RpcMethod::GetTransactionCount => "getTransactionCount",
            RpcMethod::GetVersion => "getVersion",
            RpcMethod::GetVoteAccounts => "getVoteAccounts",
            RpcMethod::IsBlockhashValid => "isBlockhashValid",
            RpcMethod::MinimumLedgerSlot => "minimumLedgerSlot",
        }
    }

    fn request_body(&self) -> Value {
        match self {
            RpcMethod::GetAccountInfo => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
                    { "encoding": "base58" }
                ]
            }),
            RpcMethod::GetBalance => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri"
                ]
            }),
            RpcMethod::GetBlock => serde_json::json!({
                "jsonrpc": "2.0","id":1,
                "method":"getBlock",
                "params": [
                    430,
                {
                    "encoding": "json",
                    "maxSupportedTransactionVersion":0,
                    "transactionDetails":"full",
                    "rewards":false
                }
                ]
            }),
            RpcMethod::GetSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetBlockCommitment => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [5]
            }),
            RpcMethod::GetBlockHeight => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetBlockProduction => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetBlockTime => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [5]
            }),
            RpcMethod::GetBlocks => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [5, 10]
            }),
            RpcMethod::GetBlocksWithLimit => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [5, 3]
            }),
            RpcMethod::GetClusterNodes => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetEpochInfo => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetEpochSchedule => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetFeeForMessage => serde_json::json!({
                "id": 1,
                "jsonrpc": "2.0",
                "method": self.method_name(),
                "params": [
                    "AQABAgIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEBAQAA",
                    {
                        "commitment": "processed"
                    }
                ]
            }),
            RpcMethod::GetFirstAvailableBlock => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetGenesisHash => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetHealth => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetIdentity => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetInflationGovernor => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetInflationRate => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetInflationReward => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    ["6dmNQ5jwLeLk5REvio1JcMshcbvkYMwy26sJ8pbkvStu","BGsqMegLpV6n6Ve146sSX2dTjUMj3M92HnU8BbNRMhF2"],
                    {"epoch": 2}
                ]
            }),
            RpcMethod::GetLargestAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetLeaderSchedule => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    null,
                    {"identity": "4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F"}
                ]
            }),
            RpcMethod::GetMaxRetransmitSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetMaxShredInsertSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetMinimumBalanceForRentExemption => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [50]
            }),
            RpcMethod::GetMultipleAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    [
                        "vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",
                        "4fYNw3dojWmQ4dXtSGE9epjRGy9pFSx62YypT7avPYvA"
                    ],
                    {"encoding": "base58"}
                ]
            }),
            RpcMethod::GetProgramAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
                    {
                    "filters": [
                            {"dataSize": 17},
                            {"memcmp": {"offset": 4,"bytes": "3Mc6vR"}}
                        ]
                    }
                ]
            }),
            RpcMethod::GetRecentPerformanceSamples => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [4]
            }),
            RpcMethod::GetRecentPrioritizationFees => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    ["CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"]
                ]
            }),
            RpcMethod::GetSignatureStatuses => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    [
                        "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"
                    ],
                    {
                        "searchTransactionHistory": true
                    }
                ]
            }),
            RpcMethod::GetSignaturesForAddress => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "Vote111111111111111111111111111111111111111",
                    {
                    "limit": 1
                    }
                ]
            }),
            RpcMethod::GetSlotLeader => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetSlotLeaders => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [100, 10]
            }),
            RpcMethod::GetStakeActivation => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "CYRJWqiSjLitBAcRxPvWpgX3s5TvmN2SuRY3eEYypFvT",
                    {
                    "epoch": 4
                    }
                ]
            }),
            RpcMethod::GetStakeMinimumDelegation => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetSupply => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetTokenAccountBalance => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "7fUAJdStEuGbc3sM84cKRL6yYaaSstyLSU4ve5oovLS7"
                ]
            }),
            RpcMethod::GetTokenAccountsByDelegate => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",
                    {
                        "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                    },
                    {
                        "encoding": "jsonParsed"
                    }
                ]
            }),
            RpcMethod::GetTokenAccountsByOwner => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                        "4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F",
                    {
                        "mint": "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E"
                    },
                    {
                        "encoding": "jsonParsed"
                    }
                ]
            }),
            RpcMethod::GetTokenLargestAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E"
                ]
            }),
            RpcMethod::GetTokenSupply => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "3wyAj7Rt1TWVPZVteFJPLa26JmLvdb1CAKEFZm3NY75E"
                ]
            }),
            RpcMethod::GetTransaction => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "2nBhEBYYvfaAe16UMNqRHre4YNSskvuYgx3M6E4JP1oDYvZEJHvoPzyUidNgNX5r9sTyN1J9UxtbCXy2rqYcuyuv",
                "json"
                ]
            }),
            RpcMethod::GetTransactionCount => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetVersion => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            RpcMethod::GetVoteAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    {
                    "votePubkey": "3ZT31jkAGhUaw8jsy4bTknwBMP8i4Eueh52By4zXcsVw"
                    }
                ]
            }),
            RpcMethod::IsBlockhashValid => serde_json::json!({
                "id": 45,
                "jsonrpc": "2.0",
                "method": self.method_name(),
                "params": [
                    "J7rBdM6AecPDEZp8aPq5iPSNKVkU5Q76F3oAV4eW5wsW",
                    {"commitment": "processed"}
                ]
            }),
            RpcMethod::MinimumLedgerSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
        }
    }
}
