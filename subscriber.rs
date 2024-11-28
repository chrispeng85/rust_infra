use crate::error::BrokerError;
use crate::message::Message;
use tokio::sync::broadcast;

pub struct Subscriber { //public struct

    name: String,
    receiver: broadcast::Receiver<Message>,

}

impl Subscriber {

    pub fn new(name: String, receiver: broadcast::Receiver<Message>) -> Self { //static constructor method

        Subscriber { name, receiver} 

    }

    pub async fn start(mut self) -> Result <(), BrokerError {
        println!("Subscriebr {} started", self.name);

        while let Ok(msg) = self.receiver.recv().await {
            println!(
                "Subscriber {} started", self.name);

            while let Ok(msg) = self.receiver.recv().await {
                println! (
                    "Subscriber {} received message on channel {}: {}",
                    self.name, msg.channel, msg.content

                );

                for (key, value) in &msg.metadata {

                    println(" Metadata - {}: {}", key, value);
                }

            }

            Ok(())
        }
    }
}