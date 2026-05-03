#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub use client::types;

#[cfg(feature = "client")]
pub use client_ext::UserCtxInterceptor;

#[cfg(feature = "client")]
pub mod client_ext;
pub mod consts;
pub mod order_ext;
#[cfg(feature = "private_client")]
mod private_client;
#[cfg(feature = "private_client")]
pub use private_client::aura_arb_rpc::aura_arb_rpc;
#[cfg(feature = "private_client")]
pub use private_client::aura_be_rpc::aura_be_rpc;
pub mod utils;

#[cfg(feature = "client")]
#[inline(always)]
const fn serde_true() -> bool {
    true
}
