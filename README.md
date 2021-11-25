# Table of Contents

- [Table of Contents](#table-of-contents)
- [flexible-fn-rs](#flexible-fn-rs)
- [What is this trying to demo?](#what-is-this-trying-to-demo)
- [How is the code structured?](#how-is-the-code-structured)
- [Named/Unnamed and Optional arguments](#namedunnamed-and-optional-arguments)
  - [Mechanism](#mechanism)
  - [Compared to C++](#compared-to-c)
    - [Named arguments](#named-arguments)
    - [Optional arguments](#optional-arguments)
  - [Compared to Python](#compared-to-python)
- [Overloading](#overloading)
  - [Mechanism](#mechanism-1)
    - [Independent function](#independent-function)
    - [Struct function](#struct-function)
    - [Trait function](#trait-function)
  - [Compared to C++/Java](#compared-to-cjava)
  - [Known issues](#known-issues)
  - [Alternatives](#alternatives)

# flexible-fn-rs

Demonstration of flexible function calls in Rust with function overloading and optional arguments

# What is this trying to demo?

This repo is trying to demo that Rust can provide all the flexbilities when doing function calls like any other languages. The flexibilities are demonstrated in 3 things:

- Name and unnamed arguments
- Optional arguments
- Function overloading
  - Parameter overloading
  - Return type overloading

# How is the code structured?

There are 3 demo, focusing on 3 kinds of functions:

- Independent functions, calling without attaching to any struct
- Struct functions, defined in struct impl blocks
- Trait functions, defined in `impl Trait for Struct` blocks

# Named/Unnamed and Optional arguments

## Mechanism

We use structs as arguments to provide these functionalities. Combined with the `derive_builder` trait, we can have all the flexibilities when instantiating the struct:

- only provide values to the fields we need and leave others as default
- named arguments with named struct fields
- unnamed arguments with tuple struct

## Compared to C++

### Named arguments

Like Rust, C++ does **not** support named arguments and also recommend using objects as arguments to achieve this

### Optional arguments

Unlike Rust, C++ does support optional arguments. However, any omitted arguments must be the last argument in the argument list. Therefore, it is not really flexible and cannot provide complex optional argments list. Generally, it is also recommended to use objects as arguments to provide API with complex optional argument list in C++.

## Compared to Python

Python has the most modern and flexible function call mechanism. Optional and named arguments are basic functionalities in the language. These features are used everywhere, they can be found in almost any Python API. By using structs with builder pattern as arguments, we can make Rust as close as possible to the flexbility of Python function argument lists.

# Overloading

## Mechanism

The only way we can have different functions with similar names in Rust is by calling them from different objects. Overloading is about calling functions with the same name with different arguments. Therefore, we can consider the different arguments as objects and implement functions with similar names on them.

### Independent function
We start with a generic trait signature of the function:

```rs
pub trait F<R> { // R generic parameter for return type
    fn f(&self) -> R;
}
```

We can then implement this for different arguments:
```rs
// arg: () - empty
// return Result<i32>
impl F<Result<i32>> for () {
    fn f(&self) -> Result<i32> {
        Ok(1)
    }
}

// arg: (&str, i32)
// return Result<HashMap<i32, String>>
impl F<Result<HashMap<i32, String>>> for (&str, i32) {
    fn f(&self) -> Result<HashMap<i32, String>> {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

// arg: Info struct
// return Result<Vec<String>>
impl F<Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
```

We can then call the functions on the argument types:
```rs
let a: Result<i32> = ().f()
let b: Result<HashMap<i32, String>> = ("abc", 1).f()
let c: Result<Vec<String>> = (&arg::Info { ... }).f()
```

However, those function calls are ugly and unintuitive. Therefore, we should write a wrapper to make them look better:
```rs
pub fn f<P: F<R>, R>(p: P) -> R {
    p.f()
}
```

Now, we can call f() in the regular way:
```rs
let a: Result<i32> = f(())
let b: Result<HashMap<i32, String>> = f(("abc", 1))
let c: Result<Vec<String>> = f(&arg::Info { ... })
```

### Struct function
We can modify this a bit to make this works as methods of another struct. The signature should now contain the struct type. The method also expose a borrowed reference to the object in other to use other fields/methods of the struct.
```rs
pub trait F<O: ?Sized, R> {
    fn f(&self, o: &O) -> R;
}
```
Now here is the wrapper:
```rs
impl O {
    pub fn f<P: F<Self, R>, R>(&self, p: P) -> R {
        p.f(self)
    }
}
```
The type of O is now Self in the wrapper. Self is unsized but generic type parameter are implicitly bounded by Sized. As a result, we remove the Sized bound for O in the signature with `O: ?Sized`. I am not sure if removing the Sized bound has any implication and whether there is any better way of doing this.

Now we can call the method on struct O:
```rs
let o = O {};
let a: Result<i32> = o.f(())
let b: Result<HashMap<i32, String>> = o.f(("abc", 1))
let c: Result<Vec<String>> = o.f(&arg::Info { ... })
```

### Trait function
This is mostly similar to struct functions. However, the wrapper should not be in the impl block but should be in the trait block as the default impl
```rs
pub trait T {
    fn f<P: F<Self, R>, R>(&self, p: P) -> R {
        p.f(self)
    }
}
```
By having the wrapper as the default impl of trait, every struct that impl the Trait will not have to implement this wrapper again. Notice that if that is the only method of the trait, we must still provide an empty impl block.
```rs
pub struct I;

impl T for I {}
```
Now we can call the method of the trait on the object:
```rs
let i = I {};
let a: Result<i32> = i.f(())
let b: Result<HashMap<i32, String>> = i.f(("abc", 1))
let c: Result<Vec<String>> = i.f(&arg::Info { ... })
```

## Compared to C++/Java

Because of this overloading mechanism (each overloaded function is a trait impl for an argument type), we will not be able to have similar function name and arguments types but different return type. This is the same as in C++/Java, we also cannot have overloaded functions with just a different return type.

## Known issues
- Error is not straight forward: when we use an overloaded method on an argument without implementing the signature for that argument, instead of `method not found in ...` we will receive `the trait ...::F<>... is not implemented for Arg`
- Less IDE suggestions and completions
- A bit tedious to implement compared to the traditional way of just choosing to a different function name.

## Alternatives

As of now, there are 2 crates provides overloaded functions:

- [overloadable](https://crates.io/crates/overloadable)
- [overloadf](https://crates.io/crates/overloadf)

However, both of them seems to be outdated and unmaintained as no commit has been made for years.
