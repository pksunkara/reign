//! Reign View is a component based templating library for Rust
//! inspired by [Vue.js](https://vuejs.org) templates.
//!
//! This library makes using templates as easy as pie.
//! It uses HTML based template syntax that are valid and can
//! be parsed by spec-compliant browsers and HTML parsers.
//! It has been developed foremost with ease of use in mind followed
//! by future extensibility, modularization and customization.
//!
//! This library also provides multiple helpers and feature gates
//! which an user can use to customize, allowing the library to be
//! used directly with multiple web frameworks like [gotham][],
//! [rocket][], [actix][], [warp][], [tide][] and [nickel][].
//!
//! # Table of contents
//!
//! * [Quickstart](#quickstart)
//! * [How it works](#how-it-works)
//! * [Template Syntax](#template-syntax)
//! * [Components](#components)
//! * [Helpers & Feature Gates](#helpers--feature-gates)
//! * [Appendix](#appendix)
//!
//! # Quickstart
//!
//! 1. Add [reign](https://docs.rs/reign) to your code base with
//!    default features ommitted
//!
//!     ```toml
//!     [dependencies]
//!     reign = { version = "*", default-features = false }
//!     ```
//!
//! 2. Initiate the templates in your `main.rs`
//!
//!     ```
//!     #![feature(proc_macro_hygiene)]
//!     #![feature(type_ascription)]
//!     use reign::prelude::*;
//!
//!     // If your templates live under `src/views` folder
//!     views!("src", "views");
//!     ```
//!
//! 3. Write a template in `src/views/pages/about.html`
//!
//!     ```html
//!     <p>
//!       {{ name }}
//!       <sub>aged {{ age: u8 }}</sub>
//!     </p>
//!     ```
//!
//! 4. Render the template
//!
//!     ```ignore
//!     # #![feature(proc_macro_hygiene)]
//!     use reign::prelude::*;
//!
//!     # mod views {
//!     #     mod pages {
//!     #         pub struct About<'a> {
//!     #             pub name: &'a str,
//!     #             pub age: u8,
//!     #         }
//!     #     }
//!     # }
//!     #
//!     fn handle() -> String {
//!         let (name, age) = ("pksunkara", 28);
//!
//!         // The macro automatically captures all the
//!         // variables it needs and returns a String
//!         render!(views::pages::About)
//!     }
//!     ```
//!
//! # How it works
//!
//! There are multiple components that goes into this templating
//! library.
//!
//! * Building a Struct out of the HTML template.
//! * Rendering the Struct into a String.
//!
//! ### Building
//!
//! Let's assume that you have written the template described in the
//! previous section at the respective path.
//!
//! Now, when you initiate the templating library by writing the following:
//!
//! ```
//! #![feature(proc_macro_hygiene)]
//! #![feature(type_ascription)]
//! use reign::prelude::*;
//!
//! views!("src", "views");
//! ```
//!
//! The library expands the `views!` macro to something like the following:
//!
//! ```
//! // It will create a module which is always called `views`
//! mod views {
//!
//!     // Then it will create a module for `pages` subdirectory
//!     mod pages {
//!
//!         // Then it will create a struct for the template file
//!         pub struct About<'a> {
//!
//!             // It will add a raw string field for every variable
//!             // found which does not have a type described
//!             pub name: &'a str,
//!
//!             // It will add a typed field for every variable found
//!             // that was given a type (once in a template is enough)
//!             pub age: u8,
//!         }
//!
//!         use std::fmt::{Display, Formatter, Result};
//!
//!         // Then it will implement std::fmt::Display for it
//!         impl Display for About<'_> {
//!             fn fmt(&self, f: &mut Formatter) -> Result {
//!                 write!(f, "{}{}{}{}{}",
//!                     "<p>\n  ", self.name, "\n  <sub>aged ", self.age, "</sub>\n</p>"
//!                 )
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! **NOTE:** The above expansion is approximate. There might be small changes
//! in the way they were expanded or other hidden things that are for internal use.
//!
//! You can read more about template syntax below [here](#template-syntax)
//!
//! ### Rendering
//!
//! When no default features are enabled and when you try to render a template
//! like the following:
//!
//! ```ignore
//! # #![feature(proc_macro_hygiene)]
//! use reign::prelude::*;
//!
//! let (name, age) = ("Pavan", 28);
//! #
//! # pub mod views {
//! #     pub mod pages {
//! #         pub struct About<'a> {
//! #             pub name: &'a str,
//! #             pub age: u8,
//! #         }
//! #         impl std::fmt::Display for About<'_> {
//! #             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//! #                 Ok(())
//! #             }
//! #         }
//! #     }
//! # }
//!
//! render!(views::pages::About);
//! ```
//!
//! The library expands the `render!` macro to something like the following:
//!
//! ```
//! # let (name, age) = ("Pavan", 28);
//! # pub mod views {
//! #     pub mod pages {
//! #         pub struct About<'a> {
//! #             pub name: &'a str,
//! #             pub age: u8,
//! #         }
//! #         impl std::fmt::Display for About<'_> {
//! #             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//! #                 Ok(())
//! #             }
//! #         }
//! #     }
//! # }
//! format!("{}", views::pages::About {
//!     name: name,
//!     age: age,
//! });
//! ```
//!
//! Which returns the following String:
//!
//! ```html
//! <p>
//!   Pavan
//!   <sub>aged 28</sub>
//! <p>
//! ```
//!
//! # Template Syntax
//!
//! Before we start talking about the template syntax,
//! let's agree on a few terms for this section so that it
//! will be easier to refer to them later on.
//!
//! An **expression** is a custom subset of all the types
//! of expressions available in Rust language. You can read
//! more about them [here](#expressions).
//!
//! A **pattern** is a custom rust pattern syntax where the
//! expressions allowed are the only ones defined in the above
//! paragraph. You can read more about them [here](#patterns)
//!
//! A **field** refers to a field of the struct that is built
//! for the template by the `views!` macro when initiating
//! the template library.
//!
//! All html style tags that are used in the template should be
//! closed either by a self closing syntax or an end tag. The only
//! exception are the tags which are allowed by HTML spec to be
//! self closing by default called **void elements**.
//!
//! ### Text
//!
//! The most basic form of templating is *"interpolation"*
//! using the *"mustache"* syntax (double curly braces).
//!
//! ```html
//! <span>Message: {{ msg }}</span>
//! ```
//!
//! The mustache tag will be replaced with the value of the
//! `msg` *field*. You can also use an *expression* inside the
//! mustache tags. Any type that has `std::fmt::Display` implemented
//! can be the final result of the *expression* defined by the mustache
//! tags.
//!
//! ```html
//! <span>Word Count: {{ msg.len() }}</span>
//! ```
//!
//! ### Attributes
//!
//! Interpolation can also be used in values of attributes.
//!
//! ```html
//! <div title="Application - {{ page_name }}"></div>
//! ```
//!
//! If you want to use `"` inside the attribute value for an
//! expression, you can refer to the HTML spec and surround the
//! value with `'`.
//!
//! ```html
//! <div title='Application - {{ "Welcome" }}'></div>
//! ```
//!
//! ### Variable Attributes
//!
//! If you want to have an attribute that is completely interpolated
//! with just one mustache tag and nothing else, you can do this.
//!
//! ```html
//! <div :title="page_name"></div>
//! ```
//!
//! The value of the attribute `title` will be the value of `page_name` field.
//!
//! ### Control Attributes
//!
//! The library doesn't allow `if` conditions and `for` loops as expressions
//! inside the mustache tags. You can use a control attribute on html tags to do this.
//!
//! ```html
//! <div !if='page_name == "home"'>Welcome</div>
//! <div !else>{{ page_name }}</div>
//! ```
//!
//! The above template prints either the first or the second `div` depending
//! on the expression provided to the `!if` control attribute.
//!
//! **NOTE:** `!else-if` is also supported as you would expect.
//!
//! `for` loops also make use of control attribute syntax like shown below.
//!
//! ```html
//! <div !for="char in page_name.chars()">{{ char }}</div>
//! ```
//!
//! The above template prints `div` tags for all the characters in `page_name`
//! field. Other than the name difference in the control attribute, `!for` needs
//! "**pattern *in* expression**" syntax.
//!
//! ### Grouping Elements
//!
//! Because `!if` and `!for` are attributes, they need to be attached to a single
//! tag. But sometimes, you might need to render more than one element. In that
//! case, you can use the `template` tag which works like an invisible wrapper.
//!
//! ```html
//! <template v-if="condition">
//!   <h1>{{ title }}</h1>
//!   <p>{{ content }}</p>
//! </template>
//! ```
//!
//! **NOTE:** A template doesn't allow multiple elements to be defined at the root.
//! Which means, you can use the `template` tag to group elements at the root of
//! the template.
//!
//! ```html
//! <template>
//!   <!DOCTYPE html>
//!   <html></html>
//! </template>
//! ```
//!
//! ### Class & Style bindings
//!
//! To be implemented
//!
//! # Components
//!
//! TODO:(doc)
//!
//! ### Slots
//!
//! TODO:(doc)
//!
//! # Helpers & Feature Gates
//!
//! TODO:(doc)
//!
//! # Appendix
//!
//! ### Expressions
//!
//! TODO:(doc)
//!
//! ### Patterns
//!
//! TODO:(doc)
//!
//! [gotham]: https://gotham.rs
//! [rocket]: https://rocket.rs
//! [actix]: https://actix.rs
//! [warp]: https://docs.rs/warp
//! [tide]: https://docs.rs/tide
//! [nickel]: https://nickel.rs
#![feature(external_doc)]
#![doc(html_root_url = "https://docs.rs/reign_view/0.1.2")]

