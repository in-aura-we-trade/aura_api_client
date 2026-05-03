mod api_types;
pub use api_types::types;
mod ct_rpc;
mod limit_orders_rpc;
mod snipe_rpc;
mod trade_rpc;
mod utility_rpc;

use crate::{
    UserCtxInterceptor,
    client::{
        ct_rpc::aura_ct_rpc::aura_ct_rpc_client::AuraCtRpcClient,
        limit_orders_rpc::aura_limit_orders_rpc::aura_limit_orders_rpc_client::AuraLimitOrdersRpcClient,
        snipe_rpc::aura_snipe_rpc::aura_snipe_rpc_client::AuraSnipeRpcClient,
        trade_rpc::aura_rpc::aura_rpc_client::AuraRpcClient,
        utility_rpc::aura_utils_rpc::aura_utils_rpc_client::AuraUtilsRpcClient,
    },
};
use std::marker::PhantomData;
use tonic::{
    service::{Interceptor, interceptor::InterceptedService},
    transport::Channel,
};

type RpcSvc<I> = InterceptedService<Channel, I>;

#[derive(Clone)]
pub struct AuraClients<I, Ctx> {
    channel: Channel,
    interceptor: I,
    _ctx: PhantomData<fn() -> Ctx>,
}

impl<I, Ctx> AuraClients<I, Ctx>
where
    I: Interceptor + Clone,
    Ctx: UserCtxInterceptor,
{
    #[inline]
    pub fn new(channel: Channel, interceptor: I) -> Self {
        Self {
            channel,
            interceptor,
            _ctx: PhantomData,
        }
    }

    #[inline]
    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    #[inline]
    pub fn aura(&self) -> AuraRpcClient<RpcSvc<I>, Ctx> {
        AuraRpcClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    #[inline]
    pub fn ct(&self) -> AuraCtRpcClient<RpcSvc<I>, Ctx> {
        AuraCtRpcClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    #[inline]
    pub fn snipe(&self) -> AuraSnipeRpcClient<RpcSvc<I>, Ctx> {
        AuraSnipeRpcClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    #[inline]
    pub fn utils(&self) -> AuraUtilsRpcClient<RpcSvc<I>, Ctx> {
        AuraUtilsRpcClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }

    #[inline]
    pub fn limit_orders(&self) -> AuraLimitOrdersRpcClient<RpcSvc<I>, Ctx> {
        AuraLimitOrdersRpcClient::with_interceptor(self.channel.clone(), self.interceptor.clone())
    }
}
