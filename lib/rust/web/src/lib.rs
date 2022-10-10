//! This module implements web bindings. It heavily uses [`wasm_bindgen`] and extends it with many
//! high-level features and bug-fixes. It also provides a mock API version allowing the native
//! compilation in order to run native tests of code which uses this API.

// === Features ===
#![allow(incomplete_features)]
#![feature(default_free_fn)]
#![feature(trait_alias)]
#![feature(negative_impls)]
#![feature(specialization)]
#![feature(auto_traits)]
#![feature(unsize)]
// === Standard Linter Configuration ===
#![deny(non_ascii_idents)]
#![warn(unsafe_code)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::let_and_return)]
// === Non-Standard Linter Configuration ===
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use crate::prelude::*;

use wasm_bindgen::prelude::wasm_bindgen;


// ==============
// === Export ===
// ==============

pub mod binding;
pub mod clipboard;
pub mod closure;
pub mod event;
pub mod platform;
pub mod resize_observer;
pub mod stream;

pub use std::time::Duration;
pub use std::time::Instant;



/// Common types that should be visible across the whole crate.
pub mod prelude {
    pub use super::traits::*;

    pub use super::Closure;
    pub use super::EventTarget;
    pub use super::Function;
    pub use super::HtmlDivElement;
    pub use super::HtmlElement;
    pub use super::JsCast;
    pub use super::JsValue;
    pub use super::Object;
    pub use enso_shapely::clone_ref::*;
    pub use std::cell::RefCell;
    pub use std::default::default;
    pub use std::fmt::Debug;
    pub use std::marker::PhantomData;
    pub use std::ops::Deref;
    pub use std::rc::Rc;
    pub use tracing;
    pub use tracing::debug;
    pub use tracing::warn;
}



// ===================
// === API Imports ===
// ===================

#[cfg(target_arch = "wasm32")]
pub use binding::wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use binding::mock::*;



// ==============
// === Traits ===
// ==============

macro_rules! gen_trait_modules {
    ( $($name:ident),* $(,)?) => {
        /// WASM-target oriented traits. Extending the possibilities of wasm-bindgen structures.
        pub mod wasm_traits {
            $(pub use super::$name::WasmTrait as $name;)*
            pub use super::binding::wasm::JsCast;
        }

        /// Mock traits. Counterpart of [`wasm_traits`].
        pub mod mock_traits {
            $(pub use super::$name::MockTrait as $name;)*
            pub use super::binding::mock::JsCast;
        }

        /// Both wasm and mock traits, unnamed.
        pub mod anonymous_traits {
            $(pub use super::$name::WasmTrait as _;)*
            $(pub use super::$name::MockTrait as _;)*
        }
    };
}

gen_trait_modules! {
    ClosureOps,
    DocumentOps,
    ElementOps,
    FunctionOps,
    HtmlCanvasElementOps,
    HtmlElementOps,
    JsValueOps,
    NodeOps,
    ObjectOps,
    ReflectOps,
    WindowOps,
}

/// All traits defined in this module.
pub mod traits {
    pub use super::anonymous_traits::*;
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::mock_traits::*;
    #[cfg(target_arch = "wasm32")]
    pub use super::wasm_traits::*;
}

