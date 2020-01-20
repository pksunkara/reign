//! Reign View is a templating library for Rust inspired by
//! [Vue.js](https://vuejs.org) templates.
//!
//! This library makes using templates as easy as pie.
//! It uses HTML based template syntax that are valid and can
//! be parsed by spec-compliant browsers and HTML parsers.
//! It has been developed foremost with ease of use in mind and
//! then future extensibility immediately afterward it.
//!
//! This library also provides multiple feature gates which an user
//! can use to customize, allowing the library to be used directly
//! with multiple web frameworks like [gotham][], [rocket][],
//! [actix][], [warp][], [tide][] and [nickel][].
//!
//! # Table of contents
//!
//! * [Quickstart](#quickstart)
//! * [How it works](#how-it-works)
//! * [Template Syntax](#template-syntax)
//! * [Helpers & Feature Gates](#helpers--feature-gates)
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
//!         render!(crate::views::pages::About)
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
//!         // Then it will implement std::format::Display for it
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
//! > **NOTE:** The above expansion is approximate. There might be small changes
//! in the way they were expanded or other hidden things that are for internal use.
//!
//! You can read more about template syntax below [here](#template-syntax)
//!
//! ### Rendering
//!
//! When you try to render a template using the following:
//!
//! ```ignore
//! # #![feature(proc_macro_hygiene)]
//! use reign::prelude::*;
//!
//! let (name, age) = ("Pavan", 28);
//!
//! # let response =
//! render!(crate::views::pages::About);
//! ```
//!
//! The library expands the `render!` macro to something like the following:
//!
//! ```ignore
//! # let (name, age) = ("Pavan", 28);
//! # let response =
//! format!("{}", crate::views::pages::About {
//!     name: name,
//!     age: age,
//! });
//! ```
//!
//! > **NOTE:** This expansion is when no default features are enabled
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
//! # Helpers & Feature Gates
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
