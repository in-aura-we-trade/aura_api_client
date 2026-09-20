use decisol::{UD128, udec128};

use solana_address::{Address, address};

/// Default additional route-tax ceiling (5%); None remains an explicit opt-out.
pub const MAX_TAX_DEFAULT: UD128 = udec128!(0.05);

pub const fn default_max_tax() -> Option<UD128> {
    Some(MAX_TAX_DEFAULT)
}

pub const AURA_MIN_TIP: u64 = 1_000_000; //0.001
pub const AURA_REVERT_MIN_TIP: u64 = 1_000_000; //0.001
pub const AURA_REVERT_ONLY_MIN_TIP: u64 = 100_000_000; //0.1
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

// High-level fallback/user settings, originally seeded from captured maxima
// plus 15%. Token-program, tax and first-account creation costs are now private
// project_us policy; profiled CLMM/DLMM paths replace the default component.
// Unobserved Pump v2, Ray AMM and Meteora DBC retain their fallback defaults.
pub const PUMP_BUY_CU: u32 = 83_000; // 72,149
pub const PUMP_SELL_CU: u32 = 81_000; // 69,726

pub const PUMP_BUY_V2_CU: u32 = 170_000;

pub const PUMP_SELL_V2_CU: u32 = 150_000;

pub const PUMP_AMM_BUY_CU: u32 = 121_000; // 104,844
pub const PUMP_AMM_SELL_CU: u32 = 140_000; // 120,214 exhausted; reserve >15% before token/fee overhead.

pub const RAY_AMM_BUY_CU: u32 = 49_000;
pub const RAY_AMM_SELL_CU: u32 = 47_000;

pub const RAY_CPMM_BUY_CU: u32 = 37_000; // 31,842
pub const RAY_CPMM_SELL_CU: u32 = 39_000; // 33,911

pub const RAY_LL_BUY_CU: u32 = 65_000; // 55,681
pub const RAY_LL_SELL_CU: u32 = 62_000; // 53,810

// Fallback only; the private work model accounts for bins and account creation.
pub const METEORA_DLMM_BUY_CU: u32 = 65_000;
pub const METEORA_DLMM_SELL_CU: u32 = 53_000; // 45,747
// Ordinary single-hop total, including router/preparation/fee overhead.
// Mainnet 3eZuftNv... at slot 445986839 spent 48,108 CU in CLMM itself.
// Dense tick-crossing stress cases can exceed this policy cap inside Raydium;
// they must not inflate ordinary trade defaults.
pub const RAY_CLMM_MAX_CU: u32 = 100_000;
pub const RAY_CLMM_BUY_CU: u32 = 71_000; // 61,574 (first buys were lower)
pub const RAY_CLMM_SELL_CU: u32 = 80_000; // 60,403 exhausted; profiled paths also need private model headroom.

pub const METEORA_DBC_BUY_CU: u32 = 100_000;
pub const METEORA_DBC_SELL_CU: u32 = 100_000;
pub const METEORA_DAMM_V2_BUY_CU: u32 = 25_000;
pub const METEORA_DAMM_V2_SELL_CU: u32 = 28_000; // Taxed sell exhausted at 26,726; add private token/tax overhead.

pub const fn meteora_dbc_buy_cu() -> u32 {
    METEORA_DBC_BUY_CU
}
pub const fn meteora_dbc_sell_cu() -> u32 {
    METEORA_DBC_SELL_CU
}
pub const fn meteora_damm_v2_buy_cu() -> u32 {
    METEORA_DAMM_V2_BUY_CU
}
pub const fn meteora_damm_v2_sell_cu() -> u32 {
    METEORA_DAMM_V2_SELL_CU
}

/// Subtract once for a connected two-hop swap, before the final CU cap.
/// Minimum headroom across captured routes, retaining 15% on route maxima:
/// DLMM sell + PumpAmm repeat buy = 53k + 121k;
/// ceil(142,936 * 1.15) = 164,377; 9,623 rounds down to 9k.
pub const TWO_HOP_CU_NEGATIVE_OFFSET: u32 = 9_000;

pub const JITO_VALIDATORS: bool = false;
pub const AURA: bool = true;
pub const BLOXROUTE: bool = false;
pub const NOZOMI: bool = false;
pub const NEXT_BLOCK: bool = false;
pub const SLOT0: bool = false;
pub const ASTRA: bool = false;
pub const BLOCK_RAZOR: bool = false;
pub const NODE1: bool = false;
pub const HELIUS: bool = false;
pub const STELLIUM: bool = false;
pub const SOYAS: bool = false;
pub const FALCON: bool = false;
pub const RAIDEN: bool = false;
pub const CIRCULAR: bool = false;
pub const FLASHBLOCK: bool = false;
pub const BLOCKSPRINT: bool = false;
pub const AURA_REVERT: bool = false;
pub const MERIDIAN: bool = false;
pub const BLOCKRUSH: bool = false;
pub const MANKA: bool = false;
pub const LANDX: bool = false;

pub const LIMIT_ORDER_EXP_DUR_DAYS_MAX: u64 = 14;
pub const LIMIT_ORDER_EXP_DUR_DAYS: u64 = 7;
pub const MAX_USER_ORDERS: usize = 1000;
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
pub const CFG_MAX_QUOTE_WHITELIST: usize = 32;
pub const CT_MAX_CFGS_PER_USER: usize = 32;

pub const SNIPE_CFG_MAX_TRACKED_MINTS: usize = 16;
pub const SNIPE_CFG_MAX_TRACKED_DEVS: usize = 1_000;
pub const SNIPE_USER_MAX_TRACKED_DEVS: usize = 1_000;
pub const SNIPE_CFG_MAX_BLACKLIST: usize = 1_000;
pub const SNIPE_USER_MAX_BLACKLIST: usize = 1_000;
pub const SNIPE_MAX_CFGS: usize = 16;

pub const AURA_TG_GROUP_LINK: &str = "https://t.me/trade_with_aura";
pub const AURA_TG_BOT_LINK: &str = "https://t.me/trade_with_aura_bot";
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
pub const fn meteora_dlmm_buy_cu() -> u32 {
    METEORA_DLMM_BUY_CU
}
pub const fn ray_clmm_buy_cu() -> u32 {
    RAY_CLMM_BUY_CU
}
pub const fn meteora_dlmm_sell_cu() -> u32 {
    METEORA_DLMM_SELL_CU
}
pub const fn ray_clmm_sell_cu() -> u32 {
    RAY_CLMM_SELL_CU
}