/// Helper for generating extensions to web API targeting the Wasm32 architecture (defined by
/// [`wasm_bindgen`]) and the mock API defined in this library (imitating [`wasm-bindgen`]).
///
/// For each extension definition, it creates two traits, each for one of these APIs. These traits
/// are then re-exported in modules generated by [`gen_trait_modules`]. In particular, all Wasm32
/// extensions are grouped in the [`wasm_traits`] module, the mock extensions are grouped in the
/// [`mock_traits`] module, and they are merged together in the [`anonymous_traits`] module. The
/// [`traits`] module contains [`wasm_traits`] or [`mock_traits`] if it was compiled for Wasm32
/// or native architecture, respectively.
///
/// This macro usage contains and required and three optional sections:
/// - The `trait` section (required) section provides functions, just like ordinary trait
///   definition.
/// - The `impl` section (optional) provides implementations of the trait functions which are copied
///   to both the Wasm32 and the mock traits.
/// - The `wasm_impl` section (optional) provides implementations of the trait functions which are
///   copied to both the Wasm32 trait only.
/// - The `wasm_mock` section (optional) provides implementations of the trait functions which are
///   copied to both the mock trait only.
///
/// For example, the following usage:
///
/// ```text
/// ops! { JsValueOps for JsValue
///     trait {
///         fn print_to_string(&self) -> String;
///         fn test(&self);
///     }
///
///     impl {
///         fn test() {
///             println!("test");
///         }
///     }
///
///     wasm_impl {
///         fn print_to_string(&self) -> String {
///             super::js_print_to_string(self)
///         }
///     }
///
///     mock_impl {
///         fn print_to_string(&self) -> String {
///             "JsValue".into()
///         }
///     }
/// }
/// ```
///
/// Generates the following output:
///
/// ```text
/// pub mod JsValueOps {
///     use super::*;
///
///     pub mod wasm {
///         use super::binding::wasm::*;
///         use super::wasm_traits::*;
///         use enso_prelude::*;
///
///         pub trait Trait {
///             fn print_to_string(&self) -> String;
///             fn test(&self);
///         }
///
///
///         impl Trait for JsValue {
///             fn test() {
///                 println!("test")
///             }
///
///             fn print_to_string(&self) -> String {
///                 super::js_print_to_string(self)
///             }
///         }
///     }
///
///     pub mod mock {
///         use super::binding::mock::*;
///         use super::mock_traits::*;
///         use enso_prelude::*;
///
///         pub trait Trait {
///             fn print_to_string(&self) -> String;
///             fn test(&self);
///         }
///
///         impl Trait for JsValue {
///             fn test() {
///                 println!("test")
///             }
///
///             fn print_to_string(&self) -> String {
///                 "JsValue".into()
///             }
///         }
///     }
///
///     pub use self::mock::Trait as MockTrait;
///     pub use self::wasm::Trait as WasmTrait;
/// }
/// ```
macro_rules! ops {
    ($(<$($arg:ident : ($($arg_tp:tt)*)),*>)? $trait:ident for $target:ident
    trait $defs:tt
    $(impl {$($body:tt)*})?
    $(wasm_impl {$($wasm_body:tt)*})?
    $(mock_impl {$($mock_body:tt)*})?
    ) => {
        /// [`$trait`] extensions.
        #[allow(non_snake_case)]
        #[allow(missing_docs)]
        #[allow(unused_imports)]
        pub mod $trait {
            use super::*;

            /// WASM bindings.
            pub mod wasm {
                use super::binding::wasm::*;
                use super::wasm_traits::*;
                pub use tracing;
                pub use tracing::warn;
                pub use std::default::default;
                /// Extensions to the [`$target`] type.
                pub trait Trait $defs
                impl $(<$($arg: $($arg_tp)*),*>)? Trait for $target $(<$($arg),*>)?
                    {$($($body)*)? $($($wasm_body)*)?}
            }

            /// Mock bindings.
            pub mod mock {
                use super::binding::mock::*;
                use super::mock_traits::*;
                pub use tracing;
                pub use tracing::warn;
                pub use std::default::default;
                /// Extensions to the [`$target`] type.
                pub trait Trait $defs
                impl $(<$($arg: $($arg_tp)*),*>)? Trait for $target $(<$($arg),*>)?
                    {$($($body)*)? $($($mock_body)*)?}
            }
            pub use self::wasm::Trait as WasmTrait;
            pub use self::mock::Trait as MockTrait;
        }
    };
}



// ==================
// === JsValueOps ===
// ==================

