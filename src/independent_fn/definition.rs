pub mod sig {
    use async_trait::async_trait;

    pub trait F<R> {
        fn f(&self) -> R;
    }

    #[async_trait]
    pub trait FAsync<R> {
        async fn f_async(&self) -> R;
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

pub fn f<P: sig::F<R>, R>(p: P) -> R {
    p.f()
}

pub async fn f_async<P: sig::FAsync<R>, R>(p: P) -> R {
    p.f_async().await
}
