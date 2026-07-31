#[derive(Debug, Clone, PartialEq)]
pub struct DeviceReading {
    pub device_id: String,
    pub metric: String,
    pub value: f64,
    pub timestamp_ms: u64,
}

impl DeviceReading {
    pub fn new(
        device_id: impl Into<String>,
        metric: impl Into<String>,
        value: f64,
        timestamp_ms: u64,
    ) -> Self {
        Self {
            device_id: device_id.into(),
            metric: metric.into(),
            value,
            timestamp_ms,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceProtocol {
    Hl7,
    Dicom,
    Proprietary(String),
}

pub fn parse_device_message(protocol: DeviceProtocol, msg: &str) -> Option<DeviceReading> {
    match protocol {
        // Minimal placeholder parsing: "device_id,metric,value,timestamp_ms"
        DeviceProtocol::Hl7 | DeviceProtocol::Dicom | DeviceProtocol::Proprietary(_) => {
            let parts: Vec<&str> = msg.split(',').collect();
            if parts.len() != 4 {
                return None;
            }
            let value: f64 = parts[2].parse().ok()?;
            let ts: u64 = parts[3].parse().ok()?;
            Some(DeviceReading::new(parts[0], parts[1], value, ts))
        }
    }
}

/// Common interface for device-protocol client adapters (MQTT, AMQP, ...),
/// so new transports plug into one shape instead of each growing an
/// unrelated `connect`/`publish` pair. Methods take `&mut self` so a real
/// (non-stub) implementation can hold and mutate a live connection handle
/// without needing interior mutability.
pub trait DeviceTransport {
    fn connect(&mut self) -> Result<(), String>;
    fn publish(&mut self, reading: &DeviceReading) -> Result<(), String>;
}

#[cfg(any(feature = "mqtt", feature = "amqp"))]
fn stub_err(protocol: &str, action: &str, detail: impl std::fmt::Display) -> String {
    format!("{protocol} adapter stub: {action} {detail} is not yet implemented")
}

#[cfg(feature = "mqtt")]
#[derive(Debug, Clone)]
pub struct MqttAdapter {
    pub broker_url: String,
    pub topic: String,
}

#[cfg(feature = "mqtt")]
impl MqttAdapter {
    pub fn new(broker_url: impl Into<String>, topic: impl Into<String>) -> Self {
        Self {
            broker_url: broker_url.into(),
            topic: topic.into(),
        }
    }
}

#[cfg(feature = "mqtt")]
impl DeviceTransport for MqttAdapter {
    fn connect(&mut self) -> Result<(), String> {
        Err(stub_err("MQTT", "connecting to", &self.broker_url))
    }

    fn publish(&mut self, _reading: &DeviceReading) -> Result<(), String> {
        Err(stub_err("MQTT", "publish to topic", &self.topic))
    }
}

#[cfg(feature = "amqp")]
#[derive(Debug, Clone)]
pub struct AmqpAdapter {
    pub broker_url: String,
    pub exchange: String,
    pub routing_key: String,
}

#[cfg(feature = "amqp")]
impl AmqpAdapter {
    pub fn new(
        broker_url: impl Into<String>,
        exchange: impl Into<String>,
        routing_key: impl Into<String>,
    ) -> Self {
        Self {
            broker_url: broker_url.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
        }
    }
}

#[cfg(feature = "amqp")]
impl DeviceTransport for AmqpAdapter {
    fn connect(&mut self) -> Result<(), String> {
        Err(stub_err("AMQP", "connecting to", &self.broker_url))
    }

    fn publish(&mut self, _reading: &DeviceReading) -> Result<(), String> {
        Err(stub_err(
            "AMQP",
            "publish to exchange",
            format!("{} (routing key {})", self.exchange, self.routing_key),
        ))
    }
}