ops! { JsValueOps for JsValue
    trait {
        /// Converts **any** `JsValue` into a `String`. Uses JS's `String` function,
        /// see: https://www.w3schools.com/jsref/jsref_string.asp
        fn print_to_string(&self) -> String;
    }

    wasm_impl {
        fn print_to_string(&self) -> String {
            super::js_print_to_string(self)
        }
    }

    mock_impl {
        fn print_to_string(&self) -> String {
            "JsValue".into()
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[allow(unsafe_code)]
    #[wasm_bindgen(js_name = "String")]
    #[allow(unused_qualifications)]
    fn js_print_to_string(s: &binding::wasm::JsValue) -> String;
}



// ==================
// === ClosureOps ===
// ==================

ops! {<T: (?Sized)> ClosureOps for Closure
    trait {
        fn as_js_function(&self) -> &Function;
    }
    impl {
        fn as_js_function(&self) -> &Function {
            self.as_ref().unchecked_ref()
        }
    }
}



// ===================
// === FunctionOps ===
// ===================


ops! { FunctionOps for Function
    trait {
        /// The `wasm-bindgen` version of this function panics if the JS code contains errors. This
        /// issue was reported and never fixed (https://github.com/rustwasm/wasm-bindgen/issues/2496).
        /// There is also a long-standing PR with the fix that was not fixed either
        /// (https://github.com/rustwasm/wasm-bindgen/pull/2497).
        fn new_with_args_fixed(args: &str, body: &str) -> Result<Function, JsValue>;
    }

    wasm_impl {
        fn new_with_args_fixed(args: &str, body: &str) -> Result<Function, JsValue> {
            new_function_with_args(args, body)
        }
    }

    mock_impl {
        crate::mock_fn! {new_with_args_fixed(_args: &str, _body: &str) -> Result<Function, JsValue>}
    }
}


// ==================
// === ReflectOps ===
// ==================

ops! { ReflectOps for Reflect
    trait {
        /// Get the nested value of the provided object. This is similar to writing `foo.bar.baz` in
        /// JavaScript, but in a safe manner, while checking if the value exists on each level.
        fn get_nested(target: &JsValue, keys: &[&str]) -> Result<JsValue, JsValue>;

        /// Get the nested value of the provided object and cast it to [`Object`]. See docs of
        /// [`get_nested`] to learn more.
        fn get_nested_object(target: &JsValue, keys: &[&str]) -> Result<Object, JsValue>;

        /// Get the nested value of the provided object. In case the object does not exist, they
        /// will be created. See docs of [`get_nested`] to learn more.
         fn get_nested_or_create(target: &JsValue, keys: &[&str]) -> Result<JsValue, JsValue>;

         /// Get the nested value of the provided object and cast it to [`Object`]. In case the
         /// object does not exist, they will be created. See docs of [`get_nested`] to learn more.
         fn get_nested_object_or_create(target: &JsValue, keys: &[&str]) -> Result<Object, JsValue>;

        /// Get the nested value of the provided object and cast it to [`String`]. See docs of
        /// [`get_nested`] to learn more.
        fn get_nested_object_printed_as_string(target: &JsValue, keys: &[&str])
            -> Result<String, JsValue>;
    }

    impl {
        fn get_nested(target: &JsValue, keys: &[&str]) -> Result<JsValue, JsValue> {
            let mut tgt = target.clone();
            for key in keys {
                let obj = tgt.dyn_into::<Object>()?;
                let key = (*key).into();
                tgt = Reflect::get(&obj, &key)?;
            }
            Ok(tgt)
        }

        fn get_nested_object(target: &JsValue, keys: &[&str]) -> Result<Object, JsValue> {
            let tgt = Self::get_nested(target, keys)?;
            tgt.dyn_into()
        }

        fn get_nested_or_create
         (target: &JsValue, keys: &[&str]) -> Result<JsValue, JsValue> {
             let mut tgt = target.clone();
             for key in keys {
                 let obj = tgt.dyn_into::<Object>()?;
                 let key = (*key).into();
                 match Reflect::get(&obj, &key) {
                     Ok(v) => {
                         if v.is_undefined() || v.is_null() {
                             tgt = Object::new().into();
                             Reflect::set(&obj, &key, &tgt)?;
                         } else {
                             tgt = v;
                         }
                     }
                     Err(_) => {
                         tgt = Object::new().into();
                         Reflect::set(&obj, &key, &tgt)?;
                     }
                 }
             }
             Ok(tgt)
         }

         fn get_nested_object_or_create(target: &JsValue, keys: &[&str]) -> Result<Object, JsValue> {
             let tgt = Self::get_nested_or_create(target, keys)?;
             tgt.dyn_into()
         }

        fn get_nested_object_printed_as_string
        (target: &JsValue, keys: &[&str]) -> Result<String, JsValue> {
            let tgt = Self::get_nested(target, keys)?;
            if tgt.is_undefined() {
                Err(Error::new("Key was not present in the target.").into())
            } else {
                Ok(tgt.print_to_string())
            }
        }
    }
}


// =================
// === WindowOps ===
// =================

ops! { WindowOps for Window
    trait {
        fn request_animation_frame_with_closure(
            &self,
            f: &Closure<dyn FnMut(f64)>,
        ) -> Result<i32, JsValue>;
        fn request_animation_frame_with_closure_or_panic(&self, f: &Closure<dyn FnMut(f64)>) -> i32;
        fn cancel_animation_frame_or_warn(&self, id: i32);
        fn performance_or_panic(&self) -> Performance;
    }

    impl {
        fn request_animation_frame_with_closure(
            &self,
            f: &Closure<dyn FnMut(f64)>,
        ) -> Result<i32, JsValue> {
            self.request_animation_frame(f.as_js_function())
        }

        fn request_animation_frame_with_closure_or_panic
        (&self, f: &Closure<dyn FnMut(f64)>) -> i32 {
            self.request_animation_frame_with_closure(f).unwrap()
        }

        fn cancel_animation_frame_or_warn(&self, id: i32) {
            self.cancel_animation_frame(id).unwrap_or_else(|err| {
                tracing::error!("Error when canceling animation frame: {err:?}");
            });
        }

        fn performance_or_panic(&self) -> Performance {
            self.performance().unwrap_or_else(|| panic!("Cannot access window.performance."))
        }
    }
}



// =================
// === ObjectOps ===
// =================

ops! { ObjectOps for Object
    trait {
        /// Get all the keys of the provided [`Object`].
        fn keys_vec(obj: &Object) -> Vec<String>;
    }

    wasm_impl {
        fn keys_vec(obj: &Object) -> Vec<String> {
            // The [`unwrap`] is safe, the `Object::keys` API guarantees it.
            Object::keys(obj)
                .iter()
                .map(|key| key.dyn_into::<JsString>().unwrap().into())
                .collect()
        }
    }

    mock_impl {
        fn keys_vec(_obj: &Object) -> Vec<String> {
            default()
        }
    }
}



// ===================
// === DocumentOps ===
// ===================

ops! { DocumentOps for Document
    trait {
        fn body_or_panic(&self) -> HtmlElement;
        fn create_element_or_panic(&self, local_name: &str) -> Element;
        fn create_div_or_panic(&self) -> HtmlDivElement;
        fn create_canvas_or_panic(&self) -> HtmlCanvasElement;
        fn get_html_element_by_id(&self, id: &str) -> Option<HtmlElement>;
        fn with_element_by_id_or_warn<F: FnOnce(Element)>(&self, id: &str, f: F);
    }

    impl {
        fn body_or_panic(&self) -> HtmlElement {
            self.body().unwrap()
        }

        fn create_element_or_panic(&self, local_name: &str) -> Element {
            self.create_element(local_name).unwrap()
        }

        fn create_div_or_panic(&self) -> HtmlDivElement {
            self.create_element_or_panic("div").unchecked_into()
        }

        fn create_canvas_or_panic(&self) -> HtmlCanvasElement {
            self.create_element_or_panic("canvas").unchecked_into()
        }

        fn get_html_element_by_id(&self, id: &str) -> Option<HtmlElement> {
            self.get_element_by_id(id).and_then(|t| t.dyn_into().ok())
        }

        fn with_element_by_id_or_warn<F: FnOnce(Element)>(&self, id: &str, f: F) {
            let root_elem = self.get_element_by_id(id);
            match root_elem {
                Some(v) => f(v),
                None => warn!("Failed to get element by ID."),
            }
        }
    }
}



// ===============
// === NodeOps ===
// ===============

ops! { NodeOps for Node
    trait {
        fn append_or_warn(&self, node: &Self);
        fn prepend_or_warn(&self, node: &Self);
        fn insert_before_or_warn(&self, node: &Self, reference_node: &Self);
        fn remove_from_parent_or_warn(&self);
        fn remove_child_or_warn(&self, node: &Self);
    }

    impl {
        fn append_or_warn(&self, node: &Self) {
            let warn_msg: &str = &format!("Failed to append child {:?} to {:?}", node, self);
            if self.append_child(node).is_err() {
                warn!(warn_msg)
            };
        }

        fn prepend_or_warn(&self, node: &Self) {
            let warn_msg: &str = &format!("Failed to prepend child \"{:?}\" to \"{:?}\"", node, self);
            let first_c = self.first_child();
            if self.insert_before(node, first_c.as_ref()).is_err() {
                warn!(warn_msg)
            }
        }

        fn insert_before_or_warn(&self, node: &Self, ref_node: &Self) {
            let warn_msg: &str =
                &format!("Failed to insert {:?} before {:?} in {:?}", node, ref_node, self);
            if self.insert_before(node, Some(ref_node)).is_err() {
                warn!(warn_msg)
            }
        }

        fn remove_from_parent_or_warn(&self) {
            if let Some(parent) = self.parent_node() {
                let warn_msg: &str = &format!("Failed to remove {:?} from parent", self);
                if parent.remove_child(self).is_err() {
                    warn!(warn_msg)
                }
            }
        }

        fn remove_child_or_warn(&self, node: &Self) {
            let warn_msg: &str = &format!("Failed to remove child {:?} from {:?}", node, self);
            if self.remove_child(node).is_err() {
                warn!(warn_msg)
            }
        }
    }
}



// ==================
// === ElementOps ===
// ==================

ops! { ElementOps for Element
    trait {
        fn set_attribute_or_warn<T: AsRef<str>, U: AsRef<str>>(&self, name: T, value: U);
    }

    impl {
        fn set_attribute_or_warn<T: AsRef<str>, U: AsRef<str>>(&self, name: T, value: U) {
            let name = name.as_ref();
            let value = value.as_ref();
            let values = format!("\"{}\" = \"{}\" on \"{:?}\"", name, value, self);
            let warn_msg: &str = &format!("Failed to set attribute {}", values);
            if self.set_attribute(name, value).is_err() {
                warn!(warn_msg)
            }
        }
    }
}



// ======================
// === HtmlElementOps ===
// ======================

ops! { HtmlElementOps for HtmlElement
    trait {
        fn set_style_or_warn(&self, name: impl AsRef<str>, value: impl AsRef<str>);
    }

    impl {
        fn set_style_or_warn(&self, name: impl AsRef<str>, value: impl AsRef<str>) {
            let name = name.as_ref();
            let value = value.as_ref();
            let values = format!("\"{}\" = \"{}\" on \"{:?}\"", name, value, self);
            let warn_msg: &str = &format!("Failed to set style {}", values);
            if self.style().set_property(name, value).is_err() {
                warn!(warn_msg);
            }
        }
    }
}



// =========================
// === HtmlCanvasElement ===
// =========================

ops! { HtmlCanvasElementOps for HtmlCanvasElement
    trait {
        fn get_webgl2_context(&self) -> Option<WebGl2RenderingContext>;
    }

    wasm_impl {
        fn get_webgl2_context(&self) -> Option<WebGl2RenderingContext> {
            let options = Object::new();
            Reflect::set(&options, &"antialias".into(), &false.into()).unwrap();
            let context = self.get_context_with_context_options("webgl2", &options).ok().flatten();
            context.and_then(|obj| obj.dyn_into::<WebGl2RenderingContext>().ok())
        }
    }

    mock_impl {
        fn get_webgl2_context(&self) -> Option<WebGl2RenderingContext> {
            None
        }
    }
}



// =============
// === Utils ===
// =============

/// Ignores context menu when clicking with the right mouse button.
pub fn ignore_context_menu(target: &EventTarget) -> EventListenerHandle {
    let closure: Closure<dyn FnMut(MouseEvent)> = Closure::new(move |event: MouseEvent| {
        const RIGHT_MOUSE_BUTTON: i16 = 2;
        if event.button() == RIGHT_MOUSE_BUTTON {
            event.prevent_default();
        }
    });
    add_event_listener_with_bool(target, "contextmenu", closure, true)
}



// =======================
// === Event Listeners ===
// =======================

/// The type of closures used for 'add_event_listener_*' functions.
pub type JsEventHandler<T = JsValue> = Closure<dyn FnMut(T)>;

/// Handler for event listeners. Unregisters the listener when the last clone is dropped.
#[derive(Clone, CloneRef)]
pub struct EventListenerHandle {
    rc: Rc<EventListenerHandleData>,
}

impl Debug for EventListenerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventListenerHandle")
    }
}

