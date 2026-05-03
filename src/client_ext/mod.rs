use crate::types::{
    ConfirmTradeKind, CtTaskId, MarketExecuteMode, OrderId, SnipeTaskId, TokenTradeState,
    TradeStateUpdate, UserNonceStrategy,
};
use core::{fmt::Display, str::FromStr};

use decisol::SolanaLamports;
use solana_address::Address;
use solana_signature::Signature;

mod balances;
mod cu;
mod debug;
mod orders;
mod stats;
mod ui;

impl nohash_hasher::IsEnabled for CtTaskId {}
impl nohash_hasher::IsEnabled for SnipeTaskId {}
impl nohash_hasher::IsEnabled for OrderId {}

pub use orders::MarketOrdersDeduper;

#[expect(clippy::derivable_impls)]
impl Default for MarketExecuteMode {
    fn default() -> Self {
        MarketExecuteMode::Always
    }
}

#[derive(Clone, Debug)]
pub enum AmountKind {
    Perc,
    Fixed,
}

impl TradeStateUpdate {
    pub fn get_sigs(&self) -> impl Iterator<Item = &Signature> {
        match self {
            TradeStateUpdate::Landed {
                signature,
                state: _,
            } => std::slice::from_ref(signature).iter(),
            TradeStateUpdate::Lost { signatures } => signatures.iter(),
        }
    }
    pub fn is_landed(&self) -> bool {
        matches!(
            self,
            Self::Landed {
                signature: _,
                state: _
            }
        )
    }
}
impl ConfirmTradeKind {
    pub fn mint(&self) -> &Address {
        match self {
            ConfirmTradeKind::Snipe {
                task_id: _,
                task_name: _,
                txn_sig: _,
                mint,
                migration: _,
                trade_ty: _,
            } => mint,
            ConfirmTradeKind::LimitOrder { token, order: _ } => token,
            ConfirmTradeKind::Swap { token, trade_ty: _ } => token,
            ConfirmTradeKind::Migration { token, trade_ty: _ } => token,
            ConfirmTradeKind::Dev {
                token,
                trade_ty: _,
                tx_sig: _,
            } => token,
            ConfirmTradeKind::Copytrade {
                cfg_id: _,
                config_name: _,
                wallet: _,
                tx_sig: _,
                trade_data,
            } => &trade_data.mint,
            ConfirmTradeKind::Arb {
                info,
                signer: _,
                profit_check: _,
            } => &info.base,
        }
    }
}

pub trait UserCtxInterceptor: Send + Sync + 'static + Sized {
    type Payload;
    fn intercept<T>(
        payload: Self::Payload,
        req: &mut tonic::Request<T>,
    ) -> Result<(), tonic::Status>;
}

impl Display for SnipeTaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for SnipeTaskId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: i64 = s.parse().map_err(|_| ())?;
        let res: Self = SnipeTaskId(num);
        Ok(res)
    }
}

impl Display for CtTaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for CtTaskId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: i64 = s.parse().map_err(|_| ())?;
        let res: Self = Self(num);
        Ok(res)
    }
}

impl Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for OrderId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: i64 = s.parse().map_err(|_| ())?;
        let res: Self = Self(num);
        Ok(res)
    }
}

impl TokenTradeState {
    pub fn total_token_balance(&self) -> SolanaLamports {
        self.base
            .value(self.ata_balance.balance + self.pda_balance.balance)
    }
}

impl FromStr for UserNonceStrategy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: i32 = s.parse().map_err(|_| ())?;
        let res: Self = num.try_into().map_err(|_| ())?;
        Ok(res)
    }
}
impl Display for UserNonceStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: i32 = *self as i32;
        write!(f, "{num}")
    }
}

impl UserNonceStrategy {
    pub fn to_ui_buy(&self) -> &str {
        match self {
            UserNonceStrategy::Durable => "🅑Nonce|Durable",
            UserNonceStrategy::Hybrid => "🅑Nonce|Hybrid",
            UserNonceStrategy::Custom => "🅑Nonce|Custom",
            UserNonceStrategy::All => "🅑Nonce|All",
            UserNonceStrategy::AllHybrid => "🅑Nonce|AllHybrid",
        }
    }
    pub fn to_ui_sell(&self) -> &str {
        match self {
            UserNonceStrategy::Durable => "ⓈNonce|Durable",
            UserNonceStrategy::Hybrid => "ⓈNonce|Hybrid",
            UserNonceStrategy::Custom => "ⓈNonce|Custom",
            UserNonceStrategy::All => "ⓈNonce|All",
            UserNonceStrategy::AllHybrid => "ⓈNonce|AllHybrid",
        }
    }
}
