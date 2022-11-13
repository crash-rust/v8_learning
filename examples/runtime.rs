use v8_learning::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();

    let mut runtime = JsRuntime::new(JsRuntimeParams::default());

    // js代码
    let code = r#"
        function hello() {
            var a = print("Hello World");
            print(a)
            return fetch("https://www.rust-lang.org/");
        }

        hello();
    "#;

    let result = runtime.execute_script(code);

    println!("result is :  {result:?}");
}
