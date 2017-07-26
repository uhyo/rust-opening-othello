extern crate opening_othello;
use std::fs;
use std::fs::File;
use std::io::Read;

use opening_othello::tree;
use tree::GameTree;
use opening_othello::serialize::serialize;

static INPUT_NAME: &str = "./data/record.db";
static OUTPUT_NAME: &str = "./data/out.db";

fn main() {
    let meta = fs::metadata(INPUT_NAME).unwrap();
    let filesize = meta.len();
    let mut file = File::open(INPUT_NAME).unwrap();

    println!("hi");
    let mut tree = GameTree::new();
    let mut pos = 0;
    // ひとつずつ読んでいく
    let mut buf = [0; 64];
    let mut cnt = 0u32;
    while pos < filesize {
        file.read_exact(&mut buf).unwrap();
        // treeに加える
        tree.add_play(&buf);

        pos += 64;
        cnt += 1;
        if cnt % 100 == 0 {
            println!("Read {} plays", cnt);
        }
    }
    println!("{} plays", tree.plays);
    println!("total score: {}", tree.score);
    // serializeする
    let outfile = File::create(OUTPUT_NAME).unwrap();
    serialize(tree, outfile);
}
