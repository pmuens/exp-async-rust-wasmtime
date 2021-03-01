use anyhow::{format_err, Result};
use futures::FutureExt;
use wasmtime::{Engine, Func, FuncType, Instance, Module, Store, Val, ValType};

pub fn compute() -> Result<Vec<i32>> {
    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/wasm/target/wasm32-unknown-unknown/debug/wasm.wasm"
    );
    let wasm_bytes = std::fs::read(wasm_path)?;

    let engine = Engine::default();
    let store = Store::new_async(&engine);

    let module = Module::from_binary(&engine, &wasm_bytes)?;

    // Host functions
    let logger = Func::wrap(&store, |param: i32| {
        println!("Logger: {}", param);
    });
    let double = Func::wrap(&store, |param: i32| param * 2);
    let heavy = Func::new_async(
        &store,
        FuncType::new(Some(ValType::I32), Some(ValType::I32)),
        (),
        move |_caller, _state, params, results| {
            Box::new(async move {
                let num = 42;
                let result = params[0].unwrap_i32() + num;
                results[0] = Val::I32(result as i32);
                Ok(())
            })
        },
    );

    let imports = [logger.into(), double.into(), heavy.into()];
    let instance = Instance::new_async(&store, &module, &imports)
        .now_or_never()
        .unwrap()
        .unwrap();

    let mut results = Vec::with_capacity(4);

    // ------ Call sync functions exported from WASM (add, mul) ------
    let add = instance
        .get_func("add")
        .ok_or(format_err!("Failed to find `add` function export"))?
        .get2_async::<i32, i32, i32>()?;
    let result = add(1, 2).now_or_never().unwrap()?;
    results.push(result);

    let mul = instance
        .get_func("mul")
        .ok_or(format_err!("Failed to find `mul` function export"))?
        .get2_async::<i32, i32, i32>()?;
    let result = mul(1, 2).now_or_never().unwrap()?;
    results.push(result);

    // ------ Call imported sync host functions (logger, double) via function exported from WASM (run) ------
    let run = instance
        .get_func("run")
        .ok_or(format_err!("Failed to find `run` function export"))?
        .get1_async::<i32, i32>()?;
    let result = run(1).now_or_never().unwrap()?;
    results.push(result);

    // ------ Call imported async host function (heavy) via function exported from WASM (compute) ------
    let compute = instance
        .get_func("compute")
        .ok_or(format_err!("Failed to find `compute` function export"))?
        .get1_async::<i32, i32>()?;
    let result = compute(1).now_or_never().unwrap()?;
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
        // 1 + (200 + 42) + 300 + (400 + 42)
        assert_eq!(results[3], 985);
    }
}
