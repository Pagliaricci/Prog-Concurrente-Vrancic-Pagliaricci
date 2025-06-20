use std::{thread, time::Duration};

pub fn simulate_io(task_id: usize) {
    println!("Thread {}: simulando I/O...", task_id);
    thread::sleep(Duration::from_millis(100));
    println!("Thread {}: tarea completada", task_id);
}

pub fn simulate_io_many(n_tasks: usize) {
    let mut handles = vec![];

    for i in 0..n_tasks {
        handles.push(thread::spawn(move || {
            simulate_io(i);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

pub fn compute_pi_parallel(n_tasks: usize, terms: usize) -> f64 {
    let terms_per_task = terms / n_tasks;
    let mut handles = vec![];

    for i in 0..n_tasks {
        let start = i * terms_per_task;
        let count = terms_per_task;

        let handle = thread::spawn(move || {
            leibniz_pi_partial(start, count)
        });

        handles.push(handle);
    }

    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}

fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>() * 4.0
}
