#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataAction {
    Preserve,
    Add,
    Modify,
    Remove,
}
