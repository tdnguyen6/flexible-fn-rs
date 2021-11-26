use async_trait::async_trait;

#[async_trait]
pub trait T {
    fn f<P: sig::F<Self>>(&self, p: P) -> P::Output {
        p.f(self)
    }

    // P must implement Sync + Send to be threadsafe for trait
    async fn f_async<P: sig::FAsync<Self> + Send + Sync>(&self, p: P) -> P::Output {
        p.f_async(self).await
    }
}

pub mod sig {
    use super::*;

    pub trait F<O: ?Sized> {
        type Output;
        fn f(&self, o: &O) -> Self::Output;
    }

    #[async_trait]
    pub trait FAsync<O: ?Sized> {
        type Output;
        async fn f_async(&self, o: &O) -> Self::Output;
    }
}

pub mod arg {
    use derive_builder::Builder;

    #[derive(Default, Builder, Debug)]
    pub struct Info<'a> {
        #[builder(default = "\"David\"")]
        name: &'a str,
        #[builder(default = "\"Matt\"")]
        father_name: &'a str,
        #[builder(default = "\"Sophia\"")]
        mother_name: &'a str,
        #[builder(default = "\"2000-03-06\"")]
        birth_day: &'a str,
        #[builder(default = "170")]
        height: i32,
        #[builder(default = "70")]
        weight: i32,
        #[builder(default = "\"English\"")]
        language: &'a str,
    }
}
