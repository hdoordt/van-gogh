# C ABI
To have Rust represent data and call and expose functions as prescribed in the C ABI, we have a number of tools.

```rust
#[no_mangle]
extern "C" fn say_hello() {
    println!("Hello there!");
}

#[repr(C)]
struct Point {
    x: f32,
    y: f32
}

#[repr(u8)]
enum Color {
    R,
    G,
    B
}

extern "C" {
    fn add(lhs: u32, rhs: u32) -> u32;
}
```

> ❓ What does the `extern "C"` prefix for `fn` do?

> ❓ What does the `#[no_mangle]` prefix do?

> ❓ How about `#[repr(C)]` and `#[repr(u8)]`?

> ❓ What's the `extern "C"` block do?


## Linking
We need to instruct the compiler to build us a _system_ library, which a linker can take and use symbols from. These systems libraries come in two flavours: _dynamic_ and _static_. A _dynamic_ library can be referred to from some executable, but is not included in it. The operating system will search for any symbols the executable refers to when starting your program, meaning that the dynamic library artifact will need to be present when running the application. The upside is that a dynamic library that is being used from many different executables, needs to be present on the system only once, saving space, and making patching easier. However, Rust defaults to static linking: linking at compile time, yielding a single, stand-alone executable. To support static linking when mixing languages, Rust can output a _static_ system library, which can then be used by some other linker to build the executable. This is the recommended way to include Rust code into your C projects.

What's more, we'll be relying on the artifact to have a specific name, so we need to state it explicitly. The way we instruct Rust to use a specific name and to build both a dynamic and a static system library is by adding the following lines to our `Cargo.toml`:

```toml
[lib]
name = "rust_in_c"
crate-type = ["cdylib", "staticlib"]
```

You can omit either `"cdylib"` or `"staticlib"`, if you need just one of them. Now, when we run `cargo build`, the artifacts can be found in `target/debug` (or `target/release`, if you've done a release build):

```shell
$ tree target/debug -L 1
target/debug
├── build
├── deps
├── examples
├── incremental
├── librust_in_c.a     <-- static library (archive)
├── librust_in_c.d
└── librust_in_c.so    <-- dynamic library (shared object)
```

I'm on Linux, so I get a `.so` file for a dynamic library, but if you're running Windows you'll get a `.dll` file, and on MacOS, a `.dylib` file is created. The static library is a `.a` file on Linux, MacOS and as well as on Windows when on MinGW, or a `.lib` file when using MSVC.

# Exercise: Calling Rust from C

Now lets get a feeling for C ABIs in a "real world" code base. In the top level directory of the 
[repository](https://github.com/hdoordt/van-gogh) you find a C++ application called *van-gogh*. For now, we want to
replace the oldest part of this software with Rust. In image_info.c and image_info.h you will find some code that can
gather some information about a PNG image.

Your task is to reimplement the functionality of image_info.c in Rust.

You can find a prepared Rust crate in the rembrandt folder. 

1. Open rembrandt/src/info.rs and image_info.h
2. Recreate the API described in the header in Rust
3. Try to build it (this should give you an error about some double symbols)
4. Rename or remove the functions in image_info.c or remove the mention of the file in CMakeLists.txt
5. Try building again, this should work now
6. Now reimplement the old logic in Rust using the [image](https://docs.rs/image/latest/image/) crate

<!--
- Exercises read image info
    - Use image crate to load a QImage using the C ABI
    - Instruct to use utf8 conversion for Cstr to Path
- Round up:
    - Who did the most checks?
    - Did you check nullability
    - Did you check encoding
    - ...
    - 
-->