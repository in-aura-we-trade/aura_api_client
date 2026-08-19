use crate::{
    consts::{
        PUMP_AMM_BUY_CU, PUMP_AMM_FIRST_BUY_OFFSET, PUMP_AMM_SELL_CU, PUMP_BUY_CU, PUMP_BUY_V2_CU,
        PUMP_FIRST_BUY_OFFSET, PUMP_SELL_CU, PUMP_SELL_V2_CU, RAY_AMM_BUY_CU,
        RAY_AMM_FIRST_BUY_OFFSET, RAY_AMM_SELL_CU, RAY_CPMM_BUY_CU, RAY_CPMM_FIRST_BUY_OFFSET,
        RAY_CPMM_SELL_CU, RAY_LL_BUY_CU, RAY_LL_FIRST_BUY_OFFSET, RAY_LL_SELL_CU,
        TOKEN_2022_OFFSET,
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
        pump_buy_v2: PUMP_BUY_V2_CU,
        pump_sell_v2: PUMP_SELL_V2_CU,
        token_2022_offset: TOKEN_2022_OFFSET,
        pump_first_buy_offset: PUMP_FIRST_BUY_OFFSET,
        pump_amm_first_buy_offset: PUMP_AMM_FIRST_BUY_OFFSET,
        ray_amm_first_buy_offset: RAY_AMM_FIRST_BUY_OFFSET,
        ray_cpmm_first_buy_offset: RAY_CPMM_FIRST_BUY_OFFSET,
        ray_ll_first_buy_offset: RAY_LL_FIRST_BUY_OFFSET,
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
