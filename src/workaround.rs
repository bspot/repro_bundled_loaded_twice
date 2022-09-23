use rquickjs::{embed, Runtime, Context, Function};
use rquickjs::{FileResolver, Func, ScriptLoader};

#[embed(path = "", name="js/embedded_module.js")]
mod embedded_module {}

fn print(s: String) {
    println!("PRINT FROM JS: {s}");
}

fn main() {
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();

    rt.set_loader(
        (EMBEDDED_MODULE, FileResolver::default()),
        (EMBEDDED_MODULE, ScriptLoader::default())
    );

    ctx.with(|ctx| {
        ctx.globals().set("print", Func::from(print)).unwrap();

        println!("------------------------------------------------------------------ first");
        ctx.compile("script1", r#"
            import { foo } from "js/embedded_module.js";
            globalThis.firstFoo = foo
            import "./js/live_module.js";
        "#).unwrap();

        println!("------------------------------------------------------------------ second");
        ctx.compile("script2", r#"
            import { foo } from "js/embedded_module.js";
            globalThis.secondFoo = foo
            import "./js/live_module.js";
        "#).unwrap();

        println!("------------------------------------------------------------------ check");
        let first_foo = ctx.globals().get::<_, Function>("firstFoo").unwrap();
        let second_foo = ctx.globals().get::<_, Function>("secondFoo").unwrap();
        println!("imported functions are the same: {:?}", first_foo == second_foo);
    });
}


