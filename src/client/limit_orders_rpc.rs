//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_limit_orders_rpc {
    use proto_rs::proto_rpc;
    use crate::types::ClearLimitOrders;
    use crate::types::ClearLimitOrdersResponse;
    use crate::types::DeleteLimitOrdersResponse;
    use crate::types::DeleteOrders;
    use crate::types::GetLimitOrders;
    use crate::types::LimitOrders;
    use crate::types::TokenLimitOrders;
    use crate::types::UpdateLimitOrdersResponse;
    use crate::types::UpdateTokenLimitOrders;
    use solana_address::Address;

    use crate::UserCtxInterceptor;

    #[proto_rpc(rpc_package = "aura_limit_orders_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "UserCtxInterceptor")]
    pub trait AuraLimitOrdersRpc {
        async fn get_token_limit_orders(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<TokenLimitOrders>, ::tonic::Status>;

        async fn get_limit_orders(
            &self,
            request: ::tonic::Request<GetLimitOrders>,
        ) -> ::core::result::Result<::tonic::Response<LimitOrders>, ::tonic::Status>;

        async fn place_limit_orders(
            &self,
            request: ::tonic::Request<UpdateTokenLimitOrders>,
        ) -> ::core::result::Result<::tonic::Response<UpdateLimitOrdersResponse>, ::tonic::Status>;

        async fn delete_limit_orders(
            &self,
            request: ::tonic::Request<DeleteOrders>,
        ) -> ::core::result::Result<::tonic::Response<DeleteLimitOrdersResponse>, ::tonic::Status>;

        async fn clear_limit_orders(
            &self,
            request: ::tonic::Request<ClearLimitOrders>,
        ) -> ::core::result::Result<::tonic::Response<ClearLimitOrdersResponse>, ::tonic::Status>;

    }

}
