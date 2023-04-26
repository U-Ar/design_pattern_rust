# Design Pattern For Rust

## 第一章：Iteratorパターン

- IntelliJが有能だったので切り替えた
- リテラル文字列はto_string()でStringに変換する必要がある
- RustにはIteratorトレイトとIntoIterトレイトが標準実装
  - Iteratorはnext()
  - IntoIterはinto_iter()
    - どちらもトレイトのimpl内でtype Item = hoge;とすることでジェネリクスに近い働きを実現していた
- 複数のmain関数を実現するには、src/binフォルダ内に作る 
- src/C01_iteratorファイル内のものを参照するには、lib.rsかmain.rsでpub mod C01_iteratorとしてモジュール宣言する
- ライフタイム注釈に注意