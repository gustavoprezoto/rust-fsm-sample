use crate::fsm::emv::{Output, TransactionContext};
use crate::vendors::emv::provider::EMVProvider;
use tokio::sync::mpsc::Sender;

pub struct LandiEMVProvider {}

impl EMVProvider for LandiEMVProvider {
    fn init() -> Self {
        // chama as funcoes de inicializacao necessarias pra cada provider
        LandiEMVProvider {}
    }

    fn process_emv_output(&mut self, output: &Output, ctx: &TransactionContext, tx: &Sender<Output>) {
        match *output {
            Output::AskForCard => {
                println!("Landi: AskForCard");

                let tx = tx.clone();
                tokio::spawn(async move {
                    println!("Spawned!");
                    let _ = tx.send(Output::AskForCard).await;
                });
                // enviar AskForCard para o front-end alterar a view
            }
            Output::AskForPin => {
                println!("Landi: AskForPin");
                // enviar o AskForPin para o front-end alterar a view
                // abrir o pinpad
            }
            Output::SendOnline => {
                println!("Landi: SendOnline");
                println!("Autorizando o valor:");
                dbg!(ctx);
                // fechar o pinpad
                // enviar a SendOnline para o front-end alterar a view
                // Montar a req. e enviar a transacao para o back-end
            }
            Output::FinishApproved => {
                println!("Landi: FinishApproved");
                // enviar para o front-end
                // ...
            }
            Output::FinishDeclined => {
                println!("Landi: FinishDeclined");
                // enviar para o front-end
            }
            Output::FinishCancelled => {
                println!("Landi: FinishCancelled");
                // enviar para o front-end
            }
            Output::Reset => {
                println!("Landi: Reset");
                // enviar para o front-end
            }
        }
    }
}