## Lightsaber proposal

#### Design Target

This project is a Rust library designed to enable developers to define a series of functionalities. These functionalities can then be used by downstream users to define their own code logic and develop in a manner similar to using a programming language, thereby facilitating a more intuitive and flexible development process.

By integrating this library, developers can define specific functions, structures, and interfaces, along with a simple syntax for their custom language. The end result is a simple language interpreter that can be provided to other users. The unique syntax and behavior of this custom language are defined by the developers themselves, using the Rust language.

> Developers using Lightsaber:
>
> 1. Define functions (abilities)
> 2. Define syntax
> 3. Use Lightsaber to generate a parser in Rust
> 4. Embed the generated parser into his own Rust program
>
> Users:
>
> 1. **Write his own logic (in a simpler way) to achieve specific goals**

#### Ideas

This project aims to implement a language interpreter generator in Rust. However, it will feature a simpler yet more powerful syntax than ANTLR g4, making it easier for developers to define their own simple scripting languages and embed the generated parser into their own programs.

#### TODOs

1. Find a simple method to define syntax
2. Find a method to define semantics for the syntax
3. Generate corresponding Rust parser

#### Inspiration & Reference

Integrating the code we generate into downstream projects is similar to adding Lua script support in Nginx. However, the difference is that you can freely define your own language according to your preferences and needs, not just limited to using existing languages such as Lua, Javascript, etc.
