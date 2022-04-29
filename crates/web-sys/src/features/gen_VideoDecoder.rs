#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VideoDecoder",
        typescript_type = "VideoDecoder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoDecoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoDecoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoDecoder",
        js_name = "state"
    )]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CodecState`, `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn state(this: &VideoDecoder) -> CodecState;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "VideoDecoder",
        js_name = "decodeQueueSize"
    )]
    #[doc = "Getter for the `decodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decode_queue_size(this: &VideoDecoder) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderInit")]
    #[wasm_bindgen(catch, constructor, js_class = "VideoDecoder")]
    #[doc = "The `new VideoDecoder(..)` constructor, creating a new instance of `VideoDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/VideoDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &VideoDecoderInit) -> Result<VideoDecoder, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoDecoder", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &VideoDecoder);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[wasm_bindgen(method, structural, js_class = "VideoDecoder", js_name = "configure")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure(this: &VideoDecoder, config: &VideoDecoderConfig);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunk")]
    #[wasm_bindgen(method, structural, js_class = "VideoDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decode(this: &VideoDecoder, chunk: &EncodedVideoChunk);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoDecoder", js_name = "flush")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flush(this: &VideoDecoder) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[wasm_bindgen(
        static_method_of = "VideoDecoder",
        js_class = "VideoDecoder",
        js_name = "isConfigSupported"
    )]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/isConfigSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_config_supported(config: &VideoDecoderConfig) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "VideoDecoder", js_name = "reset")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &VideoDecoder);
}
