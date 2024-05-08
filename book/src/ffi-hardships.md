# FFI Hardships
We need to be more creative if we want to use more elaborate types, like `Result<T, E>`, `String`, or fat pointers like `&[T]`. If your type has special rules, then you can't simply annotate your structs and call it a day. For instance, to pass a `&[u32]`, we have to keep in mind the way they are represented in Rust: two words, one for a pointer to the start of the slice, and one for the length:

```rust
#[repr(C)]
struct ImaginaryU32Slice {
    start: *const u32,
    len: usize
}
```

> ❓ In terms of information, what is different between `ImaginaryU32Slice` and `&[u32]`?

Typically in C, you'd pass both fields as separate arguments. So your `extern "C" fn` may look like this:

```rust
#[no_mangle]
extern "C" fn takes_slice(data: *const u32, len: u32) {
    /* do stuff */
}
```

> ❓ What constitutes the type `String` in Rust? How is that different from C strings?

`takes_slice` seems reasonable, but it introduces a nasty problem: we have to define the representation by hand. That's doable for slices, but how about `Strings`, being guaranteed to be UTF-8 encoded in Rust, and being guaranteed to end with `nul` in C? Which kind do we pick? What does it even mean to expose generic functions? Furthermore, we're unable to express ownership, so rules about who frees which memory allocation, how long pointers are allowed to live, how data is represented, and more must be documented very clearly and unambiguously, and that documentation needs to be read thouroughly by the user, and then interpreted as intended _and then_ implemented and maintained correctly. Whew!

If you really want to use Rust to its full extent when mixing it with another language, then either you'll have to manually write wrappers, or, if you're lucky, use some kind of smart tool to generate glue code, like [rust-bindgen](https://github.com/rust-lang/rust-bindgen) and [cbindgen](https://github.com/mozilla/cbindgen). Either way, you'll still need knowledge of the domain of your application, as well as both Rust and the language you want it to talk to. That is _hard_. And there's not a lot of material out there that covers this.
