# Practice Rust

## Rust环境配置

 ### Rust nightly build
 > use some feature on nightly or test
 > 参考
 > - https://www.rust-lang.org/tools/install
 ```
 rustup run nightly cargo build
 ```
 
 ### Rust 镜像源
```bash
export RUSTUP_DIST_SERVER=http://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT:http://mirrors.ustc.edu.cn/rust-static/rustup
```

#### 为项目配置镜像源
```toml
[registry]
index = “https://mirrors.ustc.edu.cn/crates.io-index/”
[source.crates-io]
replace-with = ‘ustc’
[source.ustc]
registry = “https://mirrors.ustc.edu.cn/crates.io-index/”

```

## 参考

- https://www.rust-lang.org/learn/get-started