#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "PermissionStatus",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "BluetoothPermissionResult",
        typescript_type = "BluetoothPermissionResult"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothPermissionResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothPermissionResult;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "BluetoothPermissionResult",
        js_name = "devices"
    )]
    #[doc = "Getter for the `devices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult/devices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn devices(this: &BluetoothPermissionResult) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "BluetoothPermissionResult",
        js_name = "devices"
    )]
    #[doc = "Setter for the `devices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothPermissionResult/devices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_devices(this: &BluetoothPermissionResult, value: &::wasm_bindgen::JsValue);
}
