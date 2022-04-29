#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AudioScheduledSourceNode",
        typescript_type = "AudioScheduledSourceNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioScheduledSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    pub type AudioScheduledSourceNode;
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "AudioScheduledSourceNode",
        js_name = "onended"
    )]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn onended(this: &AudioScheduledSourceNode) -> Option<::js_sys::Function>;
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "AudioScheduledSourceNode",
        js_name = "onended"
    )]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn set_onended(this: &AudioScheduledSourceNode, value: Option<&::js_sys::Function>);
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "AudioScheduledSourceNode",
        js_name = "start"
    )]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn start(this: &AudioScheduledSourceNode) -> Result<(), JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "AudioScheduledSourceNode",
        js_name = "start"
    )]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn start_with_when(this: &AudioScheduledSourceNode, when: f64) -> Result<(), JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "AudioScheduledSourceNode",
        js_name = "stop"
    )]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn stop(this: &AudioScheduledSourceNode) -> Result<(), JsValue>;
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "AudioScheduledSourceNode",
        js_name = "stop"
    )]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    pub fn stop_with_when(this: &AudioScheduledSourceNode, when: f64) -> Result<(), JsValue>;
}
