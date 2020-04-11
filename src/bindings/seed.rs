// Copyright © 2020 Lukas Wagner

//! Seed integration module.
//! The user doesn't need to do anything but to add a style into a
//! seed component.

#[cfg(target_arch = "wasm32")]
extern crate seed;

#[cfg(target_arch = "wasm32")]
use super::super::style::Style;
#[cfg(target_arch = "wasm32")]
use seed::virtual_dom::{At, Attrs};

#[cfg(target_arch = "wasm32")]
impl From<Style> for Attrs {
    fn from(style: Style) -> Self {
        let mut attrs = Self::empty();
        let mut classes = Vec::new();
        classes.push(style.class_name.as_str());
        attrs.add_multiple(At::Class, &classes);
        attrs
    }
}
