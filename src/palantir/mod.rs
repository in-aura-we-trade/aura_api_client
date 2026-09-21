//! Palantir indexing API, generated from solana_palantir's Rust schema.
//! Enable `palantir` with `default-features = false` for an indexing-only client.
mod generated;

pub use generated::palantir as types;
pub use generated::palantir::*;
pub use generated::palantir_rpc;
pub use generated::palantir_rpc::palantir_rpc_client::PalantirRpcClient;
/// Shared types are the same Rust types used by the trading API.
pub mod common {
    pub use crate::types::{
        BalanceShortfall, CustomQuoteWhitelist, DexType, LiqState, MigrationStatus, ProcKind,
        TokenAccountKind, TokenTypeFilter, Trade, TradeType,
    };
}
pub use common::*;

#[cfg(test)]
mod tests {
    #[test]
    fn common_exports_have_identical_type_identity() {
        use std::any::TypeId;
        macro_rules! same {
            ($($name:ident),+ $(,)?) => { $(
                assert_eq!(TypeId::of::<super::$name>(), TypeId::of::<crate::types::$name>());
                assert_eq!(TypeId::of::<super::common::$name>(), TypeId::of::<crate::types::$name>());
            )+ };
        }
        same!(
            BalanceShortfall,
            CustomQuoteWhitelist,
            DexType,
            LiqState,
            MigrationStatus,
            ProcKind,
            TokenAccountKind,
            TokenTypeFilter,
            Trade,
            TradeType
        );
    }

    #[test]
    fn requests_accept_shared_enums_without_conversion() {
        let request = super::GetDex {
            dex: crate::types::DexType::Pump,
            after: None,
            limit: 1,
        };
        let _: crate::types::DexType = request.dex;
    }
}
