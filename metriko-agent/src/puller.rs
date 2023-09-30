use std::{sync::mpsc::Sender, time::Instant};

use sysinfo::{CpuExt, SystemExt};

pub fn pull_metrics_from_machine(tx: Sender<metriko_common::MetrikoApi>) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    loop {
        let now = Instant::now();
        sys.refresh_memory();
        sys.refresh_cpu();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let avg_cpu_usage = total_cpu_usage / num_cpus as f32;

        let tx_data = tx.send(metriko_common::MetrikoApi::SubmitData {
            collector_id: 0,
            total_memory: total_memory,
            used_memory: used_memory,
            avg_cpu_usage: avg_cpu_usage,
        });
        if let Err(sending_data_fail) = tx_data {
            println!("error while sending data {sending_data_fail:?}")
        }
        let remained_time = now.elapsed().as_secs_f32();
        if remained_time < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - remained_time))
        } else {
            // metric pulling hangs a little we can give it some extra time
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0))
        }
    }
}