#[doc(include = "../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

use std::fmt::{self, write};

#[doc(hidden)]
pub use maplit;

pub mod parse;
mod slots;

pub use slots::{SlotRender, Slots};

#[cfg(feature = "views-gotham")]
use gotham::{
    helpers::http::response::{create_empty_response, create_response},
    hyper::{Body, Response, StatusCode},
    state::State,
};

/// Renders a view for [gotham](https://gotham.rs) handler.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if the crate is built with the `"views-gotham"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_gotham;
/// use std::fmt::{Formatter, Result, Display};
/// use gotham::state::State;
/// use gotham::hyper::{Body, Response};
///
/// struct CustomView<'a> {
///     msg: &'a str
/// }
///
/// impl Display for CustomView<'_> {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         write!(f, "<h1>{}</h1>", self.msg)
///     }
/// }
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render_gotham(state, CustomView {
///         msg: "Hello World!"
///     })
/// }
/// #
/// # use gotham::test::TestServer;
/// # use gotham::hyper::StatusCode;
/// #
/// # let test_server = TestServer::new(|| Ok(handler)).unwrap();
/// # let response = test_server
/// #     .client()
/// #     .get("http://localhost")
/// #     .perform()
/// #     .unwrap();
/// #
/// # assert_eq!(response.status(), StatusCode::OK);
/// #
/// # let body = response.read_body().unwrap();
/// # assert_eq!(&body[..], b"<h1>Hello World!</h1>");
/// ```
#[cfg(feature = "views-gotham")]
pub fn render_gotham<D: fmt::Display>(state: State, view: D) -> (State, Response<Body>) {
    let mut content = String::new();

    let response = match write(&mut content, format_args!("{}", view)) {
        Ok(()) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, response)
}

/// Renders a view for [warp](https://docs.rs/warp) closure.
#[cfg(feature = "views-warp")]
pub fn render_warp<D: fmt::Display>(view: D) {}
