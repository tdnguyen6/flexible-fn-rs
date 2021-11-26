use anyhow::Result;
use super::definition::arg;
use super::definition::sig;
use super::definition::T;
use async_trait::async_trait;

pub struct I;

impl T for I {}

impl sig::F<I, Result<i32>> for &arg::Info<'_> {
    fn f(&self, _o: &I) -> Result<i32> {
        Ok(5)
    }
}

impl sig::F<I, Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self, _o: &I) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<I, Result<i32>> for &arg::Info<'_> {
    async fn f_async(&self, _o: &I) -> Result<i32> {
        Ok(6)
    }
}

#[async_trait]
impl sig::FAsync<I, Result<Vec<String>>> for &arg::Info<'_> {
    async fn f_async(&self, _o: &I) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
