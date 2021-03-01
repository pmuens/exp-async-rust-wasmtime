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

#[no_mangle]
pub extern "C" fn compute(param: i32) -> i32 {
    unsafe {
        let mut result = param;
        result += heavy(200 as i32);
        result += 300;
        result += heavy(400 as i32);
        result
    }
}

extern "C" {
    fn logger(param: i32);
    fn double(param: i32) -> i32;
    fn heavy(param: i32) -> i32;
}
