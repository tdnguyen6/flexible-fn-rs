use super::definition::arg;
use super::definition::sig;
use super::definition::O;
use anyhow::Result;
use std::collections::HashMap;

impl sig::F<O, Result<i32>> for () {
    fn f(&self, _o: &O) -> Result<i32> {
        Ok(2)
    }
}

impl sig::F<O, Result<HashMap<i32, String>>> for (&str, i32) {
    fn f(&self, _o: &O) -> Result<HashMap<i32, String>> {
        Ok(HashMap::from([(self.1, String::from(self.0))]))
    }
}

impl sig::F<O, Result<Vec<String>>> for &arg::Info<'_> {
    fn f(&self, _o: &O) -> Result<Vec<String>> {
        Ok(vec![String::from("trait_fn"), format!("{:#?}", self)])
    }
}
