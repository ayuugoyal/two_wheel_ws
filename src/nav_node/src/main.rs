use rclrs::{Context, Node};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Define Position message type
#[derive(Serialize, Deserialize, Clone)]
struct Position {
    x: f64,
    y: f64,
    yaw: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new_default()?;
    let node = Node::new(&context, "rust_navigation_node")?;

    let goal_publisher =
        node.create_publisher::<Position>("goal_position", rclrs::QoSProfile::default())?;
    let current_position = Arc::new(Mutex::new(Position {
        x: 0.0,
        y: 0.0,
        yaw: 0.0,
    }));

    let current_position_clone = Arc::clone(&current_position);
    let _subscription = node.create_subscription::<Position, _>(
        "current_position",
        rclrs::QoSProfile::default(),
        move |msg: Position| {
            let mut pos = current_position_clone.lock().unwrap();
            *pos = msg;
            println!(
                "Received current position: ({}, {}, {})",
                pos.x, pos.y, pos.yaw
            );
        },
    )?;

    let mut count = 0;
    while rclrs::spin_some(&node)? {
        let goal_position = Position {
            x: (count as f64 + 1.0),
            y: (count as f64 + 1.0),
            yaw: 0.0,
        };

        goal_publisher.publish(&goal_position)?;
        println!(
            "Published goal position: ({}, {}, {})",
            goal_position.x, goal_position.y, goal_position.yaw
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        count += 1;
    }

    Ok(())
}
