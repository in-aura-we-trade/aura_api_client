//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_snipe_rpc {
    use proto_rs::proto_rpc;
    use crate::types::ApiOrders;
    use crate::types::ClearAll;
    use crate::types::CreateDefCfg;
    use crate::types::Done;
    use crate::types::GetCfgIds;
    use crate::types::Pubkeys;
    use crate::types::SnipeCfgIds;
    use crate::types::SnipeTask;
    use crate::types::SnipeTaskId;
    use crate::types::SnipeUpdate;
    use crate::types::TurnOffAll;
    use crate::types::TurnOnAll;
    use crate::types::TxnProcessors;

    use crate::UserCtxInterceptor;

    #[proto_rpc(rpc_package = "aura_snipe_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "UserCtxInterceptor")]
    pub trait AuraSnipeRpc {
        async fn snipe_new_cfg_def(
            &self,
            request: ::tonic::Request<CreateDefCfg>,
        ) -> ::core::result::Result<::tonic::Response<SnipeTask>, ::tonic::Status>;

        async fn snipe_duplicate_cfg(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<SnipeTaskId>, ::tonic::Status>;

        async fn snipe_turn_off_all_tasks(
            &self,
            request: ::tonic::Request<TurnOffAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn snipe_turn_on_all_tasks(
            &self,
            request: ::tonic::Request<TurnOnAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn snipe_del_cfg(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn snipe_clear_all_cfgs(
            &self,
            request: ::tonic::Request<ClearAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn snipe_get_cfgs(
            &self,
            request: ::tonic::Request<GetCfgIds>,
        ) -> ::core::result::Result<::tonic::Response<SnipeCfgIds>, ::tonic::Status>;

        async fn snipe_get_cfg(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<SnipeTask>, ::tonic::Status>;

        async fn snipe_get_mints(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn snipe_get_devs(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn snipe_get_blacklist(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn snipe_cfg_get_limit_orders(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<ApiOrders>, ::tonic::Status>;

        async fn snipe_cfg_get_buy_txn_proc(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<TxnProcessors>, ::tonic::Status>;

        async fn snipe_cfg_get_sell_txn_proc(
            &self,
            request: ::tonic::Request<SnipeTaskId>,
        ) -> ::core::result::Result<::tonic::Response<TxnProcessors>, ::tonic::Status>;

        async fn snipe_set_fields(
            &self,
            request: ::tonic::Request<SnipeUpdate>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

    }

}
