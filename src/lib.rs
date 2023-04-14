pub mod process;
use std::collections::VecDeque;

use process::{GranttNode, Process, ProcessState, SchedulerResult};

pub trait Dispatcher {
    fn add_process(&mut self, process: Process);
    fn run(&mut self);
}

pub struct RoundRobinDispatcher {
    queue: VecDeque<Process>,
    quantum: usize,
    result: Option<SchedulerResult>,
}

impl RoundRobinDispatcher {
    pub fn new() -> RoundRobinDispatcher {
        RoundRobinDispatcher {
            queue: VecDeque::new(),
            quantum: 2,
            result: None,
        }
    }

    pub fn set_quantum(&mut self, quantum: usize) {
        self.quantum = quantum;
    }

    pub fn print_result(&self) {
        if let Some(result) = self.result.as_ref() {
            println!(
                "{0: <20} | {1: <20} | {2: <20} | {3: <20} | {4: <20} | {5: <20}",
                "Process ID",
                "Arrival Time",
                "Burst Time",
                "Exit Time",
                "Turn Around Time",
                "Wait Time"
            );

            for process in &self.queue {
                println!(
                    "{0: <20} | {1: <20} | {2: <20} | {3: <20} | {4: <20} | {5: <20}",
                    process.pid + 1,
                    process.arrival_time,
                    process.burst_time,
                    process.exit_time.unwrap(),
                    process.turnaround_time().unwrap(),
                    process.wait_time().unwrap()
                );
            }

            println!(
                "\n\nAverage Turn Around Time: {}\nAverage Wait Time: {}",
                result.average_turnaround_time, result.average_wait_time
            );
        } else {
            panic!("Use run() to run the processes first");
        }
    }
}

impl Dispatcher for RoundRobinDispatcher {
    fn add_process(&mut self, process: Process) {
        self.queue.push_back(process);
    }

    fn run(&mut self) {
        let mut tick = 0;
        let mut last_unfinished_process_id: Option<usize> = None;
        let mut waiting_queue: VecDeque<usize> = VecDeque::with_capacity(self.queue.len());
        let mut process_to_run;
        let mut grantt_chart = VecDeque::with_capacity(self.queue.len());

        while !self.queue.iter().all(|p| p.is_finished()) {
            waiting_queue.extend(
                self.queue
                    .iter_mut()
                    .filter(|p| {
                        p.arrival_time <= tick
                            && !p.is_insystem()
                            && last_unfinished_process_id.unwrap_or(usize::MAX) != p.pid
                    })
                    .map(|p| {
                        p.state = ProcessState::Ready;
                        p.pid
                    }),
            );

            if let Some(lup) = last_unfinished_process_id {
                waiting_queue.push_back(lup);
                last_unfinished_process_id = None;
            }

            process_to_run = &mut self.queue[waiting_queue.pop_front().unwrap()];
            process_to_run.state = ProcessState::NotInSytstem;
            let mut node = GranttNode::default();
            node.pid = process_to_run.pid;
            node.start = tick;

            process_to_run.run_for(self.quantum);
            tick += self.quantum;

            node.end = tick;
            grantt_chart.push_back(node);

            if process_to_run.is_finished() {
                process_to_run.exit_time = Some(tick);
            } else {
                process_to_run.state = ProcessState::Ready;
                last_unfinished_process_id = Some(process_to_run.pid);
            }
        }

        self.result = Some(Process::compute_result(
            self.queue.iter(),
            grantt_chart,
            false,
        ))
    }
}
