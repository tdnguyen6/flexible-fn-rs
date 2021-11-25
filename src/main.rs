mod independent_fn;
mod struct_fn;
mod trait_fn;

use anyhow::Result;
use independent_fn::definition::*;
use struct_fn::definition::*;
use trait_fn::definition::*;
use trait_fn::implementation::*;

fn main() -> Result<()> {
    // independent fn
    println!("{}", f(())?);
    println!("{:#?}", f(("Independent Fn", 1))?);
    let flex_arg1 = &independent_fn::definition::arg::InfoBuilder::default()
        .birth_day("1990-12-07")
        .father_name("Independent Fn Father")
        .mother_name("Independent Fn Mother")
        .build()?;
    println!("{:#?}", f(flex_arg1)?);

    // struct fn
    let o = O {};
    println!("{:#?}", o.f(())?);
    println!("{:#?}", o.f(("Struct Fn", 2))?);
    let flex_arg2 = &struct_fn::definition::arg::InfoBuilder::default()
        .height(180)
        .father_name("Struct Fn Father")
        .mother_name("Struct Fn Mother")
        .build()?;
    println!("{:#?}", o.f(flex_arg2)?);

    // trait fn
    let i = I {};
    println!("{:#?}", i.f(())?);
    println!("{:#?}", i.f(("Trait Fn", 3))?);
    let flex_arg3 = &trait_fn::definition::arg::InfoBuilder::default()
        .father_name("Trait Fn Father")
        .mother_name("Trait Fn Mother")
        .weight(90)
        .build()?;
    println!("{:#?}", i.f(flex_arg3)?);
    Ok(())
}
