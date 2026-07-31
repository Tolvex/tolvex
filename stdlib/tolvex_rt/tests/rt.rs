use std::time::Duration;

use tolvex_rt::{
    alert::{AlertCondition, AlertDispatcher, AlertRule, CollectingSink},
    device::{parse_device_message, DeviceProtocol},
    io::{set_low_latency, tcp_connect_timeout},
    stream::{session_windows, sliding_window_sum, sliding_windows, tumbling_windows, RingBuffer},
};

#[test]
fn ring_buffer_drops_oldest_when_full() {
    let mut rb = RingBuffer::with_capacity(2);
    rb.push(1);
    rb.push(2);
    rb.push(3);

    assert_eq!(rb.dropped(), 1);
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), None);
}

#[test]
fn sliding_window_sum_basic() {
    let xs = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(sliding_window_sum(&xs, 2), vec![3.0, 5.0, 7.0]);
    assert!(sliding_window_sum(&xs, 0).is_empty());
}

#[test]
fn tumbling_windows_partitions_into_fixed_chunks() {
    let xs = vec![1, 2, 3, 4, 5];
    assert_eq!(
        tumbling_windows(&xs, 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
    assert!(tumbling_windows(&xs, 0).is_empty());
}

#[test]
fn sliding_windows_advances_by_step() {
    let xs = vec![1, 2, 3, 4, 5];
    assert_eq!(
        sliding_windows(&xs, 3, 1),
        vec![&xs[0..3], &xs[1..4], &xs[2..5]]
    );
    assert_eq!(sliding_windows(&xs, 2, 2), vec![&xs[0..2], &xs[2..4]]);
    assert!(sliding_windows(&xs, 6, 1).is_empty());
}

#[test]
fn session_windows_splits_on_inactivity_gap() {
    let events = vec![(0u64, "a"), (100, "b"), (2000, "c"), (2050, "d")];
    assert_eq!(
        session_windows(&events, 500),
        vec![vec![(0, "a"), (100, "b")], vec![(2000, "c"), (2050, "d")]]
    );
}

#[test]
fn alert_dispatcher_notifies_on_matching_condition() {
    let rules = vec![AlertRule {
        metric: "hr".to_string(),
        condition: AlertCondition::Above(100.0),
    }];
    let mut dispatcher = AlertDispatcher::new(rules, CollectingSink::default());

    dispatcher.check("hr", 72.0);
    dispatcher.check("hr", 150.0);
    dispatcher.check("bp", 200.0);

    assert_eq!(dispatcher.sink().alerts.len(), 1);
    assert_eq!(dispatcher.sink().alerts[0].value, 150.0);
}

#[cfg(feature = "mqtt")]
#[test]
fn mqtt_adapter_publish_stub_returns_err() {
    use tolvex_rt::device::{DeviceReading, DeviceTransport, MqttAdapter};

    let adapter = MqttAdapter::new("mqtt://localhost:1883", "vitals/hr");
    let reading = DeviceReading::new("dev1", "hr", 72.0, 1000);
    assert!(adapter.connect().is_err());
    assert!(adapter.publish(&reading).is_err());
}

#[cfg(feature = "amqp")]
#[test]
fn amqp_adapter_publish_stub_returns_err() {
    use tolvex_rt::device::{AmqpAdapter, DeviceReading, DeviceTransport};

    let adapter = AmqpAdapter::new("amqp://localhost", "vitals", "hr.reading");
    let reading = DeviceReading::new("dev1", "hr", 72.0, 1000);
    assert!(adapter.connect().is_err());
    assert!(adapter.publish(&reading).is_err());
}

#[test]
fn parse_device_message_csv() {
    let r = parse_device_message(DeviceProtocol::Hl7, "dev1,hr,72.5,1000").expect("reading");
    assert_eq!(r.device_id, "dev1");
    assert_eq!(r.metric, "hr");
    assert_eq!(r.value, 72.5);
    assert_eq!(r.timestamp_ms, 1000);
}

#[test]
fn tcp_helpers_compile_and_are_callable() {
    // We don't require a live server in unit tests; just ensure APIs work.
    // Connecting to localhost:0 should fail quickly.
    let _ = tcp_connect_timeout(("127.0.0.1", 0), Duration::from_millis(10)).err();

    // And we can call set_low_latency on a stream if we had one; here just ensure it typechecks.
    // (No actual stream available without opening a listener.)
    fn _typecheck_only(s: &std::net::TcpStream) {
        let _ = set_low_latency(s);
    }
}
