use v8::{CreateParams, HandleScope, Isolate, Script};

fn main() {
    // 初始化V8引擎
    init();

    // 创建isolate
    // 在浏览器中打开一个tab，就相当于打开了一个isolate
    let params = CreateParams::default();
    let isolate = &mut Isolate::new(params);

    // 创建handle_scope
    let handle_scope = &mut HandleScope::new(isolate);

    // 创建context
    let context = v8::Context::new(handle_scope);

    // 创建context scope
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // js代码
    let source = r#"
        function hello() {
            return {
              code: 200,
              message: "Hello world!"
            }
        }

        hello();
    "#;

    let source = v8::String::new(context_scope, source).unwrap();
    let script = Script::compile(context_scope, source, None).unwrap();
    let result = script.run(context_scope).unwrap();
    let vaule: serde_json::Value = serde_v8::from_v8(context_scope, result).unwrap();

    println!("result is :  {vaule:?}");
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
