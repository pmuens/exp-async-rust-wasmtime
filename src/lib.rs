use anyhow::{format_err, Result};
use wasmtime::{Func, Instance, Module, Store};

pub fn compute() -> Result<Vec<i32>> {
    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/wasm/target/wasm32-unknown-unknown/debug/wasm.wasm"
    );
    let wasm_bytes = std::fs::read(wasm_path)?;

    let store = Store::default();
    let engine = store.engine();

    let module = Module::from_binary(engine, &wasm_bytes)?;

    let logger = Func::wrap(&store, |param: i32| {
        println!("Logger: {}", param);
    });
    let double = Func::wrap(&store, |param: i32| param * 2);

    let imports = [logger.into(), double.into()];
    let instance = Instance::new(&store, &module, &imports)?;

    let mut results = Vec::with_capacity(3);

    // ------ Call functions exported from WASM (add, mul) ------
    let add = instance
        .get_func("add")
        .ok_or(format_err!("Failed to find `add` function export"))?
        .get2::<i32, i32, i32>()?;
    let result = add(1, 2)?;
    results.push(result);

    let mul = instance
        .get_func("mul")
        .ok_or(format_err!("Failed to find `mul` function export"))?
        .get2::<i32, i32, i32>()?;
    let result = mul(1, 2)?;
    results.push(result);

    // ------ Call imported host functions (logger, double) via function exported from WASM (run) ------
    let run = instance
        .get_func("run")
        .ok_or(format_err!("Failed to find `run` function export"))?
        .get1::<i32, i32>()?;
    let result = run(1)?;
    results.push(result);

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        let results = compute().unwrap();

        // 1 + 2
        assert_eq!(results[0], 3);
        // 1 * 2
        assert_eq!(results[1], 2);
        // 1 * 2 * 2
        assert_eq!(results[2], 4);
    }
}
