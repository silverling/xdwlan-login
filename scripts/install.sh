#!/bin/sh

set -eu
printf '\n'

REPO="https://github.com/silverling/xdwlan-login"
INSTALLER_DIR=""
ARGC="$#"
FIRST_ARG="${1-}"
TARGET_TRIPLE="x86_64-unknown-linux-gnu"
SYSTEMD_AVAILABLE=0
OPENRC_AVAILABLE=0

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

if [ -f /etc/os-release ]; then
  # shellcheck disable=SC1091
  . /etc/os-release
  distro="${ID:-unknown}"
else
  distro="unknown"
fi

if [ "$distro" = "alpine" ]; then
  TARGET_TRIPLE="x86_64-unknown-linux-musl"
fi

DOWNLOAD_URL="$REPO/releases/latest/download/xdwlan-login-$TARGET_TRIPLE.tar.xz"
INSTALL_SOURCE_DIR="xdwlan-login-$TARGET_TRIPLE"

create_installer_dir() {
  if has mktemp; then
    INSTALLER_DIR="$(mktemp -d "/tmp/xdwlan-login-installer.XXXXXX")"
  else
    INSTALLER_DIR="/tmp/xdwlan-login-installer.$$"
    mkdir -p "$INSTALLER_DIR"
  fi
}

cleanup() {
  if [ -n "$INSTALLER_DIR" ] && [ -d "$INSTALLER_DIR" ]; then
    rm -rf "$INSTALLER_DIR"
  fi
}

trap 'cleanup' EXIT INT TERM

assert_supported_platform() {
  arch="$(uname -m 2>/dev/null || printf 'unknown')"
  os="$(uname -s 2>/dev/null || printf 'unknown')"

  if [ "$os" != "Linux" ]; then
    error "当前系统为 $os，该安装脚本仅支持 Linux"
    exit 1
  fi

  if [ "$arch" != "x86_64" ] && [ "$arch" != "amd64" ]; then
    error "当前架构为 $arch，目前仅提供 x86_64 预编译安装包"
    exit 1
  fi
}

assert_required_tools() {
  if ! has tar; then
    error "缺少 tar 命令，请先安装 tar"
    exit 1
  fi

  if [ "$ARGC" -eq 0 ] && ! has wget && ! has curl; then
    error "请安装 wget 或 curl 以下载安装包"
    exit 1
  fi
}

usage() {
  cat <<EOF
用法:
  sh ./install.sh [本地安装包路径]

说明:
  不带参数时，脚本会自动下载并安装最新版本。
  传入本地安装包路径时，脚本会跳过下载并直接安装该文件。

示例:
  sh ./install.sh
  sh ./install.sh xdwlan-login-x86_64-unknown-linux-gnu.tar.xz
  sh ./install.sh xdwlan-login-x86_64-unknown-linux-musl.tar.xz

选项:
  -h, --help    显示本帮助信息
EOF
}

case "$FIRST_ARG" in
"") ;;
-h | --help)
  usage
  exit 0
  ;;
-*)
  printf '%s\n' "x 未知选项: $FIRST_ARG" >&2
  usage
  exit 1
  ;;
esac

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

run_as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif has sudo; then
    sudo "$@"
  else
    error "需要 root 权限，请使用 root 用户执行或安装 sudo"
    exit 1
  fi
}

download() {
  install_from_local=0

  # Check whether installing from local file
  if [ "$ARGC" -gt 1 ]; then
    error "错误：最多只能指定一个文件路径"
    exit 1
  elif [ "$ARGC" -eq 1 ]; then
    # Get tarball from explcitly provided path
    local_tarball="$FIRST_ARG"
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

  if [ "$install_from_local" -eq 1 ]; then
    info "使用本地安装包，跳过下载..."
    cp "$local_tarball" "$INSTALLER_DIR/xdwlan-login.tar.xz" || {
      error "复制安装包失败"
      exit 1
    }
    return
  fi

  # Download the latest version of the release
  info "正在下载安装包 ($TARGET_TRIPLE)..."
  if has wget; then
    if ! wget -O "$INSTALLER_DIR/xdwlan-login.tar.xz" "$DOWNLOAD_URL"; then
      error "下载安装包失败"
      exit 1
    fi
  elif has curl; then
    if ! curl -fL -o "$INSTALLER_DIR/xdwlan-login.tar.xz" "$DOWNLOAD_URL"; then
      error "下载安装包失败"
      exit 1
    fi
  else
    error "请安装 wget 或者 curl 以下载安装包 (例如， sudo apt-get install -y wget)"
    exit 1
  fi

}

