// Copyright © 2020 Lukas Wagner

pub mod ast;

use super::parser::Parser;
use ast::Scope;
#[cfg(target_arch = "wasm32")]
use ast::ToCss;
#[cfg(not(target_arch = "wasm32"))]
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref STYLE_REGISTRY: Arc<Mutex<StyleRegistry>> = Arc::new(Mutex::default());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

/// The style registry is just a global struct that makes sure no style gets lost.
/// Every style automatically registers with the style registry.
#[derive(Clone, Debug, Default)]
struct StyleRegistry {
    styles: HashMap<String, Style>,
}

unsafe impl Send for StyleRegistry {}
unsafe impl Sync for StyleRegistry {}

#[cfg(all(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct Style {
    /// The designated class name of this style
    class_name: String,
    /// The abstract syntax tree of the css
    ast: Option<Vec<Scope>>,
    /// Style DOM node the data in this struct is turned into.
    node: Option<Element>,
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Style {
    /// The designated class name of this style
    class_name: String,
    /// The abstract syntax tree of the css
    ast: Option<Vec<Scope>>,
}

#[cfg(target_arch = "wasm32")]
impl Style {
    /// Creates a new style and, stores it into the registry and returns the
    /// newly created style.
    ///
    /// This function will already mount the style to the HTML head for the browser to use.
    pub fn create<I1: Into<String>, I2: Into<String>>(
        class_name: I1,
        css: I2,
    ) -> Result<Style, String> {
        let (class_name, css) = (class_name.into(), css.into());
        let ast = Parser::parse(css)?;
        let mut new_style = Self {
            class_name: format!("{}-{}", class_name, random().to_bits()),
            ast: Some(ast),
            node: None,
        };
        new_style = new_style.mount();
        let style_registry_mutex = Arc::clone(&STYLE_REGISTRY);
        let mut style_registry = match style_registry_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        (*style_registry)
            .styles
            .insert(new_style.class_name.clone(), new_style.clone());
        Ok(new_style)
    }

    pub fn get_class_name(self) -> String {
        self.class_name
    }

    /// Mounts the styles to the document head web-sys style
    fn mount(&mut self) -> Self {
        let mut style = self.unmount();
        style.node = self.generate_element().ok();
        if let Some(node) = style.node {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("should have a head in document");
            head.append_child(&node).ok();
        }
        self.clone()
    }

    /// Unmounts the style from the HTML head web-sys style
    fn unmount(&mut self) -> Self {
        if let Some(node) = &self.node {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let head = document.head().expect("should have a head in document");
            head.remove_child(node).ok();
        }
        self.clone()
    }

    /// Takes all Scopes and lets them translate themselves into CSS.
    fn generate_css(&self) -> String {
        match &self.ast {
            Some(ast) => ast
                .clone()
                .into_iter()
                .map(|scope| scope.to_css(self.class_name.clone()))
                .fold(String::new(), |acc, css_part| {
                    format!("{}\n{}", acc, css_part)
                }),
            None => String::new(),
        }
    }

    /// Generates the `<style/>` tag web-sys style for inserting into the head of the
    /// HTML document.
    fn generate_element(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let style_element = document.create_element("style").unwrap();
        style_element
            .set_attribute("data-style", self.class_name.as_str())
            .ok();
        style_element.set_text_content(Some(self.generate_css().as_str()));
        Ok(style_element)
    }
}

/// The style represents all the CSS belonging to a single component.
#[cfg(not(target_arch = "wasm32"))]
impl Style {
    /// Creates a new style and, stores it into the registry and returns the
    /// newly created style.
    ///
    /// This function will already mount the style to the HTML head for the browser to use.
    pub fn create<I1: Into<String>, I2: Into<String>>(
        class_name: I1,
        css: I2,
    ) -> Result<Style, String> {
        let (class_name, css) = (class_name.into(), css.into());
        let small_rng = SmallRng::from_entropy();
        let new_style = Self {
            class_name: format!(
                "{}-{}",
                class_name,
                small_rng
                    .sample_iter(Alphanumeric)
                    .take(30)
                    .map(|number| number.to_string())
                    .collect::<String>()
            ),
            // TODO log out an error
            ast: Parser::parse(css).ok(),
        };
        let style_registry_mutex = Arc::clone(&STYLE_REGISTRY);
        let mut style_registry = match style_registry_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        style_registry
            .styles
            .insert(new_style.class_name.clone(), new_style.clone());
        Ok(new_style)
    }

    pub fn get_class_name(self) -> String {
        self.class_name
    }
}

impl ToString for Style {
    /// Just returns the classname
    fn to_string(&self) -> String {
        self.class_name.clone()
    }
}
