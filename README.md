# makeBrowser

「作って学ぶブラウザのしくみ」ハンズオン用リポジトリ

## 概要

WasabiOS上で動作する自作ブラウザ「saba」を実装するプロジェクトです。

## プロジェクト構成

```
makeBrowser/
└── saba/                    # ブラウザ本体
    ├── src/                 # エントリーポイント
    ├── saba_core/           # ブラウザコア機能
    │   └── src/
    │       ├── lib.rs
    │       ├── url.rs       # URLパーサー
    │       ├── http.rs      # HTTPレスポンスパーサー
    │       └── error.rs     # エラー型定義
    ├── net/wasabi/          # WasabiOS用ネットワーク実装
    │   └── src/
    │       └── http.rs      # HTTPクライアント
    └── build/wasabi/        # WasabiOS（実行環境）
```

## 実装状況

- [x] 第2章: URLパーサー（HTTPスキーム対応）
- [x] 第3章: HTTP通信
  - [x] HTTPレスポンスパーサー
  - [x] HTTPクライアント

## 必要な環境

- Rust (rustup)
- make
- clang
- qemu-system-x86_64

### Ubuntu での依存パッケージインストール

```bash
sudo apt install make clang qemu-system-x86
```

## 実行方法

```bash
cd saba
./run_on_wasabi.sh
```

## 参考

- [WasabiOS](https://github.com/hikalium/wasabi)
- 書籍「作って学ぶブラウザのしくみ」
