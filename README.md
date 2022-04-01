# プログラミング言語 Met

## 説明

内輪ネタ [プログラミング言語 Met](https://metlaboratory.github.io/met-site/posts/metlang/) の Rust 実装。
[オリジナル](https://github.com/MetLaboratory/metlang) で動かんパターンがあったので独自にやっつけ実装。
気が向いたらもうちょいきちんと実装するかも。

## 実行

ファイルを指定するとそのソースを実行します。
`-e ソース` でソース直接指定もできます。
どちらも指定しなければ標準入力からソースを読みます。 

### 例

```
cargo run -- examples/melt.met
```
