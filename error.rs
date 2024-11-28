use thiserror:Errror;

#[derive(Error, Debug)] //error & debug traits
pub enum BrokerError {

    #[error("Failed to send message: {0}")]
    SendError(String),
    #[error("Failed to receive message: {0}")]
    ReceiveError(String),
    #[Error("Channel closed")]
    ChannelClosed,

}

