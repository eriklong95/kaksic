mod eval;
mod negamax;

use std::i32;
use std::time::{Duration, Instant};

use crate::search::negamax::{negamax, Report};
use crate::{SearchCommand, SearchControl, SearchInfo};
use crossbeam_channel::{Receiver, Sender};
use shakmaty::{Chess, Move, Position};

/// Executes search tasks.
pub struct Searcher {
    cmd_rx: Receiver<SearchCommand>,
    info_tx: Sender<SearchInfo>,
}

impl Searcher {
    pub fn new(cmd_rx: Receiver<SearchCommand>, info_tx: Sender<SearchInfo>) -> Self {
        Searcher { cmd_rx, info_tx }
    }

    /// Run the searcher
    pub fn run(mut self) {
        loop {
            match self.cmd_rx.recv() {
                Ok(SearchCommand::Start { position, control }) => self.search(position, control),
                Ok(SearchCommand::Stop) => (),
                Ok(SearchCommand::Quit) | Err(_) => break,
            }
        }
    }

    fn search(&mut self, position: Chess, control: SearchControl) {
        // Determine search constraints
        let (_max_depth, _time_limit) = match control {
            SearchControl::ToDepth(depth) => (depth, u64::MAX),
            SearchControl::TimeLimit(time_limit) => (u8::MAX, time_limit),
        };

        let max_duration = std::time::Duration::from_millis(_time_limit);

        let start = std::time::Instant::now();

        let mut search_depth = 1;
        let mut best_move = self.find_best_move(position.clone(), search_depth);
        while search_depth < 4 && Instant::now().duration_since(start).lt(&max_duration) {
            search_depth += 1;
            best_move = self.find_best_move(position.clone(), search_depth);
        }

        if (max_duration - Instant::now().duration_since(start)).gt(&Duration::from_millis(500)) {
            std::thread::sleep(Duration::from_millis(400));
        }

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.

        // Output best move
        self.info_tx.send(SearchInfo::BestMove(best_move)).unwrap();
    }

    fn find_best_move(&self, position: Chess, search_depth: u8) -> Move {
        let mut report = Report { nodes_visited: 0 };
        let mut max_score = i32::MIN;
        let mut best_move: Move = position.legal_moves()[0].clone();

        for mv in position.legal_moves() {
            let result_position = position.clone().play(mv).unwrap();
            let score = -negamax(
                result_position,
                search_depth - 1,
                crate::search::eval::eval,
                &mut report,
            );

            if score > max_score {
                max_score = score;
                best_move = mv;
            }
        }
        self.send_info(
            search_depth,
            vec![best_move],
            max_score,
            report.nodes_visited,
        );

        return best_move;
    }

    fn send_info(&self, depth: u8, pv: Vec<Move>, score: i32, nodes: u64) {
        self.info_tx
            .send(SearchInfo::Info {
                depth,
                pv,
                score,
                nodes,
            })
            .unwrap();
    }
}
