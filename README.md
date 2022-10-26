# StarHttp

## 介绍

StarHttp是采用Rust开发的一个小型静态服务器Demo。项目无外部依赖项，使用线程池并发模式进行。

Demo可用于简单学习参考。感兴趣的可以阅读下源码，并提出改进意见。

## 软件架构

项目源码文件目录介绍

- src
  - config.rs 配置项（目前没有通过读取配置文件的方式进行配置，后续考虑加入）
  - error.rs 自定义异常类型
  - http.rs 服务器核心服务
  - log.rs 日志功能
  - main.rs 程序入口
  - thread.rs 线程并发功能
  - time.rs 时间工具（用于日志显示时间）

## 功能说明

目前只实现了HTTP1.1 GET方式请求（对于静态文件服务来说相对足够），线程池处理，日志记录。

后续对里面不满意的地方还会进一步修改。并且将硬编码配置分离，实现读取配置文件。

## 使用说明

### 构建

1，确保本地安装好Rust，克隆文件到本地，执行命令

```bash
git clone https://gitee.com/StarQAQz/star-http.git
cd star-http
rust build --release
```

2，等待构建完成，执行运行文件即可

```bash
#windows
./target/release/star-http.exe
#linux
./target/release/star-http
```

### 配置

配置文件中默认配置了线程池大小、静态文件服务目录以及时间时区，可根据需要进行更改

```rust
//config.rs
//静态目录
pub const STATIC_RESOURCE_PATH: &str = "./static";
//线程池大小
pub const POOL_SIZE: usize = 6;
//时区，东八区
pub const TIMEZONE: i32 = 8;
```

