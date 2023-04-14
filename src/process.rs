use std::{collections::VecDeque, str::FromStr};

#[derive(Default, Debug, Clone)]
pub struct GranttNode {
    pub start: usize,
    pub end: usize,
    pub pid: usize,
}

#[derive(Default, Debug)]
pub struct SchedulerResult {
    pub total_wait_time: usize,
    pub average_wait_time: f64,
    pub total_turnaround_time: usize,
    pub average_turnaround_time: f64,
    pub grantt_chart: VecDeque<GranttNode>,
}

#[derive(Debug, Clone)]
pub enum ProcessState {
    NotInSytstem,
    Ready,
    Finished,
}

impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::NotInSytstem
    }
}

#[derive(Default, Debug)]
pub struct Process {
    pub pid: usize,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub exit_time: Option<usize>,
    pub priority: usize,
    pub state: ProcessState,
    progress: usize,
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Process {
    pub fn new(pid: usize, arrival_time: usize, burst_time: usize, priority: usize) -> Self {
        Process {
            pid,
            arrival_time,
            priority,
            burst_time,
            exit_time: None,
            progress: 0,
            state: ProcessState::default(),
        }
    }

    pub fn compute_result<'a, I>(
        processes: I,
        mut grantt_chart: VecDeque<GranttNode>,
        minimize_chart: bool,
    ) -> SchedulerResult
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

        let mut result = SchedulerResult {
            total_wait_time,
            average_wait_time: total_wait_time as f64 / len as f64,
            total_turnaround_time,
            average_turnaround_time: total_turnaround_time as f64 / len as f64,
            grantt_chart: VecDeque::default(),
        };

        if minimize_chart && !grantt_chart.is_empty() {
            let mut minimized: VecDeque<GranttNode> = VecDeque::with_capacity(len);
            minimized.push_back(grantt_chart.pop_front().unwrap());

            while !grantt_chart.is_empty() {
                let current = grantt_chart.pop_front().unwrap();
                if minimized.back().unwrap().pid == current.pid {
                    minimized.back_mut().unwrap().end = current.pid;
                } else {
                    minimized.push_back(current);
                }
            }
            result.grantt_chart = minimized;
        } else {
            result.grantt_chart = grantt_chart;
        }

        result
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.state, ProcessState::Finished)
    }

    pub fn is_insystem(&self) -> bool {
        !matches!(self.state, ProcessState::NotInSytstem)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, ProcessState::Ready)
    }

    pub fn turnaround_time(&self) -> Option<usize> {
        Some(self.exit_time? - self.arrival_time)
    }

    pub fn wait_time(&self) -> Option<usize> {
        Some(self.turnaround_time()? - self.burst_time)
    }

    pub fn time_to_complete(&self) -> usize {
        self.burst_time - self.progress
    }

    pub fn run_for(&mut self, time: usize) -> usize {
        self.progress += time;

        if self.progress >= self.burst_time {
            self.state = ProcessState::Finished;
            self.progress = self.burst_time;
        }

        self.progress
    }

    pub fn run_once(&mut self) -> usize {
        self.run_for(1)
    }

    pub fn run_to_completion(&mut self) -> usize {
        self.run_for(self.burst_time)
    }
}

impl FromStr for Process {
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
        let priority = values
            .get(3)
            .map(|v| v.parse::<usize>().unwrap_or(0))
            .unwrap_or(0);

        Ok(Process {
            pid,
            arrival_time,
            burst_time,
            exit_time: None,
            priority,
            state: ProcessState::default(),
            progress: 0,
        })
    }
}

impl From<String> for Process {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}
