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
#[derive(Debug, Clone, EnumIter)]
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
            // 获取账户信息
            RpcMethod::GetAccountInfo => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                    { "encoding": "base58" }
                ]
            }),
            // 获取余额
            RpcMethod::GetBalance => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "EeCugDyYWgGhNNcqGsQbXpzKyoRowLvqP9rD4h1aGkKS"
                ]
            }),
            // 获取block
            RpcMethod::GetBlock => serde_json::json!({
                "jsonrpc": "2.0","id":1,
                "method":"getBlock",
                "params": [
                    283504337,
                {
                    "encoding": "json",
                    "maxSupportedTransactionVersion":0,
                    "transactionDetails":"full",
                    "rewards":false
                }
                ]
            }),
            // 获取时间槽
            RpcMethod::GetSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            //获取block的commitment
            RpcMethod::GetBlockCommitment => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [283505598]
            }),
            // 获取区块高度
            RpcMethod::GetBlockHeight => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            // 返回区块插槽的信息
            RpcMethod::GetBlockProduction => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
            // 获取区块时间
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
                "params": [283505598, 283505600]
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
                    ["5Cchr1XGEg7dbBXByV5NY2ad8jfxAM7HA3x8D56rq9Ux"],
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
                    {"identity": "5Cchr1XGEg7dbBXByV5NY2ad8jfxAM7HA3x8D56rq9Ux"}
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
                        "42MK9k2JsTiZdPJcgXBMLVt879UrbsFhbZ66kPqgQ8uB",
                        "GBfXyCU338aKXhQ2oHuvxcy7f6ufoJUPwivfnNSKAqyn"
                    ],
                    {"encoding": "base58"}
                ]
            }),
            // 获取owner是某程序账户的所有账户的pubkey
            RpcMethod::GetProgramAccounts => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                    {
                    "filters": [
                            {"dataSize": 17},
                            {"memcmp": {"offset": 4,"bytes": "3Mc6vR"}}
                        ]
                    }
                ]
            }),
            //
            RpcMethod::GetRecentPerformanceSamples => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [2]
            }),
            RpcMethod::GetRecentPrioritizationFees => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    ["GBfXyCU338aKXhQ2oHuvxcy7f6ufoJUPwivfnNSKAqyn"]
                ]
            }),
            RpcMethod::GetSignatureStatuses => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    [
                        "2z9r1fgjouat7ni2yK2nrB7eBvKnp1bYF67JWNCzs8J8esHTjEa5bFpWKcrioQ3QDVtejginQz12FdVBsPGBXPk2"
                    ],
                    {
                        "searchTransactionHistory": true
                    }
                ]
            }),
            // 通过账户地址，获取交易签名
            RpcMethod::GetSignaturesForAddress => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "Vote111111111111111111111111111111111111111",
                    {
                    "limit": 5
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
                "params": [283512635, 5]
            }),
            RpcMethod::GetStakeActivation => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "BoxEw8gJMxYxNBB5ZAmGLVUwMLStmvv7irJ2r9yaU35j",
                    {
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
                    "6EmF98D4QrQmmXHBgEwmwRA9v7GWsdFVUokQv2SmwPWJ"
                ]
            }),
            RpcMethod::GetTokenAccountsByDelegate => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                    {
                        "mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
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
                        "EeCugDyYWgGhNNcqGsQbXpzKyoRowLvqP9rD4h1aGkKS",
                    {
                        "mint": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
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
                    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
                ]
            }),
            RpcMethod::GetTokenSupply => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
                ]
            }),
            RpcMethod::GetTransaction => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name(),
                "params": [
                    "5wiSqND8Ec6gKCM2deq1B1VE5vo3bV1tEJYy77a9WawcgVcozcoQQ318sy1PnevWo4FoXVD1t8aQznzZb6W71AEB",
                    {
                        "maxSupportedTransactionVersion":0,
                        "encoding":"jsonParsed"
                    }
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
                    "votePubkey": "GHViLgbrJdZDPb6sphRbeuPNM9cmjsFuGWzrTF1sKF5n"
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
            // 返回最小时间槽在当前账本中
            RpcMethod::MinimumLedgerSlot => serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": self.method_name()
            }),
        }
    }
}
