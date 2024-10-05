#!/bin/bash

REPO="https://github.com/silverling/xdwlan-login"

# Detech the OS distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$NAME
    VER=$VERSION_ID
elif type lsb_release >/dev/null 2>&1; then
    OS=$(lsb_release -si)
    VER=$(lsb_release -sr)
else
    OS=$(uname -s)
    VER=$(uname -r)
fi


# Install dependencies
echo "正在安装依赖..."
if [[ $OS == "Ubuntu" ]]; then
    sudo apt-get update
    sudo apt-get install -y chromium-browser
elif [[ $OS == "Debian GNU/Linux" ]]; then
    sudo apt-get update
    sudo apt-get install -y chromium-browser
elif [[ $OS == "Arch Linux" ]]; then
    sudo pacman -Sy --noconfirm chromium
else
    echo "请手动通过包管理器安装 chromium 或基于此的浏览器"
    exit 1
fi


# Get the latest version of the release
INSTALLER_DIR=/tmp/xdwlan-login-installer
URL=$REPO/releases/latest/download/xdwlan-login-x86_64-unknown-linux-musl.tar.xz
mkdir -p $INSTALLER_DIR

function command_exists() {
    command -v "$1" >/dev/null 2>&1
}

echo "正在下载安装包..."
if command_exists wget; then
    wget -q --show-progress -O $INSTALLER_DIR/xdwlan-login.tar.xz $URL
elif command_exists curl; then
    curl --progress-bar -SL $URL -o $INSTALLER_DIR/xdwlan-login.tar.xz
else
    echo "请安装 wget 或者 curl 以下载安装包 (例如， sudo apt-get install -y wget)"
    exit 1
fi

[[ $? -ne 0 ]] && echo "下载失败" && exit 1

echo "正在安装..."
tar -xf $INSTALLER_DIR/xdwlan-login.tar.xz -C $INSTALLER_DIR
sudo cp $INSTALLER_DIR/xdwlan-login /usr/local/bin
sudo chmod +x /usr/local/bin/xdwlan-login
mkdir -p ~/.config/xdwlan-login

# Create systemd service file
echo "正在创建 systemd 服务文件..."
SERVICE_FILE="/etc/systemd/system/xdwlan-login.service"
cat > $SERVICE_FILE << EOF
[Unit]
Description=xdwlan-login service
After=network.target

[Service]
ExecStart=/usr/local/bin/xdwlan-login
Restart=on-failure
User=$(whoami)
Environment=XDG_CONFIG_HOME=/home/$(whoami)/.config

[Install]
WantedBy=multi-user.target
EOF

cat << EOF 
安装完成!

请新建文件 ~/.config/xdwlan-login/config.yaml，并填入以下内容:

    username: <学号>
    password: <密码>

然后运行 xdwlan-login --oneshot 即可登录校园网。

也可以不加 --oneshot 参数，让 xdwlan-login 以守护进程的方式运行，以实现自动登录和断网重连。
如果你想开机自动登录，可以开启 xdwlan-login 服务:

    sudo systemctl enable xdwlan-login.service

如果使用过程中遇到问题，请在 Issues 中反馈，谢谢!
项目地址: $REPO
EOF

exit 0