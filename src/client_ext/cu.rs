use crate::{
    consts::{
        PUMP_AMM_BUY_CU, PUMP_AMM_SELL_CU, PUMP_BUY_CU, PUMP_SELL_CU, RAY_AMM_BUY_CU,
        RAY_AMM_SELL_CU, RAY_CPMM_BUY_CU, RAY_CPMM_SELL_CU, RAY_LL_BUY_CU, RAY_LL_SELL_CU,
    },
    types::DexCu,
};

impl DexCu {
    pub const fn init() -> Self {
        Self {
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
        }
    }
}
