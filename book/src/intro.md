# Mix-in-Rust @ RustNL 2024


## Intro
> ❓ Why do we need FFI? What do we gain from it?

Imagine yourself in the following situation. You came across Rust a while ago and you like what you see. You've done some learning and hobby projects in Rust, got pretty good at it, and are wondering if you could somehow use Rust in the huge software project you're working on during your day job. After all, Rust's promises are hard to ignore: apart from Rust being [blazingly fast and memory-efficient, its type system and ownership model guarantee memory-safety and thread-safety](https://www.rust-lang.org/). Being a sane person, you understand that ['Rewrite it in Rust'](https://github.com/ansuz/RIIR) is no realistic answer. You'd like to [oxidize](https://wiki.mozilla.org/Oxidation) the code base: gradually introduce Rust in the application where possible. When you're setting up a new application, you'd like to use foreign dependencies from your Rust executable.

You're facing quite the challenge here. You'll quickly find that adding Rust to your codebase is not as trivial as you'd hope it would be. But we're here to help you tackle it. We want to provide some guidance with which you can set up a plan to introduce Rust in your projects. Because in order to do that, you'll need to know what it actually means to do so. First off, we'll have a look at how Rust does interop in general, and in following posts, we will go over some fantastic tools that make the finnicky world of interop much prettier.

> ❓ What makes FFI complicated?

> ❓ What do we need for two languages to talk to each other?

It's obvious that programming languages differ greatly in syntax. If you try to compile C code using the Rust compiler, it's going to get confused, to say the least. But even if this were somehow not a problem, programming languages differ in semantics as well. That is the hard part. To declare a variable in Rust means something else than to declare a variable in C. To call a function in Rust means something different than to call a function in Java. And to define a type in Rust means something different than to define a type in Python. Languages make different trade-offs. The devil's in the details.

It may sounds surprising, but Rust does not guarantee how its types are laid out in memory. Even between two compiler invocations, [the memory layout of your struct may differ](https://doc.rust-lang.org/reference/type-layout.html). Apart from primitives, arrays, and string slices, you can't assume that the bytes in memory are ordered in a specific way. By default, Rust retains the right to reorder struct and enum fields in order to make your application more efficient. In the same vein, Rust hasn't settled on a stable calling convention: it doesn't promise anything about the way individual function calls work. This allows Rust to optimize calls, which makes for very performant code.

Well, then how do any two programming languages communicate? The trick is the same as the one I'm pulling by writing this text in English, while I'm a Dutch native speaker. And anyone who can read English is able to follow me, even non-native speakers. You use some kind of commonly known language. In programming, the lingua franca is C. C has a standardized Application Binary Interface or ABI, so its type layout and calling conventions are well-known. Every self-respecting programming language has some means of adhering to the C ABI. Of course, Rust is no exception. But the C ABI was made to support C: a relatively simple programming language that has been around for ages. As such, when speaking C ABI, we can't express everything there is to say about Rust types. Just like how there's no good English translation of the Dutch word 'gezellig'. There are no generics, no fat pointers, no trait objects, no notion of ownership or reference lifetimes, no `async`/`await`, no `impl Drop`. We need to communicate using primitives, pointers, structs, and unions. And through documentation. Lots of it. But, spoiler: it's totally possible.

## Learning objectives
After this workshop, you'll be able to:
- Explain why FFI is hard in general and what needs to be taken into account when defining a foreign interface
- Work with the basic functionality Rust provides to define and expose a C-interface to your Rust libraries
- Use CXX to define and expose a C++-interface to your Rust libraries
- Get an intuition of how to introduce Rust in a foreign-language-based project and the amount of effort that would take
