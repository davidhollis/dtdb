use crate::data::identifiers::IdentifierPrefix;

#[derive(Debug)]
pub struct Any {}

impl IdentifierPrefix for Any {
    fn identifier_prefix() -> &'static str { "" }
}