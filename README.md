# Adobe 联网屏蔽工具

一个简单的工具，用于屏蔽 Adobe 联网连接。

## 功能特点

- 支持多个信息来源选择
- 一键更新 hosts 文件
- 显示当前屏蔽状态
- 显示信息源更新日期

## 使用方法

1. 选择信息来源
2. 点击"更新 Hosts"按钮
3. 根据提示授权管理员权限

## 开发说明

### 技术栈

- Vue 3
- TypeScript
- Tauri
- Vue I18n

### 开发命令

```bash
# 安装依赖
yarn install

# 开发模式
yarn tauri dev

# 构建应用
yarn tauri build
```

### 信息来源

- 原地址: https://a.dove.isdumb.one/list.txt
- Fastly 镜像: https://fastly.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt
- Gcore 镜像: https://gcore.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt
- Quantil 镜像: https://quantil.jsdelivr.net/gh/ignaciocastro/a-dove-is-dumb@latest/list.txt
- Ghproxy: https://gh-proxy.com/https://raw.githubusercontent.com/ignaciocastro/a-dove-is-dumb/main/list.txt

## 注意事项

- 更新 hosts 文件需要管理员权限
- 请确保选择可靠的信息来源
- 使用前请备份您的 hosts 文件

## License

MIT