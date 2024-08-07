#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NodeList , typescript_type = "NodeList")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub type NodeList;
    # [wasm_bindgen (structural , method , getter , js_class = "NodeList" , js_name = length)]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn length(this: &NodeList) -> u32;
    # [wasm_bindgen (method , structural , js_class = "NodeList" , js_name = entries)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn entries(this: &NodeList) -> ::js_sys::Iterator;
    # [wasm_bindgen (catch , method , structural , js_class = "NodeList" , js_name = forEach)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn for_each(this: &NodeList, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[cfg(feature = "Node")]
    # [wasm_bindgen (method , structural , js_class = "NodeList" , js_name = item)]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn item(this: &NodeList, index: u32) -> Option<Node>;
    # [wasm_bindgen (method , structural , js_class = "NodeList" , js_name = keys)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn keys(this: &NodeList) -> ::js_sys::Iterator;
    # [wasm_bindgen (method , structural , js_class = "NodeList" , js_name = values)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
    pub fn values(this: &NodeList) -> ::js_sys::Iterator;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, structural, js_class = "NodeList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
    pub fn get(this: &NodeList, index: u32) -> Option<Node>;
}
