mod independent_fn;
mod struct_fn;
mod trait_fn;

use anyhow::Result;
use independent_fn::definition::*;
use struct_fn::definition::*;
use trait_fn::definition::*;
use trait_fn::usage::*;

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

    let o = O {};
    println!("{:#?}", o.f(())?);
    println!("{:#?}", o.f(("Struct Fn", 2))?);
    let flex_arg2 = &struct_fn::definition::arg::InfoBuilder::default()
        .birth_day("1995-04-28")
        .father_name("Struct Fn Father")
        .mother_name("Struct Fn Mother")
        .build()?;
    println!("{:#?}", o.f(flex_arg2)?);

    let i = I {};
    println!("{:#?}", i.f(())?);
    println!("{:#?}", i.f(("Trait Fn", 2))?);
    let flex_arg3 = &trait_fn::definition::arg::InfoBuilder::default()
        .birth_day("2005-11-03")
        .father_name("Trait Fn Father")
        .mother_name("Trait Fn Mother")
        .build()?;
    println!("{:#?}", i.f(flex_arg3)?);
    Ok(())
}
