# 简介

[Demo](https://wmafire.github.io)

一个静态博客的轮子（自娱自乐，仅供参考，谨慎使用）。

类似于 hexo/hugo。目前功能十分有限。

# 编译

环境要求

* 安装 Rust
* 安装 Node

## 编译主程序

``` bash
$ cd ims
$ cargo build --release
```
## 编译模版

目前有两套模版:

### vue-template

基于 Vuejs 的一个单页应用。对SEO十分不友好，所以不打算更新了。
``` bash
$ cd ims/vue-template
$ npm install
$ npm run build
```
目标文件在 dist 目录。

### template

``` bash
$ cd ims/template
$ npm install
$ npm run debug
```

目标文件在 dist 目录。

# 使用

主程序查看 Help。

``` bash
$ ims --help
```

Tips：运行`ims build` 之前需要将编译好的模版文件放在 `$(网站目录)/theme/default/` 下,程序会根据模版文件生成静态页面。

由于模版使用的handlebars语法，功能十分有限。如果需要扩展模版功能需要修改主程序，添加 handlebars helper。

目前有的helper：

| helper    | 简介                             |
| ---------- | -------------------------------- |
| json       | 输出Json对象                     |
| count      | 输出列表个数                     |
| hash       | 生成文本的hashcode               |
| markdown   | markdown文本转HTML               |
| dateformat | 格式化日期                       |
| mdtoc      | 提取markdown标题（用于生成目录） |
| file       | 将模版输出到文件                 |
| pagination | 分页                             |

# 后续目标


+ [ ] 监视文件改动自动生成页面
+ [ ] 集成 Git Webhooks
+ [ ] 优化模版

# 简介

[Demo](http://obaiyan.com)

一个静态博客的轮子（自娱自乐，仅供参考，谨慎使用）。

类似于 hexo/hugo。目前功能十分有限。

# 编译

环境要求

* 安装 Rust
* 安装 Node

## 编译主程序

``` bash
$ cd ims
$ cargo build --release
```
## 编译模版

目前有两套模版:

### vue-template

基于 Vuejs 的一个单页应用。对SEO十分不友好，所以不打算更新了。
``` bash
$ cd ims/vue-template
$ npm install
$ npm run build
```
目标文件在 dist 目录。

### template

``` bash
$ cd ims/template
$ npm install
$ npm run debug
```

目标文件在 dist 目录。

# 使用

主程序查看 Help。

``` bash
$ ims --help
```

Tips：运行`ims build` 之前需要将编译好的模版文件放在 `$(网站目录)/theme/default/` 下,程序会根据模版文件生成静态页面。

由于模版使用的handlebars语法，功能十分有限。如果需要扩展模版功能需要修改主程序，添加 handlebars helper。

目前有的helper：

| helper    | 简介                             |
| ---------- | -------------------------------- |
| json       | 输出Json对象                     |
| count      | 输出列表个数                     |
| hash       | 生成文本的hashcode               |
| markdown   | markdown文本转HTML               |
| dateformat | 格式化日期                       |
| mdtoc      | 提取markdown标题（用于生成目录） |
| file       | 将模版输出到文件                 |
| pagination | 分页                             |

# 后续目标


+ [ ] 监视文件改动自动生成页面
+ [ ] 集成 Git Webhooks
+ [ ] 优化模版
