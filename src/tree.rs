// tree: 読んだ棋譜からゲーム木を構築

use std::collections::btree_map::{BTreeMap, Entry};

static MAX_TURNS: u32 = 20;

pub struct GameTree {
    pub children: BTreeMap<u8, Box<GameTree>>,
    pub play_keys: Vec<u8>,
    pub score: f64,
    pub plays: f64,
}

impl GameTree {
    pub fn new() -> Self {
        let children = BTreeMap::new();
        GameTree {
            children,
            play_keys: Vec::new(),
            score: 0.0,
            plays: 0.0,
        }
    }

    pub fn add_play(&mut self, buf: &[u8]) {
        let pl = read_buf(buf);
        let score = (pl.black as f64) - (pl.white as f64);
        self.add_play2(pl.plays, score, 0);
    }
    fn add_play2(&mut self, pl: &[u8], score: f64, turn: u32) {
        // bufを読んであれする
        let mut tr = self;
        if turn >= MAX_TURNS {
            // 規定のターンまで探索した
            return ();
        }
        match pl.split_first() {
            None => {
                // もう終わりだ
                ()
            },
            Some((&p, buf2)) => {
                // ひとつの手
                // スコアを変更 (average)
                tr.score = (tr.score * tr.plays + score) / (tr.plays + 1.0);
                tr.plays += 1.0;

                if p == 0xff {
                    // もう終わりじゃん
                    return ();
                }
                // 次へ
                let en = tr.children.entry(p);
                match en {
                    Entry::Vacant(e) => {
                        // この手は登録されていない
                        let ntr = GameTree::new();
                        let mut trp = e.insert(Box::new(ntr));
                        tr.play_keys.push(p);
                        trp.add_play2(buf2, score, turn+1);
                    },
                    Entry::Occupied(o) => {
                        // すでに登録
                        let mut trp = o.into_mut();
                        trp.add_play2(buf2, score, turn+1);
                    },
                }
            },
        }
    }
}

struct Play<'a> {
    plays: &'a [u8],
    black: u8,
    white: u8,
}
fn read_buf<'a>(buf: &'a [u8]) -> Play<'a> {
    let plays = &buf[0..60];
    let black = buf[62];
    let white = buf[63];

    Play {
        plays,
        black,
        white,
    }
}
