#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct ArchetypeId(&'static str);

impl ArchetypeId {
    pub fn new(id: &'static str) -> Self {
        Self(id)
    }

    pub fn as_ref(&self) -> &'static str {
        self.0
    }
}
