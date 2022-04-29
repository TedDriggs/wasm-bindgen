#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ClipboardItem",
        typescript_type = "ClipboardItem"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ClipboardItem;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PresentationStyle")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ClipboardItem",
        js_name = "presentationStyle"
    )]
    #[doc = "Getter for the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/presentationStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `PresentationStyle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn presentation_style(this: &ClipboardItem) -> PresentationStyle;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ClipboardItem",
        js_name = "lastModified"
    )]
    #[doc = "Getter for the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/lastModified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn last_modified(this: &ClipboardItem) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ClipboardItem",
        js_name = "delayed"
    )]
    #[doc = "Getter for the `delayed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/delayed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn delayed(this: &ClipboardItem) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ClipboardItem",
        js_name = "types"
    )]
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn types(this: &ClipboardItem) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "ClipboardItem", js_name = "getType")]
    #[doc = "The `getType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/getType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_type(this: &ClipboardItem, type_: &str) -> ::js_sys::Promise;
}
