# 用 Rust 手搓几个常用类型

这是我用来学习 Rust 底层原理的一个小项目。

平时我们写 Rust，经常会用到 `Box`、`Vec`、`Rc`、`Arc`、`Cell` 和 `RefCell`。这些类型用起来很方便，但我也挺好奇它们背后到底是怎么申请内存、释放内存和管理引用的，所以就在这个项目里自己动手写了一遍简化版。

项目里用了不少裸指针、手动内存分配和 `unsafe`。主要目的是帮助自己理解原理，不是为了替代标准库。

> [!WARNING]
> 这是一个学习项目，部分功能还没有写完，也没有经过完整的内存安全验证。看看思路、做做实验没问题，但不要直接用到生产项目里。

## 目前写了什么

| 文件 | 内容 |
| --- | --- |
| `my_box.rs` | 一个简单的 `Box`，实现了堆内存分配、解引用和自动释放 |
| `vector.rs` | 一个简单的 `Vec`，支持添加元素、自动扩容和迭代 |
| `string.rs` | 基于自定义 `Vec<char>` 做的字符串小实验 |
| `rc.rs` | 单线程引用计数，能克隆、读取引用数量，并在最后一个引用销毁时释放数据 |
| `arc.rs` | 使用原子计数实现的 `Arc` 基础版本 |
| `cell.rs` | 模仿 `Cell` 做的内部可变性实验，目前还需要继续完善 |
| `refcell.rs` | 模仿 `RefCell`，在运行时检查共享借用和可变借用是否冲突 |

这些实现都比较精简，只覆盖了标准库同名类型的一小部分功能。

## 在这个项目里能学到什么

- Rust 怎么在堆上申请和释放内存
- 裸指针该怎么读写
- `Deref`、`DerefMut` 和 `Drop` 是怎么配合工作的
- `Vec` 扩容时，原来的数据去了哪里
- 自定义集合怎么支持 `for` 循环
- `Rc` 和 `Arc` 怎么通过计数判断什么时候释放数据
- `RefCell` 怎么在运行时检查借用规则
- 为什么写 `unsafe` 时必须格外小心

## 怎么运行

项目使用 Rust 2024 Edition，建议安装 Rust 1.85 或更高版本。

进入项目目录后，可以先检查一下代码：

```bash
cargo check
```

然后运行示例：

```bash
cargo run
```

不过要注意，当前 `lib` 是一个“故意写错”的借用示例：

```rust
let refcell = Refcell::new(User::new());
let mut mutable_ref = refcell.borrow_mut();
let shared_ref = refcell.borrow(); // 这里会 panic
```

代码已经拿到了一个可变借用，又在它释放之前申请共享借用，所以运行到这里会触发：

```text
cannot borrow mutably borrowed value
```

这个 panic 是预期行为，正好可以看到自定义 `Refcell` 的运行时借用检查确实生效了。

如果想让程序正常往下执行，可以先释放可变借用：

```rust
let refcell = Refcell::new(User::new());

{
    let mut mutable_ref = refcell.borrow_mut();
    mutable_ref.check_mut();
}

let shared_ref = refcell.borrow();
shared_ref.check();
```

## 项目结构

```text
.
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── my_box.rs
    ├── vector.rs
    ├── string.rs
    ├── rc.rs
    ├── arc.rs
    ├── cell.rs
    └── refcell.rs
```

## 推荐阅读顺序

如果你也想顺着代码看一遍，我建议按这个顺序：

1. 先看 `my_box.rs`，从一个值的内存申请和释放开始。
2. 再看 `vector.rs`，了解连续内存、自动扩容和迭代器。
3. 接着看 `rc.rs` 和 `arc.rs`，对比普通引用计数和原子引用计数。
4. 最后看 `cell.rs`、`refcell.rs` 和 `lib`，理解内部可变性与运行时借用检查。



总之，这个仓库就是一个边写边理解 Rust 的练习场。代码不一定完整，但每一部分都在尝试回答一个问题：标准库里那些看起来很自然的功能，底层到底是怎么做出来的？
