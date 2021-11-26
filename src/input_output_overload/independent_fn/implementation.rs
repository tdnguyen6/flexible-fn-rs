use super::definition::arg;
use super::definition::sig;
use anyhow::Result;
use async_trait::async_trait;

impl sig::F<Result<i32>> for &arg::Info<'_> {
    fn f(&self) -> Result<i32> {
        Ok(1)
    }
}

impl sig::F<Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<Result<i32>> for &arg::Info<'_> {
    async fn f_async(&self) -> Result<i32> {
        Ok(2)
    }
}

#[async_trait]
impl sig::FAsync<Result<Vec<String>>> for &arg::Info<'_> {
    async fn f_async(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
