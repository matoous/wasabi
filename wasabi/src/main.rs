use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> anyhow::Result<()> {
    let wasm_bytes = std::fs::read("example/index.wasm")?;

    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));
    println!("{:?}", result[0]);

    Ok(())
}
