# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

「作って学ぶブラウザのしくみ」書籍に基づく教育目的のブラウザ実装プロジェクト。WasabiOS上で動作するRust製ブラウザを構築する。

## ビルド・テストコマンド

```bash
# saba/ ディレクトリで実行
cd saba

# ビルド（x86_64-unknown-none ベアメタル向け）
make build

# テスト実行
make test

# 単一テスト実行
cargo test --package saba_core test_url_host

# リント
make clippy

# WasabiOS上で実行
./run_on_wasabi.sh
```

## アーキテクチャ

```
saba/                       # Cargoワークスペースルート
├── src/main.rs             # エントリポイント（noli entry_point!マクロ使用）
├── saba_core/              # コアブラウザ機能（no_std、プラットフォーム非依存）
│   ├── url.rs              # URLパーサー（http://スキーム対応）
│   ├── http.rs             # HTTPレスポンスパーサー
│   ├── error.rs            # エラー型定義
│   └── renderer/           # レンダリングエンジン
│       ├── dom/            # DOMツリー
│       │   ├── node.rs     # Node, Window, Element, ElementKind
│       │   └── mod.rs
│       └── html/           # HTMLパーサー
│           ├── token.rs    # HTMLトークナイザー（状態機械ベース）
│           ├── parser.rs   # HTMLパーサー（トークン→DOM構築）※実装中
│           ├── attribute.rs # HTML属性
│           └── mod.rs
└── net/wasabi/             # WasabiOS固有ネットワーク実装
    └── http.rs             # HTTPクライアント（noli::net使用）
```

### レイヤー構造

1. **saba_core**: `no_std`対応の純粋Rustライブラリ。外部依存なし。URL解析、HTTPレスポンス解析、HTMLトークナイズ、DOM構築を担当
2. **net/wasabi**: WasabiOS専用のネットワーク層。`noli`クレートでTCP通信
3. **saba**: メインバイナリ。`#![no_std]`/`#![no_main]`で`wasabi`フィーチャー経由で依存を選択

### 重要な設定

- **ツールチェイン**: Rust nightly（rust-toolchain.toml）
- **エディション**: saba_coreは2024、他は2021
- **ターゲット**: `x86_64-unknown-none`（ベアメタル）
- **デフォルトフィーチャー**: `["wasabi"]`

## 開発環境要件

Ubuntu: `sudo apt install make clang qemu-system-x86`

## 外部依存

- **noli**: WasabiOS API抽象化（git: hikalium/wasabi, branch: for_saba）
- **WasabiOS**: ビルド時に`build/wasabi/`へ自動クローン
