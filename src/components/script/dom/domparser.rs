/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::DOMParserBinding;
use dom::bindings::utils::{DOMString, ErrorResult, WrapperCache, JSManaged};
use dom::document::Document;
use dom::element::{Element, HTMLHtmlElement, HTMLHtmlElementTypeId};
use dom::node::Node;
use dom::window::Window;

pub struct DOMParser {
    owner: JSManaged<Window>, //XXXjdm Document instead?
    wrapper: WrapperCache
}

impl DOMParser {
    pub fn new(owner: &JSManaged<Window>) -> JSManaged<DOMParser> {
        let parser = DOMParser {
            owner: owner.clone(),
            wrapper: WrapperCache::new()
        };
        JSManaged::new(parser)
    }

    pub fn Constructor(owner: &JSManaged<Window>, _rv: &mut ErrorResult) -> JSManaged<DOMParser> {
        DOMParser::new(owner)
    }

    pub fn ParseFromString(&self,
                           _s: DOMString,
                           _type: DOMParserBinding::SupportedType,
                           _rv: &mut ErrorResult)
                           -> JSManaged<Document> {
        unsafe {
            let root = HTMLHtmlElement {
                parent: Element::new(HTMLHtmlElementTypeId, ~"html")
            };

            let root = Node::as_abstract_node(root);
            Document(root, None)
        }
    }
}

