mod runtime;

use runtime::{block_on, join, new_log, pause, Log};

fn main() {}

async fn download(name: &str, steps: u32, log: &Log) -> u32 {
    log.borrow_mut().push(format!("start {name}"));
    pause(steps).await;
    log.borrow_mut().push(format!("finish {name}"));
    steps * 100
}

async fn one_after_another(log: &Log) -> u32 {
    let photo = download("photo", 3, log).await;
    let song = download("song", 1, log).await;
    photo + song
}

async fn at_the_same_time(log: &Log) -> u32 {
    let (photo, song) = join(download("photo", 3, log), download("song", 1, log)).await;
    photo + song
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
