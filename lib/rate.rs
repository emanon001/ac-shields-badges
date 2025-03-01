#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rate(u32);

impl Rate {
    pub fn new(rate: u32) -> Self {
        Self(rate)
    }

    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
