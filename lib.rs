use napi::bindgen_prelude::FromNapiValue;
use napi::sys::{napi_env, napi_value};
use napi::{self, CallContext, Env, JsArrayBuffer, JsObject};
use napi_derive::{js_function, module_exports};
use std::mem;
use std::os::raw::c_void;

extern "C" {
    pub fn v8_create_external_array_buffer(
        data: *mut u8,
        len: usize,
        capacity: usize,
        out: &mut napi_value,
        finalizer: extern "C" fn(*mut c_void, usize, *mut c_void),
    );
}

// This is a safe wrapper around a v8 API to create an external array buffer.
// While napi exposes napi_create_external_buffer, the way that method works is it
// allocates a JS Buffer and then returns the underlying ArrayBuffer, which is less efficient.
pub fn create_external_array_buffer(
    env: &mut Env,
    mut data: Vec<u8>,
) -> napi::Result<JsArrayBuffer> {
    unsafe {
        let mut out: napi_value = std::mem::zeroed();
        v8_create_external_array_buffer(
            data.as_mut_ptr(),
            data.len(),
            data.capacity(),
            &mut out,
            release_vec,
        );
        mem::forget(data);
        JsArrayBuffer::from_napi_value(env as *mut napi::Env as napi_env, out)
    }
}

extern "C" fn release_vec(finalize_data: *mut c_void, length: usize, capacity: *mut c_void) {
    let vec = unsafe { Vec::from_raw_parts(finalize_data as *mut u8, length, capacity as usize) };
    mem::drop(vec);
}

#[js_function()]
fn demo(cx: CallContext) -> napi::Result<JsArrayBuffer> {
    println!("Hello from rust");
    create_external_array_buffer(cx.env, "Hello from rust, calling c, calling v8!".as_bytes().to_vec())
}

#[module_exports]
fn init(mut exports: JsObject) -> napi::Result<()> {
    exports.create_named_method("demo", demo)?;
    Ok(())
}
