<h1>
  Xidian WLAN Login | 西电校园网登录助手
  <img src="resources/icons/avocado.png" alt="Logo" height="48" style="vertical-align: middle; margin-left: 8px;" />
</h1>

通过 Web Portal 认证方式。可以运行在后台保持网络连接。支持自动重连、开机自启。

## 使用说明

### Windows 系统

- 在 [Release](https://github.com/silverling/xdwlan-login/releases) 页面下载 zip 文件并解压
- 修改 `config.yaml`，填入学号和密码
- 运行 `xdwlan-login.exe` 即可。（程序会在系统托盘后台运行，图标为牛油果）
- （可选）右键托盘图标，选择 “开机自启”，即可开机自启

### Linux 系统

<details open>
<summary>方法一 ：一键脚本安装</summary>

```bash
curl -sSL https://github.com/silverling/xdwlan-login/raw/refs/heads/main/scripts/install.sh | bash
```

PS：在安装脚本中，该程序将被自动安装到 `/opt/xdwlan-login` 目录下，并软链接到 `/usr/local/bin/xdwlan-login`。

PS：如果你的网络无法连接到 GitHub 导致下载失败，也可以通过其他方式手动下载 [`install.sh`](https://github.com/silverling/xdwlan-login/raw/refs/heads/main/scripts/install.sh) 和 [`xdwlan-login-x86_64-unknown-linux-gnu.tar.xz`](https://github.com/silverling/xdwlan-login/releases/latest/download/xdwlan-login-x86_64-unknown-linux-gnu.tar.xz) 到同一目录，然后在该目录下执行

```bash
bash ./install.sh xdwlan-login-x86_64-unknown-linux-gnu.tar.xz
```

</details>

<details>
<summary>方法二：手动安装</summary>
下载并解压
  
```bash
curl -sSL https://github.com/silverling/xdwlan-login/releases/latest/download/xdwlan-login-x86_64-unknown-linux-gnu.tar.xz -O xdwlan-login.tar.xz
tar -xf xdwlan-login.tar.xz
```

在程序同目录下创建配置文件 `config.yaml`，填入以下内容：

```yaml
username: "学号"
password: "密码"
```

</details>

Linux 版程序有三种运行模式：

- `xdwlan-login --oneshot`：登录校园网，然后退出。
- `xdwlan-login`：登录校园网，然后持续运行，定时监测网络状态，自动断网重连。
- `sudo systemctl enable --now xdwlan-login.service`：开机自启，然后持续运行，定时监测网络状态，自动断网重连。

## 问题排查

如果遇到问题，可以查看程序所在目录下的日志文件 `log.txt` 来排查（设置环境变量 `XDWLAN_LOGIN_LOG_LEVEL='trace'` 可以调节日志层级， 或在 `config.yaml` 中设置），也可以在 [Issue](https://github.com/silverling/xdwlan-login/issues) 区反馈。

## 工作原理

该程序通过[模拟浏览器用户登录行为](https://github.com/silverling/xdwlan-login/blob/main/packages/xdwlan-login/src/lib/login.ts)，理论上说，只要你的设备可以访问登录界面，就可以使用本程序来自动登录。

认证原理：

- 当你的设备断网时，任何 HTTP 请求（非 HTTPS）都会被网关重定向到 `http://w.xidian.edu.cn` 网站，来引导用户登录。
- 本程序会自动检测是否断网，并在被重定向后，自动填入学号与密码来登录。

这种方式相比于[逆向 Javascript 代码逻辑](https://github.com/silverling/srun-login/)，有更好的鲁棒性，也更有利于维护。

## 源码编译

如果你想要测试该程序，或者其他原因，可以 Clone 本仓库并自行编译使用。

首先，在项目根目录创建 `config.yaml` 文件（可参考 [`config.examepl.yaml`](https://github.com/silverling/xdwlan-login/blob/main/config.examepl.yaml) )，写入登录信息：

```yaml
username: "23000000000"
password: "xxxxxxxxxxx"
```

然后编译程序：

```bash
# Windows
bun install
bun run -F xdwlan-login build:windows
cargo build

# Linux
bun install
bun run -F xdwlan-login build:linux
```
