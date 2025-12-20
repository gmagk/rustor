use serde::de::DeserializeOwned;
use std::io::Error;
use tokio::runtime::Runtime;

pub struct HttpClient {
    runtime: Runtime
}

impl HttpClient {

    pub fn new(runtime: Runtime) -> Self {
        Self { runtime }
    }

    pub fn request<T>(&self, url: String) -> Result<T, Error>
    where T: DeserializeOwned {
        self.runtime.block_on(async {
            match reqwest::get(url).await {
                Ok(response) => match response.json::<T>().await {
                    Ok(dto) => Ok(dto),
                    Err(e) => Err(Error::other(e))
                },
                Err(e) => { Err(Error::other(e)) }
            }
        })
    }
}