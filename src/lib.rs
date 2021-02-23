//! A Crate for Providing Some Implementation of `Top K` Algorithm
//! 

use std::cmp::{Eq, Ord};
mod quick_select;

pub trait TopK<I>
    where I: Eq + Ord + Clone + Copy
{
    /// Crate an instance implemented `TopK`
    fn new(k: usize) -> Self;

    /// Add an item
    fn add_item(&mut self, item: I);

    /// Add some items
    fn add_items<IS: IntoIterator<Item = I>>(&mut self, items: IS) {
        for item in items {
            self.add_item(item);
        }
    }

    /// Found the top k of given items
    fn top_k(&mut self) -> Result<Vec<I>, TopKErr>;
}

#[derive(Debug)]
pub enum TopKErr {
    ItemsEmpty
}