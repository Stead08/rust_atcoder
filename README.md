# パッケージ公式マニュアル
https://github.com/qryxip/cargo-compete/blob/master/README-ja.md

# qiitaの
https://qiita.com/okaponta_/items/7e82de5d1f78f547fe4b

# proconioの使い方
https://qiita.com/Pikka2048/items/a0247e792aa4f8f6dd92
 
コンテストの流れ
コンテストは以下のような流れでおこなってきます．

# コンテスト開始後，コンテストを作成
# 以下は ABC196 の例
$ cargo compete new abc196

# 上記でできるディレクトリへ移動
$ cd abc196

# 問題ページをブラウザで開く
$ cargo compete open

# rust でコードを解く. a 問題のファイルを開いている例
$ nvim ./src/bin/a.rs

# a 問題のテスト
$ cargo compete test a

# a 問題のサブミット
$ cargo compete submit a
上記のように，ブラウザ上で操作することなく，CLI上で submit までできるのがとても便利です．