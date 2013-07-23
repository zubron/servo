/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::MouseEventBinding;
use dom::bindings::utils::{ErrorResult, DOMString, JSManaged};
use dom::bindings::utils::{CacheableWrapper, WrapperCache, BindingObject};
use dom::eventtarget::EventTarget;
use dom::uievent::UIEvent;
use dom::window::Window;
use dom::windowproxy::WindowProxy;
use script_task::{global_script_context};

use js::jsapi::{JSObject, JSContext};

pub struct MouseEvent {
    parent: UIEvent,
    screen_x: i32,
    screen_y: i32,
    client_x: i32,
    client_y: i32,
    ctrl_key: bool,
    shift_key: bool,
    alt_key: bool,
    meta_key: bool,
    button: u16,
    related_target: Option<JSManaged<EventTarget>>
}

impl MouseEvent {
    pub fn new(type_: DOMString, can_bubble: bool, cancelable: bool,
               view: &Option<JSManaged<WindowProxy>>, detail: i32, screen_x: i32,
               screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool,
               shift_key: bool, alt_key: bool, meta_key: bool, button: u16,
               _buttons: u16, related_target: &Option<JSManaged<EventTarget>>)
        -> MouseEvent {
        MouseEvent {
            parent: UIEvent::new(type_, can_bubble, cancelable, view, detail),
            screen_x: screen_x,
            screen_y: screen_y,
            client_x: client_x,
            client_y: client_y,
            ctrl_key: ctrl_key,
            shift_key: shift_key,
            alt_key: alt_key,
            meta_key: meta_key,
            button: button,
            related_target: related_target.clone()
        }
    }

    pub fn Constructor(_owner: &JSManaged<Window>,
                       type_: DOMString,
                       init: &MouseEventBinding::MouseEventInit,
                       _rv: &mut ErrorResult) -> JSManaged<MouseEvent> {
        let ev = MouseEvent::new(type_, init.bubbles, init.cancelable, &init.view,
                                 init.detail, init.screenX, init.screenY, init.clientX,
                                 init.clientY, init.ctrlKey, init.shiftKey, init.altKey,
                                 init.metaKey, init.button, init.buttons,
                                 &init.relatedTarget);
        JSManaged::new(ev)
    }

    pub fn ScreenX(&self) -> i32 {
        self.screen_x
    }

    pub fn ScreenY(&self) -> i32 {
        self.screen_y
    }

    pub fn ClientX(&self) -> i32 {
        self.client_x
    }

    pub fn ClientY(&self) -> i32 {
        self.client_y
    }

    pub fn CtrlKey(&self) -> bool {
        self.ctrl_key
    }

    pub fn ShiftKey(&self) -> bool {
        self.shift_key
    }

    pub fn AltKey(&self) -> bool {
        self.alt_key
    }

    pub fn MetaKey(&self) -> bool {
        self.meta_key
    }

    pub fn Button(&self) -> u16 {
        self.button
    }

    pub fn Buttons(&self)-> u16 {
        //TODO
        0
    }

    pub fn GetRelatedTarget(&self) -> Option<JSManaged<EventTarget>> {
        self.related_target
    }

    pub fn GetModifierState(&self, _keyArg: DOMString) -> bool {
        //TODO
        false
    }

    pub fn InitMouseEvent(&mut self,
                          typeArg: DOMString,
                          canBubbleArg: bool,
                          cancelableArg: bool,
                          viewArg: &Option<JSManaged<WindowProxy>>,
                          detailArg: i32,
                          screenXArg: i32,
                          screenYArg: i32,
                          clientXArg: i32,
                          clientYArg: i32,
                          ctrlKeyArg: bool,
                          altKeyArg: bool,
                          shiftKeyArg: bool,
                          metaKeyArg: bool,
                          buttonArg: u16,
                          relatedTargetArg: &Option<JSManaged<EventTarget>>,
                          _rv: &mut ErrorResult) {
        self.parent.InitUIEvent(typeArg, canBubbleArg, cancelableArg, viewArg, detailArg);
        self.screen_x = screenXArg;
        self.screen_y = screenYArg;
        self.client_x = clientXArg;
        self.client_y = clientYArg;
        self.ctrl_key = ctrlKeyArg;
        self.alt_key = altKeyArg;
        self.shift_key = shiftKeyArg;
        self.meta_key = metaKeyArg;
        self.button = buttonArg;
        self.related_target = relatedTargetArg.clone();
    }
}

impl CacheableWrapper for MouseEvent {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        return self.parent.get_wrappercache()
    }

    fn wrap_object_shared(self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        MouseEventBinding::Wrap(cx, scope, self, &mut unused)
    }

    pub fn init_wrapper(self) -> *JSObject {
        let script_context = global_script_context();
        let cx = script_context.js_compartment.cx.ptr;
        let owner = script_context.root_frame.get_ref().window;
        self.wrap_object_shared(cx, owner.wrapper)
    }
}

impl BindingObject for MouseEvent {
    fn GetParentObject(&self, cx: *JSContext) -> *JSObject {
        self.parent.GetParentObject(cx)
    }
}
