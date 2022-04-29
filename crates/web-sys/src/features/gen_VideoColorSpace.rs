#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoColorSpace",
        typescript_type = "VideoColorSpace"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpace` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoColorSpace;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoColorSpace",
        js_name = "primaries"
    )]
    #[doc = "Getter for the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/primaries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn primaries(this: &VideoColorSpace) -> Option<VideoColorPrimaries>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoColorSpace",
        js_name = "transfer"
    )]
    #[doc = "Getter for the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/transfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer(this: &VideoColorSpace) -> Option<VideoTransferCharacteristics>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoColorSpace",
        js_name = "matrix"
    )]
    #[doc = "Getter for the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/matrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn matrix(this: &VideoColorSpace) -> Option<VideoMatrixCoefficients>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoColorSpace",
        js_name = "fullRange"
    )]
    #[doc = "Getter for the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/fullRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn full_range(this: &VideoColorSpace) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "VideoColorSpace")]
    #[doc = "The `new VideoColorSpace(..)` constructor, creating a new instance of `VideoColorSpace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<VideoColorSpace, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoColorSpace")]
    #[doc = "The `new VideoColorSpace(..)` constructor, creating a new instance of `VideoColorSpace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_init(init: &VideoColorSpaceInit) -> Result<VideoColorSpace, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[wasm_bindgen(method, structural, js_class = "VideoColorSpace", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn to_json(this: &VideoColorSpace) -> VideoColorSpaceInit;
}
