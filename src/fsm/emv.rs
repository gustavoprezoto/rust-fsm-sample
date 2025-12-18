use rust_fsm::*;

#[derive(Debug, Clone, Copy)]
pub enum Input {
    Start,
    Cancel,

    CardInserted,
    CardTapped,
    CardSwiped,

    PinEntered,

    OnlineApproved,
    OnlineDeclined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Idle,
    WaitingCard,
    WaitingPin,
    OnlineProcessing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Output {
    AskForCard,
    AskForPin,
    SendOnline,
    FinishApproved,
    FinishDeclined,
    FinishCancelled,
    Reset
}

#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub amount: u64,
}

state_machine! {
    #[state_machine(
        input(crate::fsm::emv::Input),
        state(crate::fsm::emv::State),
        output(crate::fsm::emv::Output)
    )]
    pub emv_fsm(Idle)

    Idle(Start) => WaitingCard [AskForCard],

    WaitingCard => {
        CardInserted => WaitingPin [AskForPin],
        CardTapped   => WaitingPin [AskForPin],
        CardSwiped   => WaitingPin [AskForPin],

        Cancel => Cancelled [FinishCancelled],
    },

    WaitingPin => {
        PinEntered => OnlineProcessing [SendOnline],

        Cancel => Cancelled [FinishCancelled],
    },

    OnlineProcessing => {
        OnlineApproved => Completed [FinishApproved],
        OnlineDeclined => Failed [FinishDeclined],
    },

    Completed => {
        Start => Idle [Reset],
        Cancel => Idle [Reset],
    },

    Failed => {
        Start => Idle [Reset],
        Cancel => Idle [Reset],
    }
}

