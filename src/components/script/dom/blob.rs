/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{WrapperCache, BindingObject, CacheableWrapper, JSManaged};
use dom::bindings::codegen::BlobBinding;
use script_task::{task_from_context, global_script_context};

use js::jsapi::{JSContext, JSObject};

use std::cast;

pub struct Blob {
    wrapper: WrapperCache
}

impl Blob {
    pub fn new() -> JSManaged<Blob> {
        let blob = Blob {
            wrapper: WrapperCache::new()
        };
        JSManaged::new(blob)
    }
}

impl CacheableWrapper for Blob {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe { cast::transmute(&self.wrapper) }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        BlobBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for Blob {
    fn GetParentObject(&self, cx: *JSContext) -> *JSObject {
        let script_context = task_from_context(cx);
        unsafe {
            (*script_context).root_frame.get_ref().window.wrapper
        }
    }
}
