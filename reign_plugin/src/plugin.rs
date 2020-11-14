use reign_router::Router;

pub trait Plugin {
    fn init(&self) {}

    fn router(&self, f: Box<dyn FnOnce(&mut Router)>) -> Box<dyn FnOnce(&mut Router)> {
        f
    }
}
