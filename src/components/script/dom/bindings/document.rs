/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cast;
use std::ptr;
use std::result;
use std::unstable::intrinsics;
use dom::bindings::utils::{DOMString, str, DOM_OBJECT_SLOT, unwrap};
use dom::bindings::utils::{WrapperCache, JSManaged};
use dom::bindings::utils::{jsval_to_str, WrapNewBindingObject, CacheableWrapper};
use dom::bindings::utils;
use dom::document::Document;
use dom::htmlcollection::HTMLCollection;
use js::glue::*;
use js::glue::{PROPERTY_STUB, STRICT_PROPERTY_STUB};
use js::jsapi::{JS_DefineProperties, JS_DefineFunctions};
use js::jsapi::{JSContext, JSVal, JSObject, JSBool, JSFreeOp, JSPropertySpec, JSPropertyOpWrapper};
use js::jsapi::{JSStrictPropertyOpWrapper, JSNativeWrapper, JSFunctionSpec};
use js::rust::{Compartment, jsobj};
use js::{JSPROP_NATIVE_ACCESSORS};
use js::{JS_ARGV, JSPROP_ENUMERATE, JSPROP_SHARED, JSVAL_NULL, JS_THIS_OBJECT, JS_SET_RVAL};

use std::libc::c_uint;
use std::ptr::null;

extern fn getDocumentElement(cx: *JSContext, _argc: c_uint, vp: *mut JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, cast::transmute(vp));
        if obj.is_null() {
            return 0;
        }

        let wrapper = JSManaged::from_raw::<Document>(obj);
        do wrapper.with_mut |doc| {
            let root = &mut doc.root;
            assert!(root.is_element());
            root.wrap(cx, ptr::null(), vp); //XXXjdm proper scope at some point
        }
        return 1;
    }
}

extern fn getElementsByTagName(cx: *JSContext, _argc: c_uint, vp: *JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, vp);

        let argv = JS_ARGV(cx, cast::transmute(vp));

        let arg0: DOMString;
        let strval = jsval_to_str(cx, (*argv.offset(0)));
        if strval.is_err() {
            return 0;
        }
        arg0 = str(strval.get());

        let doc = JSManaged::from_raw::<Document>(obj);
        let rval: Option<JSManaged<HTMLCollection>>;
        rval = do doc.with_imm |doc| { doc.getElementsByTagName(arg0.clone()) };
        if rval.is_none() {
            JS_SET_RVAL(cx, vp, JSVAL_NULL);
        } else {
            let rval = rval.get().wrapper;
            assert!(WrapNewBindingObject(cx, obj, rval,
                                         cast::transmute(vp)));
        }
        return 1;
    }
}

extern fn finalize(_fop: *JSFreeOp, obj: *JSObject) {
    debug!("document finalize (0x%x)!", obj as uint);
    unsafe {
        let orig_doc = unwrap::<*mut Document>(obj);
        let doc = intrinsics::uninit();
        intrinsics::move_val(&mut *orig_doc, doc);
    }
}

pub fn init(compartment: @mut Compartment) {
    JSManaged::sanity_check::<Document>();

    let obj = utils::define_empty_prototype(~"Document", None, compartment);

    let attrs = @~[
        JSPropertySpec {
         name: compartment.add_name(~"documentElement"),
         tinyid: 0,
         flags: (JSPROP_SHARED | JSPROP_ENUMERATE | JSPROP_NATIVE_ACCESSORS) as u8,
         getter: JSPropertyOpWrapper {op: getDocumentElement, info: null()},
         setter: JSStrictPropertyOpWrapper {op: null(), info: null()}},
        JSPropertySpec {
         name: null(),
         tinyid: 0,
         flags: (JSPROP_SHARED | JSPROP_ENUMERATE | JSPROP_NATIVE_ACCESSORS) as u8,
         getter: JSPropertyOpWrapper {op: null(), info: null()},
         setter: JSStrictPropertyOpWrapper {op: null(), info: null()}}];
    compartment.global_props.push(attrs);
    do attrs.as_imm_buf |specs, _len| {
        unsafe {
            assert!(JS_DefineProperties(compartment.cx.ptr, obj.ptr, specs) == 1);
        }
    }

    let methods = @~[JSFunctionSpec {name: compartment.add_name(~"getElementsByTagName"),
                                     call: JSNativeWrapper {op: getElementsByTagName, info: null()},
                                     nargs: 0,
                                     flags: 0,
                                     selfHostedName: null()},
                     JSFunctionSpec {name: null(),
                                     call: JSNativeWrapper {op: null(), info: null()},
                                     nargs: 0,
                                     flags: 0,
                                     selfHostedName: null()}];
    do methods.as_imm_buf |fns, _len| {
        unsafe {
            JS_DefineFunctions(compartment.cx.ptr, obj.ptr, fns);
        }
    }

    compartment.register_class(utils::instance_jsclass(~"DocumentInstance",
                                                       finalize,
                                                       ptr::null()));
}

pub fn create(compartment: @mut Compartment, mut doc: Document) -> JSManaged<Document> {
    let instance : jsobj = result::unwrap(
        compartment.new_object_with_proto(~"DocumentInstance", ~"Document",
                                          compartment.global_obj.ptr));
    doc.wrapper.set_wrapper(instance.ptr);

    unsafe {
        let raw_storage: *mut Document = GetInlineStorage(instance.ptr, DOM_OBJECT_SLOT) as *mut Document;
        let storage: &mut Document = &mut *raw_storage;
        intrinsics::move_val_init(storage, doc);

        compartment.define_property(~"document", RUST_OBJECT_TO_JSVAL(instance.ptr),
                                    GetJSClassHookStubPointer(PROPERTY_STUB) as *u8,
                                    GetJSClassHookStubPointer(STRICT_PROPERTY_STUB) as *u8,
                                    JSPROP_ENUMERATE);
    }
    JSManaged::from_raw(instance.ptr)
}

impl CacheableWrapper for Document {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe { cast::transmute(&self.wrapper) }
    }

    fn wrap_object_shared(self, _cx: *JSContext, _scope: *JSObject) -> *JSObject {
        fail!("nyi")
    }

    fn init_wrapper(self) -> *JSObject {
        //XXXjdm
        ptr::null()
    }
}

