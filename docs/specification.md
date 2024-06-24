# Omni Language Specification

[← README](../README.md)

---

- [Syntax](#syntax)
  - [Token Types](#token-types)
- [Builtin Items](#builtin-items)
  - [Types](#types)
    - [Primitive Types](#primitive-types)
      - [Numeric Types](#numeric-types)
      - [Other Primitives](#other-primitives)
    - [Complex Types](#complex-types)
      - [Collection Types](#collection-types)
      - [Function Types](#function-types)
  - [Commands](#commands)
    - [Function Commands](#function-commands)
    - [Macro Commands](#macro-commands)

---

## Syntax

The syntax for the Omni language has been updated from the old attempt at crossing Rust's with Lisp's with some of our own inventions due to a better understanding of both mentioned languages.

Now, instead of the initial `<func-ident>(<args>)`, we have adopted the more Lisp-based syntax of `(<func-atom> <args>)`.

Additionally, top-level functions with known argument lengths do not require surrounding parentheses, leaving more room for C- and Rust-like syntax more recognisable to most developers, such as `fun myfun ...`.

We have also adopted the vector square brack (`[`, `]`) syntax from Clojure for raw, non-evaluated lists, rather than the typical apostrophe-before-parentheses syntax (`'()`) of a qualifying Lisp.[^1]

Where before a function would have been defined

```clojure
fun(myfun (a int b int) int
    "Returns two integers, summed together"
    +(a b))
```

Now, the same function would be defined

```clojure
fun myfun [a int, b int] int
    "Returns two integers, summed together"
    + a b
```

Or, optionally with the parentheses

```clojure
(fun myfun [a int, b int] int
    "Returns two integers, summed together"
    (+ a b))
```

As you may be able to see, this newer syntax is more inline with this project's original goals:

1. Draw people from more traditional programming languages and backgrounds closer towards the power, accuracy and flexible yet stern typing of both Lisp and Rust.
2. Create a language with the power, accuracy and flexible yet stern typing of both Lisp and Rust.

### Token Types

The official list of token types in the language are now as follows:

`omni`

- `expr` (expression)
  - `ltrl` (literal)
    - `int` (integer)
    - `float` (float)
    - `char` (character)
    - `str` (string)
    - `reg` (regular expression)
  - `atom` (atom (identifier))
- `item` (item)
  - `var` (variable)
  - `const` (constant)
  - `fun` (function)
  - `type` (data type)
    - `prim` (primitive type)
    - `struc` (struct type)
    - `enum` (enum type)
    - `alias` (type alias)
  - `trait` (type trait)
- `punct` (punctuation)
  - `paren` (parenthesis)
    - `open` (open parenthesis, `(`)
    - `close` (close parenthesis, `)`)
  - `comma` (comma, `,`)
  - `brack` (bracket)
    - `open` (open bracket, `[`)
    - `close` (close bracket, `]`)
  - `brace` (brace)
    - `open` (open brace, `{`)
    - `close` (close brace, `}`)
- `spc` (whitespace)

## Builtin Items

### Types

#### Primitive Types

##### Numeric Types

| Type                 | Size (bits) |                                     Max |                                      Min |
|----------------------|------------:|----------------------------------------:|-----------------------------------------:|
| `bool`               |           1 |                                1 (true) |                                0 (false) |
| `int8`  / `byte`     |           8 |                                     127 |                                     -128 |
| `uint8` / `ubyte`    |           8 |                                     255 |                                        0 |
| `int16` / `short`    |          16 |                                   32767 |                                   -32768 |
| `uint16` / `ushort`  |          16 |                                   65535 |                                        0 |
| `int32` / `int`      |          32 |                              2147483647 |                              -2147483648 |
| `uint32` / `uint`    |          32 |                              4294967295 |                                        0 |
| `int64` / `long`     |          64 |                     9223372036854775807 |                     -9223372036854775808 |
| `uint64` / `ulong`   |          64 |                    18446744073709551615 |                                        0 |
| `int128` / `bigint`  |         128 | 170141183460469231731687303715884105727 | -170141183460469231731687303715884105728 |
| `uint128` / `bigint` |         128 | 340282366920938463463374607431768211455 |                                        0 |

##### Other Primitives

###### `char`

The `char` type is an additionally implemented alias[^2] for `uint8`.

`char` uses instantiation by single quote `'_`.

###### `str`

The `str` type is an additionally implemented alias[^2] for `list(char)`.

`str` uses instantiation by double quotes `"__"`.

#### Complex Types

##### Collection Types

- `list(T)`  
  Stack-allocated list of objects with type `T`
- `array(T, S)`  
  Stack-allocated list of `S`(ize) number of objects with type `T`
- `Vec(T)`  
  Heap-allocated list of elements of type `T`
- `HashMap(K, V)`  
  Heap-allocated list of keys of type `K` and associated values of type `V`, gettable using a hash function[^3]

##### Function Types

- `f([Args], Ret)`  
  ```clojure
  (fun takes-a-fun [arg1-is-a-fun f([int, int] int)] int
      (arg1-is-a-fun 2 2))
  ```
- ...[^4]

### Commands

Commands, here, means any built-in functions or macros that could otherwise almost be considered equivalents to keywords.

#### Function Commands

- `fun <atom> <(generics)?> [<param>, ... ] <return-type> <doc?> <body-expr>`
- `struct <atom> <(generics)?> <doc?> { <field-atom> <field-type>, ... }`
- `enum <atom> <(generics)?> <doc?> [ <variant-atom>, ... ]`
- `alias <atom> <(generics)?> <doc?> <aliased-type>`
- `trait <atom> <(generics)?> <doc?> [ <assoc-item> ... ]`
- `impl <(generics)?> <type> [ <assoc-item> ... ]`
- `impl <(generics)?> <trait> <type> [ <assoc-item-impl> ... ]`

#### Macro Commands

- `#derive(<derivable-trait>) <item>`  
  trait derivision decorator macro
- `#pub <item>`  
  public access modifer/exportation decorator macro

---

[^1]: This could also be seen as taken from Rust's array/slice `[]` syntax.  
[^2]: _Additionally implemented alias_ – Additional methods and associated items have been implemented for this type alias.  
[^3]: Yes, me am big dumdum, how you know?
[^4]: To be researched further
