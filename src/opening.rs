// serialize a GameTree.
use std::collections::VecDeque;
use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use tree::GameTree;

// データ構造:
// FILE:
//   BLOCK*
//
// BLOCK:
//   block_size: u64
//   PLAY+ // playでsortされている
//
// PLAY: (24 octets)
//   padding: 0u8 * 7
//   play: u8
//   score: f64
//   block_pointer: u64 (0 if no next block)

struct Block {
    addr: u64,
    size: u64,
    plays: Vec<Play>,
}
struct Play {
    play: u8,
    score: f64,
    block_pointer: u64,
}

static PLAY_SIZE: u64 = 24;

pub fn serialize<W>(tree: GameTree, mut dest: W) 
    where W: Write {
    // 最初のブロック
    let block = Block {
        addr: 0,
        size: 0,
        plays: Vec::new(),
    };
    // 幅優先でアレしていく
    let mut dq = VecDeque::new();
    dq.push_back((block, Box::new(tree)));
    let mut writeq = VecDeque::new();

    let mut bid = 1;
    let mut current_addr = 0;
    let mut addr_map = Vec::new();
    addr_map.push(0);

    // まずBlockを作る
    loop {
        match dq.pop_front() {
            None => {
                // emptyなのでおわり
                break;
            },
            Some((mut block, tree)) => {
                let mut cnt = 0;
                // keysはsorted
                let (keys, mut tree) = keys_of(tree);
                for p in keys.iter() {
                    let v = tree.children.remove(p).unwrap();
                    // こいつ用のblockを作る
                    let b = Block {
                        addr: 0,
                        size: 0,
                        plays: Vec::new(),
                    };
                    let score = v.score;
                    dq.push_back((b, v));

                    // この手をあらわすplay
                    let play = Play {
                        play: *p,
                        score,
                        block_pointer: bid,
                    };
                    block.plays.push(play);

                    bid += 1;
                    cnt += 1;
                }
                // playの数が分かったのでsizeを決める
                block.size = 8 + PLAY_SIZE * cnt;
                block.addr = current_addr;

                current_addr += block.size;
                addr_map.push(block.addr);
                
                // 処理済キューへ
                writeq.push_back(block);
            },
        }
    }
    // blockを書き込んでいく
    for (tbid, b) in writeq.iter().enumerate() {
        dest.write_u64::<BigEndian>(b.size).unwrap();
        for p in b.plays.iter() {
            dest.write_u64::<BigEndian>(p.play as u64).unwrap();
            dest.write_f64::<BigEndian>(p.score).unwrap();
            let baddr = addr_map[p.block_pointer as usize];
            dest.write_u64::<BigEndian>(baddr).unwrap();
        }
        if tbid % 1000 == 999 {
            println!("Written {} / {} blocks", tbid+1, bid);
        }
    }
    dest.flush().unwrap();
}

fn keys_of(tree: Box<GameTree>) -> (Vec<u8>, Box<GameTree>) {
    let keys = tree.play_keys.clone();
    return (keys, tree);
}
