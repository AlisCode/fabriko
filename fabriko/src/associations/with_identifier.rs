pub trait WithIdentifier {
    type ID;
    fn extract_id(&self) -> Self::ID;
}
