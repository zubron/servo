/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::UIEventBinding;
use dom::bindings::utils::{ErrorResult, DOMString, JSManaged};
use dom::bindings::utils::{CacheableWrapper, WrapperCache, BindingObject};
use dom::node::{AbstractNode, ScriptView};
use dom::event::Event_;
use dom::window::Window;
use dom::windowproxy::WindowProxy;

use script_task::global_script_context;

use js::jsapi::{JSObject, JSContext};

pub struct UIEvent {
    parent: Event_,
    can_bubble: bool,
    cancelable: bool,
    view: Option<JSManaged<WindowProxy>>,
    detail: i32
}

impl UIEvent {
    pub fn new(type_: DOMString, can_bubble: bool, cancelable: bool,
               view: &Option<JSManaged<WindowProxy>>, detail: i32) -> UIEvent {
        UIEvent {
            parent: Event_::new(type_),
            can_bubble: can_bubble,
            cancelable: cancelable,
            view: view.clone(),
            detail: detail
        }
    }

    pub fn Constructor(_owner: &JSManaged<Window>,
                       type_: DOMString,
                       init: &UIEventBinding::UIEventInit,
                       _rv: &mut ErrorResult) -> JSManaged<UIEvent> {
        let ev = UIEvent::new(type_, init.parent.bubbles, init.parent.cancelable,
                                  &init.view, init.detail);
        JSManaged::new(ev)
    }

    pub fn GetView(&self) -> Option<JSManaged<WindowProxy>> {
        self.view
    }

    pub fn Detail(&self) -> i32 {
        self.detail
    }

    pub fn InitUIEvent(&mut self,
                       type_: DOMString,
                       can_bubble: bool,
                       cancelable: bool,
                       view: &Option<JSManaged<WindowProxy>>,
                       detail: i32) {
        let mut rv = Ok(());
        self.parent.InitEvent(type_, can_bubble, cancelable, &mut rv);
        self.can_bubble = can_bubble;
        self.cancelable = cancelable;
        self.view = view.clone();
        self.detail = detail;
    }

    pub fn LayerX(&self) -> i32 {
        //TODO
        0
    }

    pub fn LayerY(&self) -> i32 {
        //TODO
        0
    }

    pub fn PageX(&self) -> i32 {
        //TODO
        0
    }

    pub fn PageY(&self) -> i32 {
        //TODO
        0
    }

    pub fn Which(&self) -> u32 {
        //TODO
        0
    }

    pub fn GetRangeParent(&self) -> Option<AbstractNode<ScriptView>> {
        //TODO
        None
    }

    pub fn RangeOffset(&self) -> i32 {
        //TODO
        0
    }

    pub fn CancelBubble(&self) -> bool {
        //TODO
        false
    }

    pub fn SetCancelBubble(&mut self, _val: bool) {
        //TODO
    }

    pub fn IsChar(&self) -> bool {
        //TODO
        false
    }
}

impl CacheableWrapper for UIEvent {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        return self.parent.get_wrappercache()
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        UIEventBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for UIEvent {
    fn GetParentObject(&self, cx: *JSContext) -> *JSObject {
        self.parent.GetParentObject(cx)
    }
}
