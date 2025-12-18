use crate::fsm::emv::{Output, TransactionContext};
use tokio::sync::mpsc::Sender;

pub trait EMVProvider {
    fn init() -> Self;
    fn process_emv_output(&mut self, output: &Output, ctx: &TransactionContext, tx: &Sender<Output>);
}
