#!/bin/bash

set -eu
printf '\n'

REPO="https://github.com/silverling/xdwlan-login"
DOWNLOAD_URL="$REPO/releases/latest/download/xdwlan-login-x86_64-unknown-linux-gnu.tar.xz"
INSTALLER_DIR=/tmp/xdwlan-login-installer
ARGC="$#"
ARGS=("$@")

# Helper functions for logging and utility. Copied from https://starship.rs/install.sh
BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
UNDERLINE="$(tput smul 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
BLUE="$(tput setaf 4 2>/dev/null || printf '')"
MAGENTA="$(tput setaf 5 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

distro=$(cat /etc/os-release | grep '^ID=' | cut -d'=' -f2)

info() {
    printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
    printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
    printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

completed() {
    printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}

has() {
    command -v "$1" 1>/dev/null 2>&1
}

download() {
    install_from_local=0

    # Check whether installing from local file
    if [ $ARGC -gt 1 ]; then
        error "错误：最多只能指定一个文件路径"
        exit 1
    elif [ $ARGC -eq 1 ]; then
        # Get tarball from explcitly provided path
        local_tarball="${ARGS[0]}"
        if [ -f "$local_tarball" ]; then
            install_from_local=1
        else
            error "错误：指定的文件 $local_tarball 不存在"
            exit 1
        fi
    else
        # Probe tarball in current working directory
        local_tarball="$(basename $DOWNLOAD_URL)"
        if [ -f "$local_tarball" ]; then
            install_from_local=1
        fi
    fi

    mkdir -p $INSTALLER_DIR
    if [ $install_from_local -eq 1 ]; then
        info "使用本地安装包，跳过下载..."
        cp "$local_tarball" "$INSTALLER_DIR/xdwlan-login.tar.xz" || {
            error "复制安装包失败"
            exit 1
        }
        return
    fi

    # Download the latest version of the release
    info "正在下载安装包..."
    if has wget; then
        wget -q --show-progress -O $INSTALLER_DIR/xdwlan-login.tar.xz $DOWNLOAD_URL
    elif has curl; then
        curl --progress-bar -SL -o $INSTALLER_DIR/xdwlan-login.tar.xz $DOWNLOAD_URL
    else
        error "请安装 wget 或者 curl 以下载安装包 (例如， sudo apt-get install -y wget)"
        exit 1
    fi

    if [ $? -ne 0 ]; then
        error "下载安装包失败"
        exit 1
    fi

}

install() {
    info "正在安装..."
    tar -xf $INSTALLER_DIR/xdwlan-login.tar.xz -C $INSTALLER_DIR
    [ -d "/opt/xdwlan-login" ] && sudo rm -r /opt/xdwlan-login
    sudo cp -r $INSTALLER_DIR/xdwlan-login-x86_64-unknown-linux-gnu /opt/xdwlan-login
    sudo ln -sf /opt/xdwlan-login/xdwlan-login /usr/local/bin/xdwlan-login
    sudo chmod +x /usr/local/bin/xdwlan-login
    mkdir -p ~/.config/xdwlan-login

    # Create systemd service file
    info "正在创建 systemd 服务文件..."
    SERVICE_FILE="/etc/systemd/system/xdwlan-login.service"
    cat <<EOF | sudo tee $SERVICE_FILE >/dev/null
[Unit]
Description=xdwlan-login service
After=network.target

[Service]
ExecStart=/usr/local/bin/xdwlan-login
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF
    sudo systemctl daemon-reload

}

notice() {
    completed "安装完成!"
    cat <<EOF

请修改文件 /opt/xdwlan-login/config.yaml，并填入学号和密码。
然后运行 xdwlan-login --oneshot 即可登录校园网。

也可以不加 --oneshot 参数，让 xdwlan-login 以守护进程的方式运行，以实现自动登录和断网重连。
如果你想开机自动登录，可以开启 xdwlan-login 服务:

    sudo systemctl enable --now xdwlan-login.service

如果使用过程中遇到问题，请在 Issues 中反馈，谢谢!
项目地址: $REPO

EOF
}

download
install
notice
