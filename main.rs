mod broker;
mod error;
mod message;
mod subscriber;

use broker::MessageBroker;
use message::Message;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error:Errors>> {

    let mut broker = MessageBroker::new();



    broker.create_channel("news")?;
    broker.create_create_channel("weather")?;

    let news_subscriber1 = broker.subscribe("news")?;
    let news_subscriber2 = broker.subscribe("news")?;
    let weather_subscriber = broker.subscriber("weather")?;

    let news_handle1 = tokio::spawn(news_subscriber1.start());
    let news_handle2 = tokio::spawn(news_subscriber2.start());
    let weather_handle = tokio::spawn(weather_subscriber.start());

    for i in 0..5 {

        let news_msg = Message::new(
            "news".to_string(),
            format!("Breaking news {}!", i)
    
        ).with_metadata("priority", "high");

        let weather_msg = Message::new(
            "weather".to_string(),
            format!("Weather update {}!", i)


        ).with_metadata("location", "New York");

        broker.publish(news_msg).await?;
        broker.publish(weather_msg).await?;

        sleep(Duration::from_secs(1)).await;
    }

    sleep(Duration::from_secs(2)).await;

    println!("Active channels: {:?}", broker.list_channels());

    Ok(());

}