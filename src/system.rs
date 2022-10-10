/// A system. The father of the framework.
pub struct System {}

impl System {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        System::new();
    }

    #[test]
    fn starts() {
        System::new().start();
    }
}
