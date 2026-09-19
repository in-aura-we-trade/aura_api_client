use crate::{
    consts::{
        METEORA_DLMM_BUY_CU, METEORA_DLMM_SELL_CU, PUMP_AMM_BUY_CU, PUMP_AMM_SELL_CU, PUMP_BUY_CU,
        PUMP_BUY_V2_CU, PUMP_SELL_CU, PUMP_SELL_V2_CU, RAY_AMM_BUY_CU, RAY_AMM_SELL_CU,
        RAY_CLMM_BUY_CU, RAY_CLMM_SELL_CU, RAY_CPMM_BUY_CU, RAY_CPMM_SELL_CU, RAY_LL_BUY_CU,
        RAY_LL_SELL_CU,
    },
    types::{DexCu, WsolSettings},
};

impl DexCu {
    pub const DEFAULT: Self = Self {
        pump_buy: PUMP_BUY_CU,
        pump_sell: PUMP_SELL_CU,
        pump_amm_buy: PUMP_AMM_BUY_CU,
        pump_amm_sell: PUMP_AMM_SELL_CU,
        ray_amm_buy: RAY_AMM_BUY_CU,
        ray_amm_sell: RAY_AMM_SELL_CU,
        ray_cpmm_buy: RAY_CPMM_BUY_CU,
        ray_cpmm_sell: RAY_CPMM_SELL_CU,
        ray_ll_buy: RAY_LL_BUY_CU,
        ray_ll_sell: RAY_LL_SELL_CU,
        meteora_dlmm_buy: METEORA_DLMM_BUY_CU,
        ray_clmm_buy: RAY_CLMM_BUY_CU,
        meteora_dlmm_sell: METEORA_DLMM_SELL_CU,
        ray_clmm_sell: RAY_CLMM_SELL_CU,
        pump_buy_v2: PUMP_BUY_V2_CU,
        pump_sell_v2: PUMP_SELL_V2_CU,
        meteora_dbc_buy: crate::consts::METEORA_DBC_BUY_CU,
        meteora_dbc_sell: crate::consts::METEORA_DBC_SELL_CU,
        meteora_damm_v2_buy: crate::consts::METEORA_DAMM_V2_BUY_CU,
        meteora_damm_v2_sell: crate::consts::METEORA_DAMM_V2_SELL_CU,
        wsol: WsolSettings {
            auto_wrap: true,
            auto_unwrap: crate::types::AutoUnwrapWsol::Partial,
        },
    };
}

impl Default for DexCu {
    fn default() -> Self {
        Self::DEFAULT
    }
}
