#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ToggleEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ToggleEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub type ToggleEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &ToggleEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &ToggleEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &ToggleEventInit, val: bool);
    #[wasm_bindgen(method, setter = "newState")]
    fn new_state_shim(this: &ToggleEventInit, val: &str);
    #[wasm_bindgen(method, setter = "oldState")]
    fn old_state_shim(this: &ToggleEventInit, val: &str);
}
impl ToggleEventInit {
    #[doc = "Construct a new `ToggleEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `newState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn new_state(&mut self, val: &str) -> &mut Self {
        self.new_state_shim(val);
        self
    }
    #[doc = "Change the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub fn old_state(&mut self, val: &str) -> &mut Self {
        self.old_state_shim(val);
        self
    }
}
impl Default for ToggleEventInit {
    fn default() -> Self {
        Self::new()
    }
}
