mod eval;
mod negamax;

use std::i32;

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

        let mut report = Report {
            nodes_visited: 0,
            best_move: None,
        };

        let score = negamax(position, _max_depth, crate::search::eval::eval, &mut report);

        std::thread::sleep(std::time::Duration::from_millis(500));

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.
        self.send_info(
            _max_depth,
            vec![report.best_move.unwrap()],
            score,
            report.nodes_visited,
        );

        // Output best move
        self.info_tx
            .send(SearchInfo::BestMove(report.best_move.unwrap()))
            .unwrap();
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
