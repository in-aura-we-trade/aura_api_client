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
            SwapFailure::AttemptsExhausted => {
                "Config was turned off after 100 consecutive matches produced no successful action."
            }
            SwapFailure::DlmmTwoHopArrayLimit => {
                "Insufficient DLMM liquidity within the one-bin-array limit for this two-hop swap. More bin-array accounts exceed the configured account budget. Reduce the swap amount."
            }
            SwapFailure::UnfavorableSolQuoteRate => {
                "Swap aborted: SOL/WSOL would buy over 5% less custom quote than an equivalent USD budget, and this wallet has insufficient USD for the better route. Add the USD token used by that route (USDC, USDT or USD1), or acquire the custom quote first."
            }
            SwapFailure::UnfavorableUsdQuoteRate => {
                "Swap aborted: USD (USDC, USDT or USD1) would buy over 5% less custom quote than an equivalent SOL/WSOL budget, and this wallet has insufficient spendable SOL/WSOL for the better route. Add SOL/WSOL for that route, or acquire the custom quote first."
            }
            SwapFailure::NoKnownQuoteRoute => {
                "Swap aborted: no eligible sell route to SOL/WSOL, USDC, USDT or USD1 is available within two swaps."
            }
            SwapFailure::ClmmRouteUnavailable => {
                "Swap aborted: a complete CLMM tick-array route is unavailable for this amount."
            }
            SwapFailure::ClmmTwoHopArrayLimit => {
                "Insufficient CLMM liquidity within the one-tick-array limit for this two-hop swap. Reduce the swap amount."
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
            LimitOrderFailure::DlmmTwoHopArrayLimit => {
                "Order was deleted because the two-hop swap has insufficient DLMM liquidity within its one-bin-array account limit. Reduce the swap amount."
            }
            LimitOrderFailure::UnfavorableSolQuoteRate => {
                "Order was deleted because SOL/WSOL would buy over 5% less custom quote than an equivalent USD budget, and this wallet has insufficient USD for the better route. Add the USD token used by that route (USDC, USDT or USD1), or acquire the custom quote first."
            }
            LimitOrderFailure::UnfavorableUsdQuoteRate => {
                "Order was deleted because USD (USDC, USDT or USD1) would buy over 5% less custom quote than an equivalent SOL/WSOL budget, and this wallet has insufficient spendable SOL/WSOL for the better route. Add SOL/WSOL for that route, or acquire the custom quote first."
            }
            LimitOrderFailure::NoKnownQuoteRoute => {
                "Sell could not execute: no eligible route to SOL/WSOL, USDC, USDT or USD1 is available within two swaps."
            }
            LimitOrderFailure::ClmmRouteUnavailable => {
                "Order could not execute: a complete CLMM tick-array route is unavailable for this amount."
            }
            LimitOrderFailure::ClmmTwoHopArrayLimit => {
                "Order could not execute: insufficient CLMM liquidity within the one-tick-array limit for this two-hop swap. Reduce the swap amount."
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
            UserActionError::NonceRecovery { wallet, message } => {
                write!(f, "Durable nonce setup\n{message}\n\nWallet: {wallet}")
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
