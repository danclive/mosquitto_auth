/* automatically generated by rust-bindgen */

pub const MOSQ_AUTH_PLUGIN_VERSION: u32 = 3;
pub const MOSQ_ACL_NONE: u32 = 0;
pub const MOSQ_ACL_READ: u32 = 1;
pub const MOSQ_ACL_WRITE: u32 = 2;
pub const MOSQ_ACL_SUBSCRIBE: u32 = 4;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_opt {
    pub key: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_mosquitto_opt() {
    assert_eq!(
        ::std::mem::size_of::<mosquitto_opt>(),
        16usize,
        concat!("Size of: ", stringify!(mosquitto_opt))
    );
    assert_eq!(
        ::std::mem::align_of::<mosquitto_opt>(),
        8usize,
        concat!("Alignment of ", stringify!(mosquitto_opt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_opt>())).key as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_opt),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_opt>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_opt),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_auth_opt {
    pub key: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_mosquitto_auth_opt() {
    assert_eq!(
        ::std::mem::size_of::<mosquitto_auth_opt>(),
        16usize,
        concat!("Size of: ", stringify!(mosquitto_auth_opt))
    );
    assert_eq!(
        ::std::mem::align_of::<mosquitto_auth_opt>(),
        8usize,
        concat!("Alignment of ", stringify!(mosquitto_auth_opt))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_auth_opt>())).key as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_auth_opt),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_auth_opt>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_auth_opt),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mosquitto_acl_msg {
    pub topic: *const ::std::os::raw::c_char,
    pub payload: *const ::std::os::raw::c_void,
    pub payloadlen: ::std::os::raw::c_long,
    pub qos: ::std::os::raw::c_int,
    pub retain: bool,
}
#[test]
fn bindgen_test_layout_mosquitto_acl_msg() {
    assert_eq!(
        ::std::mem::size_of::<mosquitto_acl_msg>(),
        32usize,
        concat!("Size of: ", stringify!(mosquitto_acl_msg))
    );
    assert_eq!(
        ::std::mem::align_of::<mosquitto_acl_msg>(),
        8usize,
        concat!("Alignment of ", stringify!(mosquitto_acl_msg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_acl_msg>())).topic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_acl_msg),
            "::",
            stringify!(topic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_acl_msg>())).payload as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_acl_msg),
            "::",
            stringify!(payload)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_acl_msg>())).payloadlen as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_acl_msg),
            "::",
            stringify!(payloadlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_acl_msg>())).qos as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_acl_msg),
            "::",
            stringify!(qos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<mosquitto_acl_msg>())).retain as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(mosquitto_acl_msg),
            "::",
            stringify!(retain)
        )
    );
}
extern "C" {
    pub fn mosquitto_auth_plugin_version() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_plugin_init(
        user_data: *mut *mut ::std::os::raw::c_void,
        opts: *mut mosquitto_opt,
        opt_count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_plugin_cleanup(
        user_data: *mut ::std::os::raw::c_void,
        opts: *mut mosquitto_opt,
        opt_count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_security_init(
        user_data: *mut ::std::os::raw::c_void,
        opts: *mut mosquitto_opt,
        opt_count: ::std::os::raw::c_int,
        reload: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_security_cleanup(
        user_data: *mut ::std::os::raw::c_void,
        opts: *mut mosquitto_opt,
        opt_count: ::std::os::raw::c_int,
        reload: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_acl_check(
        user_data: *mut ::std::os::raw::c_void,
        access: ::std::os::raw::c_int,
        client: *const mosquitto,
        msg: *const mosquitto_acl_msg,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_unpwd_check(
        user_data: *mut ::std::os::raw::c_void,
        client: *const mosquitto,
        username: *const ::std::os::raw::c_char,
        password: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mosquitto_auth_psk_key_get(
        user_data: *mut ::std::os::raw::c_void,
        client: *const mosquitto,
        hint: *const ::std::os::raw::c_char,
        identity: *const ::std::os::raw::c_char,
        key: *mut ::std::os::raw::c_char,
        max_key_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
