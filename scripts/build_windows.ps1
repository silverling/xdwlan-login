function Get-Version {
    $pkgid = (cargo pkgid)
    return $pkgid.Split('#')[1]
}

$name = 'xdwlan-login'
$version = Get-Version
$target = 'x86_64-pc-windows-msvc'
$dist_name = "$name-$target"

$dist_dir = 'dist'
$version_dir = "$dist_dir\$version"
$archive_dir = "$version_dir\$dist_name"
$archive_name = "$version_dir\$dist_name.zip"

# Build rust program
cargo build --release --target $target

# Build deno program
if (-not (Test-Path "node_modules")) { deno install }
deno task bundle
deno task compile:windows

# Prepare dist folder
if (-not (Test-Path $dist_dir)) { New-Item -ItemType Directory -Path $dist_dir | Out-Null }
if (Test-Path $archive_dir) { Remove-Item -Path $archive_dir -Recurse -Force }
New-Item -ItemType Directory -Path $archive_dir -Force | Out-Null

# Create archive
if (Test-Path $archive_name) { Remove-Item -Path $archive_name -Force }
Copy-Item "target\$target\release\xdwlan-login.exe" -Destination $archive_dir
Copy-Item "build\xdwlan-login-worker.exe" -Destination $archive_dir
@"
username: "23xxxxxxxxx" # 学号
password: "***********" # 密码
domain: "" # 留空表示默认，中国移动填 "@yd"，中国联通填 "@lt"，中国电信填 "@dx"
"@ | Out-File -FilePath "$archive_dir\config.yaml" -Encoding utf8
Compress-Archive -Path $archive_dir -DestinationPath $archive_name -Force