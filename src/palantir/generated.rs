//CODEGEN BELOW - DO NOT TOUCH ME
pub mod palantir {
    use proto_rs::proto_message;
    use crate::types::DexType;
    use crate::types::MigrationStatus;
    use crate::types::TradeType;
    use decisol::D128;
    use decisol::SolanaLamports;
    use decisol::UD128;
    use solana_address::Address;
    use solana_signature::Signature;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct ApiKey {
        pub key: Address,
        pub requests_per_second: u32,
        pub burst: u32,
        pub max_streams: u32,
        pub expires_at_ms: u64,
        pub enabled: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub struct Ath {
        pub price_sol: UD128,
        pub slot: u64,
        pub signature: Signature,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Candle {
        pub start_ms: u64,
        pub open: UD128,
        pub high: UD128,
        pub low: UD128,
        pub close: UD128,
        pub volume_sol: UD128,
        pub base_volume: UD128,
        pub trade_count: u64,
        pub unpriced_trade_count: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Candles {
        pub candles: ::proto_rs::alloc::vec::Vec<Candle>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CreateApiKey {
        pub requests_per_second: u32,
        pub burst: u32,
        pub max_streams: u32,
        pub expires_at_ms: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct CreditAdjustment {
        pub key: Address,
        pub amount: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Credits {
        pub remaining: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Done {
        pub ok: bool,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetBatch {
        pub base_mints: ::proto_rs::alloc::vec::Vec<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetCandles {
        pub base_mint: Address,
        pub interval_seconds: u32,
        pub from_ms: u64,
        pub to_ms: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetDex {
        pub dex: DexType,
        pub after: ::core::option::Option<Address>,
        pub limit: u32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct GetMint {
        pub base_mint: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Health {
        pub current_slot: u64,
        pub credits: u64,
        pub cache_generation: u64,
        pub durable_generation: u64,
        pub pending_bytes: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct HealthRequest;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MarketSnapshot {
        pub pool_id: Address,
        pub base_mint: Address,
        pub quote_mint: Address,
        pub dex: DexType,
        pub slot: u64,
        pub transaction_index: u64,
        pub base_liquidity: SolanaLamports,
        pub quote_liquidity: SolanaLamports,
        pub virtual_base_liquidity: SolanaLamports,
        pub virtual_quote_liquidity: SolanaLamports,
        pub price_quote: ::core::option::Option<UD128>,
        pub price_sol: ::core::option::Option<UD128>,
        pub metadata: ::core::option::Option<MintMetadata>,
        pub creator: ::core::option::Option<Address>,
        pub created: bool,
        pub migrated_from: ::core::option::Option<Address>,
        pub migration_status: MigrationStatus,
        pub inverted: bool,
        pub available: bool,
        pub opens_at_ms: i64,
        pub ath: ::core::option::Option<Ath>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct Markets {
        pub mints: ::proto_rs::alloc::vec::Vec<::std::sync::Arc<MintMarkets>>,
        pub next_after: ::core::option::Option<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MintMarkets {
        pub base_mint: Address,
        pub metadata: ::core::option::Option<MintMetadata>,
        pub markets: ::proto_rs::alloc::vec::Vec<MarketSnapshot>,
        pub windows: ::proto_rs::alloc::vec::Vec<WindowStats>,
        pub last_price_sol: ::core::option::Option<UD128>,
        pub ath: ::core::option::Option<Ath>,
        pub as_of_ms: u64,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct MintMetadata {
        pub symbol: ::proto_rs::alloc::string::String,
        pub name: ::proto_rs::alloc::string::String,
        pub uri: ::proto_rs::alloc::string::String,
        pub supply: ::core::option::Option<SolanaLamports>,
        pub mint_authority_present: bool,
        pub freeze_authority_present: bool,
        pub socials: ::proto_rs::alloc::collections::BTreeMap<::std::string::String, ::std::string::String>,
        pub transfer_fee: ::core::option::Option<UD128>,
        pub decimals: ::core::option::Option<u32>,
        pub token_program: ::core::option::Option<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct RevokeKey {
        pub key: Address,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct SubscribeTrades {
        pub base_mint: ::core::option::Option<Address>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Copy)]
    #[proto_message]
    pub struct TradeEvent {
        pub signature: Signature,
        pub slot: u64,
        pub transaction_index: u64,
        pub event_index: u32,
        pub base_mint: Address,
        pub quote_mint: Address,
        pub pool_id: Address,
        pub trader: Address,
        pub dex: DexType,
        pub side: TradeType,
        pub base_amount: SolanaLamports,
        pub quote_amount: SolanaLamports,
        pub pre_base_balance: SolanaLamports,
        pub pre_quote_balance: SolanaLamports,
        pub base_liquidity: SolanaLamports,
        pub quote_liquidity: SolanaLamports,
        pub virtual_base_liquidity: SolanaLamports,
        pub virtual_quote_liquidity: SolanaLamports,
        pub migration: bool,
        pub quote_price_sol: ::core::option::Option<UD128>,
        pub price_sol: ::core::option::Option<UD128>,
        pub volume_sol: ::core::option::Option<UD128>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[proto_message]
    pub struct WindowStats {
        pub seconds: u32,
        pub volume_sol: UD128,
        pub base_volume: UD128,
        pub trade_count: u64,
        pub unpriced_trade_count: u64,
        pub price_change_percent: ::core::option::Option<D128>,
    }

}
pub mod palantir_rpc {
    use proto_rs::proto_rpc;
    use crate::palantir::ApiKey;
    use crate::palantir::Candles;
    use crate::palantir::CreateApiKey;
    use crate::palantir::CreditAdjustment;
    use crate::palantir::Credits;
    use crate::palantir::Done;
    use crate::palantir::GetBatch;
    use crate::palantir::GetCandles;
    use crate::palantir::GetDex;
    use crate::palantir::GetMint;
    use crate::palantir::Health;
    use crate::palantir::HealthRequest;
    use crate::palantir::Markets;
    use crate::palantir::MintMarkets;
    use crate::palantir::RevokeKey;
    use crate::palantir::SubscribeTrades;
    use crate::palantir::TradeEvent;

    #[proto_rpc(rpc_package = "palantir_rpc", rpc_server = false, rpc_client = true)]
    pub trait PalantirRpc {
        type SubscribeTradesStream: ::tonic::codegen::tokio_stream::Stream<Item = ::core::result::Result<TradeEvent, ::tonic::Status>> + ::core::marker::Send;

        async fn get_mint(
            &self,
            request: ::tonic::Request<GetMint>,
        ) -> ::core::result::Result<::tonic::Response<MintMarkets>, ::tonic::Status>;

        async fn get_batch(
            &self,
            request: ::tonic::Request<GetBatch>,
        ) -> ::core::result::Result<::tonic::Response<Markets>, ::tonic::Status>;

        async fn get_dex(
            &self,
            request: ::tonic::Request<GetDex>,
        ) -> ::core::result::Result<::tonic::Response<Markets>, ::tonic::Status>;

        async fn get_candles(
            &self,
            request: ::tonic::Request<GetCandles>,
        ) -> ::core::result::Result<::tonic::Response<Candles>, ::tonic::Status>;

        async fn subscribe_trades(
            &self,
            request: ::tonic::Request<SubscribeTrades>,
        ) -> ::core::result::Result<::tonic::Response<Self::SubscribeTradesStream>, ::tonic::Status>;

        async fn put_api_key(
            &self,
            request: ::tonic::Request<ApiKey>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn create_api_key(
            &self,
            request: ::tonic::Request<CreateApiKey>,
        ) -> ::core::result::Result<::tonic::Response<ApiKey>, ::tonic::Status>;

        async fn revoke_api_key(
            &self,
            request: ::tonic::Request<RevokeKey>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn health(
            &self,
            request: ::tonic::Request<HealthRequest>,
        ) -> ::core::result::Result<::tonic::Response<Health>, ::tonic::Status>;

        async fn top_up_credits(
            &self,
            request: ::tonic::Request<CreditAdjustment>,
        ) -> ::core::result::Result<::tonic::Response<Credits>, ::tonic::Status>;

        async fn remove_credits(
            &self,
            request: ::tonic::Request<CreditAdjustment>,
        ) -> ::core::result::Result<::tonic::Response<Credits>, ::tonic::Status>;

    }

}
