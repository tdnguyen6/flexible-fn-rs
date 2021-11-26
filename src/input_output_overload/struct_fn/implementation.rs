use super::definition::arg;
use super::definition::sig;
use super::definition::O;
use anyhow::Result;
use async_trait::async_trait;

impl sig::F<O, Result<i32>> for &arg::Info<'_> {
    fn f(&self, _o: &O) -> Result<i32> {
        Ok(3)
    }
}

impl sig::F<O, Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self, _o: &O) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<O, Result<i32>> for &arg::Info<'_> {
    async fn f_async(&self, _o: &O) -> Result<i32> {
        Ok(4)
    }
}

#[async_trait]
impl sig::FAsync<O, Result<Vec<String>>> for &arg::Info<'_> {
    async fn f_async(&self, _o: &O) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
