/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::DOMParserBinding;
use dom::bindings::utils::{CacheableWrapper, WrapperCache, BindingObject};
use dom::domparser::DOMParser;
use script_task::global_script_context;

use js::jsapi::{JSContext, JSObject};

use std::cast;

impl CacheableWrapper for DOMParser {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe { cast::transmute(&self.wrapper) }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        DOMParserBinding::Wrap(cx, scope, self, &mut unused)
    }

    fn init_wrapper(self) -> *JSObject {
        let cx = global_script_context().js_compartment.cx.ptr;
        self.wrap_object_shared(cx, self.owner.wrapper)
    }
}

impl BindingObject for DOMParser {
    fn GetParentObject(&self, _cx: *JSContext) -> *JSObject {
        self.owner.wrapper
    }
}
