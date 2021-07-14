# top-k
Top K Algorithm Implementation with Rust

## Features
+ Provide `TopK` trait, with witch developer can implement `Top K` solution using different algorithm.
+ Currently finish implementation of `TopK` trait with `Quick Selection` algorithm.

## Example
```Rust
extern crate top_k;
use top_k::{TopK, TopKErr};
fn main() -> Result<(), TopKErr> {
    let data = vec![1, 2, 3, 7, 8, 9, 0];
    let mut qs = top_k::quick_select::QuickSelect::new(2);
    
    qs.add_items(data);
    let res = qs.top_k()?;
    
    if res != vec![8, 9] && res != vec![9, 8] {
        panic!()        
    }
    Ok(())
}
```

## UrlTop100
The [example](./example) folder provide a solution for `UrlTop100` problem, check and see details.  
