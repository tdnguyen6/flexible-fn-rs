use super::definition::arg;
use super::definition::sig;
use super::definition::O;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

impl sig::F<O> for () {
    type Output = Result<i32>;
    fn f(&self, _o: &O) -> Self::Output {
        Ok(2)
    }
}

impl sig::F<O> for (&str, i32) {
    type Output = Result<HashMap<i32, String>>;
    fn f(&self, _o: &O) -> Self::Output {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F<O> for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    fn f(&self, _o: &O) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<O> for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    async fn f_async(&self, _o: &O) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
