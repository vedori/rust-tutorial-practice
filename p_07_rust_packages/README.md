This directory is for practicing Rust's Crate System

# What is a crate?
- A crate is the smallest amount of code that the Rust compiler considers at a time
- Crates can be binary (compiled to an executable) or library
- Binary crates must have `main()`
- Library crates are synonymous with "libraries"
## Crate Root
- basically the source file for the crate
- `src/main.rs` for binary crates by default (no specification in toml needed)
- `src/lib.rs` for lib crates by default
- you can **declare new modules** in the root file

# What is a package?
- one or more crates that are bundled to provide a set of functionality
- contains a `Cargo.toml` file which describes how to build the specified crates
- can contain **as many binary crates** as needed
- can **only have one library crate**
- must contain at lease one binary or library crate
## Cargo
- Cargo is a package that contains the binary crate for the CLI tool `cargo`
- also contains a library crate that the binary crate depends on
- other projects can use Cargo's library crate to use the came logic the Cargo CLI uses
# Modules
- **declared** in the crate root such as `mod garden;`
- once declared, Rust's compiler **will look for module's code** in these places
1. in the crate root, the code can be written *inline* using curly braces `{}` instead 
of ending with `;`
2. `src/garden.rs`
3. `src/garden/mod.rs`
- it is just where the code is located no need to specify `mod garden {}` inside the file
# Paths
## Absolute Path
- all modules can be referred to a path starting with `crate::`
- for example an `Asparagus` type defined in the garden vegetables module would be found at
`crate::garden::vegetables::Asparagus`
- this is known as the absolute path
- many times module/code definitions will be decoupled from the module's item calls, for example
    - `mod foo {...}` and  `fn call_foo() {}` are defined in a module (could be the crate root)
    - the function may later be moved to a new module
    - using an absolute path in `call_foo()` would still work
    - an absolute path would fail if both the module and the item call were BOTH moved
so in most cases it makes sense to default to absolute paths
## Relative Path
- a relative path doesn't start with `crate::`
### Super
- a path that refers to the parent module
- **very useful** to refer to an item that we *know* would be a child of the parent module
but the parent module may be rearranged elsewhere in the module tree
# Use keyword
- the `use` keyword reduces having to type long paths
- instead of `crate::garden::vegetables::Asparagus` you can create a shortcut with `use crate::garden::vegetables::Asparagus;`
- then you just need to type `Asparagus` to make use of the type
- it has to be in scope for example you cannot make a shortcut to Asparagus on one line
then use the shortcut in a newly created submodule
For example
```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```
## Specifying functions vs other items
When specifying **functions** you should only bring its parent module into scope.
This makes it clear that **the function is NOT local**.
- even though `use crate::front_of_house::hosting::add_to_waitlist;` compiles you
should go with `use crate::front_of_house::hosting` instead'
- the function will be called with `hosting::add_to_waitlist()` instead of `add_to_waitlist()`
### Other items
When it comes to other items such as structs, and enums it is idiomatic to specify the full path
This below is an example
```Rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
### Exceptions
- If we are bringing in two items with identical names the parent module MUST be specified
- the compiler will reject it otherwise since there's no way to differentiate it
- here is an example where `std::fmt::Result` and `std::io::Result` are used as shortcuts
```Rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```
### Using the `as` keyword for duplicate items
Another way to deal with bringing two types of the same name is to give it a new local new
with the `as` keyword.
- when doing this bring the full path of the item to scope
```Rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```
# Pub keyword
- all parts of a module are private by default
- items in the **child modules can use the items in their ancestor modules**
since child modules *wrap* and hide their implementation details
- if you want to use a part of a module externally it must be declared with `pub`
## Structs
- if we use pub before a struct definition the struct's fields will still be private
- useful because there are some parts that users do not need to have control over
- when structs have a private field, **the struct needs have a constructor** a function that creates an instance of the struct
## Enums
- if an enum is public all of its variants are made public
# Submodules
this example uses `mod vegetables;`
- unless defined inline, submodules are **not defined in the crate root**
1. inline
2. they can be defined in a named file `src/PARENT_DIR_NAME/vegetables.rs`
3. can be a mod.rs file under this file structure `src/PARENT_DIR_NAME/vegetables/mod.rs`
For the restaurant crate this is the module tree
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
# Re-exporting names with `pub use`
- Useful when your crate will be used externally.
Normally when using a public module you have to type out the full path
The parent modules have to be specified public for this but this isn't always wanted
- We don't need the consumer of the API to know the relationship tree of private modules
- When the `use` keyword brings a name into scope **it is private**
- With `pub use`you can publicly export a module **as a top level module**
- this bypasses the need to mark parent modules as `pub` and to specify the implementation path
- useful when the way the code is organized isn't the same as how a user would imagine the relationship for the specified domain
    - the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms

```Rust
// restaurant crate
mod front_of_house {
    pub mod hosting {
        pub fn take_order() {}
    }
}

// --- in the crate root ---
pub use crate::front_of_house::foo;
// -------------------------

// --- the following is the restaurant crate being used externally ---

// without pub use
restaurant::front_of_house::hosting::take_order();

// with pub use
restaurant::hosting::take_order();
// plus hosting and front_of_house do not need to be declared public
```
# Using Nested Paths
Instead of
```Rust
use std::cmp::Ordering;
use std::io;
```
You can do this
```Rust
use std::{cmp::Ordering, io};
```
You can even do stuff with `self`
```Rust
use std::io::{self, Write};
```
this brings `std::io` and `std::io::Write` into scope
# Glob Operator
brings **all public items** defined in a path into scope with `*`
```Rust
use std::collections::*;
```
- this example brings all all public items defined in `std::collections` into the current scope
- can make it harder to tell what names are in scope and where a name was defined
- often **used when testing** to bring everything under test into the `tests` module
- sometimes used as a part of the **prelude pattern**
```Rust
pub mod parent {

  pub fn a() {}

  fn b() {}

  pub mod child {

    pub fn c() {}

  }

}


fn main() {

  use parent::{*, child as alias};

  // ...

}
```
The paths that are valid for this example are
- `parent::a`
- `a` (from *)
- `parent::child::c`
- `child::c` (from *)
- `alias::c`