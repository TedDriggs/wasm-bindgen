#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoEncoder",
        typescript_type = "VideoEncoder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoEncoder",
        js_name = "state"
    )]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CodecState`, `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn state(this: &VideoEncoder) -> CodecState;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoEncoder",
        js_name = "encodeQueueSize"
    )]
    #[doc = "Getter for the `encodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn encode_queue_size(this: &VideoEncoder) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoEncoder")]
    #[doc = "The `new VideoEncoder(..)` constructor, creating a new instance of `VideoEncoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/VideoEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &VideoEncoderInit) -> Result<VideoEncoder, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &VideoEncoder);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "configure")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure(this: &VideoEncoder, config: &VideoEncoderConfig);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "encode")]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn encode(this: &VideoEncoder, frame: &VideoFrame);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "VideoEncoderEncodeOptions", feature = "VideoFrame",))]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "encode")]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderEncodeOptions`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn encode_with_options(
        this: &VideoEncoder,
        frame: &VideoFrame,
        options: &VideoEncoderEncodeOptions,
    );
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "flush")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flush(this: &VideoEncoder) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[wasm_bindgen(
        static_method_of = "VideoEncoder",
        js_class = "VideoEncoder",
        js_name = "isConfigSupported"
    )]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/isConfigSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_config_supported(config: &VideoEncoderConfig) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoEncoder", js_name = "reset")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &VideoEncoder);
}
