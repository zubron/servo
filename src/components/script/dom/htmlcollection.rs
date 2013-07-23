/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLCollectionBinding;
use dom::bindings::utils::{CacheableWrapper, BindingObject, WrapperCache};
use dom::bindings::utils::{DOMString, ErrorResult, JSManaged};
use dom::node::{AbstractNode, ScriptView};
use script_task::{task_from_context, global_script_context};

use js::jsapi::{JSObject, JSContext};

use std::cast;
use std::ptr;

pub struct HTMLCollection {
    elements: ~[AbstractNode<ScriptView>],
    wrapper: WrapperCache
}

impl HTMLCollection {
    pub fn new(elements: ~[AbstractNode<ScriptView>]) -> JSManaged<HTMLCollection> {
        let collection = HTMLCollection {
            elements: elements,
            wrapper: WrapperCache::new()
        };
        let coll = JSManaged::new(collection);
        assert!(coll.wrapper != ptr::null())
        coll
    }
    
    pub fn Length(&self) -> u32 {
        self.elements.len() as u32
    }

    pub fn Item(&self, index: u32) -> Option<AbstractNode<ScriptView>> {
        debug!("item!");
        if index < self.Length() {
            Some(self.elements[index])
        } else {
            None
        }
    }

    pub fn NamedItem(&self, _cx: *JSContext, _name: DOMString, rv: &mut ErrorResult) -> *JSObject {
        *rv = Ok(());
        ptr::null()
    }

    pub fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<AbstractNode<ScriptView>> {
        *found = true;
        self.Item(index)
    }
}

impl BindingObject for HTMLCollection {
    fn GetParentObject(&self, cx: *JSContext) -> *JSContext {
        let script_context = task_from_context(cx);
        unsafe {
            (*script_context).root_frame.get_ref().window.wrapper
        }
    }
}

impl CacheableWrapper for HTMLCollection {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe {
            cast::transmute(&self.wrapper)
        }
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        HTMLCollectionBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}
