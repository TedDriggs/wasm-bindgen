#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TextTrackList",
        typescript_type = "TextTrackList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub type TextTrackList;
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TextTrackList",
        js_name = "length"
    )]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn length(this: &TextTrackList) -> u32;
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TextTrackList",
        js_name = "onchange"
    )]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn onchange(this: &TextTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "TextTrackList",
        js_name = "onchange"
    )]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn set_onchange(this: &TextTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TextTrackList",
        js_name = "onaddtrack"
    )]
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn onaddtrack(this: &TextTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "TextTrackList",
        js_name = "onaddtrack"
    )]
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn set_onaddtrack(this: &TextTrackList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TextTrackList",
        js_name = "onremovetrack"
    )]
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn onremovetrack(this: &TextTrackList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "TextTrackList",
        js_name = "onremovetrack"
    )]
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
    pub fn set_onremovetrack(this: &TextTrackList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "TextTrack")]
    #[wasm_bindgen(
        method,
        structural,
        js_class = "TextTrackList",
        js_name = "getTrackById"
    )]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
    pub fn get_track_by_id(this: &TextTrackList, id: &str) -> Option<TextTrack>;
    #[cfg(feature = "TextTrack")]
    #[wasm_bindgen(method, structural, js_class = "TextTrackList", indexing_getter)]
    #[doc = "Indexing getter."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
    pub fn get(this: &TextTrackList, index: u32) -> Option<TextTrack>;
}
