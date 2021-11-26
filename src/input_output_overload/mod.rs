mod independent_fn;
mod struct_fn;
mod trait_fn;

use anyhow::Result;
use independent_fn::definition::arg::Info as Info1;
use independent_fn::definition::arg::InfoBuilder as InfoBuilder1;
use independent_fn::definition::*;
use struct_fn::definition::arg::Info as Info2;
use struct_fn::definition::arg::InfoBuilder as InfoBuilder2;
use struct_fn::definition::*;
use trait_fn::definition::arg::Info as Info3;
use trait_fn::definition::arg::InfoBuilder as InfoBuilder3;
use trait_fn::definition::*;
use trait_fn::implementation::*;

pub async fn demo() -> Result<()> {
    // independent fn ---------------------------------------------------------
    let flex_arg1 = &InfoBuilder1::default()
        .birth_day("1990-12-07")
        .father_name("Independent Fn Father")
        .mother_name("Independent Fn Mother")
        .build()?;

    let res: Result<i32> = f(flex_arg1);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!("Sync:\n{:#?}", f::<&Info1, Result<i32>>(flex_arg1)?);

    let res: Result<Vec<String>> = f(flex_arg1);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!("Sync:\n{:#?}", f::<&Info1, Result<Vec<String>>>(flex_arg1)?);

    let res: Result<i32> = f_async(flex_arg1).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     f_async::<&Info1, Result<i32>>(flex_arg1).await?
    // );

    let res: Result<Vec<String>> = f_async(flex_arg1).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     f_async::<&Info1, Result<Vec<String>>>(flex_arg1).await?
    // );

    // struct fn ---------------------------------------------------------
    let o = O {};
    let flex_arg2 = &InfoBuilder2::default()
        .height(180)
        .father_name("Struct Fn Father")
        .mother_name("Struct Fn Mother")
        .build()?;
    let res: Result<i32> = o.f(flex_arg2);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!("Sync:\n{:#?}", o.f::<&Info2, Result<i32>>(flex_arg2)?);

    let res: Result<Vec<String>> = o.f(flex_arg2);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!(
    //     "Sync:\n{:#?}",
    //     o.f::<&Info2, Result<Vec<String>>>(flex_arg2)?
    // );

    let res: Result<i32> = o.f_async(flex_arg2).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     o.f_async::<&Info2, Result<i32>>(flex_arg2).await?
    // );

    let res: Result<Vec<String>> = o.f_async(flex_arg2).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     o.f_async::<&Info2, Result<Vec<String>>>(flex_arg2).await?
    // );

    // trait fn ---------------------------------------------------------
    let i = I {};
    let flex_arg3 = &InfoBuilder3::default()
        .father_name("Trait Fn Father")
        .mother_name("Trait Fn Mother")
        .weight(90)
        .build()?;
    let res: Result<i32> = i.f(flex_arg3);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!("Sync:\n{:#?}", i.f::<&Info3, Result<i32>>(flex_arg3)?);

    let res: Result<Vec<String>> = i.f(flex_arg3);
    println!("Sync:\n{:#?}", res?);
    // another way
    // println!(
    //     "Sync:\n{:#?}",
    //     i.f::<&Info3, Result<Vec<String>>>(flex_arg3)?
    // );

    let res: Result<i32> = i.f_async(flex_arg3).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     i.f_async::<&Info3, Result<i32>>(flex_arg3).await?
    // );

    let res: Result<Vec<String>> = i.f_async(flex_arg3).await;
    println!("Async:\n{:#?}", res?);
    // another way
    // println!(
    //     "Async:\n{:#?}",
    //     i.f_async::<&Info3, Result<Vec<String>>>(flex_arg3).await?
    // );

    Ok(())
}
