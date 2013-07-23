/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::node::{NodeBase, to_abstract_node};
use dom::bindings::utils::{jsval_to_str, WrapperCache, JSManaged, DOM_OBJECT_SLOT};
use dom::bindings::utils::{domstring_to_jsval, WrapNewBindingObject, unwrap};
use dom::bindings::utils::{str, CacheableWrapper, DOMString};
use dom::element::*;
use dom::node::{AbstractNode, Element, ElementNodeTypeId, ScriptView, Node, Comment};
use dom::node::{Text, Doctype};
use layout_interface::{ContentBoxQuery, ContentBoxResponse};
use script_task::task_from_context;
use super::utils;

use std::cast;
use std::i32;
use std::libc;
use std::libc::c_uint;
use std::comm;
use std::ptr;
use std::ptr::null;
use std::result;
use std::str;
use std::unstable::intrinsics;
use js::glue::*;
use js::jsapi::*;
use js::jsapi::{JSContext, JSVal, JSObject, JSBool, JSFreeOp, JSPropertySpec};
use js::jsapi::{JSNativeWrapper, JSTracer, JSTRACE_OBJECT};
use js::jsapi::{JSPropertyOpWrapper, JSStrictPropertyOpWrapper, JSFunctionSpec};
use js::rust::{Compartment, jsobj};
use js::{JS_ARGV, JSPROP_ENUMERATE, JSPROP_SHARED, JSVAL_NULL};
use js::{JS_THIS_OBJECT, JS_SET_RVAL, JSPROP_NATIVE_ACCESSORS};

//XXXjdm We need separate finalizers for each specialty element type like headings
extern fn finalize(_fop: *JSFreeOp, obj: *JSObject) {
    debug!("element finalize (0x%x)!", obj as uint);
    unsafe {
        let orig_elem = unwrap::<*mut Element>(obj);
        let elem = intrinsics::uninit();
        intrinsics::move_val(&mut *orig_elem, elem);
    }
}

pub extern fn trace(tracer: *mut JSTracer, obj: *JSObject) {
    fn trace_node(tracer: *mut JSTracer, node: Option<AbstractNode<ScriptView>>, name: &str) {
        if node.is_none() {
            return;
        }
        error!("tracing %s", name);
        let mut node = node.get();
        let cache = node.get_wrappercache();
        let wrapper = cache.get_wrapper();
        assert!(wrapper.is_not_null());
        unsafe {
            (*tracer).debugPrinter = ptr::null();
            (*tracer).debugPrintIndex = -1;
            do str::as_c_str(name) |name| {
                (*tracer).debugPrintArg = name as *libc::c_void;
                JS_CallTracer(cast::transmute(tracer), wrapper, JSTRACE_OBJECT as u32);
            }
        }
    }

    let node = to_abstract_node(obj);
    error!("tracing %?:", obj as uint);
    trace_node(tracer, node.parent_node(), "parent");
    trace_node(tracer, node.first_child(), "first child");
    trace_node(tracer, node.last_child(), "last child");
    trace_node(tracer, node.next_sibling(), "next sibling");
    trace_node(tracer, node.prev_sibling(), "prev sibling");
}

