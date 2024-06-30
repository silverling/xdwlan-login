# Xidian WLAN Login

西电校园网登录助手，通过 Web Portal 认证方式。可以运行在后台保持网络连接。支持自动重连、开机自启。

## 工作原理

该程序通过[调用浏览器模拟用户登录行为](https://github.com/rust-headless-chrome/rust-headless-chrome)，理论上说，只要你的设备可以打开浏览器访问登录界面，就可以使用本程序来自动登录。

同时，**这种方式也要求你的电脑上有一个浏览器**（Chrome、Edge 等 Chromium 系列浏览器）。

认证原理：

- 当你的设备断网时，任何 HTTP 请求都会被网关重定向到 `http://w.xidian.edu.cn` 网站，来引导用户登录。
- 本程序会自动检测是否断网，并在被重定向后，自动填入学号与密码来登录。

这种方式相比于[逆向 Javascript 代码逻辑](https://github.com/silverling/srun-login/)，有更好的鲁棒性，也更有利于维护。

## 使用说明

### 下载使用

- 在 Release 页面下载 zip 文件并解压
- 修改 `config.yaml`，填入学号和密码
- 运行 `xdwlan-login.exe` 即可。（程序会在系统托盘后台运行）
- （可选）右键托盘图标，选择 “AutoStart”，即可开机自启

备注：

- 如果遇到问题，可以查看程序同目录下的日志文件 `log.txt` 来排查（设置环境变量 `RUST_LOG` 可以调节日志层级），并可以在 [Issue](https://github.com/silverling/xdwlan-login/issues) 区反馈。

### 编译使用

编译程序：

```bash
git clone https://github.com/silverling/xdwlan-login.git
cd xdwlan-login
cargo build --release
```

在程序同目录创建 `config.yaml` 文件，写入登录信息：

```yaml
username: 23000000000
password: xxxxxx
```

运行程序即可。（程序会在系统托盘后台运行）
