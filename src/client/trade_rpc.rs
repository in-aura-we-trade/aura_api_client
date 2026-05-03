//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_rpc {
    use proto_rs::proto_rpc;
    use crate::types::FetchFullWalletsInfo;
    use crate::types::FetchFullWalletsInfoReq;
    use crate::types::FetchInfo;
    use crate::types::FetchInfoResponse;
    use crate::types::MarketTrade;
    use crate::types::Ping;
    use crate::types::Pong;
    use crate::types::TokenMeta;
    use crate::types::TokenPool;
    use crate::types::TokenPositions;
    use crate::types::TokenPositionsReq;
    use crate::types::TokenPositionsUi;
    use crate::types::TokenPositionsUiReq;
    use crate::types::TokenStatus;
    use crate::types::TokenTradeState;
    use crate::types::TradeResponse;
    use crate::types::UserAction;
    use crate::types::UserActionEventSub;
    use solana_address::Address;

    use crate::UserCtxInterceptor;

    #[proto_rpc(rpc_package = "aura_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "UserCtxInterceptor")]
    pub trait AuraRpc {
        type UserActivityStream: ::tonic::codegen::tokio_stream::Stream<Item = ::core::result::Result<UserAction, ::tonic::Status>> + ::core::marker::Send;

        async fn user_activity(
            &self,
            request: ::tonic::Request<UserActionEventSub>,
        ) -> ::core::result::Result<::tonic::Response<Self::UserActivityStream>, ::tonic::Status>;

        async fn user_ping(
            &self,
            request: ::tonic::Request<Ping>,
        ) -> ::core::result::Result<::tonic::Response<Pong>, ::tonic::Status>;

        async fn trade(
            &self,
            request: ::tonic::Request<MarketTrade>,
        ) -> ::core::result::Result<::tonic::Response<TradeResponse>, ::tonic::Status>;

        async fn fetch_state_info(
            &self,
            request: ::tonic::Request<FetchInfo>,
        ) -> ::core::result::Result<::tonic::Response<FetchInfoResponse>, ::tonic::Status>;

        async fn get_token_status(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<TokenStatus>, ::tonic::Status>;

        async fn get_token_most_liq_pool(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<TokenPool>, ::tonic::Status>;

        async fn get_token_meta(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<TokenMeta>, ::tonic::Status>;

        async fn get_token_trade_stats(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<TokenTradeState>, ::tonic::Status>;

        async fn get_token_positions(
            &self,
            request: ::tonic::Request<TokenPositionsReq>,
        ) -> ::core::result::Result<::tonic::Response<TokenPositions>, ::tonic::Status>;

        async fn get_token_positions_ui(
            &self,
            request: ::tonic::Request<TokenPositionsUiReq>,
        ) -> ::core::result::Result<::tonic::Response<TokenPositionsUi>, ::tonic::Status>;

        async fn fetch_full_wallet_info(
            &self,
            request: ::tonic::Request<FetchFullWalletsInfoReq>,
        ) -> ::core::result::Result<::tonic::Response<FetchFullWalletsInfo>, ::tonic::Status>;

    }

}
