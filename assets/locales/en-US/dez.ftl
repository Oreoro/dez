dez-commit = Commit
dez-version = Version
dez-confirm-quit = Are you sure you want to quit?
dez-quit = Quit
dez-open-url-hint = Paste a URL to open.
dez-invalid-url = Invalid URL: { $error }
dez-code-actions = Code Actions
dez-no-code-actions = No Code Actions Available
dez-selection-controls = Selection Controls
dez-editor-controls = Editor Controls
dez-inline-diagnostics-unavailable = Inline diagnostics are not available until regular diagnostics are enabled.
dez-inline-diagnostics = Inline Diagnostics
dez-inlay-hints = Inlay Hints
dez-semantic-highlights = Semantic Highlights
dez-code-lens = Code Lens
dez-minimap = Minimap
dez-diagnostics = Diagnostics
dez-line-numbers = Line Numbers
dez-selection-menu = Selection Menu
dez-auto-signature-help = Auto Signature Help
dez-inline-git-blame = Inline Git Blame
dez-column-git-blame = Column Git Blame
dez-vim-mode = Vim Mode
dez-helix-mode = Helix Mode
dez-repl-menu = REPL Menu
dez-repl-kernel = Kernel: { $name } ({ $language })
dez-start-repl-for = Start REPL for { $kernel }
dez-setup-repl-for = Set up dez REPL for { $language }
dez-run-selection = Run Selection
dez-run-line = Run Line
dez-view-sessions = View Sessions
dez-next-hunk = Next Hunk
dez-previous-hunk = Previous Hunk
dez-inotify-title = Could not start inotify
dez-inotify-detail = inotify_init returned { $error }
    
    This may be due to system-wide limits on inotify instances. For troubleshooting, see: https://github.com/shenghsi/dez/blob/main/docs/src/linux.md
dez-windows-watcher-title = Could not start ReadDirectoryChangesW
dez-windows-watcher-detail = ReadDirectoryChangesW initialization failed: { $error }
    
    This may occur on network filesystems and WSL paths. For troubleshooting, see: https://github.com/shenghsi/dez/blob/main/docs/src/windows.md
dez-troubleshoot-and-quit = Troubleshoot and Quit
dez-unsupported-gpu-title = Unsupported GPU
dez-unsupported-gpu-detail = dez uses { $graphics_api } for rendering and requires a compatible GPU.
    
    You are using a software-emulated GPU ({ $device_name }), which will result in poor performance.
    
    For troubleshooting, see: { $docs_url }
    Set ZED_ALLOW_EMULATED_GPU=1 to override permanently.
dez-skip = Skip
dez-preview-markdown = Preview Markdown
dez-preview-svg = Preview SVG
dez-preview-csv = Preview CSV
dez-preview-open-split = { $shortcut } to open in a split
