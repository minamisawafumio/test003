use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

        let filename = "poem.txt";

        // --snip--
        println!("In file {}", filename);

        // ファイルが見つかりませんでした
        let mut f = File::open(filename).expect("file not found");
    
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");
    
        // テキストは\n{}です
        println!("With text:\n{}", contents);
}
