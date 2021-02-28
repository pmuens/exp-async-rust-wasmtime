#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern "C" fn run(mut num: i32) -> i32 {
    unsafe {
        logger(num);
        num = double(num);
        logger(num);
        num = double(num);
        logger(num);
        num
    }
}

extern "C" {
    fn logger(param: i32);
    fn double(param: i32) -> i32;
}
