use tokio::{task, time::{sleep, Duration}};

pub async fn simulate_io(task_id: usize) {
    println!("Async Task {}: simulando I/O...", task_id);
    sleep(Duration::from_millis(100)).await;
    println!("Async Task {}: tarea completada", task_id);
}

pub async fn simulate_io_many(n_tasks: usize) {
    let mut handles = vec![];

    for i in 0..n_tasks {
        handles.push(task::spawn(simulate_io(i)));
    }

    for h in handles {
        h.await.unwrap();
    }
}

pub async fn compute_pi_async(n_tasks: usize, terms: usize) -> f64 {
    let terms_per_task = terms / n_tasks;
    let mut handles = vec![];

    for i in 0..n_tasks {
        let start = i * terms_per_task;
        let count = terms_per_task;

        let handle = task::spawn(async move {
            leibniz_pi_partial(start, count)
        });

        handles.push(handle);
    }

    let mut sum = 0.0;
    for h in handles {
        sum += h.await.unwrap();
    }

    sum
}

fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>() * 4.0
}
