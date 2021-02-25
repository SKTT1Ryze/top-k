//! 小作业：  
//! 100 GB 的 URL 文件，使用最多 1 GB 内存计算出现次数 Top 100 的 URL 和各自的出现次数。
//! **要求：**  
//! - 最多使用 1 GB 内存
//! - 性能越快越好
//! 
//! 思路：  
//! + 将大文件拆分为 100 个小文件，每次对小文件进行 Top 100 计算，然后再整合起来进行 Top 100 运算
//! + 以 `Crate` 的形式设计 Top K 计算模块，提高代码的可复用度
//! + 提供 `TopK` trait，方便使用各种算法实现 Top K 运算
//! + 目前用快速选择算法实现了 `TopK` trait: [QuickSelect](../src/quick_select.rs)
//! + 使用 Rust 的异步机制进行文件的拆分
//!     - file.read(&mut buffer).await?
//!     - file.write(&mut buffer).await?
//!     - 注意数据的同步与互斥
//! + 使用 `HashMap` 保存 url 数据和对应的计数，提高检索速率
//! 
//! URL 数据来源及处理： 
//! + 由于硬盘空间有限，存不下 100 GB 的数据，因此使用 100 M 的 `urldata.csv`, 来源于 `kaggle` 的数据集：https://www.kaggle.com/teseract/urldataset （去掉标签）
//! + 关于内存限制，这里只创建一个 100 M / 100 = 1 M 大小的数组 `buffer`，用于读取数据
//! + 将 `urldata.csv` 文件分割为 100 个小文件，命名方式为 `child_0.csv` ~ `child_99.csv` 
//! 
//! 整体流程:  
//! 异步拆分文件 -> 分步读取小文件数据 -> 对小文件进行 Top 100 计算，并保存数据 -> 汇总数据，进行最后的 Top 100 计算 -> 得出结果。  
//! 
//! 性能分析:  
//! + 优点
//!     - 快速选择算法的时间复杂度是 O(n)，最坏情况是 O(n^2)，但是我们随机选择分割点，因此可以最大程度上减少最坏情况的发生
//!     - 异步代码相对于多线程，性能高并且轻量级：产生线程的代价很昂贵。因此异步读写文件比较轻量高效
//!     - Rust 语言特有的所有权和生命周期机制，使得内存管理十分安全和高效，不会出现内存溢出的情况
//! + 缺点
//!     - 将一个大文件分成多个小文件分别进行 Top K 计算的方案是有时间代价的，特别是文件大小远远大于内存限制的容量的时候这种代价尤为昂贵，比如只能用 1 GB 内存去处理 10000 GB 大小的数据，这意味着我们必须对每个小文件的计算时间乘上 10000
//!     - 如果 url 数据重复比较多的时候，真正有效的 url 数据可能远远小于 100 GB,上面的方案没有利用到这点
//! 
//! 

extern crate top_k;
extern crate async_std;

use async_std::io;
use async_std::fs::File;
use async_std::prelude::*;
use async_std::path::Path;

use top_k::{TopK, TopKErr};

pub async fn read_and_write_file<'url, P, PS>(source: P, destinations: PS, buffer: &'url mut [u8]) -> io::Result<()>
    where
        P: AsRef<Path>,
        PS: IntoIterator<Item = P> + Clone
{
    let mut source_f = File::open(source).await?;
    let mut iter = destinations.into_iter();
    loop {
        if let Some(d) = iter.next() {
            let mut d_f = File::create(d).await?;
            // 读源文件到 buffer 里，返回读取的字节数
            let bytes = source_f.read(buffer).await?;
            // 如果返回的字节数是 0，那么就对应以下两种情况：  
            // 1. 文件读写完毕
            // 2. buffer 长度为 0
            // 无论是哪种情况，都将返回
            if bytes == 0 {
                return Ok(());
            }
            // 写数据到目标文件
            d_f.write_all(&buffer[..bytes]).await?
        }
        else {
            // 所有子文件都写入完毕，返回
            return Ok(());
        }
    }
    
}

fn main() -> io::Result<()> {
    // 计时器
    let timer = std::time::Instant::now();
    // 1 M = 1 * 2 ^ 20 bytes = (1 << 20) u8
    let mut buffer = [0u8; 1 << 20];
    let mut files = Vec::new();
    for i in 0..100 {
        files.push(format!("child_{}.csv", i));
    }
    // 将 100 M 大文件分割为 100 个小文件
    async_std::task::block_on(async {
        read_and_write_file(String::from("urldata.csv"), files.clone(), &mut buffer).await.unwrap();
    });
    let split_file_time = timer.elapsed().as_millis();
    // 分割完， 将 1 M 大小的数组 drop 掉
    drop(buffer);
    
    use std::cmp::Ordering;
    #[derive(Clone, Copy, Debug)]
    struct UrlData<'url> (
        // (url, 计数)
        &'url str, usize
    );
    impl<'url> UrlData<'url> {
        pub fn new(url: &'url str, frequency: usize) -> Self {
            UrlData (
                url,
                frequency
            )
        }
        pub fn url(&self) -> &'url str {
            self.0
        }
        pub fn frequency(&self) -> usize {
            self.1
        }
    }
    impl<'url> PartialEq<UrlData<'url>> for UrlData<'url> {
        fn eq(&self, other: &UrlData) -> bool {
            self.1 == other.1
        }
    }
    impl<'url> Eq for UrlData<'url> {}
    impl<'url> PartialOrd for UrlData<'url> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.1.partial_cmp(&other.1)
        }
    }
    impl<'url> Ord for UrlData<'url> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.1.cmp(&other.1)
        }
    }

    use std::io::Read;
    use std::collections::HashMap;
    let mut results = Vec::new();
    for f in files {
        let mut hash: HashMap<&str, usize> = HashMap::new();
        let mut url_f = std::fs::File::open(f)?;
        // 创建 buffer 读取数据，由于已经分割过大文件，因此 buffer 占用的内存不会大于 1 M
        let mut contents = String::new();
        // 将数据读进内存
        url_f.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            // 如果哈希表里面存在相应的 url,则将计数加一，否则插入一行记录
            if let Some(key) = hash.get_mut(line) {
                *key += 1;
            } else {
                hash.insert(line, 1usize);
            }
        }
        let url_datas: Vec<UrlData> = hash.into_iter().map(|u| {
            UrlData::new(u.0, u.1)
        }).collect();
        // 创建快速选择算法用于 Top K 计算
        let mut qs = top_k::quick_select::QuickSelect::new(100);
        qs.add_items(url_datas);
        // 计算该小文件的 Top 100
        match qs.top_k() {
            Ok(res) => {
                // 将结果放进 `results` 里面保存
                for r in res {
                    results.push(
                        (String::from(r.url()), r.frequency())
                    );
                }        
            }
            Err(TopKErr::ItemsEmpty) => {}, // do nothing
        };
    }

    // 将 100 个小文件的 Top 100 整合起来进行 Top 100 计算
    let mut urls = Vec::new();
    for res in &results {
        urls.push(
            UrlData::new(res.0.as_str(), res.1)
        )
    }
    let mut final_qs = top_k::quick_select::QuickSelect::new(100);
    final_qs.add_items(urls);
    let final_res = final_qs.top_k().unwrap();
    let compute_time = timer.elapsed().as_millis() - split_file_time;
    println!("{:?}", final_res);
    println!("handle files use {} milliseconds, compute use {} milliseconds", split_file_time, compute_time);
    Ok(())
}
