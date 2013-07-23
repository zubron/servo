/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{CacheableWrapper, WrapperCache, BindingObject};
use dom::bindings::utils::JSManaged;
use dom::bindings::codegen::ClientRectBinding;
use script_task::{task_from_context, global_script_context};

use js::jsapi::{JSObject, JSContext};

use std::cast;
use std::f32;

pub struct ClientRect {
    wrapper: WrapperCache,
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl ClientRect {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> JSManaged<ClientRect> {
        let rect = ClientRect {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
            wrapper: WrapperCache::new()
        };
        JSManaged::new(rect)
    }

    pub fn Top(&self) -> f32 {
        self.top
    }

    pub fn Bottom(&self) -> f32 {
        self.bottom
    }

    pub fn Left(&self) -> f32 {
        self.left
    }

    pub fn Right(&self) -> f32 {
        self.right
    }

    pub fn Width(&self) -> f32 {
        f32::abs(self.right - self.left)
    }

    pub fn Height(&self) -> f32 {
        f32::abs(self.bottom - self.top)
    }
}

impl CacheableWrapper for ClientRect {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe {
            cast::transmute(&self.wrapper)
        }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        ClientRectBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for ClientRect {
    fn GetParentObject(&self, cx: *JSContext) -> *JSObject {
        let script_context = task_from_context(cx);
        unsafe {
            (*script_context).root_frame.get_ref().window.wrapper
        }
    }
}
