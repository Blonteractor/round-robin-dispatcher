use round_robin_dispatcher::process::Process;
use round_robin_dispatcher::{Dispatcher, RoundRobinDispatcher};
use std::io::{self, Write};

fn main() {
    let mut input: Vec<Process> = Vec::new();
    let mut quantum = String::new();

    let mut handle = io::stdout().lock();

    let mut number_of_processes = String::default();
    write!(handle, "Enter number of processes to schedule: ").unwrap();
    handle.flush().unwrap();

    io::stdin()
        .read_line(&mut number_of_processes)
        .expect("Failed to get input");

    println!(
        "Enter processes one by one with the following format: {{pid}} {{arrival_time}} {{burst_time}}"
    );
    for i in 0..number_of_processes
        .trim()
        .parse::<i32>()
        .expect("Invalid value")
    {
        let mut process = String::new();
        write!(handle, "{}:", i + 1).unwrap();
        handle.flush().unwrap();
        io::stdin()
            .read_line(&mut process)
            .expect("Failed to get input");

        input.push(process.into());
    }

    write!(handle, "Enter time quantum: ").unwrap();
    handle.flush().unwrap();
    io::stdin()
        .read_line(&mut quantum)
        .expect("Failed to get input");

    let mut dispatcher = RoundRobinDispatcher::new();
    dispatcher.set_quantum(quantum.trim().parse::<usize>().expect("Invalid value"));

    for process in input {
        dispatcher.add_process(process);
    }

    dispatcher.run();

    dispatcher.print_result();
}

/*
SAMPLE OUTPUT

Enter number of processes to schedule: 4
Enter processes one by one with the following format: {pid} {arrival_time} {burst_time}
1:0 0 10
2:1 1 6
3:2 3 2
4:3 5 4

Process ID           | Arrival Time         | Burst Time           | Exit Time            | Turn Around Time     | Wait Time
1                    | 0                    | 10                   | 22                   | 22                   | 12
2                    | 1                    | 6                    | 16                   | 15                   | 9
3                    | 3                    | 2                    | 8                    | 5                    | 3
4                    | 5                    | 4                    | 18                   | 13                   | 9

Average Turn Around Time: 13.75
Average Wait Time: 8.25
*/
