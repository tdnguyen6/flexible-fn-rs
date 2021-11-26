mod independent_fn;
mod struct_fn;
mod trait_fn;

use anyhow::Result;
use independent_fn::definition::arg::InfoBuilder as InfoBuilder1;
use independent_fn::definition::*;
use struct_fn::definition::arg::InfoBuilder as InfoBuilder2;
use struct_fn::definition::*;
use trait_fn::definition::arg::InfoBuilder as InfoBuilder3;
use trait_fn::definition::*;
use trait_fn::implementation::*;

pub async fn demo() -> Result<()> {
    // independent fn ---------------------------------------------------------
    println!("{:#?}", f(())?);
    println!("{:#?}", f(("Independent Fn", 1))?);
    let flex_arg1 = &InfoBuilder1::default()
        .birth_day("1990-12-07")
        .father_name("Independent Fn Father")
        .mother_name("Independent Fn Mother")
        .build()?;
    println!("Sync:\n{:#?}", f(flex_arg1)?);
    println!("Async:\n{:#?}", f_async(flex_arg1).await?);

    // struct fn ---------------------------------------------------------
    let o = O {};
    println!("{:#?}", o.f(())?);
    println!("{:#?}", o.f(("Struct Fn", 2))?);
    let flex_arg2 = &InfoBuilder2::default()
        .height(180)
        .father_name("Struct Fn Father")
        .mother_name("Struct Fn Mother")
        .build()?;
    println!("Sync:\n{:#?}", o.f(flex_arg2)?);
    println!("Async:\n{:#?}", o.f_async(flex_arg2).await?);

    // trait fn ---------------------------------------------------------
    let i = I {};
    println!("{:#?}", i.f(())?);
    println!("{:#?}", i.f(("Trait Fn", 3))?);
    let flex_arg3 = &InfoBuilder3::default()
        .father_name("Trait Fn Father")
        .mother_name("Trait Fn Mother")
        .weight(90)
        .build()?;
    println!("Sync:\n{:#?}", i.f(flex_arg3)?);
    println!("Async:\n{:#?}", i.f_async(flex_arg3).await?);
    Ok(())
}
