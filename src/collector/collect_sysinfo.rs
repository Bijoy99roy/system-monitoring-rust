use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, ProcessStatus, ProcessesToUpdate,
    RefreshKind, System,
};
use tokio::time::interval;

use crate::models::models::ProcessSnapshot;

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn collect_snapshot(sys: &System) -> Vec<ProcessSnapshot> {
    let ts = now();

    let mut processes: Vec<ProcessSnapshot> = sys
        .processes()
        .values()
        .map(|p| ProcessSnapshot {
            pid: p.pid().as_u32(),
            name: p.name().to_string_lossy().into_owned(),
            cpu_usage: p.cpu_usage(),
            memory_bytes: p.memory(),
            memory_mb: p.memory() as f64 / 1048576.0,
            status: p.status().to_string(),
            timestamp: ts,
        })
        .collect();

    processes.sort_by(|a, b| {
        b.memory_bytes
            .partial_cmp(&a.memory_bytes)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    processes
}

pub async fn run() {
    let mut sys = System::new_with_specifics(
        RefreshKind::everything()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    sys.refresh_all();

    tokio::time::sleep(Duration::from_millis(200)).await;

    let mut tick = interval(Duration::from_secs(1));

    loop {
        tick.tick().await;

        sys.refresh_processes(ProcessesToUpdate::All, true);
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let snap = collect_snapshot(&sys);

        for p in &snap {
            println!("Process: {:?}", p);
        }
    }
}
