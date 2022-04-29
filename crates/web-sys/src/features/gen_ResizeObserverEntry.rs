#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ResizeObserverEntry",
        typescript_type = "ResizeObserverEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ResizeObserverEntry;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "target"
    )]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn target(this: &ResizeObserverEntry) -> Element;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "contentRect"
    )]
    #[doc = "Getter for the `contentRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn content_rect(this: &ResizeObserverEntry) -> DomRectReadOnly;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "borderBoxSize"
    )]
    #[doc = "Getter for the `borderBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn border_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "contentBoxSize"
    )]
    #[doc = "Getter for the `contentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn content_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "devicePixelContentBoxSize"
    )]
    #[doc = "Getter for the `devicePixelContentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/devicePixelContentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_pixel_content_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
}
