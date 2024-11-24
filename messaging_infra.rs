use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use serde::{Serialize, Deserialize}
use thiserror::Error;

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

#[derive(Debug, Error)]

pub enum MessageingError {

    #[error("Channel not found: {0}")]
    ChannelNotFound(String),
    #[error("Channel already exists: {0}")]
    ChannelExists(String),
    #[error("Failed to send message: {0}")]
    SendError(String),
    #[error("Failed to receive message: {0}")]
    ReceiveError(String),

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {

    pub channel: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,

}


impl Message {

    pub fn new(channel: String, content: String) -> Self {
        Message {

            channel,
            content,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),

        }


    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {

        self.metadata.insert(key.to_string(), value.to_string());
        self

    }
}

#[derive(Clone)]
pub struct Channel {

    name: String,
    sender: Sender<Message>,

}

pub struct MessageBroker {

    channels: Arc<Mutex<HashMap<String, Channel>>>

}

impl MessageBroker {

    pub fn new() -> Self {
        MessageBroker {
            channels: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn create_channel(&self, name: &str) -> Result<(), MessagingError> {
        let mut channels = self.channels.lock().unwrap();

        if channels.contains_key(name) {
            return Err(MessagingError:ChannelExists(name.to_string()));
            
        }

        let (sender, _) = broadcast::channel(DEFAULT_CHANNEL_CAPACITY);
        let channel = Channel {

                name: name.to_string(),
                sender,
        };

        channels.insert(name.to_string(), channel)
        Ok(())

    }

    pub fn subscribe(&self, channel_name: &str) ->Result<Receiver<Message>, MesssagingError> {

        let channels = self.channels.lock().unwrap();

        if let Some(channel) = channels.get(channel_name) {
            Ok(channel.sender.subscribe())
        
        }

        else {
            Err(MessagingError::ChannelNotFound(channel_name.to_string()))

        }
    }

    pub fn publish(&self, message: Message) -> Result <(), MessagingError> {
        let channels = self.channels.lock().unwrap();

        if let Some(channel) = channels.get(&message.channel) {
            channel.sender.send(message).map_err(|e| {

                MessagingError::SendError(e.to_string)

            })?;
            Ok(())
            
        }

        else {
            Err(MessagingError::ChannelNotFound(message.channel.clone()))

        }


    }

    pub fn list_channels(&self) -> Vec<String> {
        let channels = self.channels.lock().unwrap();
        channels.keys().cloned().collect()
    }
}


pub struct Subsriber {

    name: String,
    receiver: Receiver<Message>,
}

impl Subscriber {

    pub fn new(name: String, receiver: Receiver<Message>) -> Self {
        Subscriber {name, receiver}
    }

    pub async fn start(mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            println!(
                "Subscriber {} received message on channel {}: {}",
                self.name, msg.channel, msg.content

            );


        }


    }
 
}

#[cfg(test)]
mod tests {

    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_message_broker() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let broker = MessageBroker::new();


            broker.create_channel("test").unwrap();
            let subscriber = Subscriber::new("test_subscriber".to_string(), receiver);

            let handle = tokio::spawn(subscriber.start());

            let message = Message::new(
                "test".to_string(),
                "Hello, World!".to_string()
            
            ).with_metadata("priority", "high");

            broker.publish(message).unwrap();

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await();

            drop(broker);
            handle.abort();

        });

    }

}





