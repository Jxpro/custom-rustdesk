# Rustdesk 自定义 ID 工具

## 简介

本项目旨在为 `Rustdesk `提供自定义 ID 生成功能，方便用户记忆和管理设备。主要功能为模拟官方加密算法，将用户输入的自定义 ID 加密并输出加密后的 ID。

## 功能

*   生成加密后的自定义 ID
*   使用 UUID 作为加密密钥

## 使用方法

1.  将代码克隆到本地
2.  运行 `cargo run -- --id $id --uuid $uuid` 命令
3.  程序会输出加密后的 ID

示例：

```shell
cargo run -- --id 123456 --uuid 12345678-1234-1234-1234-123456789012
```

## 加密流程

该程序使用`sodiumoxide `库中的`crypto::secretbox` 模块进行对称加密。加密密钥来自提供的 UUID 字符串。

1.   将自定义 ID 字符串转换为字节数组。
2.   将 UUID 字符串转换为字节数组，并调整其大小以匹配密钥长度要求。
3.   使用`sodiumoxide::crypto::secretbox`模块创建密钥和`nonce`。
4.   根据`encrypt`参数选择加密或解密操作。
5.   使用`secretbox::seal`或`secretbox::open`函数进行加密或解密操作。
6.   将加密后的字节数组转换为`base64`编码字符串，并输出到控制台。

## 贡献

欢迎您对该项目进行贡献！您可以通过以下方式参与：

-   提交代码补丁或问题报告
-   提供反馈和建议
-   帮助推广项目

## 联系方式

如果有任何问题，欢迎到 [github issue](https://github.com/Jxpro/custom-rustdesk/issues) 进行讨论，或发送电子邮件到 [jxpro@qq.com](mailto:jxpro@qq.com) 来联系我