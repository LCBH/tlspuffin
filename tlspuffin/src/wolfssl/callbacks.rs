use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ffi::c_void,
};

use foreign_types::ForeignTypeRef;
use libc::{c_int, c_ulong};
use wolfssl_sys as wolf;

use crate::wolfssl::ssl::SslRef;

///
/// We need to manually use this because the `wolfSSL_CRYPTO_get_ex_new_index` funcationality does
/// not support freeing data
pub struct ExtraUserDataRegistry {
    pub user_data: HashMap<TypeId, UserData>,
}

impl ExtraUserDataRegistry {
    pub fn new() -> Self {
        Self {
            user_data: Default::default(),
        }
    }
}

pub struct UserData {
    pub data: Box<dyn Any>,
}

pub unsafe extern "C" fn msg_callback<F>(
    _write_p: c_int,
    _version: c_int,
    _content_type: c_int,
    _buf: *const c_void,
    _len: c_ulong,
    ssl: *mut wolf::WOLFSSL,
    _arg: *mut c_void,
) where
    F: Fn(&mut SslRef) + 'static,
{
    let ssl = SslRef::from_ptr_mut(ssl);

    let callback = ssl.get_user_data::<F>().expect("BUG: missing msg_callback") as *const F;

    (*callback)(ssl);
}
