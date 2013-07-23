/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{CacheableWrapper, BindingObject};
use dom::bindings::utils::{WrapperCache, DOMString, str, JSManaged};
use dom::bindings::codegen::FormDataBinding;
use dom::blob::Blob;
use script_task::{task_from_context, global_script_context};

use js::jsapi::{JSObject, JSContext};

use std::cast;
use std::hashmap::HashMap;

enum FormDatum {
    StringData(DOMString),
    BlobData { blob: JSManaged<Blob>, name: DOMString }
}

pub struct FormData {
    data: HashMap<~str, FormDatum>,
    wrapper: WrapperCache
}

impl FormData {
    pub fn new() -> JSManaged<FormData> {
        let data = FormData {
            data: HashMap::new(),
            wrapper: WrapperCache::new()
        };
        JSManaged::new(data)
    }

    pub fn Append(&mut self, name: DOMString, value: &JSManaged<Blob>, filename: Option<DOMString>) {
        let blob = BlobData {
            blob: value.clone(),
            name: filename.get_or_default(str(~"default"))
        };
        self.data.insert(name.to_str(), blob);
    }

    pub fn Append_(&mut self, name: DOMString, value: DOMString) {
        self.data.insert(name.to_str(), StringData(value));
    }
}

impl CacheableWrapper for FormData {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe {
            cast::transmute(&self.wrapper)
        }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        FormDataBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for FormData {
    fn GetParentObject(&self, cx: *JSContext) -> *JSContext {
        let script_context = task_from_context(cx);
        unsafe {
            (*script_context).root_frame.get_ref().window.wrapper
        }
    }
}