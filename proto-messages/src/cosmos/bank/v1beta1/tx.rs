use ibc_proto::{
    cosmos::bank::v1beta1::MsgSend as RawMsgSend, cosmos::base::v1beta1::Coin as RawCoin,
    google::protobuf::Any, protobuf::Protobuf,
};
use proto_types::AccAddress;
use serde::{Deserialize, Serialize};

use crate::{
    cosmos::base::v1beta1::{Coin, SendCoins},
    error::Error,
};

/// MsgSend represents a message to send coins from one account to another.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgSend {
    pub from_address: AccAddress,
    pub to_address: AccAddress,
    pub amount: SendCoins,
}

impl TryFrom<RawMsgSend> for MsgSend {
    type Error = Error;

    fn try_from(raw: RawMsgSend) -> Result<Self, Self::Error> {
        let from_address = AccAddress::from_bech32(&raw.from_address)
            .map_err(|e| Error::DecodeAddress(e.to_string()))?;

        let to_address = AccAddress::from_bech32(&raw.to_address)
            .map_err(|e| Error::DecodeAddress(e.to_string()))?;

        let coins: Result<Vec<Coin>, Error> = raw
            .amount
            .into_iter()
            .map(|coin| Coin::try_from(coin))
            .collect();

        Ok(MsgSend {
            from_address: from_address,
            to_address: to_address,
            amount: SendCoins::new(coins?)?,
        })
    }
}

impl From<MsgSend> for RawMsgSend {
    fn from(msg: MsgSend) -> RawMsgSend {
        let coins: Vec<Coin> = msg.amount.into();
        let coins = coins.into_iter().map(|coin| RawCoin::from(coin)).collect();

        RawMsgSend {
            from_address: msg.from_address.into(),
            to_address: msg.to_address.into(),
            amount: coins,
        }
    }
}

impl Protobuf<RawMsgSend> for MsgSend {}

//TODO: should to Any be implemented at the individual message type?
impl From<MsgSend> for Any {
    fn from(msg: MsgSend) -> Self {
        Any {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            value: msg.encode_vec(),
        }
    }
}
