#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "WakeLockSentinel",
        typescript_type = "WakeLockSentinel"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WakeLockSentinel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WakeLockSentinel;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "WakeLockSentinel",
        js_name = "released"
    )]
    #[doc = "Getter for the `released` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/released)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn released(this: &WakeLockSentinel) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WakeLockType")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "WakeLockSentinel",
        js_name = "type"
    )]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`, `WakeLockType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &WakeLockSentinel) -> WakeLockType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "WakeLockSentinel",
        js_name = "onrelease"
    )]
    #[doc = "Getter for the `onrelease` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/onrelease)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onrelease(this: &WakeLockSentinel) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "WakeLockSentinel",
        js_name = "onrelease"
    )]
    #[doc = "Setter for the `onrelease` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/onrelease)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onrelease(this: &WakeLockSentinel, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "WakeLockSentinel", js_name = "release")]
    #[doc = "The `release()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/release)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WakeLockSentinel`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn release(this: &WakeLockSentinel) -> ::js_sys::Promise;
}
