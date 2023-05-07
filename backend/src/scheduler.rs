use std::sync::Mutex;

use futures::Future;
use lazy_static::lazy_static;

lazy_static! {
    static ref SCHEDULER: Mutex<clokwerk::AsyncScheduler> = Mutex::new(clokwerk::AsyncScheduler::new());
}

pub fn schedule_job<F, T>(ival: clokwerk::Interval, f: F)
where
    F: 'static + FnMut() -> T + Send,
    T: 'static + Future<Output = ()> + Send,
{
    let mut res = SCHEDULER.lock().unwrap();
    res.every(ival).run(f);
}
