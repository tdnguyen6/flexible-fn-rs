pub struct O;

impl O {
    pub fn f<P: sig::F<Self, R>, R>(&self, p: P) -> R {
        p.f(self)
    }

    pub async fn f_async<P: sig::FAsync<Self, R>, R>(&self, p: P) -> R {
        p.f_async(self).await
    }
}

pub mod sig {
    use async_trait::async_trait;

    pub trait F<O: ?Sized, R> {
        fn f(&self, o: &O) -> R;
    }

    #[async_trait]
    pub trait FAsync<O: ?Sized, R> {
        async fn f_async(&self, o: &O) -> R;
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
