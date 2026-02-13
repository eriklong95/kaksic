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

        /*
         *
         *
         * Your search implementation here:
         *
         *
         */

        // Report results
        let dummy_best_move = position
            .legal_moves()
            .first()
            .expect("No legal moves found")
            .clone();
        let dummy_best_score = 42;

        // It is necessary to send info at least once to En Croissant (the user interface) before outputting best move.
        self.send_info(0, vec![dummy_best_move], dummy_best_score, 1234);

        // Output best move
        self.info_tx
            .send(SearchInfo::BestMove(dummy_best_move))
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
