mod runtime;

use runtime::{block_on, join, new_log, pause, Log};

fn main() {
    let log = new_log();
    let total = block_on(one_after_another(&log));
    println!("One after another: {total} bytes, {:?}", log.borrow());

    let log = new_log();
    let total = block_on(at_the_same_time(&log));
    println!("At the same time: {total} bytes, {:?}", log.borrow());
}

/// Logs "start NAME", waits steps steps, logs "finish NAME", and gives back steps * 100.
async fn download(name: &str, steps: u32, log: &Log) -> u32 {
    todo!()
}

/// Downloads "photo" (3 steps), then "song" (1 step). Gives back the total size.
async fn one_after_another(log: &Log) -> u32 {
    todo!()
}

/// The same two downloads, run at the same time with join.
async fn at_the_same_time(log: &Log) -> u32 {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use runtime::block_on;

    #[test]
    fn one_download() {
        let log = new_log();
        assert_eq!(block_on(download("file", 2, &log)), 200);
        assert_eq!(*log.borrow(), vec!["start file", "finish file"]);
    }

    #[test]
    fn in_order() {
        let log = new_log();
        assert_eq!(block_on(one_after_another(&log)), 400);
        assert_eq!(*log.borrow(), vec!["start photo", "finish photo", "start song", "finish song"],
                   "the song only starts after the photo finishes");
    }

    #[test]
    fn together() {
        let log = new_log();
        assert_eq!(block_on(at_the_same_time(&log)), 400);
        assert_eq!(*log.borrow(), vec!["start photo", "start song", "finish song", "finish photo"],
                   "both start right away, and the short song finishes first");
    }
}
