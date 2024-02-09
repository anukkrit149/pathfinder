use p2p_proto::receipt::{
    DeclareTransactionReceipt, DeployAccountTransactionReceipt, DeployTransactionReceipt,
    InvokeTransactionReceipt, L1HandlerTransactionReceipt,
};
use pathfinder_common::{
    receipt::{BuiltinCounters, ExecutionResources, L2ToL1Message},
    ContractAddress, EthereumAddress, Fee, L2ToL1MessagePayloadElem, TransactionHash,
};

/// Represents a simplified receipt (events and execution status excluded).
///
/// This type is not in the `p2p` to avoid `p2p` dependence on `starknet_gateway_types`.
#[derive(Clone, Debug, PartialEq)]
pub struct Receipt {
    pub transaction_hash: TransactionHash,
    pub actual_fee: Fee,
    pub execution_resources: ExecutionResources,
    pub l2_to_l1_messages: Vec<L2ToL1Message>,
    // Empty means not reverted
    pub revert_error: String,
}

impl TryFrom<p2p_proto::receipt::Receipt> for Receipt {
    type Error = anyhow::Error;

    fn try_from(proto: p2p_proto::receipt::Receipt) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        use p2p_proto::receipt::Receipt::{Declare, Deploy, DeployAccount, Invoke, L1Handler};

        match proto {
            Invoke(InvokeTransactionReceipt { common })
            | Declare(DeclareTransactionReceipt { common })
            | L1Handler(L1HandlerTransactionReceipt { common, .. })
            | Deploy(DeployTransactionReceipt { common, .. })
            | DeployAccount(DeployAccountTransactionReceipt { common, .. }) => Ok(Self {
                transaction_hash: TransactionHash(common.transaction_hash.0),
                actual_fee: Fee(common.actual_fee),
                execution_resources: ExecutionResources {
                    builtin_instance_counter: BuiltinCounters {
                        output_builtin: common.execution_resources.builtins.output.into(),
                        pedersen_builtin: common.execution_resources.builtins.pedersen.into(),
                        range_check_builtin: common.execution_resources.builtins.range_check.into(),
                        ecdsa_builtin: common.execution_resources.builtins.ecdsa.into(),
                        bitwise_builtin: common.execution_resources.builtins.bitwise.into(),
                        ec_op_builtin: common.execution_resources.builtins.ec_op.into(),
                        keccak_builtin: common.execution_resources.builtins.keccak.into(),
                        poseidon_builtin: common.execution_resources.builtins.poseidon.into(),
                        segment_arena_builtin: common
                            .execution_resources
                            .builtins
                            .segment_arena
                            .into(),
                    },
                    n_steps: common.execution_resources.steps.into(),
                    n_memory_holes: common.execution_resources.memory_holes.into(),
                },
                l2_to_l1_messages: common
                    .messages_sent
                    .into_iter()
                    .map(|x| L2ToL1Message {
                        from_address: ContractAddress(x.from_address),
                        payload: x
                            .payload
                            .into_iter()
                            .map(L2ToL1MessagePayloadElem)
                            .collect(),
                        to_address: EthereumAddress(x.to_address.0),
                    })
                    .collect(),
                revert_error: common.revert_reason,
            }),
        }
    }
}
