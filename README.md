<!-- markdownlint-disable-file MD001 MD013 MD033 -->
<h1 align = "center">虚空订阅器</h1>

这个是一个可以运行在 Cloudflare Workers 上的节点订阅器，可以生成 Base64 和 Mihomo 格式的订阅

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

<details>
<summary><h3>「 部署到Cloudflare Workers 」</h3></summary>

#### Fork 该项目

1. 在 `Code` 界面点击 <kbd>Star</kbd> 再点击 <kbd>Fork</kbd>，到达 `Create a new fork` 界面
2. 点击绿色的 <kbd>Create fork</kbd>，等待几秒钟，自动跳转到你 Fork 的仓库

#### 设置自定义域名 / 路由

点击 <kbd>wrangler.toml</kbd>，再点击铅笔图标的按钮，删掉下面这一行

```toml
routes = [{ pattern = "www.nahida.im/*", zone_name = "nahida.im" }]
```

然后在 `wrangler.toml` 文件中添加你的自定义域名（确保此域名已添加到 Cloudflare），例如：

```toml
routes = [{ pattern = "example.example.com", custom_domain = true }]
```

若你想添加路由，请用：

```toml
routes = [{ pattern = "example.example.com/*", zone_name = "example.com" }]
```

#### 设置要部署到的 Cloudflare Worker 的名称

1. 点击 <kbd>Settings</kbd> > <kbd>Secrets and variables</kbd> > <kbd>Actions</kbd>
2. 点击 `New repository secret`，`Name` 为 `NAME`，内容填你想部署到的 Cloudflare Worker 名称（确保无重名 Worker / Page 即可）

#### 设置 Cloudflare API 令牌

1. 新开一个标签页，登录 [Cloudflare 仪表板](https://dash.cloudflare.com)：
2. 点击右上角的用户图标，点击 <kbd>我的个人资料</kbd>
3. 在左侧菜单中，点击 `API 令牌`
4. 点击 <kbd>Create Token</kbd> 按钮
5. 选择 `API 令牌模板` 下的 `编辑 Cloudflare Workers` 模板
6. 账户资源选择 `所有账户`，区域资源选择 `所有区域`
7. 点击 <kbd>继续以显示摘要</kbd> 然后点击 <kbd>创建令牌</kbd>
8. 复制生成的令牌（重要：这是唯一一次显示完整令牌的机会）
9. 回到 Github 标签页，用同样的方式添加名为 `CLOUDFLARE_API_TOKEN` 的机密变量，内容为你复制的令牌

#### 配置虚空订阅器

虚空订阅器读取 YAML 格式的配置文件，你可以在 [虚空订阅器 Wiki](https://github.com/Buer-Nahida/akasha-subscriber/wiki) 中找到示例配置文件和所有配置项的解释，根据自己的需求编写配置文件

#### 开启 Github Actions

1. 转到 <kbd>Actions</kbd>，点击绿色的 <kbd>I understand my workflows, go ahead and enable them</kbd> 按钮
2. 点击侧边栏 `All workflows` 下的 <kbd>Deploy</kbd>，点击黄色条旁边的 <kbd>Enable workflow</kbd>

#### 部署到 Cloudflaer Worker

点击 <kbd>Run workflow</kbd>，再点击绿色的 <kbd>Run workflow</kbd>，等待一会，就部署成功了

</details>
```
