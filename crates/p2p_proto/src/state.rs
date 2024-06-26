use std::fmt::Debug;

use fake::Dummy;
use pathfinder_crypto::Felt;

use crate::common::{Address, Iteration};
use crate::{proto, proto_field, ToProtobuf, TryFromProtobuf};

#[derive(Debug, Clone, PartialEq, Eq, ToProtobuf, TryFromProtobuf, Dummy)]
#[protobuf(name = "crate::proto::state::ContractStoredValue")]
pub struct ContractStoredValue {
    pub key: Felt,
    pub value: Felt,
}

#[derive(Debug, Clone, PartialEq, Eq, ToProtobuf, TryFromProtobuf, Dummy)]
#[protobuf(name = "crate::proto::state::ContractDiff")]
pub struct ContractDiff {
    pub address: Address,
    #[optional]
    pub nonce: Option<Felt>,
    #[optional]
    pub class_hash: Option<Felt>,
    #[optional]
    pub is_replaced: Option<bool>,
    pub values: Vec<ContractStoredValue>,
    pub domain: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ToProtobuf, TryFromProtobuf, Dummy)]
#[protobuf(name = "crate::proto::state::StateDiffsRequest")]
pub struct StateDiffsRequest {
    pub iteration: Iteration,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Dummy)]
pub enum StateDiffsResponse {
    ContractDiff(ContractDiff),
    #[default]
    Fin,
}

impl ToProtobuf<proto::state::StateDiffsResponse> for StateDiffsResponse {
    fn to_protobuf(self) -> proto::state::StateDiffsResponse {
        use proto::state::state_diffs_response::StateDiffMessage::{ContractDiff, Fin};
        proto::state::StateDiffsResponse {
            state_diff_message: Some(match self {
                Self::ContractDiff(contract_diff) => ContractDiff(contract_diff.to_protobuf()),
                Self::Fin => Fin(proto::common::Fin {}),
            }),
        }
    }
}

impl TryFromProtobuf<proto::state::StateDiffsResponse> for StateDiffsResponse {
    fn try_from_protobuf(
        input: proto::state::StateDiffsResponse,
        field_name: &'static str,
    ) -> Result<Self, std::io::Error> {
        use proto::state::state_diffs_response::StateDiffMessage::{ContractDiff, Fin};
        match proto_field(input.state_diff_message, field_name)? {
            ContractDiff(x) => {
                TryFromProtobuf::try_from_protobuf(x, field_name).map(Self::ContractDiff)
            }
            Fin(_) => Ok(Self::Fin),
        }
    }
}
