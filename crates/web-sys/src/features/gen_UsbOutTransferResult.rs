#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBOutTransferResult",
        typescript_type = "USBOutTransferResult"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbOutTransferResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbOutTransferResult;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "USBOutTransferResult",
        js_name = "bytesWritten"
    )]
    #[doc = "Getter for the `bytesWritten` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/bytesWritten)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_written(this: &UsbOutTransferResult) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "USBOutTransferResult",
        js_name = "status"
    )]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn status(this: &UsbOutTransferResult) -> UsbTransferStatus;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[wasm_bindgen(catch, constructor, js_class = "USBOutTransferResult")]
    #[doc = "The `new UsbOutTransferResult(..)` constructor, creating a new instance of `UsbOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(status: UsbTransferStatus) -> Result<UsbOutTransferResult, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[wasm_bindgen(catch, constructor, js_class = "USBOutTransferResult")]
    #[doc = "The `new UsbOutTransferResult(..)` constructor, creating a new instance of `UsbOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_bytes_written(
        status: UsbTransferStatus,
        bytes_written: u32,
    ) -> Result<UsbOutTransferResult, JsValue>;
}
