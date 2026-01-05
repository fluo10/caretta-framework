mod api;
pub use api::*;

#[cfg(feature = "client")]
mod client_error;
#[cfg(feature = "client")]
use client_error::ClientError;

#[cfg(feature = "desktop-client")]
pub mod client;

#[cfg(feature = "server")]
pub mod service_context;
#[cfg(feature = "server")]
pub use service_context::ServiceContext;

pub mod model;

#[cfg(feature = "macros")]
pub use caretta_framework_macros::Service;