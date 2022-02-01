//! ドキュメンテーションコメントの説明用に作ったサンプルの
//! プロジェクトです。  
//! 改行を入れたい場合はGitHubなどのマークダウンと同様に行末に
//! 半角スペースを2つ入れます。ただのコメント内の改行だけでは改行が
//! 無視されます。ただしドキュメント上では半角スペースが入るようです。  
//! *斜体テスト*  
//! **太字テスト**
//! 
//! - リスト1
//! - リスト2
//! 
//! * リスト3
//! * リスト4
//! 
//! [リンクテスト](https://qiita.com/)
//! 
//! # 見出しテスト1
//! テストテキスト1
//! 
//! ## 見出しテスト2
//! テストテキスト2
//! 
//! ~~打消し線テスト~~
//! 
//! > 引用部分
//! 
//! 水平線:
//! 
//! ---
//! 
//! インラインコードテスト : `println("Hello!");`
//! 
//! ```
//! println!("コードブロックテスト");
//! ```
//! 
//! テーブル記法
//! 
//! | 見出し1 | 見出し2 |
//! |:-----------|:------------|
//! | 猫 | 犬 |
//! | 兎 | 鳥 |

/// このプロジェクトのエントリーポイントです。
/// Hello, world!が出力されます。  
/// **太字テスト**  
/// *斜体テスト*  
/// 
/// # 見出し1

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	// Rustの開発環境として「 VSCode」をセットアップする
	// https://news.mynavi.jp/techplus/article/rust-10/

	// Rustのドキュメンテーションコメントについて学ぶ
	// https://qiita.com/simonritchie/items/87d3743e138763ff3e85
	
	// 行末までコメント
	println!("Hello, world!");
	/* ブロックコメント
	   2行目もコメント
	*/
	print!("Hello, world!");
	
	println!("");
	
	// In general, the `{}` will be automatically replaced with any
	// arguments. These will be stringified.
	// 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
	// 例えば以下は文字列に変換されます
	println!("{} days", 31);

	// Without a suffix, 31 becomes an i32. You can change what type 31 is
	// by providing a suffix. The number 31i64 for example has the type i64.
	// サフィックスで型を指定しなければ31はi32として扱われます。
	// サフィックスの指定により、31の型を自由に変換することができます。

	// There are various optional patterns this works with. Positional
	// arguments can be used.
	// 引数の位置から埋め込まれる場所を指定することができます。
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	// As can named arguments.
	// 名前での指定も可能です。
	println!("{subject} {verb} {object}",
		object = "the lazy dog",
		subject = "the quick brown fox",
		verb = "jumps over");

	// Special formatting can be specified after a `:`.
	// `:` のあとにフォーマット型を指定することによる特殊なフォーマットも可能です.
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	// You can right-align text with a specified width. This will output
	// "     1". 5 white spaces and a "1".
	// 指定した幅の中に、右寄せで文字列を挿入することができます。
	// 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
	println!("{number:>width$}", number = 1, width = 6);

	// You can pad numbers with extra zeroes. This will output "000001".
	// 空白の代わりに0を使うこともできます. このアウトプットは "000001" になります.
	println!("{number:0>width$}", number = 1, width = 6);

	// Rust even checks to make sure the correct number of arguments are
	// used.
	// 引数の数が正しいかのチェックも行ってくれます。
	//(NG) println!("My name is {0}, {1} {0}", "Bond");
	// FIXME ^ Add the missing argument: "James"

	// Create a structure named `Structure` which contains an `i32`.
	// `i32`保持する `Structure` という名の構造体を定義します.
	#[allow(dead_code)]
	struct Structure(i32);

	// However, custom types such as this structure require more complicated
	// handling. This will not work.
	// このようにカスタム型を用いる場合、少々扱いが複雑になります。
	// 以下は動作しません。
	//(NG) println!("This struct `{}` won't print...", Structure(3));
	// FIXME ^ Comment out this line.
	
	// 【Rust】ループとイテレータの使い方
	// https://tyfkda.github.io/blog/2020/06/13/rust-iter.html
	
    // 所定回数ループしたい場合
    for i in 0..3 {
        println!("{}", i);
    }
	
	let dim_data = vec![1, 2, 3, 4, 5];
	println!("{}", dim_data[1]);
	
	for dim in dim_data {
		println!("{}", dim);
	}
	
	// Rustで書くソートアルゴリズム
	// https://qiita.com/jlkiri/items/f0bb8b1fa9da3bfc68ee
	
	let mut array = vec![200, 100, 500, 300, 400];
	for i in 0..array.len() {
		for j in 0..array.len() - i - 1 {
			if array[j + 1] < array[j] {
				// let tmp = array[j];
				// array[j] = array[j + 1];
				// array[j + 1] = tmp;
				array.swap(j, j + 1);
			}
		}
	}

	for dim in array {
		println!("{}", dim);
	}

	// 引数を指定して実行する方法
	// https://isolution.pro/q/so59883090/visual-studio-code-kara-rust-apurike-shon-o-kidosuru-ni-wa-do-sureba-yoi-desu-ka

	// Rustでコマンドライン引数を受け取る
	// https://ytyaru.hatenablog.com/entry/2020/09/19/000000

	let args: Vec<String> = env::args().collect();
	//let query = &args[1];
	//let filename = &args[2];

	//let (query, filename) = parse_config(&args);

	// {}を探しています
	//println!("Searching for {}", query);
	//println!("In file {}", filename);

	// ファイルが見つかりませんでした
	//let mut f = File::open(filename).expect("file not found");

	let config = parse_config(&args);

	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);

	let mut f = File::open(config.filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
			// ファイルの読み込み中に問題がありました
			.expect("something went wrong reading the file");

	// テキストは\n{}です
	println!("With text:\n{}", contents);	

}

/*
fn parse_config(args: &[String]) -> (&str, &str) {
	let query = &args[1];
	let filename = &args[2];

	(query, filename)
}
*/
struct Config {
	query: String,
	filename: String,
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let filename = args[2].clone();

	Config { query, filename }
}