impl EventListenerHandle {
    /// Constructor.
    pub fn new<T: ?Sized + 'static>(
        target: EventTarget,
        name: Rc<String>,
        closure: Closure<T>,
    ) -> Self {
        let closure = Box::new(closure);
        let data = EventListenerHandleData { target, name, closure };
        let rc = Rc::new(data);
        Self { rc }
    }
}

/// Internal structure for [`EventListenerHandle`].
///
/// # Implementation Notes
/// The [`_closure`] field contains a wasm_bindgen's [`Closure<T>`]. Dropping it causes the
/// associated function to be pruned from memory.
struct EventListenerHandleData {
    target:  EventTarget,
    name:    Rc<String>,
    closure: Box<dyn traits::ClosureOps>,
}

impl Drop for EventListenerHandleData {
    fn drop(&mut self) {
        let function = self.closure.as_js_function();
        self.target.remove_event_listener_with_callback(&self.name, function).ok();
    }
}

macro_rules! gen_add_event_listener {
    ($name:ident, $wbindgen_name:ident $(,$arg:ident : $tp:ty)*) => {
        /// Wrapper for the function defined in web_sys which allows passing wasm_bindgen
        /// [`Closure`] directly.
        pub fn $name<T: ?Sized + 'static>(
            target: &EventTarget,
            name: &str,
            closure: Closure<T>
            $(,$arg : $tp)*
        ) -> EventListenerHandle {
            // Please note that using [`ok`] is safe here, as according to MDN this function never
            // fails: https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener.
            target.$wbindgen_name(name, closure.as_js_function() $(,$arg)*).ok();
            let target = target.clone();
            let name = Rc::new(name.to_string());
            EventListenerHandle::new(target, name, closure)
        }
    };
}

