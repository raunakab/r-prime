mod components;

use components::fa::FaMsgs;
use components::emg;
use components::fa;
use r_prime::prelude::*;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Txs {
    fa_tx: Sender<FaMsgs>,
}

fn main() {
    let emg = Driver::new(emg::main);
    let mut fa = Driven::new(fa::main);

    let txs = Txs {
        fa_tx: fa.init(),
    };

    let emg_jh = emg.start(txs.clone()).unwrap();
    let fa_jh = fa.start(txs).unwrap();

    emg_jh.join().unwrap().unwrap();
    fa_jh.join().unwrap().unwrap();
}
