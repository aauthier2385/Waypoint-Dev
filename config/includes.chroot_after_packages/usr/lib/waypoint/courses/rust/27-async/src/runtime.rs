//! A tiny async runtime, written just for this lesson.
//!
//! Real projects use a full runtime like tokio, which handles timers, networking,
//! and thousands of tasks. This one only does enough to run the lesson's code.
//! You don't need to read it, but you're welcome to.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A shared record of what happened, in order.
pub type Log = Rc<RefCell<Vec<String>>>;

pub fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Runs a future to completion and gives back its result.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

/// Waits a number of steps, letting other work happen in the meantime.
/// It stands in for slow things like downloads.
pub fn pause(steps: u32) -> Pause {
    Pause { remaining: steps }
}

pub struct Pause {
    remaining: u32,
}

impl Future for Pause {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<()> {
        if self.remaining == 0 {
            Poll::Ready(())
        } else {
            self.remaining -= 1;
            Poll::Pending
        }
    }
}

/// Runs two futures at the same time and gives back both results.
pub async fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
    let mut a = std::pin::pin!(a);
    let mut b = std::pin::pin!(b);
    let mut a_output = None;
    let mut b_output = None;
    std::future::poll_fn(move |context| {
        if a_output.is_none() {
            if let Poll::Ready(output) = a.as_mut().poll(context) {
                a_output = Some(output);
            }
        }
        if b_output.is_none() {
            if let Poll::Ready(output) = b.as_mut().poll(context) {
                b_output = Some(output);
            }
        }
        if a_output.is_some() && b_output.is_some() {
            Poll::Ready((a_output.take().unwrap(), b_output.take().unwrap()))
        } else {
            Poll::Pending
        }
    })
    .await
}

fn noop_waker() -> Waker {
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    fn nothing(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, nothing, nothing, nothing);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}