gen_add_event_listener!(add_event_listener, add_event_listener_with_callback);
gen_add_event_listener!(
    add_event_listener_with_bool,
    add_event_listener_with_callback_and_bool,
    options: bool
);
gen_add_event_listener!(
    add_event_listener_with_options,
    add_event_listener_with_callback_and_add_event_listener_options,
    options: &AddEventListenerOptions
);



// =========================
// === Stack Trace Limit ===
// =========================

/// Increases the JavaScript stack trace limit to make errors more understandable.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "
    export function set_stack_trace_limit() {
        Error.stackTraceLimit = 100
    }
")]
extern "C" {
    #[allow(unsafe_code)]
    pub fn set_stack_trace_limit();
}

/// Increases the JavaScript stack trace limit to make errors more understandable.
#[cfg(not(target_arch = "wasm32"))]
pub fn set_stack_trace_limit() {}



// ============
// === Time ===
// ============

static mut START_TIME: Option<Instant> = None;
static mut TIME_OFFSET: f64 = 0.0;

/// Initializes global stats of the program, like its start time. This function should be called
/// exactly once, as the first operation of a program.
///
/// # Safety
/// This function modifies a global variable, however, it should be safe as it should be called
/// exactly once on program entry point.
#[allow(unsafe_code)]
pub fn init() -> Instant {
    unsafe {
        let now = Instant::now();
        START_TIME = Some(now);
        now
    }
}