pub fn init(compartment: @mut Compartment) {
    JSManaged::sanity_check::<Element>();

    let obj = utils::define_empty_prototype(~"Element", Some(~"Node"), compartment);
    let attrs = @~[
        JSPropertySpec {
         name: compartment.add_name(~"tagName"),
         tinyid: 0,
         flags: (JSPROP_ENUMERATE | JSPROP_SHARED | JSPROP_NATIVE_ACCESSORS) as u8,
         getter: JSPropertyOpWrapper {op: getTagName, info: null()},
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
            JS_DefineProperties(compartment.cx.ptr, obj.ptr, specs);
        }
    }

    let methods = @~[JSFunctionSpec {name: compartment.add_name(~"getClientRects"),
                                     call: JSNativeWrapper {op: getClientRects, info: null()},
                                     nargs: 0,
                                     flags: 0,
                                     selfHostedName: null()},
                     JSFunctionSpec {name: compartment.add_name(~"getBoundingClientRect"),
                                     call: JSNativeWrapper {op: getBoundingClientRect, info: null()},
                                     nargs: 0,
                                     flags: 0,
                                     selfHostedName: null()},
                     JSFunctionSpec {name: compartment.add_name(~"setAttribute"),
                                     call: JSNativeWrapper {op: setAttribute, info: null()},
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

    compartment.register_class(utils::instance_jsclass(~"GenericElementInstance",
                                                       finalize, trace));

    let _ = utils::define_empty_prototype(~"HTMLElement", Some(~"Element"), compartment);
    let _ = utils::define_empty_prototype(~"HTMLDivElement", Some(~"HTMLElement"), compartment);
    let _ = utils::define_empty_prototype(~"HTMLScriptElement", Some(~"HTMLElement"), compartment);
    let _ = utils::define_empty_prototype(~"HTMLHeadElement", Some(~"HTMLElement"), compartment);

    let obj = utils::define_empty_prototype(~"HTMLImageElement", Some(~"HTMLElement"), compartment);
    let attrs = @~[
        JSPropertySpec {name: compartment.add_name(~"width"),
         tinyid: 0,
         flags: (JSPROP_SHARED | JSPROP_ENUMERATE | JSPROP_NATIVE_ACCESSORS) as u8,
         getter: JSPropertyOpWrapper {op: HTMLImageElement_getWidth, info: null()},
         setter: JSStrictPropertyOpWrapper {op: HTMLImageElement_setWidth, info: null()}},
        JSPropertySpec {name: null(),
         tinyid: 0,
         flags: (JSPROP_SHARED | JSPROP_ENUMERATE | JSPROP_NATIVE_ACCESSORS) as u8,
         getter: JSPropertyOpWrapper {op: null(), info: null()},
         setter: JSStrictPropertyOpWrapper {op: null(), info: null()}}];
    compartment.global_props.push(attrs);
    do attrs.as_imm_buf |specs, _len| {
        unsafe {
            JS_DefineProperties(compartment.cx.ptr, obj.ptr, specs);
        }
    }
}

extern fn getClientRects(cx: *JSContext, _argc: c_uint, vp: *JSVal) -> JSBool {
  unsafe {
      let obj = JS_THIS_OBJECT(cx, vp);
      let node = to_abstract_node(obj);
      let rval = do node.with_imm_element |elem| {
          elem.getClientRects()
      };
      if rval.is_none() {
          JS_SET_RVAL(cx, vp, JSVAL_NULL);
      } else {
          let rval = rval.get().wrapper;
          assert!(WrapNewBindingObject(cx, obj, rval, cast::transmute(vp)));
      }
      return 1;
  }
}

extern fn getBoundingClientRect(cx: *JSContext, _argc: c_uint, vp: *JSVal) -> JSBool {
  unsafe {
      let obj = JS_THIS_OBJECT(cx, vp);
      let node = to_abstract_node(obj);
      let rval = do node.with_imm_element |elem| {
          elem.getBoundingClientRect()
      };
      if rval.is_none() {
          JS_SET_RVAL(cx, vp, JSVAL_NULL);
      } else {
          assert!(WrapNewBindingObject(cx, obj, rval.get().wrapper, cast::transmute(vp)));
      }
      return 1;
  }
}

extern fn setAttribute(cx: *JSContext, argc: c_uint, vp: *JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, vp);
        let node = to_abstract_node(obj);

        if (argc < 2) {
            return 0; //XXXjdm throw exception
        }

        let argv = JS_ARGV(cx, cast::transmute(vp));

        let arg0: DOMString;
        let strval = jsval_to_str(cx, (*argv.offset(0)));
        if strval.is_err() {
            return 0;
        }
        arg0 = str(strval.get());

        let arg1: DOMString;
        let strval = jsval_to_str(cx, (*argv.offset(1)));
        if strval.is_err() {
            return 0;
        }
        arg1 = str(strval.get());

        do node.as_mut_element |elem| {
            elem.set_attr(&arg0, &arg1);
        };

        return 1;
    }
}

