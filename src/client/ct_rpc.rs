//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_ct_rpc {
    use proto_rs::proto_rpc;
    use crate::types::ApiOrders;
    use crate::types::ClearAll;
    use crate::types::CreateDefCfg;
    use crate::types::CtCfgIds;
    use crate::types::CtTask;
    use crate::types::CtTaskId;
    use crate::types::CtUpdate;
    use crate::types::Done;
    use crate::types::GetCfgIds;
    use crate::types::Pubkeys;
    use crate::types::TurnOffAll;
    use crate::types::TurnOnAll;
    use crate::types::TxnProcessors;

    use crate::UserCtxInterceptor;

    #[proto_rpc(rpc_package = "aura_ct_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "UserCtxInterceptor")]
    pub trait AuraCtRpc {
        async fn ct_get_buy_blacklist(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn ct_get_sell_blacklist(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn ct_new_cfg_def(
            &self,
            request: ::tonic::Request<CreateDefCfg>,
        ) -> ::core::result::Result<::tonic::Response<CtTask>, ::tonic::Status>;

        async fn ct_duplicate_cfg(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<CtTaskId>, ::tonic::Status>;

        async fn ct_turn_off_all_tasks(
            &self,
            request: ::tonic::Request<TurnOffAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn ct_turn_on_all_tasks(
            &self,
            request: ::tonic::Request<TurnOnAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn ct_del_cfg(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn ct_clear_all_cfgs(
            &self,
            request: ::tonic::Request<ClearAll>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn ct_get_cfgs(
            &self,
            request: ::tonic::Request<GetCfgIds>,
        ) -> ::core::result::Result<::tonic::Response<CtCfgIds>, ::tonic::Status>;

        async fn ct_get_cfg(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<CtTask>, ::tonic::Status>;

        async fn ct_get_copy_wallets(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<Pubkeys>, ::tonic::Status>;

        async fn ct_cfg_get_limit_orders(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<ApiOrders>, ::tonic::Status>;

        async fn ct_cfg_get_buy_txn_proc(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<TxnProcessors>, ::tonic::Status>;

        async fn ct_cfg_get_sell_txn_proc(
            &self,
            request: ::tonic::Request<CtTaskId>,
        ) -> ::core::result::Result<::tonic::Response<TxnProcessors>, ::tonic::Status>;

        async fn ct_set_fields(
            &self,
            request: ::tonic::Request<CtUpdate>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

    }

}
