use std::{cell::RefCell, rc::Rc};

use puffin::{
    agent::{AgentDescriptor, AgentType, TLSVersion},
    algebra::dynamic_function::TypeShape,
    claims::{ClaimTrait, GlobalClaimList},
    put::PutDescriptor,
};

use crate::claims::TlsClaim;

/// Static configuration for creating a new agent state for the PUT
#[derive(Clone)]
pub struct TlsPutConfig {
    pub descriptor: AgentDescriptor,
    pub claims: GlobalClaimList<TlsClaim>,
    pub authenticate_peer: bool,
    pub extract_deferred: Rc<RefCell<Option<TypeShape>>>,
}
