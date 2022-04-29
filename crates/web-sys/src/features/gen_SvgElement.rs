#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGElement",
        typescript_type = "SVGElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub type SvgElement;
    #[wasm_bindgen(structural, method, getter, js_class = "SVGElement", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn id(this: &SvgElement) -> String;
    #[wasm_bindgen(structural, method, setter, js_class = "SVGElement", js_name = "id")]
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_id(this: &SvgElement, value: &str);
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "className"
    )]
    #[doc = "Getter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgElement`*"]
    pub fn class_name(this: &SvgElement) -> SvgAnimatedString;
    #[cfg(feature = "DomStringMap")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "dataset"
    )]
    #[doc = "Getter for the `dataset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`, `SvgElement`*"]
    pub fn dataset(this: &SvgElement) -> DomStringMap;
    #[cfg(feature = "CssStyleDeclaration")]
    #[wasm_bindgen(structural, method, getter, js_class = "SVGElement", js_name = "style")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `SvgElement`*"]
    pub fn style(this: &SvgElement) -> CssStyleDeclaration;
    #[cfg(feature = "SvgsvgElement")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ownerSVGElement"
    )]
    #[doc = "Getter for the `ownerSVGElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`, `SvgsvgElement`*"]
    pub fn owner_svg_element(this: &SvgElement) -> Option<SvgsvgElement>;
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "viewportElement"
    )]
    #[doc = "Getter for the `viewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn viewport_element(this: &SvgElement) -> Option<SvgElement>;
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "tabIndex"
    )]
    #[doc = "Getter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn tab_index(this: &SvgElement) -> i32;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "tabIndex"
    )]
    #[doc = "Setter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_tab_index(this: &SvgElement, value: i32);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oncopy"
    )]
    #[doc = "Getter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncopy(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oncopy"
    )]
    #[doc = "Setter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncopy(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(structural, method, getter, js_class = "SVGElement", js_name = "oncut")]
    #[doc = "Getter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncut(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(structural, method, setter, js_class = "SVGElement", js_name = "oncut")]
    #[doc = "Setter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncut(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpaste"
    )]
    #[doc = "Getter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpaste(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpaste"
    )]
    #[doc = "Setter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpaste(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onabort"
    )]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onabort(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onabort"
    )]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onabort(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onblur"
    )]
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onblur(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onblur"
    )]
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onblur(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onfocus"
    )]
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onfocus(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onfocus"
    )]
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onfocus(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onauxclick"
    )]
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onauxclick(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onauxclick"
    )]
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onauxclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oncanplay"
    )]
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncanplay(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oncanplay"
    )]
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncanplay(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oncanplaythrough"
    )]
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncanplaythrough(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oncanplaythrough"
    )]
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncanplaythrough(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onchange"
    )]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onchange(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onchange"
    )]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onchange(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onclick"
    )]
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onclick(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onclick"
    )]
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onclose"
    )]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onclose(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onclose"
    )]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onclose(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oncontextmenu"
    )]
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oncontextmenu(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oncontextmenu"
    )]
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oncontextmenu(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondblclick"
    )]
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondblclick(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondblclick"
    )]
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondblclick(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondrag"
    )]
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondrag(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondrag"
    )]
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondrag(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragend"
    )]
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragend"
    )]
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragenter"
    )]
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragenter(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragenter"
    )]
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragexit"
    )]
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragexit(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragexit"
    )]
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragexit(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragleave"
    )]
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragleave(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragleave"
    )]
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragover"
    )]
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragover(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragover"
    )]
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragover(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondragstart"
    )]
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondragstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondragstart"
    )]
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondragstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondrop"
    )]
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondrop(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondrop"
    )]
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondrop(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ondurationchange"
    )]
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ondurationchange(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ondurationchange"
    )]
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ondurationchange(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onemptied"
    )]
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onemptied(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onemptied"
    )]
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onemptied(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onended"
    )]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onended(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onended"
    )]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onended(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oninput"
    )]
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oninput(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oninput"
    )]
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oninput(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "oninvalid"
    )]
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn oninvalid(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "oninvalid"
    )]
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_oninvalid(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onkeydown"
    )]
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeydown(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onkeydown"
    )]
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeydown(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onkeypress"
    )]
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeypress(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onkeypress"
    )]
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeypress(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onkeyup"
    )]
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onkeyup(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onkeyup"
    )]
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onkeyup(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onload"
    )]
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onload(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onload"
    )]
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onload(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onloadeddata"
    )]
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadeddata(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onloadeddata"
    )]
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadeddata(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onloadedmetadata"
    )]
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadedmetadata(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onloadedmetadata"
    )]
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadedmetadata(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onloadend"
    )]
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onloadend"
    )]
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onloadstart"
    )]
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onloadstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onloadstart"
    )]
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onloadstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmousedown"
    )]
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmousedown(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmousedown"
    )]
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmousedown(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmouseenter"
    )]
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseenter(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmouseenter"
    )]
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmouseleave"
    )]
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseleave(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmouseleave"
    )]
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmousemove"
    )]
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmousemove(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmousemove"
    )]
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmousemove(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmouseout"
    )]
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseout(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmouseout"
    )]
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseout(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmouseover"
    )]
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseover(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmouseover"
    )]
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseover(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onmouseup"
    )]
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onmouseup(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onmouseup"
    )]
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onmouseup(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwheel"
    )]
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwheel(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwheel"
    )]
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwheel(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpause"
    )]
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpause(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpause"
    )]
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpause(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onplay"
    )]
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onplay(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onplay"
    )]
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onplay(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onplaying"
    )]
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onplaying(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onplaying"
    )]
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onplaying(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onprogress"
    )]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onprogress(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onprogress"
    )]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onprogress(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onratechange"
    )]
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onratechange(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onratechange"
    )]
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onratechange(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onreset"
    )]
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onreset(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onreset"
    )]
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onreset(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onresize"
    )]
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onresize(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onresize"
    )]
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onresize(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onscroll"
    )]
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onscroll(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onscroll"
    )]
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onscroll(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onseeked"
    )]
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onseeked(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onseeked"
    )]
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onseeked(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onseeking"
    )]
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onseeking(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onseeking"
    )]
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onseeking(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onselect"
    )]
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onselect(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onselect"
    )]
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onselect(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onshow"
    )]
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onshow(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onshow"
    )]
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onshow(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onstalled"
    )]
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onstalled(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onstalled"
    )]
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onstalled(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onsubmit"
    )]
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onsubmit(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onsubmit"
    )]
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onsubmit(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onsuspend"
    )]
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onsuspend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onsuspend"
    )]
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onsuspend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontimeupdate"
    )]
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontimeupdate(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontimeupdate"
    )]
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontimeupdate(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onvolumechange"
    )]
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onvolumechange(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onvolumechange"
    )]
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onvolumechange(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwaiting"
    )]
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwaiting(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwaiting"
    )]
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwaiting(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onselectstart"
    )]
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onselectstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onselectstart"
    )]
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onselectstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontoggle"
    )]
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontoggle(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontoggle"
    )]
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontoggle(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointercancel"
    )]
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointercancel(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointercancel"
    )]
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointercancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerdown"
    )]
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerdown(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerdown"
    )]
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerdown(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerup"
    )]
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerup(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerup"
    )]
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerup(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointermove"
    )]
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointermove(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointermove"
    )]
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointermove(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerout"
    )]
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerout(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerout"
    )]
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerout(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerover"
    )]
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerover(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerover"
    )]
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerover(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerenter"
    )]
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerenter(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerenter"
    )]
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerenter(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onpointerleave"
    )]
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onpointerleave(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onpointerleave"
    )]
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onpointerleave(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ongotpointercapture"
    )]
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ongotpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ongotpointercapture"
    )]
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ongotpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onlostpointercapture(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onlostpointercapture(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onanimationcancel"
    )]
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationcancel(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onanimationcancel"
    )]
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationcancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onanimationend"
    )]
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onanimationend"
    )]
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onanimationiteration"
    )]
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onanimationiteration"
    )]
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onanimationstart"
    )]
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onanimationstart"
    )]
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontransitioncancel"
    )]
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitioncancel(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontransitioncancel"
    )]
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitioncancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontransitionend"
    )]
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontransitionend"
    )]
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontransitionrun"
    )]
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionrun(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontransitionrun"
    )]
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionrun(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontransitionstart"
    )]
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontransitionstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontransitionstart"
    )]
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontransitionstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationiteration(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationiteration(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkitanimationstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkitanimationstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onwebkittransitionend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onwebkittransitionend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "onerror"
    )]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn onerror(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "onerror"
    )]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_onerror(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontouchstart"
    )]
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchstart(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontouchstart"
    )]
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchstart(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontouchend"
    )]
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchend(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontouchend"
    )]
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchend(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontouchmove"
    )]
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchmove(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontouchmove"
    )]
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchmove(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "SVGElement",
        js_name = "ontouchcancel"
    )]
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn ontouchcancel(this: &SvgElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "SVGElement",
        js_name = "ontouchcancel"
    )]
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn set_ontouchcancel(this: &SvgElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, structural, js_class = "SVGElement", js_name = "blur")]
    #[doc = "The `blur()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn blur(this: &SvgElement) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "SVGElement", js_name = "focus")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
    pub fn focus(this: &SvgElement) -> Result<(), JsValue>;
}
