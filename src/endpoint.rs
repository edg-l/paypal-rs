use std::borrow::Cow;
use serde::{Serialize, de::DeserializeOwned};
use crate::{SANDBOX_ENDPOINT, LIVE_ENDPOINT};

pub trait Endpoint {
    type Query: Serialize;
    type Body: Serialize;
    type Response: DeserializeOwned;

    // The endpoint relative path. Must start with a `/`
    fn relative_path(&self) -> Cow<str>;

    // The request method.
    fn method(&self) -> reqwest::Method;

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }

    fn full_path(&self, is_sandbox: bool) -> String {
        if is_sandbox {
            format!("{}{}", SANDBOX_ENDPOINT, self.relative_path())
        } else {
            format!("{}{}", LIVE_ENDPOINT, self.relative_path())
        }
    }
}