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
    │       ├── error.rs     # エラー型定義
    │       └── renderer/    # レンダリングエンジン
    │           ├── dom/     # DOMツリー（Node, Window, Element）
    │           └── html/    # HTMLトークナイザー・パーサー
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
- [ ] 第4章: HTMLパーサーとDOM構築（進行中）
  - [x] HTMLトークナイザー（状態機械ベース）
  - [x] DOMノードツリー（Node, Window, Element, ElementKind）
  - [ ] HTMLパーサー（トークン→DOM変換）
    - [x] InsertionMode状態機械（Initial〜AfterAfterBody）
    - [x] construct_tree / insert_element
    - [ ] insert_char / pop_until 等の補助メソッド

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

- [d0iasm/saba](https://github.com/d0iasm/saba) - 書籍の公式リポジトリ
- [WasabiOS](https://github.com/hikalium/wasabi)
- 書籍「作って学ぶブラウザのしくみ」

## ライセンス

[MIT](LICENSE)