install() {
  info "正在安装..."
  tar -xf "$INSTALLER_DIR/xdwlan-login.tar.xz" -C "$INSTALLER_DIR"

  if [ ! -d "$INSTALLER_DIR/$INSTALL_SOURCE_DIR" ]; then
    error "安装包内容不正确：未找到目录 $INSTALL_SOURCE_DIR"
    exit 1
  fi

  CONFIG_BACKUP="$INSTALLER_DIR/config.yaml.backup"
  if [ -f "/opt/xdwlan-login/config.yaml" ]; then
    info "检测到已有配置文件，正在备份..."
    run_as_root cp "/opt/xdwlan-login/config.yaml" "$CONFIG_BACKUP"
  fi

  [ -d "/opt/xdwlan-login" ] && run_as_root rm -r /opt/xdwlan-login
  run_as_root cp -r "$INSTALLER_DIR/$INSTALL_SOURCE_DIR" /opt/xdwlan-login

  if [ -f "$CONFIG_BACKUP" ]; then
    info "正在恢复已有配置文件..."
    run_as_root cp "$CONFIG_BACKUP" "/opt/xdwlan-login/config.yaml"
  fi

  run_as_root ln -sf /opt/xdwlan-login/xdwlan-login /usr/local/bin/xdwlan-login
  run_as_root chmod +x /usr/local/bin/xdwlan-login
  mkdir -p ~/.config/xdwlan-login

  # Create systemd service file
  if has systemctl && [ -d "/etc/systemd/system" ]; then
    SYSTEMD_AVAILABLE=1
    info "正在创建 systemd 服务文件..."
    SERVICE_FILE="/etc/systemd/system/xdwlan-login.service"
    cat <<EOF | run_as_root tee "$SERVICE_FILE" >/dev/null
[Unit]
Description=xdwlan-login service
After=network.target

[Service]
ExecStart=/opt/xdwlan-login/xdwlan-login
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF
    if ! run_as_root systemctl daemon-reload; then
      warn "systemctl daemon-reload 失败，请稍后手动执行"
    fi
  elif [ "$distro" = "alpine" ] && [ -d "/etc/init.d" ]; then
    OPENRC_AVAILABLE=1
    info "正在创建 OpenRC 服务文件..."
    OPENRC_SERVICE_FILE="/etc/init.d/xdwlan-login"
    cat <<'EOF' | run_as_root tee "$OPENRC_SERVICE_FILE" >/dev/null
#!/sbin/openrc-run

name="xdwlan-login"
description="xdwlan-login service"
command="/usr/local/bin/xdwlan-login"
command_background=true
pidfile="/run/${RC_SVCNAME}.pid"

depend() {
    need net
}
EOF
    run_as_root chmod +x "$OPENRC_SERVICE_FILE"
  else
    warn "未检测到 systemd，已跳过 systemd 服务创建（Alpine 通常使用 OpenRC）"
  fi

}

notice() {
  completed "安装完成!"
  cat <<EOF

请修改文件 /opt/xdwlan-login/config.yaml，并填入学号和密码。
然后运行 xdwlan-login --oneshot 即可登录校园网。

也可以不加 --oneshot 参数，让 xdwlan-login 以守护进程的方式运行，以实现自动登录和断网重连。
EOF

  if [ "$SYSTEMD_AVAILABLE" -eq 1 ]; then
    cat <<EOF
如果你想开机自动登录，可以开启 xdwlan-login 服务:

    sudo systemctl enable --now xdwlan-login.service

EOF
  elif [ "$OPENRC_AVAILABLE" -eq 1 ]; then
    cat <<EOF
如果你想开机自动登录，可以开启 xdwlan-login 服务:

    sudo rc-update add xdwlan-login default
    sudo rc-service xdwlan-login start

EOF
  else
    cat <<EOF
当前系统未使用 systemd，请将 xdwlan-login 添加到你所使用的服务管理器（如 OpenRC）。

EOF
  fi

  cat <<EOF

如果使用过程中遇到问题，请在 Issues 中反馈，谢谢!
项目地址: $REPO

EOF
}

main() {
  assert_supported_platform
  assert_required_tools
  create_installer_dir
  download
  install
  notice
}

main
