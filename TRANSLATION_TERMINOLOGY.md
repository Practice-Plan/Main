# Translation Terminology Reference

## Overview
This document provides the standardized terminology mapping used during the English localization of the `main` folder. All translations follow software industry conventions and maintain consistency across the entire codebase.

## Core Terminology Mapping

### Framework & Architecture
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 框架 | Framework | Base framework, core architecture |
| 基础框架 | Base Framework | Official product name |
| 中间件 | Middleware | Middleware inspection layer |
| 监测器 | Monitor | Application monitoring module |
| 接口 | Interface | FFI interface / API |
| 模块 | Module | Code module |
| 层 | Layer | Architecture layer |

### Monitoring & Metrics
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 监测 | Monitoring | Application monitoring |
| 应用监测 | Application monitoring | Monitor module functionality |
| 启动应用监测 | Start application monitoring | Begin monitoring an app |
| 停止应用监测 | Stop application monitoring | Stop monitoring an app |
| 性能指标 | Performance metric | Performance tracking data |
| 指标 | Metric | Measurement metric |
| 采样 | Sampling | Data sampling |
| 采样数 | Samples | Count of data samples |
| 运行时间 | Uptime | Application running time |

### Middleware & Inspection
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 中间检查层 | Middleware inspection layer | Core middleware module |
| 调用检查 | Call inspection | Middleware check on calls |
| 检查器 | Checker | Individual check component |
| 检查器特征 | Checker trait | Rust trait for checkers |
| 检查器名称 | Checker name | Identifier of a checker |
| 执行检查 | Execute check | Run inspection logic |
| 检查器优先级 | Checker priority | Execution order of checkers |
| 白名单 | Whitelist | Allowed callers list |
| 速率限制 | Rate limiting | Call rate throttling |
| 速率限制器 | Rate limiter | Rate limiting component |
| 参数验证 | Parameter validation | Input parameter checking |
| 参数验证器 | Parameter validator | Parameter validation component |
| 检查通过 | Passed / Inspection passed | Check result |
| 检查失败 | Rejected / Inspection failed | Check result |
| 权限验证 | Permission verification | Access control check |
| 请求过滤 | Request filtering | Request filtering |

### Database & Data
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 数据库 | Database | SQLite database |
| 数据库实例 | Database instance | Database object |
| 数据库路径 | Database path | File path of database |
| 命令映射 | Command mapping | Command-to-action mapping |
| 默认命令映射 | Default command mappings | Pre-configured mappings |
| 获取命令映射 | Get command mapping | Retrieve mapping |
| 更新命令映射 | Update command mapping | Modify mapping |
| 删除命令映射 | Remove command mapping | Delete mapping |
| 列出所有命令映射 | List all command mappings | Enumerate mappings |
| 获取命令详情 | Get command details | Retrieve mapping details |

### DLL & Loading
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 动态加载 | Dynamic loading | DLL dynamic loading |
| DLL 加载器 | DLL loader | DLL loading component |
| 加载 DLL | Load DLL | Load a dynamic library |
| 卸载 DLL | Unload DLL | Unload a dynamic library |
| 已加载的 DLL 列表 | List of loaded DLLs | Enumerate loaded libraries |
| DLL 搜索路径 | DLL search path | Paths to search for DLLs |
| 尝试直接加载 | Try direct loading | Attempt immediate load |
| 尝试调用释放函数 | Try to call free function | Attempt memory deallocation |

### FFI & C Interface
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 释放字符串 | Free string | Deallocate string memory |
| 要释放的字符串指针 | String pointer to free | Pointer parameter description |
| 健康检查 | Health check | System health verification |
| 框架健康 | Framework is healthy | Health check result |
| 框架异常 | Framework anomaly | Health check result |
| 注册应用程序 | Register application | FFI function description |
| 应用唯一标识符 | Unique application identifier | app_id parameter |
| 应用名称 | Application name | app_name parameter |
| 版本号 | Version number | version parameter |
| 调用者标识 | Caller identifier | caller_id parameter |
| 目标接口名称 | Target interface name | interface_name parameter |
| 参数 JSON 字符串 | Parameter JSON string | params_json parameter |
| 指标名称 | Metric name | metric_name parameter |
| 指标值 | Metric value | value parameter |
| 错误码 | Error code | error_code parameter |
| 错误消息 | Error message | message parameter |
| 版本字符串 | Version string | Return value description |
| 静态分配，不要释放 | Statically allocated, do not free | Memory ownership note |
| 释放字符串资源 | Free string resource | Memory deallocation |
| 字符串指针 | String pointer | Pointer parameter |
| 初始化框架 | Initialize framework | framework_init function |
| 关闭框架，释放所有资源 | Shutdown framework, release all resources | framework_shutdown function |
| 启动应用监测 | Start application monitoring | framework_start_app function |
| 停止应用监测 | Stop application monitoring | framework_stop_app function |
| 执行调用检查 | Execute call inspection | framework_check_call function |
| 记录性能指标 | Record performance metric | framework_record_metric function |
| 记录错误 | Record error | framework_record_error function |
| 获取框架版本 | Get framework version | framework_version function |
| 健康检查 | Health check | framework_health_check function |
| 错误码定义 | Error code definitions | Macro definitions section |

