use std::collections::VecDeque;
use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use rusty_machine::linalg::{Matrix, Vector};
use rusty_machine::learning::lin_reg::LinRegressor;

use tree::GameTree;
use othello::board;
use othello::board::{Board, Move};
use othello::strategy::search::Evaluator;

// データ構造:
// FILE:
//   TURN*
// TURN:
//   place (f64) // 石の場所の評価値の係数
//   stable (f64)// 確定石の評価値の係数
//   count (f64) // 置ける場所の評価値の係数

pub fn serialize<W>(tree: GameTree, mut dest: W)
    where W: Write {
    // 60手分の評価を用意
    let mut inputs: Vec<Vec<f64>> = Vec::with_capacity(60);
    let mut targets: Vec<Vec<f64>> = Vec::with_capacity(60);
    for _ in 0..60 {
        inputs.push(Vec::new());
        targets.push(Vec::new());
    }

    let board = board::make_board();
    let mut evaluator = Evaluator::new();

    // 幅優先で木を探索
    let mut dq = VecDeque::new();
    dq.push_back((0, board, Box::new(tree)));

    loop {
        match dq.pop_front() {
            None => {
                break;
            },
            Some((i, board, mut tree)) => {
                // i手目のtreeだ
                evaluator.reset();
                let place = evaluator.eval_place(&board);
                let stable = evaluator.eval_stable(&board);
                let count = evaluator.eval_putnum(&board);
                // inputに追加
                let ip = &mut inputs[i];
                ip.push(place as f64);
                ip.push(stable as f64);
                ip.push(count as f64);
                // 結果も追加
                targets[i].push(tree.score);

                // 次の手を探しに行く
                let keys = tree.play_keys.clone();
                for p in keys.iter() {
                    let v = tree.children.remove(p).unwrap();
                    let mut next = board.clone();
                    let mv =
                        if *p == 0x88 {
                            Move::Pass
                        } else {
                            Move::Put {
                                x: (*p >> 4),
                                y: *p & 0x07,
                            }
                        };
                    next.apply_move(mv).unwrap();
                    dq.push_back((i+1, next, v));
                }
            },
        }
    }
    // 各indexに対して学習を行う
    for i in 0..60 {
        println!("Learning index {} ...", i);
        let input_v = inputs.remove(0);
        let len = input_v.len();
        let input_mat = Matrix::new(len / 3, 3, input_v);

        let target_v = targets.remove(0);
        let target_vec = Vector::new(target_v);

        let mut lin_mod = LinRegressor::default();
        lin_mod.train_with_optimization(&input_mat, &target_vec);

        let params = lin_mod.parameters().unwrap();
        let place = params[0];
        let stable = params[1];
        let count = params[2];
        println!("place  = {}", place);
        println!("stable = {}", place);
        println!("count  = {}", place);

        dest.write_f64::<BigEndian>(place).unwrap();
        dest.write_f64::<BigEndian>(stable).unwrap();
        dest.write_f64::<BigEndian>(count).unwrap();
    }

}
