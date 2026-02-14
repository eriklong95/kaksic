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

        let mut bestmove = position.legal_moves()[0].clone();
        let mut max_score = 0;
        let mut total_nodes = 0;

        for mv in position.legal_moves() {
            let position_clone = position.clone();
            let result_position = position_clone.play(mv).unwrap();
            let (value, nodes) =
                negamax(result_position, _max_depth - 1, crate::search::eval::eval);
            let negated_value = -value;
            if negated_value > max_score {
                max_score = negated_value;
                bestmove = mv;
            }
            total_nodes += nodes;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.
        self.send_info(_max_depth, vec![bestmove], max_score, total_nodes);

        // Output best move
        self.info_tx.send(SearchInfo::BestMove(bestmove)).unwrap();
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
