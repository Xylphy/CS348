use rust::read_input;

#[derive(Clone, Copy)]
struct Process {
    pid: usize,
    arrival: usize,
    burst: usize,
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let num_processes: usize = read_input("Enter the number of processes: ");
    let mut processes: Vec<Process> = Vec::new();

    for i in 0..num_processes {
        let arrival: usize = read_input(&format!("Enter arrival time for process {i}: "));
        let burst: usize = read_input(&format!("Enter burst time for process {i}: "));
        processes.push(Process {
            pid: i,
            arrival,
            burst,
        });
    }

    processes.sort_by_key(|p| p.arrival);

    let mut start_times: Vec<Option<usize>> = vec![None; num_processes];
    let mut finish_times: Vec<usize> = vec![0; num_processes];
    let mut turnaround_times: Vec<usize> = vec![0; num_processes];
    let mut waiting_times: Vec<usize> = vec![0; num_processes];
    let mut response_times: Vec<usize> = vec![0; num_processes];

    let mut remaining_times: Vec<usize> = processes.iter().map(|p| p.burst).collect();
    let mut is_process_completed: Vec<bool> = vec![false; num_processes];

    let mut time: usize = 0;
    let mut completed: usize = 0;
    let mut last_pid: Option<usize> = None;

    while completed < num_processes {
        let mut idx: Option<usize> = None;

        for i in 0..num_processes {
            if processes[i].arrival <= time && !is_process_completed[i] {
                idx = idx.map_or(Some(i), |j| {
                    let left = (remaining_times[i], processes[i].arrival, processes[i].pid);
                    let right = (remaining_times[j], processes[j].arrival, processes[j].pid);
                    if left < right { Some(i) } else { Some(j) }
                });
            }
        }

        if let Some(i) = idx {
            if last_pid != Some(processes[i].pid) {
                match last_pid {
                    Some(prev) => println!("t={time}: switch P{prev} -> P{}", processes[i].pid),
                    None => println!("t={time}: start P{}", processes[i].pid),
                }
                last_pid = Some(processes[i].pid);
            }

            if start_times[i].is_none() {
                start_times[i] = Some(time);
                response_times[i] = time - processes[i].arrival;
            }

            // Preemptive SJF (SRTF): run exactly one time unit, then reschedule.
            remaining_times[i] -= 1;
            time += 1;

            if remaining_times[i] == 0 {
                is_process_completed[i] = true;
                completed += 1;
                finish_times[i] = time;
                turnaround_times[i] = finish_times[i] - processes[i].arrival;
                waiting_times[i] = turnaround_times[i] - processes[i].burst;
            }
        } else {
            if last_pid.is_some() {
                println!("t={time}: CPU idle");
                last_pid = None;
            }
            time += 1;
        }
    }

    println!(
        "Process\tArrival Time\tBurst Time\tStart Time\tFinish Time\tTurnaround Time\tWaiting Time\tResponse Time"
    );
    for i in 0..num_processes {
        println!(
            "{}\t{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}",
            processes[i].pid,
            processes[i].arrival,
            processes[i].burst,
            start_times[i].unwrap_or(0),
            finish_times[i],
            turnaround_times[i],
            waiting_times[i],
            response_times[i]
        );
    }

    println!(
        "Average Turnaround Time: {:.2}",
        turnaround_times.iter().sum::<usize>() as f64 / num_processes as f64
    );
    println!(
        "Average Waiting Time: {:.2}",
        waiting_times.iter().sum::<usize>() as f64 / num_processes as f64
    );
    println!(
        "Average Response Time: {:.2}",
        response_times.iter().sum::<usize>() as f64 / num_processes as f64
    );
}
