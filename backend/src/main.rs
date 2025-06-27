use extism::*;

fn main() {
    let file = Wasm::file("target/wasm32-unknown-unknown/debug/simple_plugin.wasm");
    let manifest = Manifest::new([file]);

    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let result = plugin.call::<&str, &str>("greet", "Artem").unwrap();
    println!("{}", result);

    let result = plugin.call::<&str, &str>("greet", "Egor").unwrap();
    println!("{}", result);

    let result = plugin.call::<&str, &str>("greet", "Andrey").unwrap();
    println!("{}", result);
}
