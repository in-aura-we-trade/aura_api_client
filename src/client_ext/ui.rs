use core::fmt::Display;

use crate::{
    client_ext::AmountKind,
    types::{SwapAmount, TxnNonce, UserNonceStrategy},
};

impl Display for TxnNonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxnNonce::Custom => f.write_str("Custom"),
            TxnNonce::Durable => f.write_str("Durable"),
            TxnNonce::None => f.write_str("None"),
        }
    }
}

impl UserNonceStrategy {
    pub fn next(&self) -> Self {
        match self {
            UserNonceStrategy::Durable => UserNonceStrategy::Hybrid,
            UserNonceStrategy::Hybrid => UserNonceStrategy::Custom,
            UserNonceStrategy::Custom => UserNonceStrategy::All,
            UserNonceStrategy::All => UserNonceStrategy::AllHybrid,
            UserNonceStrategy::AllHybrid => UserNonceStrategy::Durable,
        }
    }
}

impl SwapAmount {
    pub fn amount_kind(&self) -> AmountKind {
        match self {
            SwapAmount::Buy(_) => AmountKind::Fixed,
            SwapAmount::BuyPerc { amount: _ } => AmountKind::Perc,
            SwapAmount::SellPerc { amount: _ } => AmountKind::Perc,
            SwapAmount::SellOut(_) => AmountKind::Fixed,
            SwapAmount::SellInit => AmountKind::Perc,
        }
    }
}
