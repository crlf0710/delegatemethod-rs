# delegatemethod-rs
Delegate method calls to a field.

## Examples
Suppose you have a struct called `Inner`, with methods `fn1` and `fn2`.

You have another struct called `Outer` with fields using `Inner` as type.

You can use `delegate_method` to forward some method implementations to the fields.

```rust
    #[derive(Copy, Clone)]
    struct Outer {
        inner1: Inner,
        inner2: Inner,
    }

    impl Outer {
        delegate_method! {
            // the <as FieldType> is optional below
            // it is only used for methods without self or &self or &mut self argument
            inner as Inner:
            pub fn fn1();
            
            // here comes another group, using another field.
            inner2 as Inner:
            pub fn fn2() -> usize;
        }
    }
```

## Acknowledgements
* The idea comes from [Tommy McGuire's blog](http://maniagnosis.crsr.net/2016/01/another-rust-spot-delegation.html),
but adjusted to match the Rust syntax better.

