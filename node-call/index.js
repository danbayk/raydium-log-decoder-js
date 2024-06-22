const ffi = require('ffi-napi');

const absolute_path = '<REPLACE WITH ABSOLUTE PATH TO libraydium_amm>'; // use absolute_path instead of relative_path if libraydium_amm.dylib is not found
const relative_path = '../program/target/release/libraydium_amm';
const lib = ffi.Library(relative_path, {
    'decode_ray_log_swap_amount_in_lib':['uint64', ['CString']],
    'decode_ray_log_swap_amount_out_lib':['uint64', ['CString']],
    'decode_ray_log_swap_direction_lib':['uint64', ['CString']],
    'decode_ray_log_init_pc_amount_lib':['uint64', ['CString']],
    'decode_ray_log_init_coin_amount_lib':['uint64', ['CString']],
    'decode_ray_log_init_coin_decimals_lib':['uint8', ['CString']],

});

// [functions that decode Raydium swap logs]
function decode_ray_log_swap_amount_in(log){
    return lib.decode_ray_log_swap_amount_in_lib(log);
}
// exports.decode_ray_log_swap_amount_in = decode_ray_log_swap_amount_in; // example of exporting function for use in another js file

function decode_ray_log_swap_amount_out(log){
    return lib.decode_ray_log_swap_amount_out_lib(log);
}

function decode_ray_log_swap_direction(log){
    return lib.decode_ray_log_swap_direction_lib(log);
}


// [functions that decode Raydium init logs]
function decode_ray_log_init_pc_amount(log){
    return lib.decode_ray_log_init_pc_amount_lib(log);
}

function decode_ray_log_init_coin_amount(log){
    return lib.decode_ray_log_init_coin_amount_lib(log);
}

function decode_ray_log_init_coin_decimals(log){
    return lib.decode_ray_log_init_coin_decimals_lib(log);
}

// example usage

// console.log(decode_ray_log_init_pc_amount('APwjd2YAAAAACQkFAAAAAAAAAAEAAAAAAAAAAF7QsgAAAAAAAFKs37IkHd8JQslI1liNO1LA+QCtHRAZRnG3Q6wZeLoJMXZbycgW'));

// output: (3000000000) ---> 3000000000 / LAMPORTS_PER_SOL = 3 SOL added as liquidity