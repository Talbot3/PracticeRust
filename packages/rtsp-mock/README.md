## RTSP Camera Mock
To run, `cargo run  100`.

Server will create 100 rtsp available at 
`rtsp://0.0.0.0:8554/1`
...
`rtsp://0.0.0.0:8554/100`


## 环境lib安装
### linux

```bash
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev

apt-get install libgstreamer-plugins-bad1.0-dev
```

### macos

```bash
brew install gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-libav;
# [ISSUE `Package 'libffi', required by 'gobject-2.0', not found`](https://github.com/otrv4/pidgin-otrng/issues/104)
export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig";
```

## 交插编译

```
## 服务器环境
rustup target add x86_64-unknown-linux-gnu
## 默认开发环境
cargo build --target="x86_64-apple-darwin"

## 在开发环境交插编译
cargo build --target="x86_64-unknown-linux-gnu"

```

## project page

https://crates.io/crates/rtsp-mock

## Thanks

- valmirjunior0088/rtsp-camera-mock.git

## Refers

- https://gstreamer.freedesktop.org/documentation/tutorials/basic/concepts.html?gi-language=c