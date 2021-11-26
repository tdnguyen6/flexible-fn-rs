use super::definition::arg;
use super::definition::sig;
use super::definition::T;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct I;

impl T for I {}

impl sig::F<I> for () {
    type Output = Result<i32>;
    fn f(&self, _o: &I) -> Self::Output {
        Ok(3)
    }
}

impl sig::F<I> for (&str, i32) {
    type Output = Result<HashMap<i32, String>>;
    fn f(&self, _o: &I) -> Self::Output {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F<I> for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    fn f(&self, _o: &I) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}

#[async_trait]
impl sig::FAsync<I> for &arg::Info<'_> {
    type Output = Result<Vec<String>>;
    async fn f_async(&self, _o: &I) -> Self::Output {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
