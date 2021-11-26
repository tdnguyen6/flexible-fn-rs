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

We start with trait signatures of the functions:

```rs
pub trait F {
    type Output;
    fn f(&self) -> Self::Output;
}

#[async_trait]
pub trait FAsync {
    type Output;
    async fn f_async(&self) -> Self::Output;
}
```

We can then implement this for different arguments:

```rs
// arg: () - empty
// return Result<i32>
impl F for () {
    type Output = Result<i32>;
    fn f(&self) -> Self::Output {
        Ok(1)
    }
}

// arg: (&str, i32)
// return Result<HashMap<i32, String>>
impl F for (&str, i32) {
    type Output = Result<HashMap<i32, String>>;
    fn f(&self) -> Self::Output {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

// arg: Info struct
// return Result<Vec<String>>
impl F for &Info<'_> {
    type Output = Result<Vec<String>>;
    fn f(&self) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

// async version
// arg: Info struct
// return Result<Vec<String>>
#[async_trait]
impl FAsync for &Info<'_> {
    type Output = Result<Vec<String>>;
    async fn f_async(&self) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
```

We can then call the functions on the argument types:

```rs
let a: Result<i32> = ().f()
let b: Result<HashMap<i32, String>> = ("abc", 1).f()
let c: Result<Vec<String>> = (&Info { ... }).f()
let d: Result<Vec<String>> = (&Info { ... }).f_async().await?
```

However, those function calls are ugly and unintuitive. Therefore, we should write some wrappers to make them look better:

```rs
pub fn f<P: F>(p: P) -> P::Output {
    p.f()
}

pub async fn f_async<P: FAsync>(p: P) -> P::Output {
    p.f_async().await
}
```

Now, we can call `f()` and `f_async()` in the regular way:

```rs
let a: Result<i32> = f(())
let b: Result<HashMap<i32, String>> = f(("abc", 1))
let c: Result<Vec<String>> = f(&Info { ... })
let d: Result<Vec<String>> = f_async(&Info { ... }).await?
```

### Struct function

We can modify this a bit to make this works as methods of another struct. The signature should now be generic with the struct type as a parameter. The method also expose a borrowed reference to the object in other to use other fields/methods of the struct.

```rs
pub trait F<O: ?Sized> {
    type Output;
    fn f(&self, o: &O) -> Self::Output;
}

#[async_trait]
pub trait FAsync<O: ?Sized> {
    type Output;
    async fn f_async(&self, o: &O) -> Self::Output;
}
```

Now here is the wrapper:

```rs
impl O {
    pub fn f<P: F<Self>>(&self, p: P) -> P::Output {
        p.f(self)
    }

    pub async fn f_async<P: FAsync<Self>>(&self, p: P) -> P::Output {
        p.f_async(self).await
    }
}
```

The type of O is now Self in the wrapper. Self is unsized but generic type parameter are implicitly bounded by Sized. As a result, we remove the Sized bound for O in the signature with `O: ?Sized`. I am not sure if removing the Sized bound has any implication and whether there is any better way of doing this.

Now we can call the method on struct O:

```rs
let o = O {};
let a: Result<i32> = o.f(())
let b: Result<HashMap<i32, String>> = o.f(("abc", 1))
let c: Result<Vec<String>> = o.f(&Info { ... })
let d: Result<Vec<String>> = o.f_async(&Info { ... }).await?
```

### Trait function

This is mostly similar to struct functions. However, the wrapper should not be in the impl block but should be in the trait block as the default impl

```rs
#[async_trait]
pub trait T {
    fn f<P: F<Self>>(&self, p: P) -> P::Output {
        p.f(self)
    }

    // P must implement Sync + Send to be threadsafe for trait
    async fn f_async<P: FAsync<Self> + Send + Sync>(&self, p: P) -> P::Output {
        p.f_async(self).await
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
let c: Result<Vec<String>> = i.f(&Info { ... })
let d: Result<Vec<String>> = i.f_async(&Info { ... }).await?
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
