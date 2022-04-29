#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TreeWalker",
        typescript_type = "TreeWalker"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeWalker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeWalker`*"]
    pub type TreeWalker;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(structural, method, getter, js_class = "TreeWalker", js_name = "root")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn root(this: &TreeWalker) -> Node;
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TreeWalker",
        js_name = "whatToShow"
    )]
    #[doc = "Getter for the `whatToShow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeWalker`*"]
    pub fn what_to_show(this: &TreeWalker) -> u32;
    #[cfg(feature = "NodeFilter")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TreeWalker",
        js_name = "filter"
    )]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`, `TreeWalker`*"]
    pub fn filter(this: &TreeWalker) -> Option<NodeFilter>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        structural,
        method,
        getter,
        js_class = "TreeWalker",
        js_name = "currentNode"
    )]
    #[doc = "Getter for the `currentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn current_node(this: &TreeWalker) -> Node;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        structural,
        method,
        setter,
        js_class = "TreeWalker",
        js_name = "currentNode"
    )]
    #[doc = "Setter for the `currentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn set_current_node(this: &TreeWalker, value: &Node);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "firstChild"
    )]
    #[doc = "The `firstChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn first_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "lastChild"
    )]
    #[doc = "The `lastChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn last_child(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "nextNode"
    )]
    #[doc = "The `nextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn next_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "nextSibling"
    )]
    #[doc = "The `nextSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn next_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "parentNode"
    )]
    #[doc = "The `parentNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn parent_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "previousNode"
    )]
    #[doc = "The `previousNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn previous_node(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        structural,
        js_class = "TreeWalker",
        js_name = "previousSibling"
    )]
    #[doc = "The `previousSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
    pub fn previous_sibling(this: &TreeWalker) -> Result<Option<Node>, JsValue>;
}
