#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStreamDefaultReader",
        typescript_type = "ReadableStreamDefaultReader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ReadableStreamDefaultReader;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ReadableStreamDefaultReader",
        js_name = "closed"
    )]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn closed(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStreamDefaultReader")]
    #[doc = "The `new ReadableStreamDefaultReader(..)` constructor, creating a new instance of `ReadableStreamDefaultReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/ReadableStreamDefaultReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(stream: &ReadableStream) -> Result<ReadableStreamDefaultReader, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "ReadableStreamDefaultReader",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn read(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "ReadableStreamDefaultReader",
        js_name = "releaseLock"
    )]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn release_lock(this: &ReadableStreamDefaultReader);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "ReadableStreamDefaultReader",
        js_name = "cancel"
    )]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cancel(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "ReadableStreamDefaultReader",
        js_name = "cancel"
    )]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn cancel_with_reason(
        this: &ReadableStreamDefaultReader,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
}
