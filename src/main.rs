use round_robin_dispatcher::process::Process;
use round_robin_dispatcher::{Dispatcher, RoundRobinDispatcher};

fn main() {
    let input = vec![
        Process::new(0, 0, 10, 2),
        Process::new(1, 1, 6, 5),
        Process::new(2, 3, 2, 3),
        Process::new(3, 5, 4, 1),
    ];

    let mut dispatcher = RoundRobinDispatcher::new();
    dispatcher.set_quantum(2);

    for process in input {
        dispatcher.add_process(process);
    }

    dispatcher.run();

    dispatcher.print_result();
}
