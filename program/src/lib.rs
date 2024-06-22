use std::ffi::CStr;
use std::os::raw::c_char;

#[macro_use]
pub mod log;

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod invokers;
pub mod math;
pub mod processor;
pub mod state;

// Export current solana-sdk types for downstream users who may also be building with a different solana-sdk version
pub use solana_program;

// solana_program::declare_id!("");

// #[no_mangle]
// pub extern fn decode_log(log: *const c_char) -> i32{
//     let c_str = unsafe {
//         assert!(!log.is_null());
//         CStr::from_ptr(log)
//     };
//     let log_str = match c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => return 0, // Handle conversion error, for example, by returning 0
//     };
//     let value: u64 = log::decode_ray_log(log_str);
//     return value as i32;
// }

#[no_mangle]
pub extern fn decode_ray_log_swap_amount_in_lib(log: *const c_char) -> u64 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u64 = log::decode_ray_log_swap_amount_in(log_str);
    return value;
}

#[no_mangle]
pub extern fn decode_ray_log_swap_amount_out_lib(log: *const c_char) -> u64 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u64 = log::decode_ray_log_swap_amount_out(log_str);
    return value;
}

#[no_mangle]
pub extern fn decode_ray_log_swap_direction_lib(log: *const c_char) -> u64 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u64 = log::decode_ray_log_swap_direction(log_str);
    return value;
}

#[no_mangle]
pub extern fn decode_ray_log_init_pc_amount_lib(log: *const c_char) -> u64 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u64 = log::decode_ray_log_init_pc_amount(log_str);
    return value;
}

#[no_mangle]
pub extern fn decode_ray_log_init_coin_amount_lib(log: *const c_char) -> u64 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u64 = log::decode_ray_log_init_coin_amount(log_str);
    return value;
}

#[no_mangle]
pub extern fn decode_ray_log_init_coin_decimals_lib(log: *const c_char) -> u8 {
    let c_str = unsafe {
    assert!(!log.is_null());
    CStr::from_ptr(log)
    };
    let log_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    let value: u8 = log::decode_ray_log_init_coin_decimals(log_str);
    return value;
}