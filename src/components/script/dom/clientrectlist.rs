/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::ClientRectListBinding;
use dom::bindings::utils::{WrapperCache, CacheableWrapper, BindingObject, JSManaged};
use dom::clientrect::ClientRect;
use script_task::{task_from_context, global_script_context};

use js::jsapi::{JSObject, JSContext};

use std::cast;

pub struct ClientRectList {
    wrapper: WrapperCache,
    rects: ~[JSManaged<ClientRect>]
}

impl ClientRectList {
    pub fn new(rects: ~[JSManaged<ClientRect>]) -> JSManaged<ClientRectList> {
        let list = ClientRectList {
            wrapper: WrapperCache::new(),
            rects: rects
        };
        JSManaged::new(list)
    }

    pub fn Length(&self) -> u32 {
        self.rects.len() as u32
    }

    pub fn Item(&self, index: u32) -> Option<JSManaged<ClientRect>> {
        if index < self.rects.len() as u32 {
            Some(self.rects[index])
        } else {
            None
        }
    }

    pub fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<JSManaged<ClientRect>> {
        *found = index < self.rects.len() as u32;
        self.Item(index)
    }
}

impl CacheableWrapper for ClientRectList {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe {
            cast::transmute(&self.wrapper)
        }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        ClientRectListBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for ClientRectList {
    fn GetParentObject(&self, cx: *JSContext) -> *JSObject {
        let script_context = task_from_context(cx);
        unsafe {
            (*script_context).root_frame.get_ref().window.wrapper
        }
    }
}
