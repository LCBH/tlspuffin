//! Definition of the VariableData trait. A VariableData can contain any data which has a `'static`
//! type. This is true for [`rustls::msgs::message::Message`] for example.

use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use rustls::{
    internal::msgs::{
        enums::Compression,
        handshake::{ClientExtension, HandshakePayload, ServerExtension},
        message::{Message, MessagePayload},
    },
    msgs::handshake::ServerKeyExchangePayload,
    CipherSuite,
};

use crate::error::Error;

pub trait VariableData: Debug {
    fn boxed(&self) -> Box<dyn VariableData>;
    fn boxed_any(&self) -> Box<dyn Any>;
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
}

/// A VariableData is cloneable and has a `'static` type. This data type is used throughout
/// tlspuffin to handle data of dynamic size.
impl<T: 'static> VariableData for T
where
    T: Clone + Debug,
{
    fn boxed(&self) -> Box<dyn VariableData> {
        Box::new(self.clone())
    }

    fn boxed_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn type_id(&self) -> TypeId {
        Any::type_id(self)
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
