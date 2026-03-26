<img src=https://github.com/DrCMWither/qsisp/blob/master/docs/logo/qsisp.png width=300 />

# qƨisp

[DE](README_DE.md)|[EN](README_EN.md)|[FR](README_FR.md)|[ZH](README.md)

### 编程语言：创造你自己！

---

## qƨisp 是什么？

**qƨisp** （如果你的字体不支持，就叫它 qU+01A8isp。qsisp 并非这门语言的标准名称！）是一门基于 locale 的多语言 Lisp 方言。

它会根据你的系统语言改变：

* 关键字；
* 括号；
* 字符串；
* 注释；
* 你的心理状态！


## 特性

### 基于 locale 的语法

创造你自己的代码：用你自己熟悉的语言写 qƨisp！你不用学 qƨisp，或者 Lisp 的关键词，甚至不一定用学英语：qƨisp 会适配你。


## 副作用

* 再也无法直视 ASCII 括号；
* 精神崩溃；
* IDE：**我不干了**！


## 示例

```lisp
“始
  “定 x （10）
  “如 （<= x 10）
    “印（x）
””””
```

请尝试用法语版本重写。

或者全部语言混用。

我们建议不要。


## 如何本地运行/开发

1. 克隆该项目；

```bash
git clone https://github.com/DrCMWither/qsisp.git
cd qsisp
```

2. 更新你的本地环境。该项目要求 `Rust >= 1.75` 和 `cargo`；

3. 直接运行；或者构建 release。

```bash
cargo run -- example/test.qs
cargo build --release
```

## 获奖记录

* 跨文化编程语言恐怖奖
* reader 层面精神攻击特别提名
* 2026 最难蚌 Lisp 方言


## 未来计划

* 真正的 RTL 解析；
* 混合语言 AST；
* IDE 插件（基本不可用）；
* 形式语义（如果还有人活着的话）。

_顺带一提——这门语言的名称不是回文，只是镜像。_