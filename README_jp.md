nwkcalc
===

nwkcalcはIPv4ネットワーク設定値計算用のツールです。

## 説明

IPv4ネットワークのIPアドレス計算を行うためのツールとしては下記のようなものが存在します。
- Linux環境
  - ipcalc
  - ipcount
  - sipcalc
- Webサイトの公開ツール

当ツールはこれらのツールには及ばないものの、下記を目標に開発を行っています。
- コマンドラインで気軽に使用できる。
- マルチプラットフォームで使用できる。

## 比較

他のツールに存在しない機能として、IPアドレスの列挙出力を行うことができます。
この機能を使用することで、例えばIPアドレスを一括で指定するためのスクリプトを作成する際に利用することができます。

出力例
```
$ nwkcalc -e 192.168.1.1/24 > iplist.txt
$ cat iplist.txt
192.168.1.1/255.255.255.0
192.168.1.2/255.255.255.0
  :
192.168.1.253/255.255.255.0
192.168.1.254/255.255.255.0
```

## 使い方

**現在開発中のため、今後コマンドライン引数は変更を行う可能性があります。**

```
nwkcalc [OPTIONS] <ADDRESS>

ARGS:
    <ADDRESS>    IP address string (IPv4 only)

OPTIONS:
    -e, --enumerate    Enumerated display of IP addresses
    -h, --help         Print help information
        --prefix       Prefix display of subnet masks when displaying IP address enumeration
    -V, --version      Print version information
```

### コマンド例
パラメータ（IPアドレス）に基づいたネットワーク情報が表示されます。
```bash
$ nwkcalc 192.168.1.1/255.255.255.0
```
出力
```
Address:   192.168.1.1
NetMask:   255.255.255.0 (=24)
Network:   192.168.1.0/24
HostMin:   192.168.1.1
HostMax:   192.168.1.254
Broadcast: 192.168.1.255
Hosts/Net: 254
```

サブネットマスクはプレフィックス表記(1-32)とすることも可能です。
```bash
$ nwkcalc 192.168.1.1/24
```
出力
```
Address:   192.168.1.1
NetMask:   255.255.255.0 (=24)
Network:   192.168.1.0/24
HostMin:   192.168.1.1
HostMax:   192.168.1.254
Broadcast: 192.168.1.255
Hosts/Net: 254
```

-eオプションを付けることで、パラメータ（IPアドレス）に基づいたネットワーク内のすべてのIPアドレスを列挙表示することができます。
```bash
$ nwkcalc -e 192.168.1.1/24
```
出力
```
192.168.1.1/255.255.255.0
192.168.1.2/255.255.255.0
  :
192.168.1.253/255.255.255.0
192.168.1.254/255.255.255.0
```

'--prefix'オプションを付けることで、列挙表示時のサブネットマスクをプレフィックス(1-32)値とすることができます。
```bash
$ nwkcalc -e --prefix 192.168.1.1/24
```
出力
```
192.168.1.1/24
192.168.1.2/24
  :
192.168.1.253/24
192.168.1.254/24
```

## インストール

### 動作確認環境

下記環境で動作することを確認しています。
- macOS Monterrey (12.4) 
- Windows 10 (Version 21H2)
- Linux (Debian, Ubuntu...)

### 具体的な手順

1. Rustの開発環境をインストールしておきます。（具体的なインストール方法については下記サイト等を参照して下さい。）  
[Rust をインストール \- Rustプログラミング言語](https://www.rust-lang.org/ja/tools/install)

2. 当リポジトリを任意のディレクトリ上にクローンします。
```bash
$ git clone https://github.com/ma-kappa/nwkcalc.git
```

3. クローンしたローカルリポジトリディレクトリに移動しビルドを行います。
```bash
$ cd nwkcalc
$ cargo build --release
```

4. 下記コマンドでインストールします。
```bash
$ cargo install --path .
```
> インストール先は  $HOME/.cargo/bin (Windowsの場合 %USERPROFILE%¥.cargo¥bin) 直下となります。

5. コマンドラインシェル環境で正常に実行できることを確認します。
```bash
$ nwkcalc 192.168.1.1/24
```

## ライセンス

カレントディレクトリの[LICENSEファイル](./LICENSE)を参照して下さい。
