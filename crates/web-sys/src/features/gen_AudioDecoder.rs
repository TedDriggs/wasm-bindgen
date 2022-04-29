#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioDecoder",
        typescript_type = "AudioDecoder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDecoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDecoder;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "AudioDecoder",
        js_name = "state"
    )]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`, `CodecState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn state(this: &AudioDecoder) -> CodecState;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "AudioDecoder",
        js_name = "decodeQueueSize"
    )]
    #[doc = "Getter for the `decodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decode_queue_size(this: &AudioDecoder) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AudioDecoder")]
    #[doc = "The `new AudioDecoder(..)` constructor, creating a new instance of `AudioDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/AudioDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`, `AudioDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &AudioDecoderInit) -> Result<AudioDecoder, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "AudioDecoder", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &AudioDecoder);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(method, structural, js_class = "AudioDecoder", js_name = "configure")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`, `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configure(this: &AudioDecoder, config: &AudioDecoderConfig);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunk")]
    #[wasm_bindgen(method, structural, js_class = "AudioDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`, `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn decode(this: &AudioDecoder, chunk: &EncodedAudioChunk);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "AudioDecoder", js_name = "flush")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flush(this: &AudioDecoder) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[wasm_bindgen(
        static_method_of = "AudioDecoder",
        js_class = "AudioDecoder",
        js_name = "isConfigSupported"
    )]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/isConfigSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`, `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_config_supported(config: &AudioDecoderConfig) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, structural, js_class = "AudioDecoder", js_name = "reset")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &AudioDecoder);
}
