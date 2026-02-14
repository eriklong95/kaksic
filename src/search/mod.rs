mod eval;
mod negamax;

use crate::search::negamax::negamax;
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

        // which of the legal moves should we take?

        let mut best_move = position.legal_moves()[0].clone();
        let mut max_value = 0;

        for mv in position.legal_moves() {
            let position_clone = position.clone();
            let result_position = position_clone.play(mv).unwrap();
            let value = -negamax(result_position, 3, crate::search::eval::eval);
            if value > max_value {
                max_value = value;
                best_move = mv;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.
        self.send_info(0, vec![best_move], max_value, 1234);

        // Output best move
        self.info_tx.send(SearchInfo::BestMove(best_move)).unwrap();
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
