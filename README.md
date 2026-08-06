# 拼豆像素画板 Pixel-Board

单文件网页应用：像素画绘制 + 拼豆色卡匹配 + 导出。浏览器直接打开 `index.html` 即可使用，无需服务器。

## 功能特性

- **像素绘制**：左键绘制 / 右键擦除，支持导入图片，Ctrl+滚轮缩放，Ctrl+Z / Ctrl+Y 撤销重做
- **色卡匹配**：马卡龙 / COCO 内置色卡，支持导入自定义色卡（.txt）；匹配后颜色自动吸附到最近色号
- **标注编号**：匹配后每个格子带色号编号，方便对照购买拼豆
- **色号清单导出**：导出图片时可附带色号清单（色块 + 编号 + 用量），可单独下载或拼接到主图下方
- **框选导出局部**：两次点击确定对角顶点，框选区域导出
- **深色 / 浅色模式**：右上角一键切换
- **像素空间自定义**：最大 500×500

## 安装包下载

> 每个版本在 [Releases](https://github.com/Flashzzz42/pixel-board/releases) 发布，按系统 / 芯片架构分类。

| 系统 | 架构 | 安装包 | 说明 |
| --- | --- | --- | --- |
| Windows | x64 | `pixel-board-Setup-1.0.0-win-x64.exe` | NSIS 安装程序 |
| macOS | Apple Silicon（M1–M5） | `pixel-board-1.0.0-mac-arm64.dmg` | 原生运行，无需转译 |
| macOS | Intel | `pixel-board-1.0.0-mac-x64.dmg` | x64 |

> macOS 安装包未签名（无 Apple 开发者证书），首次打开会提示「无法验证开发者」：右键 App → **打开**，或 系统设置 → 隐私与安全性 → **仍要打开**。

## 本地使用

直接双击打开 `index.html`，所有数据保存在浏览器本地（localStorage），无需联网。

## 技术说明

- 核心是一个自包含的 `index.html`（HTML + CSS + JS 全部内联），无任何外部依赖。
- 桌面版由 Electron 封装（Windows / macOS 通用），打包工程与工作流见私有模板仓库 `electron-packaging-template`。
- 色卡数据：`coco-colors.txt` / `mard-colors.txt`（可经应用内「导入色卡」加载）。
