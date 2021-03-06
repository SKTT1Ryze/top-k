## 题目
100 GB 的 URL 文件，使用最多 1 GB 内存计算出现次数 Top 100 的 URL 和各自的出现次数。

## **要求：**  
- 最多使用 1 GB 内存
- 性能越快越好

## 思路
+ 将大文件拆分为 100 个小文件，每次对小文件进行 Top 100 计算，然后再整合起来进行 Top 100 运算
+ 以 `Crate` 的形式设计 Top K 计算模块，提高代码的可复用度
+ 提供 `TopK` trait，方便使用各种算法实现 Top K 运算
+ 目前用快速选择算法实现了 `TopK` trait: [QuickSelect](../src/quick_select.rs)
+ 使用 Rust 的异步机制进行文件的拆分
    - file.read(&mut buffer).await?
    - file.write(&mut buffer).await?
    - 注意数据的同步与互斥
+ 使用 `HashMap` 保存 url 数据和对应的计数，提高检索速率

## URL 数据来源及处理
+ 由于硬盘空间有限，存不下 100 GB 的数据，因此使用 100 M 的 `urldata.csv`, 来源于 `kaggle` 的数据集：https://www.kaggle.com/teseract/urldataset （去掉标签）
+ 关于内存限制，这里只创建一个 100 M / 100 = 1 M 大小的数组 `buffer`，用于读取数据
+ 将 `urldata.csv` 文件分割为 100 个小文件，命名方式为 `child_0.csv` ~ `child_99.csv` 

## 整体流程
异步拆分文件 -> 分步读取小文件数据 -> 对小文件进行 Top 100 计算，并保存数据 -> 汇总数据，进行最后的 Top 100 计算 -> 得出结果。  

## 性能分析
+ 优点
    - 快速选择算法的时间复杂度是 O(n)，最坏情况是 O(n^2)，但是我们随机选择分割点，因此可以最大程度上减少最坏情况的发生
    - 异步代码相对于多线程，性能高并且轻量级：产生线程的代价很昂贵。因此异步读写文件比较轻量高效
    - Rust 语言特有的所有权和生命周期机制，使得内存管理十分安全和高效，不会出现内存溢出的情况
+ 缺点
    - 将一个大文件分成多个小文件分别进行 Top K 计算的方案是有时间代价的，特别是文件大小远远大于内存限制的容量的时候这种代价尤为昂贵，比如只能用 1 GB 内存去处理 10000 GB 大小的数据，这意味着我们必须对每个小文件的计算时间乘上 10000
    - 如果 url 数据重复比较多的时候，真正有效的 url 数据可能远远小于 100 GB,上面的方案没有利用到这点

## 测试
+ 準確性测试
    - `TopK` 算法的单元测试，比如 [quick_select_test](../src/quick_select.rs)
    - todo()!

+ 性能测试
    - 运行：`cargo run`
    - CPU: Intel Core i7-8750H @ 12x 4.1GHz
    - GPU: Intel Corporation UHD Graphics 630
    - OS: Manjaro 20.2.1 Nibia
    - Kernel: x86_64 Linux 5.10.15-1-MANJARO
    - 文件处理时间（拆分大文件）: 182 milliseconds
    - 计算时间（Top K 计算）: 16700 milliseconds


## 优化
+ 更好的数据处理框架
+ 使用分布式系统进行各个小文件的 Top K 计算
