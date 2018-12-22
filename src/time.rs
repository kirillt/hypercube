pub struct TimeTracker {
    first_tracked: Option<usize>,
    last_tracked: usize,
    time_slept: usize
}

impl TimeTracker {
    pub fn new() -> Self {
        TimeTracker {
            first_tracked: None,
            last_tracked: 0,
            time_slept: 0
        }
    }

    pub fn local_time(&mut self, time: usize, paused: bool) -> usize {
        if paused {
            self.time_slept = match self.first_tracked {
                Some(first_tracked) => time - self.last_tracked - first_tracked,
                None => time - self.last_tracked
            };
            self.last_tracked
        } else {
            let result = match self.first_tracked {
                Some(first_tracked) => {
                    if self.time_slept > 0 {
                        let result = time - first_tracked - self.time_slept;
                        self.first_tracked = Some(first_tracked + self.time_slept);
                        self.time_slept = 0;
                        result
                    } else {
                        time - first_tracked
                    }
                },
                None => {
                    self.first_tracked = Some(time);
                    0
                }
            };
            self.last_tracked = result;
            result
        }
    }
}