/// Start time of the program. Please note that the program should call the `init` function as
/// its first operation.
///
/// # Safety
/// The following modifies a global variable, however, even in case of a race condition, nothing
/// bad should happen (the variable may be initialized several times). Moreover, the variable
/// should be initialized on program start, so this should be always safe.
#[allow(unsafe_code)]
pub fn start_time() -> Instant {
    unsafe {
        match START_TIME {
            Some(time) => time,
            None => init(),
        }
    }
}

/// Time difference between the start time and current point in time.
///
/// # Safety
/// The following code will always be safe if the program called the `init` function on entry.
/// Even if that did not happen, the worst thing that may happen is re-initialization of the
/// program start time variable.
#[allow(unsafe_code)]
#[cfg(target_arch = "wasm32")]
pub fn time_from_start() -> f64 {
    unsafe { window.performance_or_panic().now() + TIME_OFFSET }
}

/// Time difference between the start time and current point in time.
///
/// # Safety
/// The following code will always be safe if the program called the `init` function on entry.
/// Even if that did not happen, the worst thing that may happen is re-initialization of the
/// program start time variable.
#[allow(unsafe_code)]
#[cfg(not(target_arch = "wasm32"))]
pub fn time_from_start() -> f64 {
    unsafe { start_time().elapsed().as_millis() as f64 + TIME_OFFSET }
}

