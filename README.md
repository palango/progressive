progressive
===========

A rust library for showing progress of iterators and loops.

[![Build Status](https://travis-ci.org/palango/progressive.svg?branch=master)](https://travis-ci.org/palango/progressive)
[![Current Version](http://meritbadge.herokuapp.com/progressive)](https://crates.io/crates/progressive)
[![License](https://img.shields.io/crates/l/progressive.svg)]()

[![asciicast](https://asciinema.org/a/apt9cru9fqm65jqkvw8t9otpv.png)](https://asciinema.org/a/apt9cru9fqm65jqkvw8t9otpv)
### [Documentation](https://palango.github.io/progressive/)

### Usage

To use `progressive`, add this to your `Cargo.toml`:

```toml
[dependencies]
progressive = "0.1"
```

And this to your crate root:

```rust
extern crate progressive;
```

Here's a simple example that shows how to wrap an iterator tin order to get progress information:

```rust
extern crate progressive;

use progressive::progress;
use std::time::Duration;

fn main() {
    for _ in progress(0..30) {
        // do something expensive here
        std::thread::sleep(Duration::from_millis(200));
    }
}
```

For an example run `cargo run --example basic`
