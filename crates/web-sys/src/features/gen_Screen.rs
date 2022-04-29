#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Screen",
        typescript_type = "Screen"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Screen` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub type Screen;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "availWidth"
    )]
    #[doc = "Getter for the `availWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn avail_width(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "availHeight"
    )]
    #[doc = "Getter for the `availHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn avail_height(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn width(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn height(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "colorDepth"
    )]
    #[doc = "Getter for the `colorDepth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn color_depth(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "pixelDepth"
    )]
    #[doc = "Getter for the `pixelDepth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn pixel_depth(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "top"
    )]
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn top(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "left"
    )]
    #[doc = "Getter for the `left` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/left)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn left(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "availTop"
    )]
    #[doc = "Getter for the `availTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn avail_top(this: &Screen) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        structural,
        catch,
        method,
        getter,
        js_class = "Screen",
        js_name = "availLeft"
    )]
    #[doc = "Getter for the `availLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn avail_left(this: &Screen) -> Result<i32, JsValue>;
    #[cfg(feature = "ScreenOrientation")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "Screen",
        js_name = "orientation"
    )]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenOrientation`*"]
    pub fn orientation(this: &Screen) -> ScreenOrientation;
    #[cfg(feature = "ScreenColorGamut")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "Screen",
        js_name = "colorGamut"
    )]
    #[doc = "Getter for the `colorGamut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorGamut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenColorGamut`*"]
    pub fn color_gamut(this: &Screen) -> ScreenColorGamut;
    #[cfg(feature = "ScreenLuminance")]
    #[wasm_bindgen(structural, method, getter, js_class = "Screen", js_name = "luminance")]
    #[doc = "Getter for the `luminance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/luminance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenLuminance`*"]
    pub fn luminance(this: &Screen) -> Option<ScreenLuminance>;
    #[wasm_bindgen(structural, method, getter, js_class = "Screen", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn onchange(this: &Screen) -> Option<::js_sys::Function>;
    #[wasm_bindgen(structural, method, setter, js_class = "Screen", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
    pub fn set_onchange(this: &Screen, value: Option<&::js_sys::Function>);
}
