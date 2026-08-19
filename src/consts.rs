use decisol::{UD128, udec128};

use solana_address::{Address, address};

pub const AURA_MIN_TIP: u64 = 1_000_000; //0.001
pub const AURA_REVERT_MIN_TIP: u64 = 100_000_000; //0.1
pub const AURA_FEE_PUBKEYS: &[Address] = &[
    address!("AURAfdCVLzjnaeDGELTSXNyQwugaTm7QkxirQqq1MPZ6"),
    address!("AURAx5Apw8sY3aSpD54nfAGM2aL2ErQi83Yupf9vRjGF"),
    address!("AURAx29or3jHHCHcTwmapKZi4ez7VoA6x8QkJrVx6Mgg"),
    address!("AURAxaRk25kSHsEi5aG9kb5Ev6L6jemgDnh5gUTbKbEZ"),
    address!("AURAxG4VyF8xJByB1vEpYfrnPBCoe6oMWrHobB3GgP7U"),
    address!("AURAxiKmSfXsztJPW2Z65ih417tMFwwnjs6a4d6nVxxV"),
    address!("AURAxVLCHrtCHNtdKoAUtXM33uhDtjG8qoj5bsLbbz3Z"),
];

pub const PFEE_UTIL: u64 = 50_000;

pub const TA_RENT: u64 = 2039582;

pub const WRAP_WSOL_CU: u32 = 40_000;
pub const UNWRAP_WSOL_CU: u32 = 50_000;
pub const OPEN_TA_CU: u32 = 40_000;
pub const OPEN_UTILS_CU: u32 = 100_000;
pub const OPEN_NONCE_CU: u32 = 10_000;

pub const PUMP_BUY_CU: u32 = 115_000;
pub const PUMP_SELL_CU: u32 = 95_000;

pub const PUMP_BUY_V2_CU: u32 = 140_000;

pub const PUMP_SELL_V2_CU: u32 = 135_000;

pub const PUMP_AMM_BUY_CU: u32 = 150_000;
pub const PUMP_AMM_SELL_CU: u32 = 140_000;

pub const RAY_AMM_BUY_CU: u32 = 49_000;
pub const RAY_AMM_SELL_CU: u32 = 47_000;

pub const RAY_CPMM_BUY_CU: u32 = 70_000;
pub const RAY_CPMM_SELL_CU: u32 = 57_000;

pub const RAY_LL_BUY_CU: u32 = 130_000;
pub const RAY_LL_SELL_CU: u32 = 92_000;

pub const TOKEN_2022_OFFSET: u32 = 0;
pub const PUMP_FIRST_BUY_OFFSET: u32 = 0;
pub const PUMP_AMM_FIRST_BUY_OFFSET: u32 = 0;
pub const RAY_AMM_FIRST_BUY_OFFSET: u32 = 0;
pub const RAY_CPMM_FIRST_BUY_OFFSET: u32 = 0;
pub const RAY_LL_FIRST_BUY_OFFSET: u32 = 0;

pub const JITO_VALIDATORS: bool = true;
pub const JITO_BUNDLED: bool = false;
pub const AURA: bool = true;
pub const BLOXROUTE: bool = false;
pub const NOZOMI: bool = true;
pub const NEXT_BLOCK: bool = false;
pub const SLOT0: bool = true;
pub const ASTRA: bool = true;
pub const BLOCK_RAZOR: bool = false;
pub const NODE1: bool = false;
pub const HELIUS: bool = true;
pub const STELLIUM: bool = true;
pub const TPU_PEN: bool = false;
pub const SOYAS: bool = true;
pub const FALCON: bool = true;
pub const RAIDEN: bool = false;
pub const CIRCULAR: bool = true;
pub const FLASHBLOCK: bool = false;
pub const BLOCKSPRINT: bool = true;
pub const AURA_REVERT: bool = true;
pub const MOON: bool = false;
pub const BLOCKRUSH: bool = false;
pub const MANKA: bool = true;
pub const LANDX: bool = true;

pub const LIMIT_ORDER_EXP_DUR_DAYS_MAX: u64 = 14;
pub const LIMIT_ORDER_EXP_DUR_DAYS: u64 = 7;
pub const MAX_USER_ORDERS: usize = 200;
pub const MIN_WITHDRAW_AMOUNT: u64 = 3_000_000;

pub const SLIPPAGE_DEFAULT: UD128 = udec128!(0.2);

pub const MIN_POS_VALUE_FILTER: u64 = 100_000;

pub const BUY_TIPS_LAMPORTS: u64 = 1_000_000;
pub const BUY_FEE_LAMPORTS: u64 = 1_000_000;

