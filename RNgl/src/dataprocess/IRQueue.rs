use dataprocess::Process::NUM_QUEUE;

// use std::intrinsics::volatile_copy_memory;
pub struct IRQueue {
    Mindex: usize,
    _r: usize,
    _f: usize,
    //   rq: Vec<usize>,
    _total: usize,
    rq: [usize; NUM_QUEUE],
}


impl IRQueue {
    pub fn new(size: usize) -> IRQueue {
        IRQueue {
            Mindex: size,
            _f: 0,
            _r: 0,
            //  rq: vec![0; size],
            _total: 0,
            rq: [0; NUM_QUEUE],
        }
    }
    pub fn enqueue(&mut self, value: usize) {

        let mut data = &mut self.rq;
        data[self._f] = value;
        //        //println!("enqueue value = {},  self.rq[{}] = {}, _f ={} _r ={} ",
        // value,
        // self._f,
        // data[self._f],
        // self._f,
        // self._r);
        let f = self._f + 1;
        self._f = f % self.Mindex;
        self._total += 1;

    }

    pub fn dequeue(&mut self) -> usize {

        let value = self.rq[self._r];
        //        //println!("dequeue value {} {} r = {} _f ={}",
        // value,
        // self.rq[self._r],
        // self._r,
        // self._f);
        let r = self._r + 1;
        self._r = r % self.Mindex;
        if self._total > 1 {
            self._total -= 1;
        }

        return value;
    }
    pub fn run(&self) -> bool {
        self._total > 1
    }

    //    pub fn fget_r(&self ) -> usize{
    //
    // }
}

// pub fn changl_value(data: & mut [usize],index:usize ,usize value)
// {
// data[index] = value;
// }
