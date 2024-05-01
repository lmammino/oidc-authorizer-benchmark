use std::{collections::HashMap, fmt::Display};

pub struct Stats {
    requests_sent: usize,
    completed_requests: usize,
    status_codes_count: HashMap<&'static str, usize>,
}

impl Default for Stats {
    fn default() -> Self {
        let mut status_codes_count = HashMap::with_capacity(5);
        status_codes_count.insert("1xx", 0);
        status_codes_count.insert("2xx", 0);
        status_codes_count.insert("3xx", 0);
        status_codes_count.insert("4xx", 0);
        status_codes_count.insert("5xx", 0);
        Stats {
            requests_sent: 0,
            completed_requests: 0,
            status_codes_count,
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Requests sent: {}\nCompleted requests: {}\nStatus codes count: {:?}",
            self.requests_sent, self.completed_requests, self.status_codes_count
        )
    }
}

impl Stats {
    pub fn sent_requests(&self) -> usize {
        self.requests_sent
    }

    pub fn inc_requests_sent(&mut self) {
        self.requests_sent += 1;
    }

    pub fn inc_completed_requests(&mut self) {
        self.completed_requests += 1;
    }

    pub fn inc_status_code(&mut self, status_code: u16) {
        let status_code_category = match status_code {
            100..=199 => "1xx",
            200..=299 => "2xx",
            300..=399 => "3xx",
            400..=499 => "4xx",
            500..=599 => "5xx",
            _ => panic!("Invalid status code"),
        };
        *self
            .status_codes_count
            .get_mut(status_code_category)
            .unwrap() += 1;
    }
}
