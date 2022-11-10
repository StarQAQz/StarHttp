# StarHttp

## 介绍

StarHttp是采用Rust开发的一个小型静态服务器。项目无外部依赖项，使用线程池并发模式进行。

项目可用于简单学习参考。感兴趣的可以阅读下源码，并提出改进意见。

目前项目已用于部署个人博客：https://www.bluestar.zone/

## 软件架构

项目源码文件目录介绍

- STAR-HTTP
  - src
    - config.rs		配置读取功能
    - error.rs		自定义异常类型
    - hex.rs 		url中文字符utf-8编码转义
    - http.rs 		服务器核心服务
    - log.rs 		日志功能
    - main.rs 		程序入口
    - thread.rs 	线程并发功能
    - time.rs 		时间工具（用于日志显示时间）
  - config.toml 	配置
  - static 默认静态目录
    - index.html 默认首页页面
    - 404.html 可配置404页面
    - 500.html 可配置500页面

## 更新说明

**2022-11-10**

修复content-type类型不正确导致页面加载失败。

实现自定义首页、404、500界面配置。修改配置文件位置以及定位方式。部署在线文档。

接下来会提供windows及linux打包程序的下载，并且后续会继续优化不满意的地方，增加新功能。

（当前不满意的地方有点多，不一一列举！QAQ）

**2022-11-03**

现已实现硬编码配置分离，实现配置文件读取。修复URL的UTF-8编码导致请求页面失败。

后续会继续优化，并将项目部署一个小demo，添加自定义404，500界面。

**2022-10-26**

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

config.toml配置文件中默认配置了线程池大小、静态文件服务目录以及时间时区，可根据需要进行更改

```toml
#静态目录(尽量使用绝对路径)
static_resource_path = "./static"
#配置默认首页（可修改，默认index.html）（相对静态目录的路径）
index_page_path = "index首页.html"
#配置自定义404页面（可选）（相对静态目录的路径）
page404_path = "404.html"
#配置自定义500页面（可选）（相对静态目录的路径）
page500_path = "500.html"
#线程池大小
thread_pool_size = 6
#时区
timezone = 8
#IP
ip = "127.0.0.1"
#Port
port = 80
```

