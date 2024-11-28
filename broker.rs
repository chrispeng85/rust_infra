use crate::error::BrokerError
use crate::message::Message;
use crate::subscriber::Subscriber;
use std::collections::HashMap;
use tokio::sync::broadcast;

const DEFAULT_CHANNEL_CAPACITY: usize = 100;

pub struct MessageBroker {
    channels: HashMap<String, broadcast::Sender<Message>>,

}

impl MessageBroker { //implementation of MessageBroker struct

    pub fn new() ->Self {  //constructor method

        MessageBroker {
            channels: HashMap::new(),

        }

    }

    pub fn create_channel(&mut self, name: &str) -> Result<(), BrokerError> {

        if self.channels.contains_key(name) {
            return Ok(());

        }

        let (sender, _) = broadcast::channel(DEFAULT_CHANNEL_CAPACITY);
        self.channels.insert(name.to_string(), sender);
        println!("Created channel: {}", name);
        Ok(())

    }

    pub fn subscribe(&self, channel_name: &str) -> Result<Subscriber, BrokerError> {

        let sender = self.channels.get(channel_name).ok_or_else(|| {
            BrokerError::SendError(format!("Channel not found: {}", channel_name))


        })?;

        Ok(Subscriber::new(
            format!("Subscriber-{}", rand::random::<u32>()),
            sender.subscribe(),
        ))

    }

    pub aysnc fn publish(&self, message:Message) -> Result<(), BrokerError> {

        let sender = self.channels.get(&message.channel).ok_or_else(|| {

                BrokerError::SendError(format!("Channel not found: {}", message.channel))
        })?;

        sender.send(message).map_err( |e| {
            BrokerError::SendError(format!("Failed to send message: {}", e))

        })?;

        Ok(())

    }

    pub fn list_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()

    }


}