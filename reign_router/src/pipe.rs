use crate::Middleware;

use std::sync::Arc;

pub(crate) type MiddlewareItem = Box<dyn Middleware + Send + Sync + 'static>;

/// Middleware pipe that contains a list of middlewares to run
///
/// Can only be created by being used in the router definition
///
/// # Examples
///
/// ```
/// use reign::router::{Router, middleware::Runtime};
///
/// fn router(r: &mut Router) {
///     r.pipe("common").add(Runtime::default());
/// }
/// ```
pub struct Pipe {
    pub(crate) middlewares: Vec<Arc<MiddlewareItem>>,
}

impl Pipe {
    pub(crate) fn new() -> Self {
        Self {
            middlewares: vec![],
        }
    }

    /// Add a middleware to the pipe
    #[allow(clippy::should_implement_trait)]
    pub fn add<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.push(Arc::new(Box::new(middleware)));
        self
    }
}
