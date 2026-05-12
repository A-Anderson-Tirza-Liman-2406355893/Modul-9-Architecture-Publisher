use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

#[tokio::main]
async fn main() {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ");

    let channel = conn
        .create_channel()
        .await
        .expect("Failed to create channel");

    let _queue = channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    let messages = vec![
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406355893-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406355893-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406355893-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406355893-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406355893-Emir".to_owned(),
        },
    ];

    for message in messages {
        let payload = serde_json::to_vec(&message).expect("Failed to serialize message");
        
        let _confirm = channel
            .basic_publish(
                "",
                "user_created",
                BasicPublishOptions::default(),
                &payload,
                Default::default(),
            )
            .await
            .expect("Failed to publish message");

        println!("Published message: {:?}", message);
    }

    println!("All messages published successfully!");
}