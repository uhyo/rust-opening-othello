// tree: 読んだ棋譜からゲーム木を構築

use std::collections::btree_map::{BTreeMap, Entry};

static MAX_TURNS: u32 = 25;

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
        let mut transform = Transform::new();
        self.add_play2(pl.plays, score, 0, &mut transform);
    }
    fn add_play2(&mut self, pl: &[u8], score: f64, turn: u32, transform: &mut Transform) {
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
                if turn == 0 {
                    // 最初なのでtransform
                    transform.init((p >> 4), (p & 0x0f));
                }
                // 次へ
                let p = transform.get(p);
                let en = tr.children.entry(p);
                match en {
                    Entry::Vacant(e) => {
                        // この手は登録されていない
                        let ntr = GameTree::new();
                        let mut trp = e.insert(Box::new(ntr));
                        tr.play_keys.push(p);
                        trp.add_play2(buf2, score, turn+1, transform);
                    },
                    Entry::Occupied(o) => {
                        // すでに登録
                        let mut trp = o.into_mut();
                        trp.add_play2(buf2, score, turn+1, transform);
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

// 座標変換
struct Transform {
    table: Vec<u8>,
}
impl Transform {
    fn new() -> Self {
        let table = vec![0u8; 256];
        Transform {
            table,
        }
    }
    fn init(&mut self, x: u8, y: u8) {
        // C4をどこに移すかで4パターンある
        let mut table = &mut self.table;
        if x == 2 && y == 3 {
            // identity
            for x in 0..8 {
                for y in 0..8 {
                    table[(x << 4) | y] = ((x as u8) << 4) | (y as u8);
                }
            }
        } else if x == 3 && y == 2 {
            // x/y swap
            for x in 0..8 {
                for y in 0..8 {
                    table[(x << 4) | y] = ((y as u8) << 4) | (x as u8);
                }
            }
        } else if x == 5 && y == 4 {
            // ~x/~y
            for x in 0..8 {
                for y in 0..8 {
                    table[(x << 4) | y] = (((7-x) as u8) << 4) | ((7-y) as u8);
                }
            }
        } else {
            //~x/~y swap
            for x in 0..8 {
                for y in 0..8 {
                    table[(x << 4) | y] = (((7-y) as u8) << 4) | ((7-x) as u8);
                }
            }
        }
        // special
        table[0x88] = 0x88;
        table[0xff] = 0xff;
    }
    fn get(&self, v: u8) -> u8 {
        self.table[v as usize]
    }
}
