//CODEGEN BELOW - DO NOT TOUCH ME
pub mod types {
    use proto_rs::proto_message;
    use chrono::DateTime;
    use chrono::TimeDelta;
    use chrono::Utc;
    use decisol::Lamports;
    use decisol::QuoteLamports;
    use decisol::QuoteLamportsKind;
    use decisol::Sol;
    use decisol::SolanaLamports;
    use decisol::SolanaLamportsKind;
    use decisol::Usd;
    use fastnum::UD128;
    use solana_address::Address;
    use solana_keypair::Keypair;
    use solana_signature::Signature;
    use solana_transaction_error::TransactionError;
    use teloxide_core::types::UserId;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum Action {
        Insert,
        Delete,
        Clear,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ActivationTime(
        pub u64,
    );

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ApiKeyReq {
        pub key: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ApiKeyResp;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ApiLimitOrder {
        pub state: OrderState,
        pub order: RawOrder,
        pub trigger: OrderEventTrigger,
        pub wallet: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ApiOrders {
        pub orders: ::proto_rs::alloc::vec::Vec<ApiLimitOrder>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ArbAttemptInfo {
        pub hop_0: DexType,
        pub hop_1: DexType,
        pub hop_0_id: Address,
        pub hop_1_id: Address,
        pub quote: QuoteLamportsKind,
        pub base: Address,
        pub amount_in: u64,
        pub profit: u64,
        pub trigger_tx_sig: Signature,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum ArbProfitCheck {
        Ta(
            Address,
        ),
        Sol,
        WsolAndSol(
            Address,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum BeAction {
        TradeCallback(
            TradeStateUpdate,
        ),
        TradeEvent(
            ConfirmState,
        ),
        OrderEvent(
            TokenLimitOrders,
        ),
        Ping(
            Ping,
        ),
        Pong(
            Pong,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct BeUserAction {
        pub id: UserId,
        pub value: BeAction,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ClearAll;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ClearLimitOrders;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ClearLimitOrdersResponse;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ConfigPubkeys {
        pub act: Action,
        pub pubkeys: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ConfirmState {
        pub sigs: ::proto_rs::alloc::vec::Vec<Signature>,
        pub event_slot: u64,
        pub event_idx: u64,
        pub user_id: UserId,
        pub limit_state: ::core::option::Option<LimitConfirmState>,
        pub confirm_trade_kind: ConfirmTradeKind,
        pub nonce: TxnNonce,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum ConfirmTradeKind {
        Snipe {
            task_id: SnipeTaskId,
            task_name: ::proto_rs::alloc::string::String,
            txn_sig: Signature,
            mint: Address,
            migration: bool,
            trade_ty: TradeType,
        },
        LimitOrder {
            token: Address,
            order: ApiLimitOrder,
        },
        Swap {
            token: Address,
            trade_ty: TradeType,
        },
        Migration {
            token: Address,
            trade_ty: TradeType,
        },
        Dev {
            token: Address,
            trade_ty: TradeType,
            tx_sig: Signature,
        },
        Copytrade {
            cfg_id: CtTaskId,
            config_name: ::proto_rs::alloc::string::String,
            wallet: Address,
            tx_sig: Signature,
            trade_data: Trade,
        },
        Arb {
            info: ArbAttemptInfo,
            signer: Address,
            profit_check: ArbProfitCheck,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum CopyTradeBuyMode {
        BuyOnlyLimits,
        BuyFixed,
        BuyPercSelf,
        BuyPercMaster,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum CopyTradeSellMode {
        SellPercSelf,
        SellPercMaster,
        SellOnlyLimits,
        SellFixed,
        SellInit,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CreateDefCfg;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CreateNoncesReq {
        pub wallet: Address,
        pub amount: u8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CreateNoncesResp {
        pub sig: Signature,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtCfgId {
        pub id: CtTaskId,
        pub name: ::proto_rs::alloc::string::String,
        pub is_on: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtCfgIds {
        pub ids: ::proto_rs::alloc::vec::Vec<CtCfgId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum CtMigrationFilter {
        NoFilter,
        OnlyBefore,
        OnlyAfter,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtTask {
        pub cfg_id: ::core::option::Option<CtTaskId>,
        pub user_id: UserId,
        pub flags: CtTaskFlags,
        pub values: CtTaskValues,
        pub triggers: CtTaskTriggers,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtTaskFlags {
        pub is_on: bool,
        pub buy_blacklist: bool,
        pub sell_blacklist: bool,
        pub limit_orders_on_buys: bool,
        pub limit_orders_on_sells: bool,
        pub buy_freezeable: bool,
        pub buy_mintable: bool,
        pub pump: bool,
        pub pump_mayhem: bool,
        pub bonk: bool,
        pub ray_amm: bool,
        pub ray_cpmm: bool,
        pub pump_amm: bool,
        pub ray_ll: bool,
        pub moon: bool,
        pub reverse_sell: bool,
        pub reverse_buy: bool,
        pub use_age: bool,
        pub use_mcap: bool,
        pub min_age: u64,
        pub max_age: u64,
        pub min_buy: u64,
        pub max_buy: u64,
        pub config_limits: bool,
        pub buys_left: i8,
        pub follow_buys: FollowSwaps,
        pub follow_sells: FollowSwaps,
        pub buy_migration_filter: CtMigrationFilter,
        pub sell_migration_filter: CtMigrationFilter,
        pub buy_user_nonce: UserNonceStrategy,
        pub sell_user_nonce: UserNonceStrategy,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub struct CtTaskId(
        pub i64,
    );

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtTaskTriggers {
        pub copy_wallets: ::proto_rs::alloc::vec::Vec<Address>,
        pub buy_blacklist: ::proto_rs::alloc::vec::Vec<Address>,
        pub sell_blacklist: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtTaskValues {
        pub name: ::proto_rs::alloc::string::String,
        pub limit_orders: ApiOrders,
        pub allowed_buys: u8,
        pub token_buys: Lru<Address, u32, 32>,
        pub allowed_sells: u8,
        pub token_sells: Lru<Address, u32, 32>,
        pub buy_mode: CopyTradeBuyMode,
        pub sell_mode: CopyTradeSellMode,
        pub buy_amount: QuoteLamports,
        pub sell_amount: QuoteLamports,
        pub buy_amount_perc: UD128,
        pub sell_amount_perc: UD128,
        pub min_mcap: UD128,
        pub max_mcap: UD128,
        pub buy_tip: Lamports,
        pub sell_tip: Lamports,
        pub buy_fee: Lamports,
        pub sell_fee: Lamports,
        pub buy_slippage: UD128,
        pub sell_slippage: UD128,
        pub buy_procs: TxnProcessors,
        pub sell_procs: TxnProcessors,
        pub max_slot_latency_buy: u8,
        pub max_slot_latency_sell: u8,
        pub wallet: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CtUpdate {
        pub cfg_id: CtTaskId,
        pub updates: ::proto_rs::alloc::vec::Vec<CtUpdateField>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum CtUpdateField {
        ConfigLimits(
            bool,
        ),
        LeftBuys(
            u8,
        ),
        BuyMode(
            CopyTradeBuyMode,
        ),
        SellMode(
            CopyTradeSellMode,
        ),
        BuyAmountFixed(
            QuoteLamports,
        ),
        BuyAmountPerc(
            UD128,
        ),
        SellAmountFixed(
            QuoteLamports,
        ),
        SellAmountPerc(
            UD128,
        ),
        TxnProcBuy(
            TxnProcessors,
        ),
        TxnProcSell(
            TxnProcessors,
        ),
        LimitOrders(
            ApiOrders,
        ),
        Wallets(
            ConfigPubkeys,
        ),
        BuyBlackList(
            ConfigPubkeys,
        ),
        SellBlackList(
            ConfigPubkeys,
        ),
        BuyBlacklistSwitch(
            bool,
        ),
        SellBlacklistSwitch(
            bool,
        ),
        Name(
            ::proto_rs::alloc::string::String,
        ),
        MinMcap(
            UD128,
        ),
        MaxMcap(
            UD128,
        ),
        BuySlippage(
            UD128,
        ),
        BuyTip(
            Lamports,
        ),
        BuyFee(
            Lamports,
        ),
        MaxSlotLatencyBuy(
            u8,
        ),
        SellSlippage(
            UD128,
        ),
        SellTip(
            Lamports,
        ),
        SellFee(
            Lamports,
        ),
        MaxSlotLatencySell(
            u8,
        ),
        IsOn(
            bool,
        ),
        BuyMigrationFilter(
            CtMigrationFilter,
        ),
        SellMigrationFilter(
            CtMigrationFilter,
        ),
        FollowBuys(
            FollowSwaps,
        ),
        FollowSells(
            FollowSwaps,
        ),
        PumpMayhem(
            bool,
        ),
        Pump(
            bool,
        ),
        RayAmm(
            bool,
        ),
        RayCpmm(
            bool,
        ),
        PumpAmm(
            bool,
        ),
        RayLl(
            bool,
        ),
        Bonk(
            bool,
        ),
        UseAge(
            bool,
        ),
        MinAge(
            u64,
        ),
        MaxAge(
            u64,
        ),
        BuyFreezable(
            bool,
        ),
        BuyMintable(
            bool,
        ),
        UseMcap(
            bool,
        ),
        MinMasterBuy(
            Lamports,
        ),
        MaxMasterBuy(
            Lamports,
        ),
        UseLimitOrdersForBuys(
            bool,
        ),
        UseLimitOrdersForSells(
            bool,
        ),
        ReverseBuy(
            bool,
        ),
        ReverseSell(
            bool,
        ),
        AllowedBuys(
            u8,
        ),
        AllowedSells(
            u8,
        ),
        BuyNonce(
            UserNonceStrategy,
        ),
        SellNonce(
            UserNonceStrategy,
        ),
        SetWallet(
            Address,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct DeleteLimitOrdersResponse {
        pub total_orders_after: u32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct DeleteOrders {
        pub mint: Address,
        pub all: bool,
        pub ids: ::proto_rs::alloc::vec::Vec<OrderId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub struct DexCu {
        pub pump_buy: u32,
        pub pump_sell: u32,
        pub pump_amm_buy: u32,
        pub pump_amm_sell: u32,
        pub ray_amm_buy: u32,
        pub ray_amm_sell: u32,
        pub ray_cpmm_buy: u32,
        pub ray_cpmm_sell: u32,
        pub ray_ll_buy: u32,
        pub ray_ll_sell: u32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum DexType {
        Pump,
        PumpAmm,
        RayAmm,
        RayCpmm,
        RayLl,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum Direction {
        Above,
        Below,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Done;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct FetchFullWalletsInfo {
        pub active: Address,
        pub balances: WalletBalances,
        pub accounts_state: WalletUtilAccountsInfo,
        pub wallets: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct FetchFullWalletsInfoReq;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct FetchInfo;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct FetchInfoResponse {
        pub wallet: Address,
        pub balances: WalletBalances,
        pub token_accounts: ::core::option::Option<WalletTaInfo>,
        pub wallets_num: u32,
        pub limit_orders_num: u32,
        pub ct_cfgs_num: u32,
        pub snipes_num: u32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum FollowSwaps {
        FollowOff,
        First,
        All,
        Custom,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetCfgIds;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetDexCu;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetLimitOrders;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetUserReq;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetUserResp {
        pub user: ::core::option::Option<UserBe>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetUsersReq;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct LimitConfirmState {
        pub order_id: OrderId,
        pub meta: OrderMeta,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct LimitOrders {
        pub orders: ::proto_rs::alloc::vec::Vec<ApiLimitOrder>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct LiqState {
        pub pc_raw: u64,
        pub token_raw: u64,
        pub pc_v: u64,
        pub token_v: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Lru<K, V, const CAP: usize> {
        pub items: ::proto_rs::alloc::vec::Vec<LruPair<K, V>>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct LruPair<K, V> {
        pub k: K,
        pub v: V,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MainUniSub;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MakeWithdrawReq {
        pub destination: Address,
        pub amount: Lamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MakeWithdrawResp {
        pub sig: Signature,
        pub fee: Lamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum MarketExecuteMode {
        Always,
        OnlyInProfit,
        OnlyInLoss,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MarketTrade {
        pub wallet: ::core::option::Option<Address>,
        pub amount: SwapAmount,
        pub mint: Address,
        pub slippage: UD128,
        pub tip: Lamports,
        pub procs: ::core::option::Option<TxnProcessors>,
        pub nonce: UserNonceStrategy,
        pub priority_fee: Lamports,
        pub slot_latency: ::core::option::Option<u8>,
        pub expire_at: ::core::option::Option<DateTime<Utc>>,
        pub rpc_nonce: ::core::option::Option<u64>,
        pub max_price_impact: ::core::option::Option<UD128>,
        pub limit_orders: ApiOrders,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum MigrationStatus {
        Pre,
        Post,
        NonMigratable,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct OpenTaRequest {
        pub owner: Address,
        pub mint: Address,
        pub kind: TokenAccountKindUi,
        pub is_2022: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum OrderEventTrigger {
        Immediate,
        Migration,
        DevBuy,
        DevSell,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Copy, Hash)]
    #[proto_message]
    pub struct OrderId(
        pub i64,
    );

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct OrderMeta {
        pub trigger: OrderEventTrigger,
        pub left_attempts: i8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum OrderState {
        Placed {
            id: OrderId,
            left_attempts: i8,
            expire_timestamp_utc: i64,
            status: OrderStatus,
            activate_timestamp_utc: u64,
        },
        Api {
            id: ::core::option::Option<OrderId>,
            expire_dur: ::core::option::Option<TimeDelta>,
            activate_dur: ::core::option::Option<TimeDelta>,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum OrderStatus {
        Execute,
        Skip,
        Remove,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum ParsedTradeUi {
        Buy {
            mint: Address,
            ticker: ::proto_rs::alloc::string::String,
            base: SolanaLamports,
            quote: QuoteLamports,
            mc_usd: ::core::option::Option<UD128>,
            price_usd: UD128,
        },
        Sell {
            mint: Address,
            ticker: ::proto_rs::alloc::string::String,
            base: SolanaLamports,
            quote: QuoteLamports,
            pnl: ::core::option::Option<UD128>,
            mc_usd: ::core::option::Option<UD128>,
            price_usd: UD128,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Ping {
        pub count: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Pong {
        pub count: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum ProcKind {
        Aura,
        Jito,
        Bloxroute,
        Nozomi,
        NextBlock,
        Slot0,
        Astra,
        BlockRazor,
        Node1,
        TpuPen,
        Helius,
        Stellium,
        Soyas,
        Falcon,
        Raiden,
        Circular,
        FlashBlock,
        Moon,
        Blocksprint,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ProcessorStats {
        pub total_slot_lat: u64,
        pub sent_txn_count: u64,
        pub landed_txn_count: u64,
        pub error_txn_count: u64,
        pub prio_fee: u64,
        pub tip: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Pubkeys {
        pub values: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct QuoteState {
        pub usd_buy: Usd,
        pub usd_sell: Usd,
        pub usd_sell_unknown: Usd,
        pub sol_buy: Sol,
        pub sol_sell: Sol,
        pub sol_sell_unknown: Sol,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct RawOrder {
        pub slippage: UD128,
        pub tip: Lamports,
        pub fee: Lamports,
        pub target: Target,
        pub amount: SwapAmount,
        pub procs: TxnProcessors,
        pub nonce: UserNonceStrategy,
        pub slot_latency: u8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct RegisterUserReq {
        pub keypair: Keypair,
        pub ref_id: ::core::option::Option<UserId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct RemoveWallet {
        pub to_remove: Address,
        pub new: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum SnipeBuyAmount {
        Fixed,
        PercSelf,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeCfgId {
        pub id: SnipeTaskId,
        pub name: ::proto_rs::alloc::string::String,
        pub is_on: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeCfgIds {
        pub ids: ::proto_rs::alloc::vec::Vec<SnipeCfgId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum SnipeSellAmount {
        Fixed,
        PercSelf,
        Init,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeTask {
        pub cfg_id: ::core::option::Option<SnipeTaskId>,
        pub user_id: UserId,
        pub flags: SnipeTaskFlags,
        pub values: SnipeTaskValues,
        pub triggers: SnipeTaskTriggers,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeTaskFlags {
        pub is_on: bool,
        pub blacklist: bool,
        pub snipe_all: bool,
        pub creation: bool,
        pub migration: bool,
        pub pump: bool,
        pub pump_mayhem: bool,
        pub ray_amm: bool,
        pub ray_cpmm: bool,
        pub pump_amm: bool,
        pub ray_ll: bool,
        pub bonk: bool,
        pub use_time: bool,
        pub min_age: u64,
        pub max_age: u64,
        pub min_dev_buy: u64,
        pub max_dev_buy: u64,
        pub buy_freezable: bool,
        pub buy_mintable: bool,
        pub use_mcap: bool,
        pub config_limits: bool,
        pub buys_left: i8,
        pub buy_mode: bool,
        pub sell_mode: bool,
        pub buy_user_nonce: UserNonceStrategy,
        pub sell_user_nonce: UserNonceStrategy,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub struct SnipeTaskId(
        pub i64,
    );

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeTaskTriggers {
        pub devs: ::proto_rs::alloc::vec::Vec<Address>,
        pub mints: ::proto_rs::alloc::vec::Vec<Address>,
        pub blacklist: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeTaskValues {
        pub name: ::proto_rs::alloc::string::String,
        pub min_mcap: UD128,
        pub max_mcap: UD128,
        pub limit_orders: ApiOrders,
        pub use_limit_orders: bool,
        pub latest_state_slippage: bool,
        pub allow_multiple_actions_for_mint: bool,
        pub allow_multiple_actions_for_dev: bool,
        pub buy_amount: SnipeBuyAmount,
        pub buy_perc: UD128,
        pub buy_exact: QuoteLamports,
        pub buy_slippage: UD128,
        pub buy_tip: Lamports,
        pub buy_fee: Lamports,
        pub max_slot_latency_buy: u8,
        pub sell_amount: SnipeSellAmount,
        pub sell_perc: UD128,
        pub sell_exact: QuoteLamports,
        pub sell_slippage: UD128,
        pub sell_tip: Lamports,
        pub sell_fee: Lamports,
        pub max_slot_latency_sell: u8,
        pub buy_procs: TxnProcessors,
        pub sell_procs: TxnProcessors,
        pub wallet: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SnipeUpdate {
        pub cfg_id: SnipeTaskId,
        pub updates: ::proto_rs::alloc::vec::Vec<SnipeUpdateField>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum SnipeUpdateField {
        BuyMode(
            bool,
        ),
        BuyAmount(
            SnipeBuyAmount,
        ),
        SellAmount(
            SnipeSellAmount,
        ),
        SellMode(
            bool,
        ),
        ConfigLimits(
            bool,
        ),
        LeftBuys(
            u8,
        ),
        TxnProcBuy(
            TxnProcessors,
        ),
        TxnProcSell(
            TxnProcessors,
        ),
        LimitOrders(
            ApiOrders,
        ),
        Devs(
            ConfigPubkeys,
        ),
        Mints(
            ConfigPubkeys,
        ),
        BlackList(
            ConfigPubkeys,
        ),
        BlacklistSwitch(
            bool,
        ),
        Name(
            ::proto_rs::alloc::string::String,
        ),
        MinMcap(
            UD128,
        ),
        MaxMcap(
            UD128,
        ),
        UseLimitOrders(
            bool,
        ),
        LatestStateSlippage(
            bool,
        ),
        AllowMultipleActionsForMint(
            bool,
        ),
        AllowMultipleActionsForDev(
            bool,
        ),
        BuyAmountPerc(
            UD128,
        ),
        SellAmountPerc(
            UD128,
        ),
        BuyAmountFixed(
            QuoteLamports,
        ),
        SellAmountFixed(
            QuoteLamports,
        ),
        BuySlippage(
            UD128,
        ),
        BuyTip(
            Lamports,
        ),
        BuyFee(
            Lamports,
        ),
        MaxSlotLatencyBuy(
            u8,
        ),
        SellSlippage(
            UD128,
        ),
        SellTip(
            Lamports,
        ),
        SellFee(
            Lamports,
        ),
        MaxSlotLatencySell(
            u8,
        ),
        IsOn(
            bool,
        ),
        SnipeAll(
            bool,
        ),
        Creation(
            bool,
        ),
        Migration(
            bool,
        ),
        Pump(
            bool,
        ),
        PumpMayhem(
            bool,
        ),
        RayAmm(
            bool,
        ),
        RayCpmm(
            bool,
        ),
        PumpAmm(
            bool,
        ),
        RayLl(
            bool,
        ),
        Bonk(
            bool,
        ),
        UseAge(
            bool,
        ),
        MinAge(
            u64,
        ),
        MaxAge(
            u64,
        ),
        BuyFreezable(
            bool,
        ),
        BuyMintable(
            bool,
        ),
        UseMcap(
            bool,
        ),
        MinDevBuy(
            Lamports,
        ),
        MaxDevBuy(
            Lamports,
        ),
        BuyNonce(
            UserNonceStrategy,
        ),
        SellNonce(
            UserNonceStrategy,
        ),
        SetWallet(
            Address,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Social {
        pub name: ::proto_rs::alloc::string::String,
        pub value: ::proto_rs::alloc::string::String,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum SwapAmount {
        Buy(
            QuoteLamports,
        ),
        BuyPerc {
            amount: UD128,
        },
        SellPerc {
            amount: UD128,
        },
        SellOut(
            QuoteLamports,
        ),
        SellInit,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum Target {
        Price {
            price: UD128,
            direction: Direction,
        },
        Profit {
            init_profit_perc: UD128,
            recalced_profit: ::core::option::Option<UD128>,
            direction: Direction,
        },
        MovingPerc {
            price_perc: UD128,
            local_ath: ::core::option::Option<UD128>,
            direction: Direction,
        },
        PricePerc {
            price_perc: UD128,
            price: ::core::option::Option<UD128>,
            direction: Direction,
        },
        Mcap {
            mcap: UD128,
            price: ::core::option::Option<UD128>,
            direction: Direction,
        },
        Market {
            mode: MarketExecuteMode,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum TokenAccountKind {
        Ata,
        CanonicalPda {
            bump: u8,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum TokenAccountKindUi {
        Ata,
        Pda,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenBalanceInner {
        pub pda_balance: TokenBalanceSealed,
        pub ata_balance: TokenBalanceSealed,
        pub pda_bump: u8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenBalanceSealed {
        pub balance: u64,
        pub slot: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenLimitOrders {
        pub mint: Address,
        pub orders: ::proto_rs::alloc::vec::Vec<ApiLimitOrder>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenMeta {
        pub supply: ::core::option::Option<SolanaLamports>,
        pub tax_bps: UD128,
        pub ticker: ::proto_rs::alloc::string::String,
        pub name: ::proto_rs::alloc::string::String,
        pub mint_auth: bool,
        pub freeze_auth: bool,
        pub socials: ::proto_rs::alloc::vec::Vec<Social>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPool {
        pub mint: Address,
        pub pool_id: Address,
        pub pool_type: DexType,
        pub migration_status: MigrationStatus,
        pub price: UD128,
        pub base_liq_raw: SolanaLamports,
        pub quote_liq_raw: QuoteLamports,
        pub base_liq_v: SolanaLamports,
        pub quote_liq_v: QuoteLamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPosition {
        pub mint: Address,
        pub ticker: ::proto_rs::alloc::string::String,
        pub pnl: ::core::option::Option<UD128>,
        pub quote_value: QuoteLamports,
        pub orders_num: u32,
        pub last_traded_slot: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPositions {
        pub v: ::proto_rs::alloc::vec::Vec<TokenPosition>,
        pub sol_balance: Lamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPositionsReq;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPositionsUi {
        pub positions: ::proto_rs::alloc::vec::Vec<TokenPosition>,
        pub sol_balance: Lamports,
        pub selected: ::core::option::Option<TokenTradeState>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenPositionsUiReq {
        pub mint: ::core::option::Option<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenStatus {
        pub most_liq_pool: TokenPool,
        pub token_meta: TokenMeta,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenTradeState {
        pub pda_balance: TokenBalanceSealed,
        pub ata_balance: TokenBalanceSealed,
        pub base: SolanaLamportsKind,
        pub is_2022: bool,
        pub base_buy: SolanaLamports,
        pub base_sell: SolanaLamports,
        pub buy_n: u32,
        pub sell_n: u32,
        pub last_traded_slot: u64,
        pub quote: QuoteState,
        pub total_quote_position: QuoteLamports,
        pub mint: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TokenTradeStateInner {
        pub base_buy: SolanaLamports,
        pub base_sell: SolanaLamports,
        pub quote: QuoteState,
        pub buy_n: u32,
        pub sell_n: u32,
        pub last_traded_slot: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Trade {
        pub mint: Address,
        pub pool_id: Address,
        pub trader: Address,
        pub dex_data: DexType,
        pub base: SolanaLamports,
        pub quote: QuoteLamports,
        pub side: TradeType,
        pub pre_base_balance: SolanaLamports,
        pub pre_quote_balance: QuoteLamports,
        pub liq: LiqState,
        pub migration: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TradeResponse {
        pub slot: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum TradeStateUpdate {
        Landed {
            signature: Signature,
            state: TxnConfirmState,
        },
        Lost {
            signatures: ::proto_rs::alloc::vec::Vec<Signature>,
        },
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum TradeType {
        Sell,
        Buy,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TransactionErrorWithAccount {
        pub account: ::core::option::Option<Address>,
        pub err: TransactionError,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TurnOffAll;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TurnOnAll;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum TxnConfirmState {
        Failed {
            tips: TxnTipsParsed,
            cu: u32,
            err: TransactionErrorWithAccount,
            proc: ProcKind,
            slot_sent: u64,
            slot_land: u64,
            idx_sent: u64,
            idx_land: u64,
        },
        Confirmed {
            trades: ::proto_rs::alloc::vec::Vec<ParsedTradeUi>,
            tips: TxnTipsParsed,
            cu: u32,
            proc: ProcKind,
            slot_sent: u64,
            slot_land: u64,
            idx_sent: u64,
            idx_land: u64,
        },
        Lost,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum TxnNonce {
        Custom,
        Durable,
        None,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TxnProcessors {
        pub jito_validators: bool,
        pub jito_bundled: bool,
        pub aura: bool,
        pub bloxroute: bool,
        pub nozomi: bool,
        pub next_block: bool,
        pub slot0: bool,
        pub astra: bool,
        pub block_razor: bool,
        pub node1: bool,
        pub tpu_penetrator: bool,
        #[serde(default = "crate::serde_true")]
        pub helius: bool,
        #[serde(default = "crate::serde_true")]
        pub stellium: bool,
        #[serde(default = "crate::serde_true")]
        pub soyas: bool,
        #[serde(default = "crate::serde_true")]
        pub falcon: bool,
        #[serde(default = "crate::serde_true")]
        pub raiden: bool,
        #[serde(default = "crate::serde_true")]
        pub circular: bool,
        #[serde(default = "crate::serde_true")]
        pub flash_block: bool,
        #[serde(default = "crate::serde_true")]
        pub moon: bool,
        #[serde(default = "crate::serde_true")]
        pub blocksprint: bool,
        pub aura_revert: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TxnProcessorsStats {
        pub jito: ProcessorStats,
        pub aura: ProcessorStats,
        pub bloxroute: ProcessorStats,
        pub nozomi: ProcessorStats,
        pub next_block: ProcessorStats,
        pub slot0: ProcessorStats,
        pub astra: ProcessorStats,
        pub block_razor: ProcessorStats,
        pub tpu_pen: ProcessorStats,
        pub node1: ProcessorStats,
        pub stellium: ProcessorStats,
        pub helius: ProcessorStats,
        pub soyas: ProcessorStats,
        pub falcon: ProcessorStats,
        pub raiden: ProcessorStats,
        pub circular: ProcessorStats,
        pub flashblock: ProcessorStats,
        pub moon: ProcessorStats,
        pub blocksprint: ProcessorStats,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TxnProcsStatReq;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct TxnTipsParsed {
        pub priority: Lamports,
        pub jito: Lamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum UnwrapWsolAmount {
        All,
        Some(
            u64,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UnwrapWsolRequest {
        pub owner: Address,
        pub kind: TokenAccountKindUi,
        pub amount: UnwrapWsolAmount,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UpdateLimitOrdersResponse {
        pub total_orders: u32,
        pub ids: ::proto_rs::alloc::vec::Vec<OrderId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UpdateNoncesReq {
        pub wallet: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UpdateNoncesResp {
        pub found: u8,
        pub updated: u8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UpdateTokenLimitOrders {
        pub mint: Address,
        pub orders: ApiOrders,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub enum UserAction {
        TradeCallback(
            TradeStateUpdate,
        ),
        TradeEvent(
            ConfirmState,
        ),
        OrderEvent(
            TokenLimitOrders,
        ),
        TokenTradeStats(
            TokenTradeState,
        ),
        Ping(
            Ping,
        ),
        Pong(
            Pong,
        ),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UserActionEventSub;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UserBe {
        pub active_wallet: Address,
        pub wallets: ::proto_rs::alloc::vec::Vec<Address>,
        pub api_key: Address,
        pub balance: Lamports,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub enum UserNonceStrategy {
        Durable,
        Hybrid,
        Custom,
        All,
        AllHybrid,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Users {
        pub users: ::proto_rs::alloc::vec::Vec<UserId>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct UtilAccsInfo {
        pub pump_uva: bool,
        pub pump_amm_uva: bool,
        pub pump_amm_uva_ata: bool,
        pub custom_nonce: bool,
        pub durable_nonces: u8,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct WalletBalances {
        pub sol: TokenBalanceSealed,
        pub wsol_ata: TokenBalanceSealed,
        pub usdc_ata: TokenBalanceSealed,
        pub usd1_ata: TokenBalanceSealed,
        pub usdt_ata: TokenBalanceSealed,
        pub wsol_pda: TokenBalanceSealed,
        pub usdc_pda: TokenBalanceSealed,
        pub usd1_pda: TokenBalanceSealed,
        pub usdt_pda: TokenBalanceSealed,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct WalletTaInfo {
        pub wsol_ata: bool,
        pub usdc_ata: bool,
        pub usd1_ata: bool,
        pub usdt_ata: bool,
        pub wsol_pda: bool,
        pub usdc_pda: bool,
        pub usd1_pda: bool,
        pub usdt_pda: bool,
        pub positions: u32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct WalletUtilAccountsInfo {
        pub token_accounts: WalletTaInfo,
        pub util_accs: UtilAccsInfo,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct WrapWsolRequest {
        pub owner: Address,
        pub kind: TokenAccountKindUi,
        pub amount: u64,
    }

}
