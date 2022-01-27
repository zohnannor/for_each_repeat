# `for_each_repeat`

Ever got frustrated when you realize that the `for` loop you were writing...

```rust
fn foo(mut iter: impl Iterator<Item=i32>) {
    for i in iter {
        // do something...
        if i == 42 {
            // ughh
        }
    }
}
```

...needs to repeat the current iteration in some cases?  
You may though: "Oh come on now I need to _think_ on how to rewrite this into a `while` loop!"

Fear not! Cause we've got

## `Repeat`

Import [`for_each_repeat::ForEachRepeat`] and put your `for` loop's body into a closure, that has to return [`LoopControl`] type.
Using `Repeat` you are able to redo current iteration with the same iterator value.

```rust
use for_each_repeat::{ForEachRepeat, LoopControl};

fn foo(mut iter: impl Iterator<Item=i32>) {
    let _: Option<()> = iter.for_each_repeat(|i| {
        // do something...
        if i == 42 {
            // process it...
            return LoopControl::Repeat(i);
        }
        LoopControl::Continue(())
    });
}
```

## `#![no_std]`

This crate can be used in `no_std` environment.

[`for_each_repeat::foreachrepeat`]: https://docs.rs/for_each_repeat/*/for_each_repeat/trait.ForEachRepeat.html
[`loopcontrol`]: https://docs.rs/for_each_repeat/*/for_each_repeat/enum.LoopControl.html
