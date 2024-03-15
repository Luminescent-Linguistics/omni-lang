# Omni-lang Programming Language

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

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
