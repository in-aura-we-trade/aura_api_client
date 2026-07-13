#[cfg(feature = "api-types")]
pub mod client;
#[cfg(feature = "api-types")]
pub use client::types;

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

#[cfg(feature = "api-types")]
#[inline(always)]
const fn serde_true() -> bool {
    true
}
