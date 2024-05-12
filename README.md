# winmsg
Feature rich `Rust` `crate` for creating a [message box](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw) in `Windows` 

[![Crates](https://img.shields.io/crates/v/winmsg?logo=rust)](https://crates.io/crates/winmsg/)
[![License](https://img.shields.io/github/license/thewh1teagle/winmsg?color=00aaaa&logo=license)](LICENSE.txt)

# Install
```shell
cargo add winmsg
```

# Basic usage
```rust
use winmsg::{message_box, Options};

fn main() {
    message_box(Options {
        title: "Hello world!".into(),
        description: "How are you?".into(),
        ..Default::default()
    });
}
```

# Examples

See [examples](examples)