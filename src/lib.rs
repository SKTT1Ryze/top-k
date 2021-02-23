//! A Crate for Providing Some Implementation of `Top K` Algorithm
//! 

pub trait TopK {
    type Item: std::cmp::Eq + std::cmp::Ord;
    /// Crate an instance implemented `TopK`
    fn new() -> Self;

    fn set_k(&mut self, k: usize) -> Result<(), TopKErr>;

    /// Add an item
    fn add_item(&mut self, item: Self::Item) -> Result<(), TopKErr>;

    /// Add some items
    fn add_items<I: IntoIterator<Item = Self::Item>>(&mut self, items: I) -> Result<(), TopKErr>;

    /// Found the top k of given items
    fn top_k(&self) -> Result<Vec<Self::Item>, TopKErr>;
}


pub enum TopKErr {
    OverFlow,
    SetKErr,
    Empty
}