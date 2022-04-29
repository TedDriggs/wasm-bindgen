#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PannerNode",
        typescript_type = "PannerNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub type PannerNode;
    #[cfg(feature = "PanningModelType")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "panningModel"
    )]
    #[doc = "Getter for the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    pub fn panning_model(this: &PannerNode) -> PanningModelType;
    #[cfg(feature = "PanningModelType")]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "panningModel"
    )]
    #[doc = "Setter for the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    pub fn set_panning_model(this: &PannerNode, value: PanningModelType);
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "positionX"
    )]
    #[doc = "Getter for the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_x(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "positionY"
    )]
    #[doc = "Getter for the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_y(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "positionZ"
    )]
    #[doc = "Getter for the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn position_z(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "orientationX"
    )]
    #[doc = "Getter for the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_x(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "orientationY"
    )]
    #[doc = "Getter for the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_y(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "orientationZ"
    )]
    #[doc = "Getter for the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    pub fn orientation_z(this: &PannerNode) -> AudioParam;
    #[cfg(feature = "DistanceModelType")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "distanceModel"
    )]
    #[doc = "Getter for the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    pub fn distance_model(this: &PannerNode) -> DistanceModelType;
    #[cfg(feature = "DistanceModelType")]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "distanceModel"
    )]
    #[doc = "Setter for the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    pub fn set_distance_model(this: &PannerNode, value: DistanceModelType);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "refDistance"
    )]
    #[doc = "Getter for the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn ref_distance(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "refDistance"
    )]
    #[doc = "Setter for the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_ref_distance(this: &PannerNode, value: f64);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "maxDistance"
    )]
    #[doc = "Getter for the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn max_distance(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "maxDistance"
    )]
    #[doc = "Setter for the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_max_distance(this: &PannerNode, value: f64);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "rolloffFactor"
    )]
    #[doc = "Getter for the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn rolloff_factor(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "rolloffFactor"
    )]
    #[doc = "Setter for the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_rolloff_factor(this: &PannerNode, value: f64);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "coneInnerAngle"
    )]
    #[doc = "Getter for the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_inner_angle(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "coneInnerAngle"
    )]
    #[doc = "Setter for the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_inner_angle(this: &PannerNode, value: f64);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "coneOuterAngle"
    )]
    #[doc = "Getter for the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_outer_angle(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "coneOuterAngle"
    )]
    #[doc = "Setter for the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_outer_angle(this: &PannerNode, value: f64);
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "PannerNode",
        js_name = "coneOuterGain"
    )]
    #[doc = "Getter for the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn cone_outer_gain(this: &PannerNode) -> f64;
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "PannerNode",
        js_name = "coneOuterGain"
    )]
    #[doc = "Setter for the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_cone_outer_gain(this: &PannerNode, value: f64);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "PannerNode")]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<PannerNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "PannerOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "PannerNode")]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`, `PannerOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PannerOptions,
    ) -> Result<PannerNode, JsValue>;
    #[wasm_bindgen(
        method,
        structural,
        js_class = "PannerNode",
        js_name = "setOrientation"
    )]
    #[doc = "The `setOrientation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_orientation(this: &PannerNode, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method, structural, js_class = "PannerNode", js_name = "setPosition")]
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_position(this: &PannerNode, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method, structural, js_class = "PannerNode", js_name = "setVelocity")]
    #[doc = "The `setVelocity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
    pub fn set_velocity(this: &PannerNode, x: f64, y: f64, z: f64);
}
