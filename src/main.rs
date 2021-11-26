mod input_output_overload;
mod input_overload;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("---------------------------------------------------------------------------------------------------------------------------------------------------------------------------\nInput Overloading Only:");
    input_overload::demo().await?;
    println!("---------------------------------------------------------------------------------------------------------------------------------------------------------------------------\nInput and Output Overloading:");
    input_output_overload::demo().await?;

    Ok(())
}
