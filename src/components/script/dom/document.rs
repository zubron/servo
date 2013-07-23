/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::document;
use dom::bindings::utils::{DOMString, WrapperCache, JSManaged};
use dom::htmlcollection::HTMLCollection;
use dom::node::{AbstractNode, ScriptView};
use dom::window::Window;
use script_task::global_script_context;

use js::jsapi::{JS_AddObjectRoot, JS_RemoveObjectRoot, JS_GetRuntime, JS_GC};
use servo_util::tree::{TreeNodeRef, TreeUtils};

pub struct Document {
    root: AbstractNode<ScriptView>,
    wrapper: WrapperCache,
    window: Option<JSManaged<Window>>,
}

impl Drop for Document {
    fn drop(&self) {
        debug!("dropping document with wrapper 0x%x", self.wrapper.wrapper as uint)
    }
}

pub fn Document(root: AbstractNode<ScriptView>, window: Option<JSManaged<Window>>) -> JSManaged<Document> {
    unsafe {
        let doc = Document {
            root: root,
            wrapper: WrapperCache::new(),
            window: window
        };
        let compartment = global_script_context().js_compartment;
        do root.with_base |base| {
            assert!(base.wrapper.get_wrapper().is_not_null());
            let rootable = base.wrapper.get_rootable();
            JS_AddObjectRoot(compartment.cx.ptr, rootable);
        }
        document::create(compartment, doc)
    }
}

impl Document {
    pub fn getElementsByTagName(&self, tag: DOMString) -> Option<JSManaged<HTMLCollection>> {
        debug!("getElementByTagName");
        let mut elements = ~[];
        let tag = tag.to_str();
        let _ = for self.root.traverse_preorder |child| {
            if child.is_element() {
                do child.with_imm_element |elem| {
                    if elem.tag_name == tag {
                        elements.push(child);
                    }
                }
            }
        };
        Some(HTMLCollection::new(elements))
    }

    pub fn content_changed(&self) {
        for self.window.iter().advance |window| {
            do window.with_imm |win| {
                win.content_changed()
            }
        }
    }

    pub fn teardown(&self) {
        unsafe {
            let compartment = global_script_context().js_compartment;
            do self.root.with_base |node| {
                assert!(node.wrapper.get_wrapper().is_not_null());
                let rootable = node.wrapper.get_rootable();
                JS_RemoveObjectRoot(compartment.cx.ptr, rootable);
            }
            let runtime = JS_GetRuntime(compartment.cx.ptr);
            JS_GC(runtime);
        }
    }
}

