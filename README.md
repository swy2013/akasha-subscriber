<!-- markdownlint-disable-file MD001 MD013 MD033 -->
<h1 align = "center">虚空订阅器</h1>

这个是一个可以运行在 Cloudflare Workers 上的节点订阅器，可以生成 Base64 和 Mihomo 格式的订阅

> [!WARNING]
>
> 提出意见前请看 [计划中的功能 / 节点类型](https://github.com/Buer-Nahida/akasha-subscriber/issues/2)，确保无此类计划再提出！
>
> 目前只支持以下协议（未打钩的是未完全支持的），若有其他类型节点需要支持，请打开一个 [请求支持 Issue](https://github.com/Buer-Nahida/akasha-subscriber/issues/new?template=%E8%AF%B7%E6%B1%82%E6%94%AF%E6%8C%81%E6%96%B0%E8%8A%82%E7%82%B9%E7%B1%BB%E5%9E%8B.md) 并**将模板补全，不要只写个标题就交上来，如果你不知如何提问请请先阅读 [提问的智慧](https://github.com/ryanhanwu/How-To-Ask-Questions-The-Smart-Way/blob/main/README-zh_CN.md) 以免给开发者造成麻烦**
>
> - [ ] vless（目前只支持以下分支）
>   - [x] ws
>   - [x] tls
>   - [x] tcp
>   - [x] reality
>   - [x] xtls-rprx-vision
> - [x] hysteria2
> - [ ] ss（目前只支持以下分支）
>   - [x] shadowtls

## 免责声明

本免责声明适用于 GitHub 上的 [`akasha-subscriber`](https://github.com/Buer-Nahida/akasha-subscriber) 项目（以下简称“本项目”）。

以下文字中“作者”或“我”指代 [`Buer-Nahida`](https://github.com/Buer-Nahida)，也就是本项目的所有者、主要开发者、主要维护者、项目创立者

### 用途

本项目仅供教育、研究和安全测试目的而设计和开发。旨在为安全研究人员、学术界人士及技术爱好者提供一个探索和实践网络通信技术的工具。

### 合法性

在下载和使用本项目代码时，必须遵守使用者所适用的法律和规定。使用者有责任确保其行为符合所在地区的法律框架、规章制度及其他相关规定。

### 免责

1. 我强调本项目仅应用于合法、道德和教育目的。
2. 我绝不认可、绝不支持、绝不鼓励任何形式的非法使用。如果发现本项目被用于任何非法或不道德的活动，我将对此强烈谴责。
3. 我对任何人或组织利用本项目代码从事的任何非法活动不承担责任。使用本项目代码所产生的任何后果，均由使用者自行承担。
4. 我不对使用本项目代码可能引起的任何直接或间接损害负责。

## 许可证

本项目的源代码及编译产物均用 AGPL-3.0 协议许可，这意味着无论使用此项目搭建网站还是二次修改/分发此项目的源码，你都必须开放源代码且用 AGPL-3.0 协议许可，详细信息请参阅 [LICENSE](./LICENSE) 文件。

## 部署

截止到2025年01月29日，本项目无法在 Cloudflare Pages 上部署

CLoudflare Worker 部署方式见 [「 部署到 Cloudflare Workers 」](https://github.com/Buer-Nahida/akasha-subscriber/wiki/%E9%83%A8%E7%BD%B2%E5%88%B0-Cloudflare-Workers)
