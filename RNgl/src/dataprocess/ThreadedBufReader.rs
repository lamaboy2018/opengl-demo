// this version with reader doing compact
// writer can not do compact as it's not safe
use std::cell::UnsafeCell;
use std::ptr;
use std::marker::{Sync};
use std::io::{Read, Result};
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize};
use std::sync::atomic::Ordering::{SeqCst, Acquire, Release};

static DEFAULT_BUFFER_SIZE : usize = 64 * 1024 * 1024;

pub struct Reader<R: Read> {
    tbr: Arc<ThreadedBufReader<R>>
}

pub struct Writer<R: Read> {
    tbr: Arc<ThreadedBufReader<R>>
}

pub struct ThreadedBufReader<R: Read> {
    reader: UnsafeCell<R>,
    buf: UnsafeCell<Vec<u8>>,
    rpos: AtomicUsize,
    wpos: AtomicUsize,
}

unsafe impl <R: Read> Sync for ThreadedBufReader<R> {}

impl <R: Read> ThreadedBufReader<R> {
    pub fn with_capacity(r: R, cap: usize) -> (Reader<R>, Writer<R>) {
        let tbr = Arc::new(ThreadedBufReader {
            reader: UnsafeCell::new(r),
            rpos: AtomicUsize::new(0),
            wpos: AtomicUsize::new(0),
            buf: UnsafeCell::new(vec![0; cap]),
        });

        (
            Reader {tbr: tbr.clone()},
            Writer {tbr: tbr.clone()}
        )
    }

    pub fn new(r: R) -> (Reader<R>, Writer<R>) {
        ThreadedBufReader::with_capacity(r, DEFAULT_BUFFER_SIZE)
    }
}

impl<R: Read> Reader<R> {
    pub fn compact(&self) {
        let tbr = &self.tbr;
        let buf = unsafe { &mut *tbr.buf.get() };
        let wpos = tbr.wpos.load(Acquire);

        if wpos != buf.len() {
            return;
        }

        let rpos = tbr.rpos.load(Acquire);

        let rem = wpos - rpos;
        if rem > 0 {
            //println!("NON ZERO");
            let not_read_ptr = buf[rpos..wpos].as_ptr();
            unsafe { ptr::copy(not_read_ptr, buf[0..].as_mut_ptr(), rem); }
        }

        tbr.rpos.store(0, Release);
        // wpos store has to be last
        tbr.wpos.store(rem, Release);
    }

    pub fn read(&self) -> &[u8] {
        let tbr = &self.tbr;
        let rpos = tbr.rpos.load(Acquire);
        let wpos = tbr.wpos.load(Acquire);
        let src = unsafe {& *tbr.buf.get()};
        &src[rpos..wpos]
    }

    pub fn consume_local(&self, amt: usize) {
        let tbr = &self.tbr;
        let wpos = tbr.wpos.load(Acquire);
        let old_pos = tbr.rpos.fetch_add(amt, SeqCst);
        //println!("Consuming: {:?} wpos: {:?} rpos: {:?}", amt, wpos, (old_pos + amt));
        if (old_pos + amt) > wpos {
            panic!("Invalid amount {}", amt);
        }
    }
}

impl<R: Read> Writer<R> {

    pub fn fill_buf_local(&self) -> Result<usize> {
        let tbr = &self.tbr;
        let reader = unsafe { &mut *tbr.reader.get() };
        let buf = unsafe { &mut *tbr.buf.get() };
        let wpos = tbr.wpos.load(Acquire);
        // if below is important
        // because we could get interrupted meawhile reader would compact
        // and writer position will be different, so we can not unconditionally
        // do else and store new wpos
        if wpos == buf.len() {
            Ok(0)
        } else {
            let new_read = try!(reader.read(&mut buf[wpos..]));
            tbr.wpos.store(wpos + new_read, Release);
            //println!("Fill buffer wpos: {:?} new_read: {:?}", wpos, new_read);
            Ok(new_read)
        }
    }
}