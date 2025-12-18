pub mod fsm;
pub mod vendors;

#[cfg(all(test, feature = "landi"))]
mod tests {
    use crate::fsm::engine::Engine;
    use crate::fsm::emv::{State, Input, TransactionContext, Output};
    use tokio::sync::mpsc;
    use Output::AskForCard;

    #[tokio::test]
    async fn basic_flow() {
        let (tx, mut rx) = mpsc::channel(32);
        let mut engine = Engine::new(tx);

        let current_state = engine.get_emv_current_state();
        assert_eq!(State::Idle, *current_state);

        let ctx = TransactionContext{amount: 0};
        engine.process_emv_event(&Input::Start, &ctx);

        let current_state = engine.get_emv_current_state();
        assert_eq!(State::WaitingCard, *current_state);

        // Recebendo output da stream
        let output = rx.recv().await.unwrap();
        assert_eq!(output, AskForCard);
    }
}
