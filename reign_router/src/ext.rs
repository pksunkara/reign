use crate::{hyper::StatusCode, Error};

pub trait OptionExt {
    type Inner;

    fn ok_or_404(self) -> Result<Self::Inner, Error>;
}

impl<T> OptionExt for Option<T> {
    type Inner = T;

    fn ok_or_404(self) -> Result<Self::Inner, Error> {
        self.ok_or_else(|| Error::Status(StatusCode::NOT_FOUND))
    }
}