#[allow(non_implicitly_copyable_typarams)]
extern fn HTMLImageElement_getWidth(cx: *JSContext, _argc: c_uint, vp: *mut JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, cast::transmute(vp));
        if obj.is_null() {
            return 0;
        }

        let node = to_abstract_node(obj);
        let width = match node.type_id() {
            ElementNodeTypeId(HTMLImageElementTypeId) => {
                let script_context = task_from_context(cx);
                let (port, chan) = comm::stream();
                match (*script_context).query_layout(ContentBoxQuery(node, chan), port) {
                    Ok(ContentBoxResponse(rect)) => rect.size.width.to_px(),
                    Err(()) => 0
                }
                // TODO: if nothing is being rendered(?), return zero dimensions
            }
            ElementNodeTypeId(_) => fail!(~"why is this not an image element?"),
            _ => fail!(~"why is this not an element?")
        };

        *vp = RUST_INT_TO_JSVAL(
                (width & (i32::max_value as int)) as libc::c_int);
        return 1;
    }
}

#[allow(non_implicitly_copyable_typarams)]
extern fn HTMLImageElement_setWidth(cx: *JSContext, _argc: c_uint, vp: *mut JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, cast::transmute(vp));
        if obj.is_null() {
            return 0;
        }

        let node = to_abstract_node(obj);
        match node.type_id() {
            ElementNodeTypeId(HTMLImageElementTypeId) => {
                do node.as_mut_element |elem| {
                    let arg = ptr::offset(JS_ARGV(cx, cast::transmute(vp)), 0);
                    elem.set_attr(&str(~"width"),
                                  &str((RUST_JSVAL_TO_INT(*arg) as int).to_str()))
                }
            }
            ElementNodeTypeId(_) => fail!(~"why is this not an image element?"),
            _ => fail!(~"why is this not an element?")
        };

        return 1;
    }
}

extern fn getTagName(cx: *JSContext, _argc: c_uint, vp: *mut JSVal) -> JSBool {
    unsafe {
        let obj = JS_THIS_OBJECT(cx, cast::transmute(vp));
        if obj.is_null() {
            return 0;
        }

        let node = to_abstract_node(obj);
        do node.with_imm_element |elem| {
            let s = str(copy elem.tag_name);
            *vp = domstring_to_jsval(cx, &s);            
        }
    }
    return 1;
}

macro_rules! generate_node_base(($type_:path, $field:expr, $field2:expr) => (
    impl<View> NodeBase<View> for $type_ {
        fn base_node(&self) -> &Node<View> {
            //XXXjdm I am 100% done with ICEs resulting from 'self lifetime annotations
            unsafe { cast::transmute(&$field) }
        }

        fn base_node_mut(&mut self) -> &mut Node<View> {
            //XXXjdm I am 100% done with ICEs resulting from 'self lifetime annotations
            unsafe { cast::transmute(&mut $field2) }
        }
    }
))

generate_node_base!(Comment, self.parent.parent, self.parent.parent)
generate_node_base!(Doctype<ScriptView>, self.parent, self.parent)
generate_node_base!(Element, self.parent, self.parent)
generate_node_base!(Text, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLAnchorElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLAsideElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLBRElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLBodyElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLBoldElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLDivElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLFontElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLFormElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLHRElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLHeadElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLHeadingElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLHtmlElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLImageElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLInputElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLItalicElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLLinkElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLListItemElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLMetaElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLOListElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLOptionElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLParagraphElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLScriptElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLSectionElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLSelectElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLSmallElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLSpanElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLStyleElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLTableBodyElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLTableCellElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLTableElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLTableRowElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLTitleElement, self.parent.parent, self.parent.parent)
generate_node_base!(HTMLUListElement, self.parent.parent, self.parent.parent)
generate_node_base!(UnknownElement, self.parent.parent, self.parent.parent)

