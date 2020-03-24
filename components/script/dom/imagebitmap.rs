/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::ImageBitmapBinding::ImageBitmapMethods;
use crate::dom::bindings::root::DomRoot;
use crate::dom::globalscope::GlobalScope;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use dom_struct::dom_struct;
use std::vec::Vec;

#[dom_struct]
pub struct ImageBitmap {
    reflector_: Reflector,
    width: u32,
    height: u32,
    bitmap_data: DomRefCell<Vec<u8>>,
}

impl ImageBitmap {
    fn new_inherited(width_arg: u32, height_arg: u32) -> ImageBitmap {
        ImageBitmap {
            reflector_: Reflector::new(),
            width: width_arg,
            height: height_arg,
            bitmap_data: DomRefCell::new(vec![]),
        }
    }

    #[allow(dead_code)]
    pub fn new(global: &GlobalScope, width: u32, height: u32) -> Fallible<DomRoot<ImageBitmap>> {
        //assigning to a variable the return object of new_inherited
        let imagebitmap = Box::new(ImageBitmap::new_inherited(width, height));

        Ok(reflect_dom_object(imagebitmap, global))
    }
}

impl ImageBitmapMethods for ImageBitmap {
    // https://html.spec.whatwg.org/multipage/#dom-imagebitmap-height
    fn height(&self) -> u32 {
        //to do: add a condition for checking detached internal slot
        //and return 0 if set to true
        self.height
    }

    // https://html.spec.whatwg.org/multipage/#dom-imagebitmap-width
    fn width(&self) -> u32 {
        //to do: add a condition to check detached internal slot
        ////and return 0 if set to true
        self.width
    }

	//https://html.spec.whatwg.org/multipage/imagebitmap-and-animations.html#dom-imagebitmap-close
	fn close(&self) {
		//to do: set detached internal slot to true
		drop(self.bitmap_data);
	}
}
