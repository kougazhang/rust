# Rust 基础

## 二. 基本语法

## 2. 数据类型

### 复合类型

```rust
// 初始化
let mut vec = Vec::new();
// 添加元素
vec.push(1);
// 索引
vec[0]
```

### 4. 控制流

**for 循环**

```rust
for <condition> {
  ...
  break;
}
```

### 7. 命令行

**获取命令行参数**

```rust
use std::env;

let args: Vec<String> = env::args().collect();
println!("{:?}", args);
```



## Cargo 包管理

+ 创建新项目: `cargo new <packageName>`
+ 编译: `cargo build`
  + 二进制文件: `./target/debug/<packageName>`

