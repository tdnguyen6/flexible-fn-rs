use super::definition::arg;
use super::definition::sig;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

impl sig::F for () {
    type Output = Result<i32>;
    fn f(&self) -> Self::Output {
        Ok(1)
    }
}

impl sig::F for (&str, i32) {
    type Output = Result<HashMap<i32, String>>;
    fn f(&self) -> Self::Output {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    fn f(&self) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    async fn f_async(&self) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
