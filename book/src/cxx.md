# CXX
C's type system is not that elaborate, but when interacting with programming languages that have a more sophisticated type system, much more information can be kept. For instance, C++'s types are able to convey concepts that are like Rusts slices, `Vec<T>`'s and `Box<T>`'s. Given that, bindings generator tools can make FFI much more ergonomic.

Let's look at the [CXX](https://cxx.rs/) crate. Here's an important quote from the documentation:

> This library provides a safe mechanism for calling C++ code from Rust and Rust code from C++. **It carves out a regime of commonality where Rust and C++ are semantically very similar and guides the programmer to express their language boundary effectively within this regime.** CXX fills in the low level stuff so that you get a safe binding, preventing the pitfalls of doing a foreign function interface over unsafe C-style signatures.

That means that CXX focuses not on providing as much expression power as possible like for instance [PyO3](https://pyo3.rs) does, but on defining a subset of both C++ and Rust concepts which translates cheaply and safely. That means that you'll become limited in the ways that you can expose your Rust functionality if you're using CXX.

<figure>

<img src="https://cxx.rs/overview.svg">

<figcaption>Taken from <a href="https://cxx.rs">cxx.rs</a></figcaption>
</figure>

## Tour of CXX docs
Let's go over the CXX documentation a bit. Here's a couple of links to interesting pages:

- https://cxx.rs/#example
- https://cxx.rs/extern-rust.html
- https://cxx.rs/extern-c++.html
- https://cxx.rs/shared.html
- https://cxx.rs/bindings.html

# Exercise: Calling Rust from C++ using CXX
Let's now get a feeling of how to work with CXX!

If you click the 'Tools' button on your Van Gogh application, it'll open the tools pane, which contains a button that converts the image to greyscale, as well as a knob that hue rotates the image. They're connected to the `on_btnToolGreyscale_clicked` and `on_dialHueRotate_valueChanged` slots of the `GoghViewer` class respectively. This exercise is about implementing the functionality in Rust, using the [`image`](https://docs.rs/image/0.25.1/image/index.html) crate.

Open `rembrandt/src/proc.rs`. A minimal setup for your CXX bridge is provided already, to keep the build system happy. Define your CXX bridge in this file, exposing the functionality needed to convert images to greyscale and to do hue rotation. Call the functionality from the slots in `goghviewer.cc`.

Make sure not to introduce dangling pointers, null dereferences, or reading of uninitalized data! Also, ensure no memory is leaked.

<!--
- Round up
    - What did you learn?
    - How do you feel about this? ....
    - Show some other FFI lib impls for other languages
!-->