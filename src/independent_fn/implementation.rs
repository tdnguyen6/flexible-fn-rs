use super::definition::arg;
use super::definition::sig;
use anyhow::Result;
use std::collections::HashMap;
use async_trait::async_trait;

impl sig::F<Result<i32>> for () {
    fn f(&self) -> Result<i32> {
        Ok(1)
    }
}

impl sig::F<Result<HashMap<i32, String>>> for (&str, i32) {
    fn f(&self) -> Result<HashMap<i32, String>> {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F<Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<Result<Vec<String>>> for &arg::Info<'_> {
    async fn f_async(&self) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