### CLI & User Interface
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 显示版本信息 | Display version information | Help text / function doc |
| 显示帮助信息 | Display help information | Help text / function doc |
| 显示系统信息 | Display system information | Help text / function doc |
| 显示框架状态 | Display framework status | Help text / function doc |
| 添加命令映射 | Add command mapping | Help text |
| 删除命令映射 | Remove command mapping | Help text |
| 列出所有命令映射 | List all command mappings | Help text |
| 获取命令详情 | Get command details | Help text |
| 加载 DLL | Load DLL | Help text |
| 卸载 DLL | Unload DLL | Help text |
| 列出已加载的 DLL | List loaded DLLs | Help text |
| 命令管理 | Command management | Function doc comment |
| 数据库状态 | Database status | Status output section |
| DLL 状态 | DLL status | Status output section |
| 处理 -version 标志 | Handle -version flag | Code comment |

### Testing & Benchmarking
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 集成测试 | Integration tests | Test file header |
| 性能基准测试 | Performance benchmark tests | Benchmark file header |
| 基础使用示例 | Basic usage example | Example file header |
| 演示如何在 Rust 中使用框架 | Demonstrates how to use the framework in Rust | Example file doc |
| 创建监测器 | Create monitor | Test code comment |
| 注册应用 | Register application | Test code comment |
| 启动应用 | Start application | Test code comment |
| 验证状态 | Verify state | Test code comment |
| 记录指标 | Record metrics | Test code comment |
| 停止应用 | Stop application | Test code comment |
| 配置白名单 | Configure whitelist | Test code comment |
| 配置速率限制 | Configure rate limiting | Test code comment |
| 配置参数验证 | Configure parameter validation | Test code comment |
| 并发注册应用 | Concurrent application registration | Test code comment |
| 并发记录指标 | Concurrent metric recording | Test code comment |
| 启动不存在的应用 | Start non-existent application | Test code comment |
| 重复注册 | Duplicate registration | Test code comment |
| 合法调用 | Valid call | Example test case label |
| 未知调用者 | Unknown caller | Example test case label |
| 缺少必要参数 | Missing required parameters | Example test case label |
| 完整参数调用 | Complete parameter call | Example test case label |
| 调用检查测试 | Call Inspection Tests | Example section header |
| 性能指标 | Performance Metrics | Example section header |
| 应用状态 | Application Status | Example section header |
| 示例完成 | Example Complete | Example ending message |

### Configuration & Dependencies
| Chinese (Original) | English (Translated) | Context / Notes |
|-------------------|---------------------|-----------------|
| 日志框架 | Logging framework | Cargo.toml dependency comment |
| 跨平台支持 | Cross-platform support | Cargo.toml dependency comment |
| 错误处理 | Error handling | Cargo.toml dependency comment |
| 序列化支持 | Serialization support | Cargo.toml dependency comment |
| 时间处理 | Time handling | Cargo.toml dependency comment |
| 并发支持 | Concurrency support | Cargo.toml dependency comment |
| FFI 支持 | FFI support | Cargo.toml dependency comment |
| DLL 动态加载 | DLL dynamic loading | Cargo.toml dependency comment |
| 命令行解析 | Command-line parsing | Cargo.toml dependency comment |
| 数据库 | Database | Cargo.toml dependency comment |
| 测试工具 | Testing utilities | Cargo.toml dev-dependency comment |

## Translation Principles

1. **Accuracy**: All translations accurately convey the original meaning using standard software engineering terminology.
2. **Consistency**: Term mapping is consistent across all source files, headers, tests, and examples.
3. **Natural English**: Translations follow native English speaker conventions and read naturally.
4. **Rust Naming Conventions**: Identifier names follow Rust snake_case/camel_case conventions.
5. **Code Structure Preservation**: No logic changes, only text and comment translations.
