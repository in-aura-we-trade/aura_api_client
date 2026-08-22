use core::fmt::Display;

use crate::types::LimitOrderFailure;
use crate::types::SwapFailure;
use crate::types::UserActionError;

impl From<&SwapFailure> for &'static str {
    fn from(reason: &SwapFailure) -> Self {
        match reason {
            SwapFailure::DurableNonceUnavailable => {
                "No durable nonce is available. Open a D.Nonce account or select a nonce strategy with fallback."
            }
            SwapFailure::InsufficientBalance => {
                "Selected wallet has insufficient balance for this swap."
            }
            SwapFailure::ZeroAmount => "Calculated swap amount is zero.",
            SwapFailure::PriceImpactExceeded => {
                "Swap exceeded the configured maximum price impact."
            }
            SwapFailure::WalletNotFound => "The configured wallet no longer exists.",
            SwapFailure::NoTransactionSignatures => {
                "All transaction processors returned no signature - increase your tip or choose Aura processor"
            }
            SwapFailure::NoEnabledEvents => {
                "Config was turned off because neither creation nor migration events are enabled."
            }
            SwapFailure::NoEnabledActions => {
                "Config was turned off because it has no enabled executable action."
            }
        }
    }
}

impl Display for SwapFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.into())
    }
}

impl From<&LimitOrderFailure> for &'static str {
    fn from(reason: &LimitOrderFailure) -> Self {
        match reason {
            LimitOrderFailure::DurableNonceUnavailable => {
                "Order was deleted because no durable nonce is available. Open a D.Nonce account or change its nonce strategy."
            }
            LimitOrderFailure::BuyPriceImpactOrSlippageExceeded => {
                "Buy order was deleted because the price moved beyond its configured slippage / maximum price-impact protection."
            }
            LimitOrderFailure::InsufficientBalance => {
                "Order was deleted because the selected wallet has insufficient balance."
            }
            LimitOrderFailure::ZeroAmount => {
                "Order was deleted because its calculated swap amount is zero."
            }
            LimitOrderFailure::NoPool => {
                "Timed order was deleted because no pool exists for the token."
            }
            LimitOrderFailure::NoActivePool => {
                "Timed order was deleted because the token has no active pool."
            }
            LimitOrderFailure::NoTransactionSignatures => {
                "Order was deleted because no transaction processor returned a signature - increase your tip or choose Aura processor"
            }
            LimitOrderFailure::TransactionTimedOut => {
                "Order was deleted after its transaction could not be confirmed before timeout."
            }
            LimitOrderFailure::TransactionFailed => {
                "Order was deleted after its transaction failed on-chain."
            }
            LimitOrderFailure::InvalidOrderOrWallet => {
                "Orders were not placed because at least one references an unknown wallet or order."
            }
            LimitOrderFailure::BatchRejected => {
                "Orders were not placed because active orders already exist or the account reached its order limit."
            }
        }
    }
}

impl Display for LimitOrderFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.into())
    }
}

impl Display for UserActionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UserActionError::Swap { mint, reason } => {
                write!(f, "Swap failed\n{reason}\n\nMint: {mint}")
            }
            UserActionError::LimitOrder {
                mint,
                order_id: _,
                reason,
            } => write!(f, "Limit order stopped\n{reason}\n\nMint: {mint}"),
            UserActionError::Snipe {
                mint,
                task_id: _,
                task_name,
                reason,
            } => write!(
                f,
                "Snipe failed\nConfig: {task_name}\n{reason}\n\nMint: {mint}"
            ),
            UserActionError::Copytrade {
                mint,
                cfg_id: _,
                config_name,
                reason,
            } => write!(
                f,
                "Copytrade failed\nConfig: {config_name}\n{reason}\n\nMint: {mint}"
            ),
        }
    }
}

impl From<&UserActionError> for String {
    fn from(error: &UserActionError) -> Self {
        error.to_string()
    }
}

impl From<UserActionError> for String {
    fn from(error: UserActionError) -> Self {
        error.to_string()
    }
}
