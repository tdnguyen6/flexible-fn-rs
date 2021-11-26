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
    - [Output type overloading only](#output-type-overloading-only)
  - [Compared to C++/Java](#compared-to-cjava)
  - [Known issues](#known-issues)
  - [Alternatives](#alternatives)

# flexible-fn-rs

Demonstration of flexible function calls in Rust with function overloading, named arguments and optional arguments

# What is this trying to demo?

This repo is trying to demo that Rust can provide all the flexbilities when doing function calls like any other languages. The flexibilities are demonstrated in 3 things:

- Name and unnamed arguments
- Optional arguments
- Function overloading
  - Parameter overloading
  - Return type overloading

# How is the code structured?

There are 2 demos, focusing on 2 kinds of overloading mechanisms:

- Overload input types only (each input type has exactly 1 output type)
- Overload both input and output types (each input type can have many output types)

The 2 demos are further splitted into 3 subcategories:

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

The only way we can have different functions with similar names in Rust is by calling them from different objects. Overloading is about calling functions with the same name with different arguments. Therefore, we can consider the different arguments as objects and implement functions with similar names on them. Rust's operator overloading also uses the same mechanism (see [this](https://doc.rust-lang.org/rust-by-example/trait/ops.html))

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
let a = ().f() // Result<i32>
let b = ("abc", 1).f() // Result<HashMap<i32, String>>
let c = (&Info { ... }).f() // Result<Vec<String>>
let d = (&Info { ... }).f_async().await // Result<Vec<String>>
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
let a: = f(()) // Result<i32>
let b = f(("abc", 1)) // b: Result<HashMap<i32, String>>
let c = f(&Info { ... }) // Result<Vec<String>>
let d = f_async(&Info { ... }).await // Result<Vec<String>>
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
let a = o.f(()) // Result<i32>
let b = o.f(("abc", 1)) // Result<HashMap<i32, String>>
let c = o.f(&Info { ... }) // Result<Vec<String>>
let d = o.f_async(&Info { ... }).await // Result<Vec<String>>
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
let a = i.f(()) // Result<i32>
let b = i.f(("abc", 1)) // Result<HashMap<i32, String>>
let c = i.f(&Info { ... }) // Result<Vec<String>>
let d = i.f_async(&Info { ... }).await // Result<Vec<String>>
```


### Output type overloading only
In the above overloading implementations, we **intentionally** used Output type var (`type Output;`) instead of making the Output type as a generic parameter. This disallows user from having multiple implementations for the same Input type but with different Output types. 

Rust's official operator overloading mechanism also implemented the same exact limitation (more on this: https://stackoverflow.com/a/39118492/12361118). This limitation helped to reduce complexity and ensure things align with the traditional overloading logics.

However, we can remove that limitation and allow users to implement overloaded functions that differentiate only on the return type. We can start by converting the `type Output;` in to generic type param:
```rs
pub trait F<R> {
    fn f(&self) -> R;
}

#[async_trait]
pub trait FAsync<R> {
    async fn f_async(&self) -> R;
}
```

Then, we rewrite the wrapper:
```rs
pub fn f<P: sig::F<R>, R>(p: P) -> R {
    p.f()
}

pub async fn f_async<P: sig::FAsync<R>, R>(p: P) -> R {
    p.f_async().await
}
```

Now, let's implement some examples:
```rs
impl F<Result<i32>> for &Info<'_> {
    fn f(&self) -> Result<i32> {
        Ok(1)
    }
}

impl F<Result<Vec<String>>> for &Info<'_> {
    fn f(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl FAsync<Result<i32>> for &Info<'_> {
    async fn f_async(&self) -> Result<i32> {
        Ok(2)
    }
}

#[async_trait]
impl FAsync<Result<Vec<String>>> for &Info<'_> {
    async fn f_async(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
```

However, when we try to use it, we immediately receive errors from the compiler saying that our function calls are ambiguous, it does not know which exact function we want to use. Therefore, we **must explicitly specify the return type** whenever we call these functions.
```rs
let flex_arg = &InfoBuilder::default()
        .birth_day("1990-12-07")
        .father_name("Independent Fn Father")
        .mother_name("Independent Fn Mother")
        .build()?;

// one way to specify type
let res: Result<i32> = f(flex_arg);

// another way to specify type
f::<&Info, Result<i32>>(flex_arg);
```

## Compared to C++/Java

In C++/C#/Java, functions are chosen based on their signatures, each signature consists of function name and arguments' types. As a result, each combination of function name and arguments' types uniquely define a function. We can overload functions by changing this combination in those languages. As the signature does not contain return type, one limitation of function overloading in those languages is that we cannot have the same signature (combination of function name and arguments' types) but different return types - a.k.a. output type overloading.

In Rust, due to the power of traits and generics, we have much more flexibility to overload functions compared to C++/C#/Java. In Rust, we can overload not only input types but also output types. However, we should notice that when we overload output types only (same input types, different output types), we **must explicitly provide the output type** or the compiler will not be able to figure out which function we want to use.

## Known issues

- Error is not straight forward: when we use an overloaded method on an argument without implementing the signature for that argument, instead of `method not found in ...` we will receive `the trait ...::F<>... is not implemented for Arg`
- Less IDE suggestions and completions
- A bit tedious to implement compared to the traditional way of just choosing to a different function name.

## Alternatives

As of now, there are 2 crates provides overloaded functions:

- [overloadable](https://crates.io/crates/overloadable)
- [overloadf](https://crates.io/crates/overloadf)

However, both of them seems to be outdated and unmaintained as no commit has been made for years.