pub fn create<T: NodeBase<ScriptView>>(cx: *JSContext, mut node: T) -> jsobj {
    JSManaged::sanity_check::<T>();

    let proto = match node.base_node().type_id {
        ElementNodeTypeId(HTMLDivElementTypeId) => ~"HTMLDivElement",
        ElementNodeTypeId(HTMLHeadElementTypeId) => ~"HTMLHeadElement",
        ElementNodeTypeId(HTMLImageElementTypeId) => ~"HTMLImageElement",
        ElementNodeTypeId(HTMLScriptElementTypeId) => ~"HTMLScriptElement",
        ElementNodeTypeId(_) => ~"HTMLElement",
        _ => fail!(~"element::create only handles elements")
    };

    //XXXjdm the parent should probably be the node parent instead of the global
    //TODO error checking
    let compartment = utils::get_compartment(cx);
    let obj = result::unwrap(compartment.new_object_with_proto(~"GenericElementInstance",
                                                               proto,
                                                               compartment.global_obj.ptr));

    let cache = &mut node.base_node_mut().wrapper;
    assert!(cache.get_wrapper().is_null());
    cache.set_wrapper(obj.ptr);

    unsafe {
        let raw_storage: *mut T = GetInlineStorage(obj.ptr, DOM_OBJECT_SLOT) as *mut T;
        let storage: &mut T = &mut *raw_storage;
        debug!("(0x%x) storing in 0x%x", obj.ptr as uint, ptr::to_unsafe_ptr(storage) as uint);
        intrinsics::move_val_init(storage, node);
        debug!("%?", storage);
    }

    return obj;
}

macro_rules! generate_cacheable_wrapper(($type_:path) => (
    impl CacheableWrapper for $type_ {
        fn get_wrappercache(&mut self) -> &mut WrapperCache {
            let node: &mut Node<ScriptView> = self.base_node_mut();
            node.get_wrappercache()
        }

        fn wrap_object_shared(self, _cx: *JSContext, _scope: *JSObject) -> *JSObject {
            fail!(~"need to implement wrapping");
        }

        pub fn init_wrapper(self) -> *JSObject {
            //XXXjdm
            ptr::null()
        }
    }
))

generate_cacheable_wrapper!(Comment)
generate_cacheable_wrapper!(Doctype<ScriptView>)
generate_cacheable_wrapper!(Element)
generate_cacheable_wrapper!(HTMLAnchorElement)
generate_cacheable_wrapper!(HTMLAsideElement)
generate_cacheable_wrapper!(HTMLBRElement)
generate_cacheable_wrapper!(HTMLBodyElement)
generate_cacheable_wrapper!(HTMLBoldElement)
generate_cacheable_wrapper!(HTMLDivElement)
generate_cacheable_wrapper!(HTMLFontElement)
generate_cacheable_wrapper!(HTMLFormElement)
generate_cacheable_wrapper!(HTMLHRElement)
generate_cacheable_wrapper!(HTMLHeadElement)
generate_cacheable_wrapper!(HTMLHeadingElement)
generate_cacheable_wrapper!(HTMLHtmlElement)
generate_cacheable_wrapper!(HTMLImageElement)
generate_cacheable_wrapper!(HTMLInputElement)
generate_cacheable_wrapper!(HTMLItalicElement)
generate_cacheable_wrapper!(HTMLLinkElement)
generate_cacheable_wrapper!(HTMLListItemElement)
generate_cacheable_wrapper!(HTMLMetaElement)
generate_cacheable_wrapper!(HTMLOListElement)
generate_cacheable_wrapper!(HTMLOptionElement)
generate_cacheable_wrapper!(HTMLParagraphElement)
generate_cacheable_wrapper!(HTMLScriptElement)
generate_cacheable_wrapper!(HTMLSectionElement)
generate_cacheable_wrapper!(HTMLSelectElement)
generate_cacheable_wrapper!(HTMLSmallElement)
generate_cacheable_wrapper!(HTMLSpanElement)
generate_cacheable_wrapper!(HTMLStyleElement)
generate_cacheable_wrapper!(HTMLTableBodyElement)
generate_cacheable_wrapper!(HTMLTableCellElement)
generate_cacheable_wrapper!(HTMLTableElement)
generate_cacheable_wrapper!(HTMLTableRowElement)
generate_cacheable_wrapper!(HTMLTitleElement)
generate_cacheable_wrapper!(HTMLUListElement)
generate_cacheable_wrapper!(UnknownElement)
