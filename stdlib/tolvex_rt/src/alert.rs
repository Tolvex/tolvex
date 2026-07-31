use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertCondition {
    Above(f64),
    Below(f64),
    OutsideRange(f64, f64),
}

impl AlertCondition {
    pub fn matches(&self, value: f64) -> bool {
        match self {
            AlertCondition::Above(threshold) => value >= *threshold,
            AlertCondition::Below(threshold) => value <= *threshold,
            AlertCondition::OutsideRange(lo, hi) => value < *lo || value > *hi,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlertRule {
    pub metric: String,
    pub condition: AlertCondition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub metric: String,
    pub value: f64,
    pub message: String,
}

pub trait NotificationSink {
    fn notify(&mut self, alert: &Alert);
}

#[derive(Debug, Default)]
pub struct CollectingSink {
    pub alerts: Vec<Alert>,
}

impl NotificationSink for CollectingSink {
    fn notify(&mut self, alert: &Alert) {
        self.alerts.push(alert.clone());
    }
}

pub struct AlertDispatcher<S: NotificationSink> {
    rules_by_metric: HashMap<String, Vec<AlertCondition>>,
    sink: S,
}

impl<S: NotificationSink> AlertDispatcher<S> {
    pub fn new(rules: Vec<AlertRule>, sink: S) -> Self {
        let mut rules_by_metric: HashMap<String, Vec<AlertCondition>> = HashMap::new();
        for rule in rules {
            rules_by_metric
                .entry(rule.metric)
                .or_default()
                .push(rule.condition);
        }
        Self {
            rules_by_metric,
            sink,
        }
    }

    pub fn check(&mut self, metric: &str, value: f64) {
        let Some(conditions) = self.rules_by_metric.get(metric) else {
            return;
        };
        for condition in conditions {
            if condition.matches(value) {
                self.sink.notify(&Alert {
                    metric: metric.to_string(),
                    value,
                    message: format!("{metric} triggered alert: {value}"),
                });
            }
        }
    }

    pub fn sink(&self) -> &S {
        &self.sink
    }
}
