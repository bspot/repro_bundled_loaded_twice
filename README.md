# repro_bundled_loaded_twice

Reproduction for [Issue #89](https://github.com/DelSkayn/rquickjs/issues/89) in the https://github.com/DelSkayn/rquickjs Rust wrapper for QuickJS.

Under some circumstances, compiled modules stored in a `Bundle` will be re-loaded every time they are imported instead of just once. 
This can happen particularly easy when using the `embed` macro to create the bundle. 

To run the reproduction:
```
cargo run --bin repro
```

Output:
```
------------------------------------------------------------------ first
resolving module 'script1':
resolving module 'js/embedded_module.js':
resolving module 'js/live_module.js':
local 0: 0x56406853b850
start instantiating module 'script1':
start instantiating module 'js/embedded_module.js':
instantiating module 'js/embedded_module.js':
exported bindings:
 name=foo local="<null>" type=0 idx=0
done instantiate
start instantiating module 'js/live_module.js':
instantiating module 'js/live_module.js':
exported bindings:
done instantiate
instantiating module 'script1':
exported bindings:
import var_idx=0 name=foo: local export (var_ref=0x56406853b850)
done instantiate
PRINT FROM JS: script module loaded
PRINT FROM JS: live module loaded
------------------------------------------------------------------ second
resolving module 'script2':
resolving module 'js/embedded_module.js':
local 0: 0x564068530190
start instantiating module 'script2':
start instantiating module 'js/embedded_module.js':
instantiating module 'js/embedded_module.js':
exported bindings:
 name=foo local="<null>" type=0 idx=0
done instantiate
instantiating module 'script2':
exported bindings:
import var_idx=0 name=foo: local export (var_ref=0x564068530190)
done instantiate
PRINT FROM JS: script module loaded
------------------------------------------------------------------ check
imported functions are the same: false
```

As you can see, the `embedded_module` stored in the bundle is instantiated both times it is imported. This does not happen to the `live_module` loaded 
directly from file.

The reason this is happening is that the modules's name in the `Bundle` is `embedded_module`, while the name in the compiled byte code is `js/embedded_module.js`.
1. The module is requested as `embedded_module`.
2. In `Bundle::revolve` this is resolved to `embedded_module`. (https://github.com/DelSkayn/rquickjs/blob/master/core/src/loader/bundle.rs#L45)
3. However, the module name in the compiled byte code is `js/embedded_module.js`.
4. When the module is requested again, QuickJS tries to find the module name `embedded_module` in the context's loaded modules.
5. It doesn't find it, because the loaded module's name is `js/embedded_module.js`.
6. So QuickJS loads the module again.

We can work around the issue by ensuring that the module's name in the bundle is the same as the name in the bytecode. 

```
cargo run --bin workaround
```

Output: 

```
------------------------------------------------------------------ first
resolving module 'script1':
resolving module 'js/embedded_module.js':
resolving module 'js/live_module.js':
local 0: 0x5595a7265820
start instantiating module 'script1':
start instantiating module 'js/embedded_module.js':
instantiating module 'js/embedded_module.js':
exported bindings:
 name=foo local="<null>" type=0 idx=0
done instantiate
start instantiating module 'js/live_module.js':
instantiating module 'js/live_module.js':
exported bindings:
done instantiate
instantiating module 'script1':
exported bindings:
import var_idx=0 name=foo: local export (var_ref=0x5595a7265820)
done instantiate
PRINT FROM JS: script module loaded
PRINT FROM JS: live module loaded
------------------------------------------------------------------ second
resolving module 'script2':
start instantiating module 'script2':
instantiating module 'script2':
exported bindings:
import var_idx=0 name=foo: local export (var_ref=0x5595a7265820)
done instantiate
------------------------------------------------------------------ check
imported functions are the same: true
```

Here, the embedded module is loaded only once.

