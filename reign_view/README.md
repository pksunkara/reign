Reign View is a component based HTML templating library for Rust inspired by
[Vue.js](https://vuejs.org) templates.

This library makes using templates as easy as pie. It uses HTML based template
syntax that are valid and can be parsed by spec-compliant browsers and HTML parsers
<sup>[[1][ann]]</sup><sup>[[2][ann]]</sup>. It has been developed foremost with ease
of use in mind followed by future extensibility, modularization and customization.

This library also provides multiple helpers and feature gates which an user can use
to customize, allowing the library to be used directly with or without [reign_router][].

Please refer to [Guides](https://reign.rs/guides) on several how-to scenairos.

Please refer to [API documentation](https://docs.rs/reign_view) for more details.

**NOTE**: Minimum supported Rust version is **1.45.0**

# Table of contents

* [Quickstart](#quickstart)
* [How it works](#how-it-works)
* [Template Syntax](#template-syntax)
* [Components](#components)
* [Helpers & Feature Gates](#helpers--feature-gates)
* [Appendix](#appendix)

# Quickstart

1. Add [Reign][] to your code base with default features ommitted and `view` feature enabled

    ```toml
    [dependencies]
    reign = { version = "*", features = ["view"], default-features = false }
    ```

2. Initiate the templates in your `main.rs`

    ```rust,ignore
    use reign::prelude::*;

    // If your templates live under `src/views` folder
    views!("src", "views");
    ```

3. Write a template in `src/views/pages/about.html`

    ```html
    <p>
      {{ name }}
      <sub>aged {{ age: u8 }}</sub>
    </p>
    ```

4. Render the template

    ```rust,ignore
    use reign::prelude::*;

    let (name, age) = ("John", 28);

    // The macro automatically captures all the
    // variables it needs, builds the view to display
    // and returns a `String`
    //
    // `pages::about` is the unique path of the above template
    render!(pages::about)
    ```

# How it works

There are multiple steps that goes into this templating library.

* Building a *view* out of the HTML template.
* Rendering the *view* into a String.

### Building

Let's assume that you have written the template described in the
previous section at the respective path.

Now, when you initiate the templating library by writing the following:

```rust,ignore
use reign::prelude::*;

views!("src", "views");
```

The library expands the `views!` macro to something like the following:

```rust
// It will create a module which is always called `views`
mod views {

    // Then it will create a module for `pages` subdirectory
    pub mod pages {

        // Then it will create a struct for the template file
        pub struct About<'a> {

            // It will add a raw string field for every variable
            // found which does not have a type described
            pub name: &'a str,

            // It will add a typed field for every variable found
            // that was given a type (once in a template is enough)
            pub age: u8,
        }

        use std::fmt::{Display, Formatter, Result};

        // Then it will implement std::fmt::Display for it
        impl Display for About<'_> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "{}{}{}{}{}",
                    "<p>\n  ", self.name, "\n  <sub>aged ", self.age, "</sub>\n</p>"
                )
            }
        }
    }
}
```

The above expansion is approximate. There might be small changes in the
way they were expanded or other hidden things that are for internal use.

You can read more about template syntax below [here](#template-syntax)

### Rendering

When the plain view feature is enabled and when you try to render a template
like the following:

```rust,ignore
use reign::prelude::*;

let (name, age) = ("John", 28);

render!(pages::about);
```

The library expands the `render!` macro to something like the following:

```rust,ignore
format!("{}", crate::views::pages::About {
    name: name.as_ref(),
    age,
});
```

Which returns the following String:

```html
<p>
  John
  <sub>aged 28</sub>
<p>
```

# Template Syntax

Before we start talking about the template syntax, let's agree on a few terms
for this section so that it will be easier to refer to them later on.

An **expression** is a custom subset of all the types of expressions available
in Rust language. You can read about them [here](#expressions).

A **pattern** is a custom rust pattern syntax where the expressions allowed are
the only ones defined in the above paragraph. You can read more about them
[here](#patterns).

A **field** refers to a field of the struct that is built for the template by
the `views!` macro when initiating the template library.

All html style tags that are used in the template should be closed either by a
self closing syntax or an end tag. The only exception are the tags which are
allowed by HTML spec to be self closing by default called **void elements**.

### Text

The most basic form of templating is *"interpolation"* using the *"mustache"*
syntax (double curly braces).

```html
<span>Message: {{ msg }}</span>
```

The mustache tag will be replaced with the value of the `msg` *field*. You can
also use an *expression* inside the mustache tags. Any type that has
`std::fmt::Display` implemented can be the final result of the *expression*
defined by the mustache tags.

```html
<span>Word Count: {{ msg.len() }}</span>
```

### Attributes

Interpolation can also be used in values of attributes.

```html
<div title="Application - {{ page_name }}"></div>
```

If you want to use `"` inside the attribute value for an *expression*, you can
follow the HTML spec and surround the value with `'`.

```html
<div title='Application - {{ "Welcome" }}'></div>
```

### Variable Attributes

If you want to have an attribute that is completely interpolated with just one
mustache tag and nothing else, you can do this.

```html
<div :title="page_name"></div>
```

The value of the attribute `title` will be the value of `page_name` field.

### Control Attributes

The library doesn't allow `if` conditions and `for` loops as expressions
inside the mustache tags. You can use a control attribute on html tags to do this.

```html
<div !if='page_name == "home"'>Welcome</div>
<div !else>{{ page_name }}</div>
```

The above template prints either the first or the second `div` depending
on the *expression* provided to the `!if` control attribute.

`!else-if` is also supported as you would expect.

`for` loops also make use of control attribute syntax like shown below.

```html
<div !for="char in page_name.chars()">{{ char }}</div>
```

The above template prints `div` tags for all the characters in `page_name`
field. Other than the name difference in the control attribute, `!for` needs
*pattern **in** expression* syntax.

### Grouping Elements

Because `!if` and `!for` are attributes, they need to be attached to a single
tag. But sometimes, you might need to render more than one element. In that
case, you can use the `template` tag which works like an invisible wrapper.

```html
<template !if="condition">
  <h1>{{ title }}</h1>
  <p>{{ content }}</p>
</template>
```

A template doesn't allow multiple elements to be defined at the root. Which
means, you can use the `template` tag to group elements at the root of
the template.

```html
<template>
  <!DOCTYPE html>
  <html></html>
</template>
```

### Class & Style bindings

To be implemented

# Components

Every template can be used as a component by default and thus is reusable. Let us
assume we have a template `src/views/shared/button.html` with the following:

```html
<button :href="href">{{text}}</button>
```

The generated view for this would look like the following:

```rust
struct Button<'a> {
    href: &'a str,
    text: &'a str,
}
```

The above template can be used in other templates using the following syntax:

```HTML
<navbar>
  <shared:button href="/" text="Dashboard" />
  <shared:button href="/settings" text="Account" />
</navbar>
```

The attributes on a component work just like the attirbutes on a normal HTML element
described [above](#attributes).

Any template can be used as a component. We can refer to the template by using it's
tag reference. Tag reference can be achieved by joining all the parts in the path of the
component with `:` after converting them to kebab case. A template that lives at
`src/views/users/avatar.html` can be used with `users:avatar`, and similarily a template
that lives at `src/views/common/simple/small_icon.html` can be used with `common:simple:small-icon`.

### Slots

Just like with HTML elements, it’s often useful to be able to pass content to a
component, like this:

```html
<div>
  <shared:button href="/">
    <span class="icon"></span>
    Dashboard
  </shared:button>
</div>
```

Fortunately, this is possible by using our custom `slot` element in the `button.html`.

```html
<button :href="href">
  <slot></slot>
</button>
```

The `<slot></slot>` above will be replaced by the elements we described inside
`shared:button` element when it's used.

### Scope

When we use a variable inside a component's slot, such as:

```html
<div>
  <shared:button href="/">{{ link_text }}</shared:button>
</div>
```

That slot tries to access this templates fields (i.e. the same `scope`). The slot does
not have access to `<shared:button>`'s fields. For example, trying to access `href`
would not work:

```html
<div>
  <shared:button href="/">{{ href }}</shared:button>
</div>
```

It would instead create a `href` field on the `<shared:button>` template.

### Fallback

To be implemented

### Named Slots

There are times when it’s useful to have multiple slots. For example, in a `<base-layout>`
component with the following template:

```html
<div class="container">
  <header>
    <!-- We want header content here -->
  </header>
  <main>
    <!-- We want main content here -->
  </main>
  <footer>
    <!-- We want footer content here -->
  </footer>
</div>
```

For these cases, the `<slot>` element has a special attribute, `name`, which can be
used to define additional slots:

```html
<div class="container">
  <header>
    <slot name="header"></slot>
  </header>
  <main>
    <slot></slot>
  </main>
  <footer>
    <slot name="footer"></slot>
  </footer>
</div>
```

A `<slot>` outlet without `name` implicitly has the name `"default"`.

To provide content to named slots, we can use a special attribute on a `<template>`,
providing the name of the slot:

```html
<layout:base>
  <template #header>
    <h1>Here might be a page title</h1>
  </template>

  <p>A paragraph for the main content.</p>
  <p>And another one.</p>

  <template #footer>
    <p>Here's some contact info</p>
  </template>
</layout:base>
```

Now everything inside the `<template>` elements will be passed to the corresponding
slots. Any content not wrapped in a `<template>` is assumed to be for the default slot.

However, we can still wrap default slot content in a `<template>` if you wish to be explicit.

# Helpers & Feature Gates

There are multiple feature gates on [Reign][] to help the user select what he wants from the library.

Please refer to [examples](https://github.com/pksunkara/reign/tree/master/examples)
to see how they are used.

Please refer to [reign_derive](https://docs.rs/reign_derive) for more information about the
usage of macros.

##### view

* `views!` can be used to build the views.
* `render!` can be used to render a view into a string.

##### view-backend

* `views!` can be used to build the views.
* Enables `render` helper and `render!` renders a view into [hyper][] response.
* Enables `redirect` helper for [hyper][] response.
* Enables `json` helper and `json!` serializes a value into [hyper][] response.

# Appendix

### Expressions

Allowed expressions are described below with `bop` being a binary operator and
`uop` being an unary operator. `...` represents possible repetitions.

* `literal`
* `ident`
* `[expr, ...]`
* `expr bop expr`
* `uop expr`
* `expr(expr, ...)`
* `expr.ident`
* `expr.number`
* `expr[expr]`
* `(expr)`
* `[expr; expr]`
* `(expr, ...)`
* `& expr`
* `expr as type`
* `expr: type`
* `expr..expr`
* `type { ident: expr, ..expr, ... }`

### Patterns

Allowed patterns are described below with `expr` represents the above mentioned
*expression*. `...` represents possible repetitions.

* `ident`
* `_`
* `& pat`
* `type { pat, .., ... }`
* `(pat, ...)`
* `type(pat, .., ...)`

### Annotations

1. Tag names can contain `:` which is not completely supported by pure HTML5
   spec but most of the parsers support it.
2. We also assume the parsers are made with Web Components specification in mind.

[ann]: #annotations
[reign_router]: https://docs.rs/reign_router
[hyper]: https://hyper.rs
[Reign]: https://docs.rs/reign
