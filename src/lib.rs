#[cfg(feature = "api-types")]
pub mod client;
#[cfg(feature = "api-types")]
pub use client::types;
#[cfg(feature = "api-types")]
mod user_action_error;

#[cfg(feature = "client-generic")]
pub use client_ext::UserCtxInterceptor;

#[cfg(feature = "client-generic")]
pub mod client_ext;
#[cfg(feature = "api-types")]
#[path = "client_ext/debug.rs"]
mod client_ext_debug;
pub mod consts;
pub mod order_ext;
#[cfg(feature = "private_client")]
mod private_client;
#[cfg(feature = "private_client")]
pub use private_client::aura_arb_rpc::aura_arb_rpc;
#[cfg(feature = "private_client")]
pub use private_client::aura_be_rpc::aura_be_rpc;
pub mod utils;

#[cfg(all(feature = "api-types", not(target_arch = "wasm32")))]
pub use teloxide_core::types::UserId;
#[cfg(all(feature = "api-types", target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[proto_rs::proto_message]
pub struct UserId(pub u64);

#[cfg(feature = "api-types")]
#[inline(always)]
const fn serde_true() -> bool {
    true
}
