pub struct SchedulerResult {
    pub total_wait_time: usize,
    pub average_wait_time: f64,
    pub total_turnaround_time: usize,
    pub average_turnaround_time: f64,
}

pub enum ProcessState {
    NotInSytstem,
    Ready,
    Finished,
}

pub struct Process {
    pub pid: usize,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub exit_time: Option<usize>,
    pub state: ProcessState,
    pub progress: usize,
}

impl Process {
    pub fn new(pid: usize, arrival_time: usize, burst_time: usize) -> Self {
        Process {
            pid,
            arrival_time,
            burst_time,
            exit_time: None,
            progress: 0,
            state: ProcessState::NotInSytstem,
        }
    }

    pub fn compute_result<'a, I>(processes: I) -> SchedulerResult
    where
        I: IntoIterator<Item = &'a Process>,
    {
        let mut total_turnaround_time = 0;
        let mut total_wait_time = 0;

        let mut len = 0;
        for process in processes.into_iter() {
            len += 1;
            total_turnaround_time += process.turnaround_time().unwrap();
            total_wait_time += process.wait_time().unwrap();
        }

        let result = SchedulerResult {
            total_wait_time,
            average_wait_time: total_wait_time as f64 / len as f64,
            total_turnaround_time,
            average_turnaround_time: total_turnaround_time as f64 / len as f64,
        };

        result
    }

    pub fn turnaround_time(&self) -> Option<usize> {
        Some(self.exit_time? - self.arrival_time)
    }

    pub fn wait_time(&self) -> Option<usize> {
        Some(self.turnaround_time()? - self.burst_time)
    }
}

impl std::str::FromStr for Process {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.trim().split_whitespace().collect();
        if values.len() < 3 {
            return Err("Process string should contain at least 3 values".to_string());
        }

        let pid = values[0]
            .parse::<usize>()
            .map_err(|_| "Invalid pid".to_string())?;
        let arrival_time = values[1]
            .parse::<usize>()
            .map_err(|_| "Invalid arrival_time".to_string())?;
        let burst_time = values[2]
            .parse::<usize>()
            .map_err(|_| "Invalid burst_time".to_string())?;
        Ok(Process {
            pid,
            arrival_time,
            burst_time,
            exit_time: None,
            state: ProcessState::NotInSytstem,
            progress: 0,
        })
    }
}

impl From<String> for Process {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}
