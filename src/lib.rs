#![allow(unused_variables)]
#![allow(dead_code)]

use std::os::raw::{c_void, c_char, c_long};
use std::ffi::CStr;

const MOSQ_AUTH_PLUGIN_VERSION: i32 = 3;

const MOSQ_ACL_NONE: i32 = 0x00;
const MOSQ_ACL_READ: i32 = 0x01;
const MOSQ_ACL_WRITE: i32 = 0x02;
const MOSQ_ACL_SUBSCRIBE: i32 = 0x04;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto {
    _pad0: i32,
    _pad1: i32,
    address: *const c_char,
    id: *const c_char,
    username: *const c_char,
    password: *const c_char,
    _unused: [u8; 0]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_opt {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_auth_opt {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_acl_msg {
    pub topic: *const c_char,
    pub payload: *const c_void,
    pub payloadlen: c_long,
    pub qos: i32,
    pub retain: bool,
}

#[no_mangle]
pub extern fn mosquitto_auth_plugin_version() -> i32 {
    println!("{:?}", "mosquitto_auth_plugin_version");
    return MOSQ_AUTH_PLUGIN_VERSION
}

#[no_mangle]
pub extern fn mosquitto_auth_plugin_init(
    user_data: *mut *mut c_void,
    opts: *mut mosquitto_opt,
    opt_count: i32
) -> i32 {
    println!("{:?}", "mosquitto_auth_plugin_init");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_plugin_cleanup(
    user_data: *mut *mut c_void,
    opts: *mut mosquitto_opt,
    opt_count: i32
) -> i32 {
    println!("{:?}", "mosquitto_auth_plugin_cleanup");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_security_init(
    user_data: *mut *mut c_void,
    opts: *mut mosquitto_opt,
    opt_count: i32,
    reload: bool
) -> i32 {
    println!("{:?}", "mosquitto_auth_security_init");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_security_cleanup(
    user_data: *mut *mut c_void,
    opts: *mut mosquitto_opt,
    opt_count: i32,
    reload: bool
) -> i32 {
    println!("{:?}", "mosquitto_auth_security_cleanup");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_acl_check(
    user_data: *mut c_void,
    access: i32,
    client: *const mosquitto,
    msg: *const mosquitto_acl_msg
) -> i32 {

    match access {
        MOSQ_ACL_READ => {
            println!("{:?}", "MOSQ_ACL_READ");
        }
        MOSQ_ACL_WRITE => {
            println!("{:?}", "MOSQ_ACL_WRITE");
        }
        MOSQ_ACL_SUBSCRIBE => {
            println!("{:?}", "MOSQ_ACL_SUBSCRIBE");
        }
        _ => {
            println!("{:?}", "MOSQ_ACL_NONE");
        }
    }

    let client: mosquitto = unsafe { *client };

    println!("{:?}", client);
    let address = unsafe { CStr::from_ptr(client.address) };
    println!("{:?}", address);

    let id = unsafe { CStr::from_ptr(client.id) };
    println!("{:?}", id);

    let username = unsafe { CStr::from_ptr(client.username) };
    println!("{:?}", username);

    let password = unsafe { CStr::from_ptr(client.password) };
    println!("{:?}", password);


    println!("{:?}", "mosquitto_auth_acl_check");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_unpwd_check(
    user_data: *mut c_void,
    client: *const mosquitto,
    username: *const c_char,
    password: *const c_char
) -> i32 {
    println!("{:?}", "mosquitto_auth_unpwd_check");
    return 0
}

#[no_mangle]
pub extern fn mosquitto_auth_psk_key_get(
    user_data: *mut c_void,
    client: *const mosquitto,
    hint: *const c_char,
    identity: *const c_char,
    key: *mut c_char,
    max_key_len: i32,
) -> i32 {
    println!("{:?}", "mosquitto_auth_psk_key_get");
    return 0
}
