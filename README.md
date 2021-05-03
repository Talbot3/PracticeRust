# Practice Rust

## Rust环境配置

 ### Rust 镜像源
```bash
export RUSTUP_DIST_SERVER=http://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT:http://mirrors.ustc.edu.cn/rust-static/rustup
```

 ### Rust nightly build
 > use some feature on nightly or test
 > 参考
 > - https://www.rust-lang.org/tools/install
 ```
 ## 安装最新稳定版
 rustup update stable
 ## 安装最新开发版
 rustup run nightly cargo build
 ```
 

#### 为项目配置镜像源
> 在 $HOME/.cargo/config 中添加如下内容：

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

```


## 特性 Trait
> 相比C++的的Class，使用新的特性替换类的概念要好很多，更符合C流派的风格。

- 支持操作符重载，用于自定义类型时操作很方便，更直观。

## 参考
- https://rustwiki.org/zh-CN//rust-by-example/
- https://kaisery.github.io/trpl-zh-cn/
- https://cargo.budshome.com/
- https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
- https://www.rust-lang.org/learn/get-started
- https://zsiciarz.github.io/24daysofrust/book/vol2/day23.html
- https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
- https://dev.to/ender_minyard/full-stack-developer-s-roadmap-2k12