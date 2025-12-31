# Strune
<p align="center">
  <img src="./logo.svg" alt="logo" width="120">
</p>

Struneは、**シンプルで指向性のある知識構造**です。基本的には単語帳・リンク集として運用する想定ですが、どのように使用するかはあなた次第です。

[English](./README.md)
## 構造

コアデータ構造は `strune_core/src/node.rs` で定義されています：

```rust
pub struct Node<T = Value> {
    pub label: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub options: T,
}
```

- **label**: 知識ノードの名前。これが主な識別子です。
- **description**: 知識の説明。基本的に最大200文字程度の短文です。
- **dependencies**: その知識から見た親に当たる知識群。関連するノードのラベル（文字列）を含みます。
- **options**: ユーザーの自由に定義できる拡張フィールドです。ジェネリック型 `T` により型安全なカスタムフィールドが可能（デフォルトは `serde_json::Value`）。

dependencies は単なるリレーショナルリンクではないことに注意してください。
解釈は最終的にユーザーに委ねられますが、基本的にこの知識を構成または含有する要素を記述します。

例：
- `Unity` には dependencies に `ゲームエンジン` などが記述されます。
- `Blender` と `FBX` は互いに dependencies に記載されます。

## 曖昧じゃない？
はい。知識は元来親子のようなシンプルな関係で記述できるものでは在りません。そこを無理やりこじつけてビジュアル化してやろうという概念です。

最大の特徴はシンプルさと指向性にあり、それらの表現力に期待しています。

# パッケージ群

このプロジェクトは Rust ワークスペース形式の monorepo として構成されています。

> ## ⚠️ ベータ版
> 以下のコンポーネントはベータ版であり、破壊的な API 変更が行われる可能性があります。

## strune_core

**場所**: `strune_core/`

Strune のコアデータ構造ライブラリ。他の全てのパッケージが依存する基本的な `Node<T>` 型を定義します。

**機能**:
- 型安全なカスタムオプションをサポートするジェネリック `Node<T>` 構造体
- Serde ベースの JSON シリアライズ/デシリアライズ
- 異なるオプション型間の変換のための `format_options<T>()` メソッド
- デバッグ用の Display トレイト実装

**依存関係**: `serde`, `serde_json`

## loader

**場所**: `crates/loader/`

様々なファイルフォーマットから Strune ノードをコア `Node` 構造体に読み込むためのライブラリ。

**機能**:
- **Markdown パーサー** (`markdown.rs`) – 拡張 Markdown ファイルを Strune ノードに変換
  - 階層的な見出し構造をサポート（`#` = label、`##` = セクション）
  - description、dependencies、options の処理をカスタマイズ可能
- **JSON ローダー** (`json.rs`) – 直接 JSON ファイルをパース

**主な関数**:
- `load_nodes_from_markdown<T>(path)` – デフォルトのパーサーで Markdown からノードを読み込み
- `load_nodes_from_markdown_detail<T>()` – カスタムパース関数で読み込み
- `trim_text()` – テキスト正規化ヘルパー
- `list_text_to_array()` – Markdown リストを文字列配列に変換

### Markdown 構文

生の JSON を書くのは面倒なので、Strune ではミニマルな Markdown ベースの構文を導入しています：

```markdown
# <label>
## description
<description>
## dependencies
- <dependence01>
- <dependence02>
or
- <dependence01>
- <dependence02>
## options
<options>
```

label 以外のフィールドはすべてオプションです。

**依存関係**: `strune_core`, `regex`, `serde`, `serde_json`, `once_cell`, `thiserror`

## operation

**場所**: `crates/operation/`

Strune ノードグラフの分析と操作のための機能を提供します。このパッケージは、ノードに計算フィールドを追加するためのトレイトベースの拡張性を導入しています。

**機能**:
- **Dependents** (`dependents.rs`) – 逆依存関係の計算
  - 逆依存関係にアクセスするための `HasDependents` / `MaybeDependents` トレイト
  - 各ノードに依存するノードを計算する `fill_dependents()` 関数
  - 簡単なトレイト実装のためのマクロサポート
- **Slug** (`slug.rs`) – URL フレンドリーな識別子の生成
  - slug フィールドにアクセスするための `HasSlug` / `MaybeSlug` トレイト
  - `label_slug_map()` でラベルから slug へのマッピングを作成
  - カスタム slug がない場合はラベルにフォールバック

**主な関数**:
- `fill_dependents<T>(nodes)` – 逆依存関係情報を設定
- `label_slug_map<T>(nodes)` – ラベルから slug へのマッピングを生成

**依存関係**: `strune_core`

## tera-render

**場所**: `crates/tera-render/`

[Tera](https://keats.github.io/tera/) テンプレートエンジンを使用して Strune ノードから静的 HTML サイトを生成するレンダリングエンジン。

**機能**:
- Tera によるテンプレートベースの HTML 生成
- 依存関係と逆依存関係のリンクを自動生成
- Slug ベースの URL ルーティング
- ブランディングとベースパスの設定が可能

**主要コンポーネント**:
- `ingwaz.rs` – メインレンダリングロジック
  - `render()` – 全ノードの HTML ファイルを生成
  - `render_node_page()` – 個別のノードページをレンダリング
  - `RenderNode` – リンク生成のためのヘルパー構造体

**テンプレートコンテキスト変数**:
- `current_node` – レンダリング中のノード
- `dependencies` – リンク付きの親ノードリスト
- `dependents` – リンク付きの子ノードリスト
- `base_path`, `brand_logo`, `brand_title` – サイト設定

**依存関係**: `strune_core`, `operation`, `tera`, `serde`, `serde_json`

## cli

**場所**: `cli/`

Strune のコマンドラインインターフェース。すべてのパッケージを統合するメイン実行ファイルです。

**機能**:
- Strune ノードを含む Markdown ファイルの読み込み
- 逆依存関係の自動計算
- テンプレートを使用した静的 HTML サイトの生成
- 出力ディレクトリへの公開アセットのコピー
- 出力ディレクトリのクリアと再構築

**現在のワークフロー**:
1. `content/sample.md` からノードを読み込み
2. `operation::fill_dependents()` を使用して逆依存関係を設定
3. Tera テンプレートを使用して全ノードを `dist/` ディレクトリにレンダリング
4. `public/` アセットを `dist/public/` にコピー

**依存関係**: `strune_core`, `loader`, `operation`, `tera-render` (alias: `render`), `serde`, `serde_json`, `anyhow`