pub const SELL_FEE_LAMPORTS: u64 = 1_000_000;
pub const SELL_TIPS_LAMPORTS: u64 = 1_000_000;

pub const AUTO_ORDER_AMOUNT_PERC_DEFAULT: UD128 = udec128!(0.5);
pub const AUTO_ORDER_TARGET_PRICE_PERC_DEFAULT: UD128 = udec128!(1.3);
pub const DEV_SELL_TRIGGER_PERC_MIN_DEFAULT: UD128 = udec128!(0.01);
pub const DEV_SELL_TRIGGER_QUOTE_MIN_LAMPORTS_DEFAULT: u64 = 100_000;

pub const MAX_PRICE_IMPACT_DEF: UD128 = udec128!(0.5);

pub const AURA_PROGRAM: Address = address!("AURAsuSzLv3v8SX3Y7emMFzGTWiepJK3Sm4x77GB9n1G");

pub const MAX_SLOT_LATENCY_DEF: u8 = 10;

pub const MAX_WALLETS: usize = 5;
pub const NONCE_ACCOUNT_RENT_LAMPORTS: u64 = 1_447_680;
pub const CT_CFG_MAX_TRACKED_WALLETS: usize = 16;
pub const CFG_MAX_BLACKLIST: usize = 32;
pub const CT_MAX_CFGS_PER_USER: usize = 32;

pub const SNIPE_CFG_MAX_TRACKED_MINTS: usize = 16;
pub const SNIPE_CFG_MAX_TRACKED_DEVS: usize = 16;
pub const SNIPE_MAX_CFGS: usize = 16;

pub const AURA_TG_GROUP_LINK: &str = "https://t.me/trade_with_aura_bot";
pub const AURA_TG_BOT_LINK: &str = "https://t.me/trade_with_aura";
pub const AURA_MANUAL_LINK: &str = "https://aura-15.gitbook.io/aura-user-manual";
pub const AURA_SITE_LINK: &str = "https://aura.rehab";

pub const AURA_API_LINK: &str = "http://trade.aura.rehab:40051";
pub const AURA_SENDER_LINK: &str = "http://sender.aura.rehab:50054";
pub const AURA_TXN_LINK: &str = "http://txn.aura.rehab:50053";

/// Public request and spam-ban policy for an Aura API rate-limit scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApiRateLimit {
    /// Maximum accepted requests in one one-second bucket.
    pub requests_per_second: u32,
    /// Maximum accepted requests in one one-minute bucket.
    pub requests_per_minute: u32,
    /// One-second request count that triggers a temporary ban, inclusively.
    pub per_second_ban_threshold: u32,
    /// One-minute request count that triggers a temporary ban, inclusively.
    pub per_minute_ban_threshold: u32,
    /// Duration of a temporary ban.
    pub ban_secs: u64,
}

/// Rate-limit policy applied to an authenticated API key.
pub const API_KEY_RATE_LIMIT: ApiRateLimit = ApiRateLimit {
    requests_per_second: 4,
    requests_per_minute: 60,
    per_second_ban_threshold: 10,
    per_minute_ban_threshold: 150,
    ban_secs: 24 * 3600,
};

/// Rate-limit policy applied to a non-local client IP address.
pub const API_IP_RATE_LIMIT: ApiRateLimit = ApiRateLimit {
    requests_per_second: 4,
    requests_per_minute: 60,
    per_second_ban_threshold: 10,
    per_minute_ban_threshold: 150,
    ban_secs: 24 * 3600,
};

pub const MINIMUM_DELAY_SLOTS: u8 = 2;

pub const fn pump_buy_v2_cu() -> u32 {
    PUMP_BUY_V2_CU
}
pub const fn pump_sell_v2_cu() -> u32 {
    PUMP_SELL_V2_CU
}
pub const fn token_2022_offset() -> u32 {
    TOKEN_2022_OFFSET
}
pub const fn pump_first_buy_offset() -> u32 {
    PUMP_FIRST_BUY_OFFSET
}
pub const fn pump_amm_first_buy_offset() -> u32 {
    PUMP_AMM_FIRST_BUY_OFFSET
}
pub const fn ray_amm_first_buy_offset() -> u32 {
    RAY_AMM_FIRST_BUY_OFFSET
}
pub const fn ray_cpmm_first_buy_offset() -> u32 {
    RAY_CPMM_FIRST_BUY_OFFSET
}
pub const fn ray_ll_first_buy_offset() -> u32 {
    RAY_LL_FIRST_BUY_OFFSET
}
