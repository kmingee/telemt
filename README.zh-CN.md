# Telemt — 基于 Rust + Tokio 的 MTProxy

[![最新版本](https://img.shields.io/github/v/release/kmingee/telemt?color=neon)](https://github.com/kmingee/telemt/releases/latest) [![Stars](https://img.shields.io/github/stars/telemt/telemt?style=social)](https://github.com/telemt/telemt/stargazers) [![Forks](https://img.shields.io/github/forks/telemt/telemt?style=social)](https://github.com/telemt/telemt/network/members)

[🇬🇧 English](README.md) · [🇷🇺 Русский](README.ru.md) · 🇨🇳 简体中文

> [!NOTE]
>
> 自 2026 年 8 月 22 日起，Telemt 团队正在设计 MTProxy 的 Web 实现。
>
> 这一方案计划通过设备上的 WebView 建立数据通道。

<p align="center">
  <a href="https://t.me/telemtrs">
    <img src="https://github.com/user-attachments/assets/30b7e7b9-974a-4e3d-aab6-b58a85de4507" width="240"/>
  </a>
</p>

**Telemt** 是一个使用 Rust 编写的高性能、安全且功能丰富的 Telegram MTProxy 服务器。它实现了 Telegram 官方代理协议，并加入了多项面向生产环境的改进。

## 一键安装与更新

在 Linux 上可以执行：

```bash
curl -fsSL https://raw.githubusercontent.com/telemt/telemt/main/install.sh | sh
```

> 一键安装脚本面向 Linux。FreeBSD 和 Windows 用户请参阅下方的[预编译版本下载](#下载预编译版本)，或自行编译。

- [英文快速入门指南](docs/Quick_start/QUICK_START_GUIDE.en.md)
- [俄文快速入门指南](docs/Quick_start/QUICK_START_GUIDE.ru.md)

## 主要功能

Telemt 的 **TLS 前置（TLS-fronting）**实现经过大量调试和验证，目标是使代理流量的外部行为尽可能接近真实 HTTPS 服务。项目作者认为其实现已具有很高的一致性，但并不声称完全无法被识别。相关分析参见[英文 FAQ 中的检测与流量验证说明](docs/FAQ.en.md#recognizability-for-dpi-and-crawler)。

Telemt 还设计了高性能的 **Middle-End 连接池**，用于管理与 Telegram Middle-End 代理之间的连接，在常见负载场景下追求更低的开销和更好的吞吐表现。

- 支持 Telegram 官方的全部 MTProto 代理模式：
  - Classic：经典模式；
  - Secure：使用 `dd` 前缀的安全模式；
  - Fake TLS：使用 `ee` 前缀并结合 SNI 前置的伪装 TLS 模式。
- 重放攻击防护。
- 可选的流量伪装：将未识别的连接转发到真实的 HTTPS 网站。
- 可配置的 TCP 保活（keepalive）、超时时间、IPv6 和快速模式（Fast Mode）。
- 支持通过 `Ctrl+C` 优雅退出。
- 支持通过 `RUST_LOG` 配置 `trace`、`debug` 等详细日志。

## 常见问题

- [英文 FAQ](docs/FAQ.en.md)
- [俄文 FAQ](docs/FAQ.ru.md)

## 进一步了解 Telemt

- [架构文档](docs/Architecture)
- [所有配置参数](docs/Config_params)
- [自行编译 Telemt](#自行编译)
- [在 BSD 上运行](docs/Quick_start/OPENBSD_QUICK_START_GUIDE.en.md)（英文）
- [为什么使用 Rust？](#为什么使用-rust)

## 下载预编译版本

此仓库的发布工作流支持为以下平台生成构建产物；正式发布后可在 [Releases 页面](https://github.com/kmingee/telemt/releases) 下载：

| 操作系统 | 架构 | 下载文件 |
| --- | --- | --- |
| FreeBSD | x86_64 | `telemt-x86_64-freebsd.tar.gz` |
| Windows | x86_64 | `telemt-x86_64-windows.zip` |
| Linux | x86_64 / ARM64 | GNU（glibc）和 MUSL 格式的 `.tar.gz` 压缩包 |

FreeBSD 二进制文件在 FreeBSD 14.3 环境中构建；Windows 压缩包内包含原生的 `telemt.exe`。每个压缩包都附带一个对应的 `.sha256` 校验文件，下载后可核对文件完整性。

解压后，将可执行文件和自己的 `config.toml` 配置文件放在同一个目录下，然后运行：

- **FreeBSD / Linux**：`./telemt config.toml`
- **Windows**（PowerShell）：`.\telemt.exe config.toml`

Windows 版本以前台进程方式运行，不支持 Unix 专用的守护进程控制和服务管理命令。

## 自行编译

需要预先安装 Rust 工具链及 Cargo，然后执行：

```bash
# 克隆本仓库
git clone https://github.com/kmingee/telemt.git
# 进入项目目录
cd telemt
# 编译 Release 版本
cargo build --release

# 当前 Release 配置使用 lto = "fat" 来获得更充分的优化（参见 Cargo.toml）。
# 内存较小（约 1 GB）的构建环境可以考虑将 LTO 改为 "thin"。

# 以下安装步骤适用于类 Unix 系统，需要相应权限
sudo install -m 0755 ./target/release/telemt /usr/local/bin/telemt
# 启动服务
telemt config.toml
```

Windows 自行编译后，可执行文件通常位于 `target\release\telemt.exe`。

## 为什么使用 Rust？

- 适合长时间运行的服务，具有良好的可靠性和幂等性。
- 使用 RAII 实现确定性的资源管理。
- 无需垃圾回收器。
- 内存安全机制有助于缩小攻击面。
- 基于 Tokio 构建异步网络架构。

## 支持 Telemt

Telemt 是免费、开源的软件，由开发者利用业余时间维护。如果它对你有帮助，可以考虑赞助项目的持续开发。

支持 BTC、ETH、USDT 等 350 多种加密货币：

<p align="center">
  <a href="https://nowpayments.io/donation?api_key=2bf1afd2-abc2-49f9-a012-f1e715b37223" target="_blank" rel="noreferrer noopener">
    <img src="https://nowpayments.io/images/embeds/donation-button-white.svg" alt="加密货币赞助按钮（NOWPayments）" height="80">
  </a>
</p>

也可以直接使用 Monero（XMR）赞助：

```text
8Bk4tZEYPQWSypeD2hrUXG2rKbAKF16GqEN942ZdAP5cFdSqW6h4DwkP5cJMAdszzuPeHeHZPTyjWWFwzeFdjuci3ktfMoB
```

所有赞助均用于项目基础设施、开发和研究。

![Telemt 架构示意图](docs/assets/telemt.png)
