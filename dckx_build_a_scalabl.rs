use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::task;
use std::collections::HashMap;
use std::error::Error;

// IoT device structure
struct IoTDevice {
    id: String,
    name: String,
}

// IoT device notifier
struct IoTNotifier {
    devices: HashMap<String, IoTDevice>,
    tx: mpsc::Sender<String>,
}

impl IoTNotifier {
    fn new() -> IoTNotifier {
        let (tx, _) = mpsc::channel(10);
        IoTNotifier {
            devices: HashMap::new(),
            tx,
        }
    }

    fn add_device(&mut self, device: IoTDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    fn notify(&self, device_id: &str, message: &str) -> Result<(), Box<dyn Error>> {
        if let Some(device) = self.devices.get(device_id) {
            println!("Notifying {} ({}) - {}", device.name, device_id, message);
            self.tx.send(message.to_string())?;
            Ok(())
        } else {
            Err("Device not found".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut notifier = IoTNotifier::new();

    // Add some devices
    notifier.add_device(IoTDevice { id: "dev1".to_string(), name: "Temperature Sensor".to_string() });
    notifier.add_device(IoTDevice { id: "dev2".to_string(), name: "Humidity Sensor".to_string() });

    // Notify devices
    notifier.notify("dev1", "Temperature reached 25°C")?;
    notifier.notify("dev2", "Humidity level is 60%")?;

    // Consume notifications
    task::spawn(async move {
        while let Some(message) = notifier.tx.recv().await {
            println!("Received notification: {}", message);
        }
    });

    Ok(())
}