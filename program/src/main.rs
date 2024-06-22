mod log;
use raydium_amm::decode_ray_log_swap_amount_in_lib;
use raydium_amm::decode_ray_log_swap_amount_out_lib;
use raydium_amm::decode_ray_log_swap_direction_lib;
use raydium_amm::decode_ray_log_init_pc_amount_lib;
use raydium_amm::decode_ray_log_init_coin_amount_lib;
use raydium_amm::decode_ray_log_init_coin_decimals_lib;
use std::ffi::CString;

fn main(){
    // [example for calling functions via Rust]

    // let log = "AwBnBHYAAAAAAAAAAAAAAAABAAAAAAAAALCQd3AFAAAAzB5uHMAYAwCRBjGwCwAAAE1QazYCHgAA";
    // log::decode_ray_log(log);
    // let log = "AwBnBHYAAAAAAAAAAAAAAAABAAAAAAAAALCQd3AFAAAAzB5uHMAYAwCRBjGwCwAAAE1QazYCHgAA";
    // let c_string = CString::new(log).expect("CString::new failed");
    // let ptr = c_string.as_ptr();
    // println!("{}", decode_ray_log_swap_amount_in_lib(ptr));
    // println!("{}", decode_ray_log_swap_amount_out_lib(ptr));
    // println!("{}", decode_ray_log_swap_direction_lib(ptr));
    // println!("{}", decode_ray_log_init_pc_amount_lib(ptr));
    // println!("{}", decode_ray_log_init_coin_amount_lib(ptr));
    // println!("{}", decode_ray_log_init_coin_decimals_lib(ptr));
}