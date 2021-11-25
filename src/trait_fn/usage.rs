use std::collections::HashMap;

use anyhow::Result;

use super::definition::sig;
use super::definition::arg;
use super::definition::T;

pub struct I;

impl T for I {}

impl sig::F<I, Result<i32>> for () {
    fn f(&self, _o: &I) -> Result<i32> {
        Ok(3)
    }
}

impl sig::F<I, Result<HashMap<i32, String>>> for (&str, i32) {
    fn f(&self, _o: &I) -> Result<HashMap<i32, String>> {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F<I, Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self, _o: &I) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
