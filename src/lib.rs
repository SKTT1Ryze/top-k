//! A Crate for Providing Some Implementation of `Top K` Algorithm
//! This crate provide `TopK` trait, 
//! with witch developer can implement `Top K` solution using different algorithm, such as `quick selection`.  
//! 
//! Currently only provide `QuickSelection` witch implemented `TopK` trait.  
//! See `src/quick_select.rs`.  
//! 
//! example:  
//! ```Rust
//! extern crate top_k;
//! use top_k::{TopK, TopKErr};
//! fn main() -> Result<(), TopKErr> {
//!     let data = vec![1, 2, 3, 7, 8, 9, 0];
//!     let mut qs = top_k::quick_select::QuickSelect::new(2);
//!     qs.add_tiems(data);
//!     let res = qs.top_k()?;
//!     if res != vec![8, 9] && res != vec![9, 8] {
//!         panic!()        
//!     }
//!     Ok(())
//! }
//! ```
//! 
//! 
//! 
//! 
//! 
//! 

#![deny(missing_docs)]

use std::cmp::{Eq, Ord};
pub mod quick_select;

/// `TopK` trait allows us implement `Top K` solutions with different algorithm.  
/// The generic `I` identifies something can be compare, 
/// implemented `std::cmp::Eq` and `std::cmp::Ord` trait.  
pub trait TopK<I>
    where I: Eq + Ord + Clone + Copy
{
    /// Crate an instance implemented `TopK` with specified `K`
    fn new(k: usize) -> Self;

    /// Add an item
    fn add_item(&mut self, item: I);

    /// Add some items, which implemented `IntoIterator` trait bound.  
    /// Example: [I; _], Vec<I>, etc.  
    fn add_items<IS: IntoIterator<Item = I>>(&mut self, items: IS) {
        for item in items {
            self.add_item(item);
        }
    }

    /// Reset the `TopK`, most commonly clear the items inside.  
    fn reset(&mut self);

    /// Found the top k of given items.  
    /// Return TopKErr if some errors occur.  
    fn top_k(&mut self) -> Result<Vec<I>, TopKErr>;
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum TopKErr {
    ItemsEmpty
}