extern crate opening_othello;
extern crate getopts;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::env;

use getopts::Options;

use opening_othello::tree;
use tree::GameTree;
use opening_othello::{opening, evaluate};

static RECORD_NAME: &str = "./data/record.db";
static OUT_OPENING_NAME: &str = "./data/opening.db";
static OUT_EVAL_NAME: &str = "./data/eval.db";

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("f", "file", "input file name", "FILE");
    opts.optopt("o", "out", "output file name", "FILE");
    opts.optflag("r", "replace", "replace existing output file");
    opts.optflag("", "opening", "Generate an opening book.");
    opts.optopt("t", "turn", "[opening] turn number to consider", "NUMBER");
    opts.optflag("", "evaluate", "Generate an evaluation parameter.");

    let opts = opts.parse(&args[1..]).unwrap();

    let input_name = opts.opt_str("file").unwrap_or(String::from(RECORD_NAME));

    let meta = fs::metadata(input_name.as_str()).unwrap();
    let filesize = meta.len();
    let mut file = File::open(input_name.as_str()).unwrap();

    // turn number
    let default_turn =
        if opts.opt_present("evaluate") {
            64
        } else {
            25
        };
    let turns = opts.opt_str("turn").map_or(Ok(default_turn), |s| s.parse()).unwrap();

    println!("Loaded {}", input_name);
    // GameTreeをとりあえず作る
    let mut tree = GameTree::new();
    let mut pos = 0;
    // ひとつずつ読んでいく
    let mut buf = [0; 64];
    let mut cnt = 0u32;
    while pos < filesize {
        file.read_exact(&mut buf).unwrap();
        // treeに加える
        tree.add_play(&buf, turns);

        pos += 64;
        cnt += 1;
        if cnt % 100 == 0 {
            println!("Read {} plays", cnt);
        }
    }
    println!("{} plays", tree.plays);
    println!("total score: {}", tree.score);

    let replace = opts.opt_present("replace");
    if opts.opt_present("opening") {
        // serializeする
        let outfilename = opts.opt_str("out").unwrap_or(String::from(OUT_OPENING_NAME));
        let outfile = OpenOptions::new().write(true).create(true).create_new(!replace).open(outfilename.as_str()).unwrap();
        opening::serialize(tree, outfile);
    } else if opts.opt_present("evaluate") {
        let outfilename = opts.opt_str("out").unwrap_or(String::from(OUT_EVAL_NAME));
        let outfile = OpenOptions::new().write(true).create(true).create_new(!replace).open(outfilename.as_str()).unwrap();
        evaluate::serialize(tree, outfile, turns);
    }
}
