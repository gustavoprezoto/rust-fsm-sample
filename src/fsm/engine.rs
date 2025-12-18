use crate::fsm::emv::emv_fsm::StateMachine as EMVStateMachine;
use crate::fsm::emv::{Input as EMVInput, Output, State, TransactionContext as EMVTransactionContext, TransactionContext};
use tokio::sync::mpsc;

use crate::vendors::emv::provider::EMVProvider;

#[cfg(feature = "landi")]
use crate::vendors::emv::landi;

#[cfg(feature = "positivo")]
use crate::vendors::emv::positivo;

#[cfg(feature = "landi")]
pub type ActiveEMVProvider = landi::provider::LandiEMVProvider;

#[cfg(feature = "positivo")]
pub type ActiveEmvProvider = positivo::provider::PositivoEMVProvider;

pub struct Engine {
    emv_engine: EMVStateMachine,
    emv_vendor: ActiveEMVProvider,

    tx: mpsc::Sender<Output>,
}

impl Engine {
    pub fn new(tx: mpsc::Sender<Output>) -> Self {
        let emv_engine = EMVStateMachine::new();
        let emv_vendor = ActiveEMVProvider::init();

        Engine {
            emv_engine,
            emv_vendor,
            tx
        }
    }

    // Funcao exposta para o flutter.
    pub fn get_emv_current_state(&self) -> &State {
        self.emv_engine.state()
    }

    // Funcao exposta para o flutter.
    pub fn process_emv_event(&mut self, input: &EMVInput, ctx: &EMVTransactionContext) {
        let result = self.emv_engine.consume(input);

        match result {
            Ok(output) => {
                self.process_emv_output(output, ctx);
            }
            Err(_err) => {
                println!("failed EMV input")
            }
        }
    }

    fn process_emv_output(&mut self, output: Option<Output>, ctx: &TransactionContext) {
        match output {
            Some(output) => {
                self.emv_vendor.process_emv_output(&output, ctx, &self.tx);
            }
            None => {}
        }
    }
}
