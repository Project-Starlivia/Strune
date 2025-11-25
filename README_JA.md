# Strune
Struneは、**シンプルで指向性のある知識構造**です。基本的には単語帳・リンク集として運用する想定ですが、どのように使用するかはあなた次第です。

## データ構造
```json
{
    "label": "string",
    "description": "string",
    "dependence": label[],
    "options": any
}
```
- label: 知識の名前。
- description: 知識の説明。基本的に最大200文字程度の短文です。
- dependence: その知識から見た親に当たる知識群。データ形式は他のlabel(文字列)の配列です。
- options: ユーザーの自由に定義できる拡張フィールドです。

dependenceは単に関係性のリンクではない点に注意してください。解釈は最終的にユーザーに委ねられますが、基本的に含有されている要素を記述します。

例
- `Unity`のdependenceには`ゲームエンジン`などが記述されます。
- `Blender`と`FBX`は互いにdependenceに記載されます。

## 曖昧じゃない？
はい。知識は元来親子のようなシンプルな関係で記述できるものでは在りません。そこを無理やりこじつけてビジュアル化してやろうという概念です。

最大の特徴はシンプルさと指向性にあり、それらの表現力に期待しています。

# パッケージ群
とりあえずmonoRepoで構築しており、rustが書いてみたかったのでrustメインで採用しています。
> ## ⚠️ ベータ
> 次に続く項目は未だベータ版であり、壊滅的なapi変更をする可能性があります。データ構造は変えるつもりはありません。
## Markdownパーサー - MdParser
Markdown→Jsonパーサーライブラリです。

jsonを書くのはしんどいしとはいえEditor作るのも重いと感じたので、独自構文のMd形式で記述できるようにしてみました。
```markdown
# <label>
## description
<description>
## dependence
- <dependence01>
- <dependence02>
or
- [[<dependence01>]]
- [[<dependence02>]]
## options
<options>
```
options、dependenceは任意です。obsidianで使いたかったので`[[<dependence01>]]`にも対応しました。
## 表示部分 - PerseusView
Perseusを使ったssgによるビューワーです。アニメーションとか頑張りました。