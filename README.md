# Design Pattern For Rust

## 第１章：Iteratorパターン

- IntelliJが有能だったので切り替えた
- リテラル文字列はto_string()でStringに変換する必要がある
- RustにはIteratorトレイトとIntoIterトレイトが標準実装
  - Iteratorはnext()
  - IntoIterはinto_iter()
    - どちらもトレイトのimpl内でtype Item = hoge;とすることでジェネリクスに近い働きを実現していた
- 複数のmain関数を実現するには、src/binフォルダ内に作る 
- src/C01_iteratorファイル内のものを参照するには、lib.rsかmain.rsでpub mod C01_iteratorとしてモジュール宣言する
- ライフタイム注釈に注意

## 第２章：Adapterパターン

- 引数に文字列を取るときはスライス(&str)にすればリテラルもぶちこめるので簡易
- 関数スタック内で新しく作った構造体はあくまで関数スコープ内でのみ存在する
  - 初期化と同時に構造体への参照を別の構造体に持たせたいならBoxで包むなりする必要がある

## 第３章：Template Methodパターン

- traitの関数シグネチャにはpubは不要
  - traitとして定義してる時点で公開前提だよね、って思想っぽい
- println!ではリテラルのみ出力できる
  - 文字列とか出したいときは println!("{}",string);
- リテラルからStringを生成したいときはString::from("")

## 第４章：Factoryパターン

- インタフェース上で抽象メソッドを利用したデフォルト動作の定義みたいなことが許されないので、
  - Factoryパターンの恩恵は全く得られていない
  - そもそもこういうインタフェースをベースにするという考え方がRustと相性悪そう
- 参照でなく値を渡してしまうと、その時点で渡したオブジェクト先に所有権が移る(moveされる)

## 第５章：Singletonパターン

- Singletonである必要が本当にあるのか？
  - 少なくともenumで処理できる範囲ではその必要はない
  - 必要が出てきたら明示的に配列なりにプールすることになるだろう
- デフォルトではenumに比較演算子は使えない 
  - 等値比較を可能にしたいなら #[derive(PartialEq)]

## 第７章：Builderパターン

- 文字列を書き出す場合、write!やwriteln!マクロを使ってprint!と同様にできる
  - File::createでファイルオブジェクトを作る
- BuilderをDirectorに持たせて、Director側で操作を行いたいとき、
  - Builderの可変参照を渡す
    - ただしライフタイム注釈を付けて、Builderが解体された時点でDirectorも解体されることを保証する

## 第８章：AbstractFactoryパターン

- 保留、継承を前提としてない？
  - いや、enumと値の設定を使えばいけるかもしれない、後でやる

## 第９章

- traitにはデフォルト実装を提供できる！
  - デフォルト実装を利用する際には、impl MyTrait for MyStruct {} みたいに空宣言でOK
- Implクラスを「持たせる」よりも、都度引数として渡す方が見通しが良い
  - オブジェクト指向の否定、副作用のなさの方を重視
- フィールド無しの構造体は、new関数を使わずともMyStruct{}で初期化できる