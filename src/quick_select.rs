//! `QuickSelect` is an implementation of `TopK` trait, 
//! which use the quick select algorithm  
//! 

use std::cmp::{Eq, Ord};
use rand::Rng;
use crate::{TopK, TopKErr};

/// Implementation of `TopK` trait with `quick select` algorithm
pub struct QuickSelect<I: Eq + Ord + Clone + Copy> {
    items: Vec<I>,
    k: usize,
}

impl<I: Eq + Ord + Clone + Copy> TopK<I> for QuickSelect<I> {
    fn new(k: usize) -> Self {
        Self {
            items: Vec::new(),
            k,
        }
    }

    fn add_item(&mut self, item: I) {
        self.items.push(item);
    }

    /// Use quick selection algorithm to find the top k values.  
    /// Return `TopKErr::ItemsEmpty` if `self.items` is empty.  
    /// Return entire `self.items` if `self.items.len()` < `self.k`.  
    /// 
    fn top_k(&mut self) -> Result<Vec<I>, TopKErr> {
        if self.items.is_empty() {
            return Err(TopKErr::ItemsEmpty);
        }
        let mut res = Vec::new();
        if self.k > self.items.len() {
            for item in &self.items {
                res.push(*item);
                return Ok(res);
            }
        }
        let mut pivot;
        let mut left = 0;
        let mut right = self.items.len() -1;
        let mut k = self.k;
        let mut order = rand::thread_rng();
        loop {
            // randomly get pivot 
            let index = order.gen_range(left..right + 1);
            // swap the pivot to the back of seleted part of `self.items`
            self.items.swap(index, right);
            pivot = self.items[right];
            let mut i = left;
            let mut j = left;
            while j != right {
                if self.items[j] <= pivot {
                    // if <= pivot, swap the value at i and j, 
                    // and both add one
                    self.items.swap(i, j);
                    i += 1;
                    j += 1;
                } else {
                    // else j add one
                    j += 1;
                }
            }
            if pivot != self.items[j] {
                panic!()
            }
            if right - i + 1 == k {
                // found the kth value, return
                break;
            }
            // continue
            // swap the items[i] and items[j]
            self.items.swap(i, j);
            if right - i + 1 > k {
                // the top k at the right of pivot
                left = i + 1;
            } else {
                // the top k at the left of pivot
                k = k - (right - i + 1);
                right = i - 1;
            }
        }
        for item in &self.items {
            if *item >= pivot {
                res.push(*item);
            }
        }
        Ok(res)
    }

}

#[test]
fn simple_test_qucik_selection() -> Result<(), TopKErr> {
    let mut qs = QuickSelect::<usize>::new(2);
    qs.add_items(vec![1, 2, 4, 5, 7, 0, 9, 3]);
    let res = qs.top_k()?;
    if res != vec![7, 9] && res != vec![9, 7] {
        panic!("test qucik selection top k failed, res: {:?}", res);
    }
    Ok(())
}

#[test]
fn large_test_quick_selection() -> Result<(), TopKErr> {
    use rand::Rng;
    let mut qs = QuickSelect::<usize>::new(50);
    let mut data = Vec::new();
    let mut random = rand::thread_rng();
    for _ in 0..1000 {
        data.push(random.gen_range(0..1000));
    }
    qs.add_items(data.clone());
    data.sort();
    for item in qs.top_k()? {
        if item < data[950] {
            panic!("item {} in result is not the top k.", item)
        }
    }
    Ok(())
}
