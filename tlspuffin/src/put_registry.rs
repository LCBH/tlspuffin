use puffin::{
    agent::{AgentDescriptor, AgentName},
    error::Error,
    put::{Put, PutDescriptor, PutName},
    put_registry::PutRegistry,
};

pub const OPENSSL111_PUT: PutName = PutName(['O', 'P', 'E', 'N', 'S', 'S', 'L', '1', '1', '1']);
pub const WOLFSSL520_PUT: PutName = PutName(['W', 'O', 'L', 'F', 'S', 'S', 'L', '5', '2', '0']);
pub const TCP_CLIENT_PUT: PutName = PutName(['T', 'C', 'P', 'C', 'L', 'I', 'E', 'N', 'T', '_']);
pub const TCP_SERVER_PUT: PutName = PutName(['T', 'C', 'P', 'S', 'E', 'R', 'V', 'E', 'R', '_']);

pub const PUT_REGISTRY: PutRegistry = PutRegistry(&[
    crate::tcp::new_tcp_client_factory,
    crate::tcp::new_tcp_server_factory,
    #[cfg(feature = "openssl-binding")]
    crate::openssl::new_openssl_factory,
    #[cfg(feature = "wolfssl-binding")]
    crate::wolfssl::new_wolfssl_factory,
]);

pub const CURRENT_PUT_NAME: PutName = {
    cfg_if::cfg_if! {
        if #[cfg(feature = "openssl-binding")] {
            OPENSSL111_PUT
        } else if #[cfg(feature = "wolfssl-binding")] {
            WOLFSSL520_PUT
        } else {
            DUMMY_PUT
        }
    }
};

pub fn current_put() -> PutDescriptor {
    PutDescriptor {
        name: CURRENT_PUT_NAME,
        ..PutDescriptor::default()
    }
}
