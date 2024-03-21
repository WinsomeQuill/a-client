use tokio::time::Duration;

pub struct Statistics {
    count_success_response: u32,
    max_time_response: u128,
    min_time_response: u128,
    avg_time_response: u128,
    total_time_server_connection: u128,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            count_success_response: 0,
            max_time_response: 0,
            min_time_response: 0,
            avg_time_response: 0,
            total_time_server_connection: 0,
        }
    }
}

impl Statistics {
    pub fn add_success_response(&mut self) {
        self.count_success_response += 1;
    }

    pub fn update_time_stats(&mut self, duration: Duration) {
        let milliseconds = duration.as_millis();

        if self.min_time_response == 0 {
            self.min_time_response = milliseconds;
        }

        if self.min_time_response > milliseconds {
            self.min_time_response = milliseconds;
        }

        if self.max_time_response < milliseconds {
            self.max_time_response = milliseconds;
        }

        self.avg_time_response = (self.max_time_response + self.min_time_response) / 2;
    }

    pub fn update_total_time_connection(&mut self, duration: Duration) {
        self.total_time_server_connection = duration.as_millis();
    }

    pub fn print_report(&self) {
        println!();
        println!("Total success responses: {}", self.count_success_response);
        println!("Max time responses: {} ms", self.max_time_response);
        println!("Min time responses: {} ms", self.min_time_response);
        println!("Avg time responses: {} ms", self.avg_time_response);
        println!("Total time connection: {} ms", self.total_time_server_connection);
        println!();
    }
}