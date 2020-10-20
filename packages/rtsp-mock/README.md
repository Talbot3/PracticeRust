## RTSP Camera Mock
To run, `cargo run  100`.

Server will create 100 rtsp available at 
`rtsp://0.0.0.0:8554/1`
...
`rtsp://0.0.0.0:8554/100`


## 交插编译

```
## 服务器环境
rustup target add x86_64-unknown-linux-gnu
## 默认开发环境
cargo build --target="x86_64-apple-darwin"

## 在开发环境交插编译
cargo build --target="x86_64-unknown-linux-gnu"

```

## Thanks

- valmirjunior0088/rtsp-camera-mock.git