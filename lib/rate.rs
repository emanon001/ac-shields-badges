#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rate(pub(crate) u32);

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
