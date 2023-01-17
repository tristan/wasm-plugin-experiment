use std::env;

use wasmtime::{
    component::{Component, Linker},
    Config,
    Engine,
    Store,
    Module,
};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!({
    path: "host.wit",
    tracing: false,
    async: false,
});

struct HostState;

impl test::Test for HostState {
    fn give_me_string(&mut self) -> anyhow::Result<String> {
        Ok("HELLO WORLD".to_string())
    }
}

impl logging::Logging for HostState {
    fn log(&mut self, s: String) -> anyhow::Result<()> {
        println!("LOGGING: {s}");
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker: Linker<HostState> = Linker::new(&engine);
    logging::add_to_linker(&mut linker, |s| s)?;
    test::add_to_linker(&mut linker, |s| s)?;

    let mut store = Store::new(&engine, HostState);
    let plugin_file = env::args().nth(1).expect("Missing plugin input file");

    // TODO: use this to organize plugin dependencies
    let module = Module::from_file(&engine, &plugin_file)?;
    for import in module.imports() {
        println!("requires module: {}", import.module());
    }

    let component_wasm = {
        // manual encode component from raw wasm to avoid the plugin dev having to use wasm-tools or cargo component
        let wasm = wat::parse_file(plugin_file)?;
        ComponentEncoder::default().module(&wasm)?.encode()?
    };

    let component = Component::from_binary(&engine, &component_wasm)?;
    let instance = linker.instantiate(&mut store, &component)?;

    println!("------ calling hello-world ------");

    let hello_world = instance
        .get_func(&mut store, "hello-world")
        .unwrap()
        .typed::<(), (), _>(&store)
        .unwrap();
    hello_world.call(&mut store, ())?;
    hello_world.post_return(&mut store)?;

    println!("------ calling hello-int ------");

    let hello_int = instance
        .get_func(&mut store, "hello-int")
        .unwrap()
        .typed::<(u32,), (), _>(&store)
        .unwrap();
    hello_int.call(&mut store, (777,))?;
    hello_int.post_return(&mut store)?;

    println!("------ calling hello-string ------");

    let hello_string = instance
        .get_func(&mut store, "hello-string")
        .unwrap()
        .typed::<(&str,), (), _>(&store)
        .unwrap();
    hello_string.call(&mut store, ("TEST2",))?;
    hello_string.post_return(&mut store)?;

    println!("------ calling return-string ------");

    let return_string = instance
        .get_func(&mut store, "return-string")
        .unwrap()
        .typed::<(&str,), (String,), _>(&store)
        .unwrap();
    let (res,) = return_string.call(&mut store, ("TEST3",))?;
    return_string.post_return(&mut store)?;
    println!("{res}");

    Ok(())
}