/// Simulates a time interval. This function will exit immediately, but the next time you will
/// check the `time_from_start`, it will be increased.
///
/// # Safety
/// This function is safe only in single-threaded environments.
#[allow(unsafe_code)]
pub fn simulate_sleep(duration: f64) {
    unsafe { TIME_OFFSET += duration }
}



// =============
// === Panic ===
// =============

/// Enables forwarding panic messages to `console.error`.
#[cfg(target_arch = "wasm32")]
pub fn forward_panic_hook_to_console() {
    std::panic::set_hook(Box::new(report_panic))
}

/// Enables forwarding panic messages to `console.error`.
#[cfg(not(target_arch = "wasm32"))]
pub fn forward_panic_hook_to_console() {}

#[cfg(target_arch = "wasm32")]
fn report_panic(info: &std::panic::PanicInfo) {
    // Formats the info to display properly in the browser console. See crate docs for details.
    let msg = console_error_panic_hook::format_panic(info);
    if let Some(api) = enso_debug_api::console() {
        api.error(&msg);
    }
    web_sys::console::error_1(&msg.into());
}



// =============
// === Sleep ===
// =============

#[cfg(target_arch = "wasm32")]
/// Sleeps for the specified amount of time.
///
/// This function might sleep for slightly longer than the specified duration but never less. This
/// function is an async version of std::thread::sleep, its timer starts just after the function
/// call.
pub async fn sleep(duration: Duration) {
    use gloo_timers::future::TimeoutFuture;
    TimeoutFuture::new(duration.as_millis() as u32).await
}

#[cfg(not(target_arch = "wasm32"))]
pub use async_std::task::sleep;



// ====================
// === TimeProvider ===
// ====================

/// Trait for an entity that can retrieve current time.
pub trait TimeProvider {
    /// Returns current time, measured in milliseconds.
    fn now(&self) -> f64;
}

impl TimeProvider for Performance {
    fn now(&self) -> f64 {
        self.now()
    }
}
