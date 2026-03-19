# PDF阅读器

基于Tauri框架开发的功能完整的PDF阅读器，支持数字签名验证。

## 技术栈

- **前端**: Vue 2.7 (选项式API) + JavaScript
- **后端**: Rust + Tauri 2.x
- **PDF渲染**: pdf.js
- **构建工具**: Vite

## 功能特性

### 界面布局
- 标题栏：显示应用名称和当前文件名
- 菜单栏：文件、编辑、视图、导航、工具、帮助菜单
- 工具栏：常用功能快捷按钮
- 导航面板：缩略图、书签、签名标签页
- 主工作区：PDF内容显示区
- 状态栏：页面信息、缩放比例、签名状态

### PDF阅读功能
- 打开和读取PDF文件
- 页面导航（首页/上一页/下一页/末页）
- 缩放功能（放大/缩小/实际大小/适合宽度/适合页面）
- 页面旋转
- 单页和连续滚动视图模式

### 签名验证功能
- 签名状态指示灯
- 状态栏签名状态显示
- 导航面板签名列表
- 签名验证API框架

## 项目结构

```
pdf-reader/
├── src/                          # 前端源代码
│   ├── components/               # Vue组件
│   │   ├── TitleBar.vue         # 标题栏
│   │   ├── MenuBar.vue          # 菜单栏
│   │   ├── ToolBar.vue          # 工具栏
│   │   ├── NavigationPanel.vue  # 导航面板
│   │   ├── PDFViewer.vue        # PDF查看器
│   │   └── StatusBar.vue        # 状态栏
│   ├── styles/
│   │   └── main.css             # 全局样式
│   ├── App.vue                  # 主应用组件
│   └── main.js                  # 应用入口
├── src-tauri/                    # Tauri/Rust后端
│   ├── src/
│   │   ├── lib.rs               # Rust库（包含命令定义）
│   │   └── main.rs              # Rust主程序
│   ├── capabilities/             # Tauri权限配置
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml               # Rust依赖配置
│   └── tauri.conf.json          # Tauri配置
├── dist/                         # 前端构建输出
├── index.html                    # HTML入口
├── package.json                  # 项目依赖
└── vite.config.js                # Vite配置
```

## 使用说明

### 开发模式

1. 安装依赖：
```bash
npm install
```

2. 运行前端开发服务器：
```bash
npm run dev
```

3. 运行Tauri应用（需要Rust环境）：
```bash
npm run tauri dev
```

### 构建前端

```bash
npm run build
```

前端构建输出在 `dist/` 目录中。

### 打包应用

**注意**: 完整的Tauri应用打包需要解决图标配置问题。当前项目结构已经准备好，可以在此基础上继续完善。

## 当前状态

✅ **前端功能完整**
- 所有Vue组件已实现
- PDF渲染功能已集成
- 界面布局完整
- 浏览器模式下可正常运行（使用浏览器文件选择API）

✅ **后端框架完整**
- Tauri 2.x项目结构已创建
- 签名验证API框架已实现
- 文件I/O功能已配置

⚠️ **需要完善的部分**
- Tauri应用图标配置需要进一步调试
- 完整的数字签名验证逻辑需要实现
- 更多PDF交互功能可以扩展

## 浏览器模式

应用支持在纯浏览器模式下运行，不依赖Tauri：
1. 运行 `npm run dev`
2. 在浏览器中打开 http://localhost:5173
3. 使用"打开文件"按钮选择PDF文件

## 后续开发建议

1. **完善Tauri集成**: 解决图标和窗口配置问题
2. **实现签名验证**: 在Rust后端添加真实的PDF签名验证逻辑
3. **添加更多功能**: 书签管理、文本搜索、注释等
4. **优化性能**: PDF渲染和大文件处理优化
5. **测试**: 添加单元测试和集成测试

## 许可证

MIT
