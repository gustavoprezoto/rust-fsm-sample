use crate::fsm::emv::{Input, Output, TransactionContext};
use crate::vendors::emv::provider::EMVProvider;

pub struct PositivoEMVProvider {}

impl EMVProvider for PositivoEMVProvider {
    fn init() -> Self {
        PositivoEMVProvider {};
    }

    fn process_emv_output(&mut self, output: &Output, ctx: &TransactionContext) {
        match *output {
            Output::AskForCard => {
                println!("Positivo: AskForCard");
                // enviar AskForCard para o front-end alterar a view
            }
            Output::AskForPin => {
                println!("Positivo: AskForPin");
                // enviar o AskForPin para o front-end alterar a view
                // abrir o pinpad
            }
            Output::SendOnline => {
                println!("Positivo: SendOnline");
                // fechar o pinpad
                // enviar a SendOnline para o front-end alterar a view
                // Montar a req. e enviar a transacao para o back-end
            }
            Output::FinishApproved => {
                println!("Positivo: FinishApproved");
                // enviar para o front-end
                // ...
            }
            Output::FinishDeclined => {
                println!("Positivo: FinishDeclined");
                // enviar para o front-end
            }
            Output::FinishCancelled => {
                println!("Positivo: FinishCancelled");
                // enviar para o front-end
            }
            Output::Reset => {
                println!("Positivo: Reset");
                // enviar para o front-end
            }
        }
    }
}