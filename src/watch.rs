use std::{path::Path, sync::mpsc::channel, time::Duration};

use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};

// Select recommended watcher for debouncer.
// Using a callback here, could also be a channel.
pub fn watch() {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(5), None, tx).unwrap();
    debouncer
        .watcher()
        .watch(Path::new("./src"), RecursiveMode::Recursive)
        .unwrap();

    debouncer
        .cache()
        .add_root(Path::new("./src"), RecursiveMode::Recursive);

    for res in rx {
        match res {
            Err(e) => eprintln!("Error: {:?}", e),
            Ok(val) => {
                println!("Event: {:?}", val[0]);
            }
        }
    }
}
