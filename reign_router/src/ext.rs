use crate::{hyper::StatusCode, Error};

/// Extension trait for [`Option<T>`] containing methods for ease of use in handles.
pub trait OptionExt {
    type Inner;

    fn ok_or_404(self) -> Result<Self::Inner, Error>;
}

impl<T> OptionExt for Option<T> {
    type Inner = T;

    /// Transforms the [`Option<T>`] into a [`Result<T, E>`], mapping `Some(v)`
    /// to `Ok(v)` and `None` to 404 [`Error`].
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize, Clone)]
    /// struct User(String);
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     let user = req.session::<User>().ok_or_404()?;
    ///
    ///     Ok(user.0.clone())
    /// }
    /// ```
    fn ok_or_404(self) -> Result<Self::Inner, Error> {
        self.ok_or_else(|| Error::Status(StatusCode::NOT_FOUND))
    }
}
