dez-commit = 提交
dez-version = 版本
dez-confirm-quit = 确定要退出吗？
dez-quit = 退出
dez-open-url-hint = 粘贴要打开的 URL。
dez-invalid-url = 无效的 URL：{ $error }
dez-code-actions = 代码操作
dez-no-code-actions = 没有可用的代码操作
dez-selection-controls = 选择控制
dez-editor-controls = 编辑器控制
dez-inline-diagnostics-unavailable = 启用常规诊断后才能使用内联诊断。
dez-inline-diagnostics = 内联诊断
dez-inlay-hints = 内嵌提示
dez-semantic-highlights = 语义高亮
dez-code-lens = 代码镜头
dez-minimap = 迷你地图
dez-diagnostics = 诊断
dez-line-numbers = 行号
dez-selection-menu = 选择菜单
dez-auto-signature-help = 自动签名帮助
dez-inline-git-blame = 内联 Git 追责信息
dez-column-git-blame = 列 Git 追责信息
dez-vim-mode = Vim 模式
dez-helix-mode = Helix 模式
dez-repl-menu = REPL 菜单
dez-repl-kernel = 内核：{ $name }（{ $language }）
dez-start-repl-for = 为 { $kernel } 启动 REPL
dez-setup-repl-for = 为 { $language } 设置 dez REPL
dez-run-selection = 运行所选内容
dez-run-line = 运行当前行
dez-view-sessions = 查看会话
dez-next-hunk = 下一个更改块
dez-previous-hunk = 上一个更改块
dez-inotify-title = 无法启动 inotify
dez-inotify-detail = inotify_init 返回 { $error }
    
    这可能是因为系统范围的 inotify 实例数量限制。故障排除说明请参阅：https://github.com/Oreoro/dez/blob/main/docs/src/linux.md
dez-windows-watcher-title = 无法启动 ReadDirectoryChangesW
dez-windows-watcher-detail = ReadDirectoryChangesW 初始化失败：{ $error }
    
    这可能发生在网络文件系统和 WSL 路径中。故障排除说明请参阅：https://github.com/Oreoro/dez/blob/main/docs/src/windows.md
dez-troubleshoot-and-quit = 故障排除并退出
dez-unsupported-gpu-title = 不支持的 GPU
dez-unsupported-gpu-detail = dez 使用 { $graphics_api } 进行渲染，需要兼容的 GPU。
    
    当前正在使用软件模拟 GPU（{ $device_name }），这会导致性能很差。
    
    故障排除说明请参阅：{ $docs_url }
    设置 ZED_ALLOW_EMULATED_GPU=1 可永久覆盖此限制。
dez-skip = 跳过
dez-preview-markdown = 预览 Markdown
dez-preview-svg = 预览 SVG
dez-preview-csv = 预览 CSV
dez-preview-open-split = 使用 { $shortcut } 在拆分视图中打开
