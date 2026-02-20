pub trait Image {
    fn as_bitmap(&self) -> Vec<u8>;
}
