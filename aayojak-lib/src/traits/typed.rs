/// Typed is a generic trait which helps getting the struct type as metadata to the object
pub trait Typed {
    /// get type of implementor as a string slice
    fn get_type(&self) -> &str;
}
