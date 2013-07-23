/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::element;
use dom::bindings::node::NodeBase;
use dom::bindings::utils;
use dom::bindings::utils::{DOM_OBJECT_SLOT, CacheableWrapper, WrapperCache, JSManaged, unwrap};
use dom::node::{Text, Comment, Doctype, TextNodeTypeId, CommentNodeTypeId};
use dom::node::{DoctypeNodeTypeId, ScriptView};

use js::jsapi::{JSFreeOp, JSObject, JSContext};
use js::glue::{GetInlineStorage};
use js::rust::{Compartment, jsobj};

use std::ptr;
use std::result;
use std::unstable::intrinsics;

extern fn finalize_text(_fop: *JSFreeOp, obj: *JSObject) {
    debug!("text finalize (0x%x)!", obj as uint);
    unsafe {
        let orig_text = unwrap::<*mut Text>(obj);
        let text = intrinsics::uninit();
        intrinsics::move_val(&mut *orig_text, text);
    }
}

extern fn finalize_comment(_fop: *JSFreeOp, obj: *JSObject) {
    debug!("comment finalize (0x%x)!", obj as uint);
    unsafe {
        let orig_comment = unwrap::<*mut Comment>(obj);
        let comment = intrinsics::uninit();
        intrinsics::move_val(&mut *orig_comment, comment);
    }
}

extern fn finalize_doctype(_fop: *JSFreeOp, obj: *JSObject) {
    debug!("doctype finalize (0x%x)!", obj as uint);
    unsafe {
        let orig_doctype = unwrap::<*mut Doctype<ScriptView>>(obj);
        let doctype = intrinsics::uninit();
        intrinsics::move_val(&mut *orig_doctype, doctype);
    }
}

pub fn init(compartment: @mut Compartment) {
    JSManaged::sanity_check::<Comment>();
    JSManaged::sanity_check::<Text>();
    JSManaged::sanity_check::<Doctype<ScriptView>>();

    let _ = utils::define_empty_prototype(~"CharacterData", Some(~"Node"), compartment);
    
    let _ = utils::define_empty_prototype(~"TextPrototype",
                                          Some(~"CharacterData"),
                                          compartment);
    let _ = utils::define_empty_prototype(~"CommentPrototype",
                                          Some(~"CharacterData"),
                                          compartment);
    let _ = utils::define_empty_prototype(~"DocumentTypePrototype",
                                          Some(~"Node"),
                                          compartment);

    compartment.register_class(utils::instance_jsclass(~"Text",
                                                       finalize_text,
                                                       element::trace));
    compartment.register_class(utils::instance_jsclass(~"Comment",
                                                       finalize_comment,
                                                       element::trace));
    compartment.register_class(utils::instance_jsclass(~"DocumentType",
                                                       finalize_doctype,
                                                       element::trace));

    
}

pub fn create<T: NodeBase<ScriptView>>(cx: *JSContext, mut node: T) -> jsobj {
    let (proto, instance) = match node.base_node().type_id {
      TextNodeTypeId => (~"TextPrototype", ~"Text"),
      CommentNodeTypeId => (~"CommentPrototype", ~"Comment"),
      DoctypeNodeTypeId => (~"DocumentTypePrototype", ~"DocumentType"),
      _ => fail!(~"text::create only handles textual nodes")
    };

    //XXXjdm the parent should probably be the node parent instead of the global
    //TODO error checking
    let compartment = utils::get_compartment(cx);
    let obj = result::unwrap(compartment.new_object_with_proto(instance,
                                                               proto,
                                                               compartment.global_obj.ptr));

    let cache = node.base_node_mut().get_wrappercache();
    assert!(cache.get_wrapper().is_null());
    cache.set_wrapper(obj.ptr);

    unsafe {
        let raw_storage: *mut T = GetInlineStorage(obj.ptr, DOM_OBJECT_SLOT) as *mut T;
        let storage: &mut T = &mut *raw_storage;
        debug!("(0x%x) storing in 0x%x", obj.ptr as uint, ptr::to_unsafe_ptr(storage) as uint);
        intrinsics::move_val_init(storage, node);
    }

    return obj;
}

impl CacheableWrapper for Text {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        self.parent.parent.get_wrappercache()
    }

    fn wrap_object_shared(self, _cx: *JSContext, _scope: *JSObject) -> *JSObject {
        fail!(~"need to implement wrapping");
    }

    pub fn init_wrapper(self) -> *JSObject {
        //XXXjdm
        ptr::null()
    }
}
