//CODEGEN BELOW - DO NOT TOUCH ME
pub mod aura_utils_rpc {
    use proto_rs::proto_rpc;
    use crate::types::ApiKeyReq;
    use crate::types::ApiKeyResp;
    use crate::types::CreateNoncesReq;
    use crate::types::CreateNoncesResp;
    use crate::types::DexCu;
    use crate::types::Done;
    use crate::types::GetDexCu;
    use crate::types::MakeWithdrawReq;
    use crate::types::MakeWithdrawResp;
    use crate::types::OpenTaRequest;
    use crate::types::RemoveWallet;
    use crate::types::TxnProcessorsStats;
    use crate::types::TxnProcsStatReq;
    use crate::types::UnwrapWsolRequest;
    use crate::types::UpdateNoncesReq;
    use crate::types::UpdateNoncesResp;
    use crate::types::WrapWsolRequest;
    use decisol::Lamports;
    use solana_address::Address;
    use solana_keypair::Keypair;
    use solana_signature::Signature;

    use crate::UserCtxInterceptor;

    #[proto_rpc(rpc_package = "aura_utils_rpc", rpc_server = false, rpc_client = true, rpc_client_ctx = "UserCtxInterceptor")]
    pub trait AuraUtilsRpc {
        async fn change_api_key(
            &self,
            request: ::tonic::Request<ApiKeyReq>,
        ) -> ::core::result::Result<::tonic::Response<ApiKeyResp>, ::tonic::Status>;

        async fn txn_procs_stat(
            &self,
            request: ::tonic::Request<TxnProcsStatReq>,
        ) -> ::core::result::Result<::tonic::Response<TxnProcessorsStats>, ::tonic::Status>;

        async fn switch_wallet(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn remove_wallet(
            &self,
            request: ::tonic::Request<RemoveWallet>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn add_wallet(
            &self,
            request: ::tonic::Request<Keypair>,
        ) -> ::core::result::Result<::tonic::Response<Lamports>, ::tonic::Status>;

        async fn wrap_wsol(
            &self,
            request: ::tonic::Request<WrapWsolRequest>,
        ) -> ::core::result::Result<::tonic::Response<Signature>, ::tonic::Status>;

        async fn unwrap_wsol(
            &self,
            request: ::tonic::Request<UnwrapWsolRequest>,
        ) -> ::core::result::Result<::tonic::Response<Signature>, ::tonic::Status>;

        async fn open_ta(
            &self,
            request: ::tonic::Request<OpenTaRequest>,
        ) -> ::core::result::Result<::tonic::Response<Signature>, ::tonic::Status>;

        async fn open_util_accs(
            &self,
            request: ::tonic::Request<Address>,
        ) -> ::core::result::Result<::tonic::Response<Signature>, ::tonic::Status>;

        async fn make_withdraw(
            &self,
            request: ::tonic::Request<MakeWithdrawReq>,
        ) -> ::core::result::Result<::tonic::Response<MakeWithdrawResp>, ::tonic::Status>;

        async fn create_nonces(
            &self,
            request: ::tonic::Request<CreateNoncesReq>,
        ) -> ::core::result::Result<::tonic::Response<CreateNoncesResp>, ::tonic::Status>;

        async fn update_nonces(
            &self,
            request: ::tonic::Request<UpdateNoncesReq>,
        ) -> ::core::result::Result<::tonic::Response<UpdateNoncesResp>, ::tonic::Status>;

        async fn dex_cu_set(
            &self,
            request: ::tonic::Request<DexCu>,
        ) -> ::core::result::Result<::tonic::Response<Done>, ::tonic::Status>;

        async fn dex_cu_get(
            &self,
            request: ::tonic::Request<GetDexCu>,
        ) -> ::core::result::Result<::tonic::Response<DexCu>, ::tonic::Status>;

    }

}
