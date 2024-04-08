# Omni-lang Programming Language

![GitHub License](https://img.shields.io/github/license/Luminescent-Linguistics/omni-lang)

The Omni programming language is a work-in-progress language based on the style, syntax and features of [Rust](https://rust-lang.org) and [Lisp](https://lisp-lang.org), in order to create something else new and exciting.

This is a project I started as a fun idea in the autumn of 2023, as of this first upload (2024-03-14), I still do not plan on creating the compiler in the forseeable future. For now, I will continue to work on the syntax, features and (make-believe) builtins.

If you wish to contribute or to try and create your own compiler for the language, you are welcome to do so.

---

## The Language

### Syntax

The syntax for Omni is similar to that of Lisp, except that function names are outside the parentheses, meaning the first element in a list will not be the function name. This style is more similar to that of the more popular C-like languages.

A few other exceptions to the syntax are also present, the first of which being that packages and modules are their own entities in code, and so do not require a string representing the relative path to them to be included in another source code file.

&nbsp;

The most common way to start off with learning a language is to make a "Hello, world!" program. An example of how you might do this in Omni is shown below.

```rust
fun(main () Empty
    "The main process of the program."
    print("Hello, world!\n"))
```

### Features

Omni supports most features from both Rust and Lisp. Here, you can find a comprehensive list of the "currently supported"[^1] features.

[^1]: The language is not currently real, so nothing is actually supported.

1. [Variables and Constants](#variables-and-constants)
2. [Functions](#functions)
3. [Types](#types)
4. [Macros](#macros)
5. [Modules and Packages](#modules-and-packages)

#### Variables and Constants

Variables in Omni are declared with the `let` macro. The `let` macro takes an identifier for the variable name, a type, and optionally a value of the same type as provided for the variable.

```rust
let(myVariable int 5)
```

You can also assign late the values of the variables, using the `set` macro.

```rust
let(myVariable int)
set(myVariable 5)
```

These variables are immutable, however, meaning their value cannot be changed. In order to create a mutable variable, you must instead use the `let-mut` macro, and can change the value again using the `set` macro.

```omni
let-mut(myMutableVariable int 5)
print-formatted("%d\n", myMutableVariable)
```

#### Functions

To create a function in Omni, you have already seen in the "Hello, world!" example, you start by calling the `fun` macro. Then, the first argument is the function name (the identifier), followed by the parameters and the return type. Then comes the documentation string, which could be left blank, but at least a short description is recommended, though you should come back later and add it then if you feel it would break your flow. After the documentation string comes the function body, in which you can provide any number of additional operations, provided the final one evaluates to the same type as the function's return type. Additionally, you may exit the function's operations early by using the `return` operation and providing a return value that matches the type the function is meant to return. If the return type is `Empty` or `()`, then any other function or operation that also returns one of those, thereby acting as a procedure, may be called last.

```rust
fun(early-returning-func (
        parameter String
    ) bool
    "A function that takes in a string parameter and returns true if the string 
    is equal to \"Hello, world!\", and false if it is not."
    if(==(parameter1 "Hello, world!")
        return(true))
    false)
```

#### Types

Omni is a strongly- and statically-typed language that allows you to create your own types in the form of `struct`s and `enum`s.

Additionally, the language allows for the making of `trait`s – collections of shared members for implementing on types, including only static members (no properties, only methods and associated constants and types).

#### Macros

Macros, as in Rust and Lisp, are compile-time evaluated operations that allow for metaprogramming, as well as in-language configurations.

Omni's macros are also used in place of some traditional keywords, acting as modifiers to other code.

> _(More to be added)_

#### Modules and Packages

Modules and packages are useful code-sharing tools, and no language is complete without them, in my opinion.

To make use of modules in Omni, you must first create a `*.omni` file, with the name of the module in place of the asterisk (`*`). Then, you can define any constant and static entities (types, functions, actual constants, so on…) inside that file, making sure to mark them as public with the `#(pub)` flag macro

---

## Contributors

- [Frank Hudson](https://github.com/Frank-Hudson/)

---

## License

The license for this project is the [GNU General Public License, Version 3.0](https://www.gnu.org/licenses/gpl-3.0).

## Copyright

> Omni-lang, a general purpose programming language.
> Copyright (C) 2024  Frank Hudson
>
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
>
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU General Public License for more details.
>
> You should have received a copy of the GNU General Public License
> along with this program.  If not, see <https://www.gnu.org/licenses/>.
>
>
> You can contact the author at <frank.hudson.v0@gmail.com>.
