use std::{
    path::Path,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use notify::{Event, RecursiveMode, Watcher};

pub fn watch(path: &str, with_htmx: bool) {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx).expect("Failed to create watcher");

    watcher
        .watch(Path::new(path), RecursiveMode::Recursive)
        .expect("Failed to watch path");

    let debounce_dur = Duration::from_millis(2000);
    let mut last_ev = Instant::now();
    for res in rx {
        match res {
            Ok(val) => {
                handle_change(with_htmx, val, debounce_dur, &mut last_ev);
            }
            Err(_) => {
                println!("No changes detected");
            }
        }
    }
}

fn handle_change(with_htmx: bool, val: Event, debounce_dur: Duration, last_ev: &mut Instant) {
    if last_ev.elapsed() <= debounce_dur {
        return;
    }

    *last_ev = Instant::now();

    if val
        .paths
        .iter()
        .any(|path| path.extension().unwrap() == "go")
    {
        println!("Go file changed");
    }
    if with_htmx
        && val
            .paths
            .iter()
            .any(|path| path.extension().unwrap() == "html")
    {
        println!("HTML file changed");
    }
}
