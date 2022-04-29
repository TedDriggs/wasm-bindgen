#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ImageTrackList",
        typescript_type = "ImageTrackList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageTrackList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ImageTrackList;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ImageTrackList",
        js_name = "ready"
    )]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ready(this: &ImageTrackList) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ImageTrackList",
        js_name = "length"
    )]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn length(this: &ImageTrackList) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ImageTrackList",
        js_name = "selectedIndex"
    )]
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn selected_index(this: &ImageTrackList) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageTrack")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "ImageTrackList",
        js_name = "selectedTrack"
    )]
    #[doc = "Getter for the `selectedTrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/selectedTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrack`, `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn selected_track(this: &ImageTrackList) -> Option<ImageTrack>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageTrack")]
    #[wasm_bindgen(method, structural, js_class = "ImageTrackList", indexing_getter)]
    #[doc = "Indexing getter."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageTrack`, `ImageTrackList`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get(this: &ImageTrackList, index: u32) -> Option<ImageTrack>;
}
