# neat-translator

[English](README_en.md)

使用~~有道词典、柯林斯等词典~~以及Qwen、DeepSeek等AI的API的翻译工具。

> [!note] 注意
> 有道词典不提供 toC 的词典翻译服务；柯林斯词典的 API 申请涉及 Google 网站，不方便使用；牛津词典 API 价格过高。
> 故暂时使用 AI 代替词典 API。后续将考虑 mdict 解析。

基于 Rust + Slint 开发，提供跨平台支持，资源占用少，性能高，响应快，且不含任何广告。

## 功能特性

- [ ] **闪电般快速翻译**：基于Rust高性能特性实现近乎即时的翻译效果
- [ ] **跨平台兼容性**：完美支持 Windows/macOS/Linux，未来将添加对 Android / iOS / WebAssembly / HarmonyOS 4+ 的支持
  > 正在实现对 Android 的实验性支持
- [ ] **极简界面**：专注文本输入与翻译结果，无冗余元素干扰
- [ ] **多语言互译**：支持100余种语言的高精度互译
- [ ] **剪贴板集成**：自动检测复制文本实现快速翻译
- [ ] **~~低~~ 较低内存占用**：Slint 运行时内存消耗通常低于~~50MB~~ 200MB
