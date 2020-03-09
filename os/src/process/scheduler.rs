use super::Tid;
use alloc::{collections::BinaryHeap, vec::Vec};
use core::cmp::{Ordering, Reverse};

pub trait Scheduler {
    fn push(&mut self, tid: Tid);
    fn pop(&mut self) -> Option<Tid>;
    fn tick(&mut self) -> bool;
    fn exit(&mut self, tid: Tid);
    fn set_priority(&mut self, priority: usize);
}

#[derive(Default)]
struct RRInfo {
    valid: bool,
    time: usize,
    prev: usize,
    next: usize,
}

pub struct RRScheduler {
    threads: Vec<RRInfo>,
    max_time: usize,
    current: usize,
}

impl RRScheduler {
    pub fn new(max_time_slice: usize) -> Self {
        let mut rr = RRScheduler {
            threads: Vec::default(),
            max_time: max_time_slice,
            current: 0,
        };
        rr.threads.push(RRInfo {
            valid: false,
            time: 0,
            prev: 0,
            next: 0,
        });
        rr
    }
}

impl Scheduler for RRScheduler {
    fn push(&mut self, tid: Tid) {
        let tid = tid + 1;
        if tid + 1 > self.threads.len() {
            self.threads.resize_with(tid + 1, Default::default);
        }

        if self.threads[tid].time == 0 {
            self.threads[tid].time = self.max_time;
        }

        let prev = self.threads[0].prev;
        self.threads[tid].valid = true;
        self.threads[prev].next = tid;
        self.threads[tid].prev = prev;
        self.threads[0].prev = tid;
        self.threads[tid].next = 0;
    }

    fn pop(&mut self) -> Option<Tid> {
        let ret = self.threads[0].next;
        if ret != 0 {
            let next = self.threads[ret].next;
            let prev = self.threads[ret].prev;
            self.threads[next].prev = prev;
            self.threads[prev].next = next;
            self.threads[ret].prev = 0;
            self.threads[ret].next = 0;
            self.threads[ret].valid = false;
            self.current = ret;
            Some(ret - 1)
        } else {
            None
        }
    }

    // 当前线程的可用时间片 -= 1
    fn tick(&mut self) -> bool {
        let tid = self.current;
        if tid != 0 {
            self.threads[tid].time -= 1;
            if self.threads[tid].time == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }

    fn exit(&mut self, tid: Tid) {
        let tid = tid + 1;
        if self.current == tid {
            self.current = 0;
        }
    }

    fn set_priority(&mut self, priority: usize) {
        // unimplemented!()
    }
}

struct StrideInfo {
    stride: u32,
    priority: u32,
    time: usize,
    present: bool,
}

impl Default for StrideInfo {
    fn default() -> Self {
        Self {
            stride: 0,
            priority: 1,
            time: 0,
            present: false,
        }
    }
}

#[derive(Eq)]
struct StrideNode {
    stride: u32,
    tid: Tid,
}

impl Ord for StrideNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let c: i32 = self.stride.overflowing_sub(other.stride).0 as i32;
        if c < 0 {
            return Ordering::Greater;
        } else if c > 0 {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for StrideNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StrideNode {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride
    }
}

pub struct StrideScheduler {
    threads: Vec<StrideInfo>,
    priority_queue: BinaryHeap<StrideNode>,
    max_time: usize,
    BIG_STRIDE: u32, // (1 << 32) - 1
    current: Tid,
}

impl Scheduler for StrideScheduler {
    fn push(&mut self, tid: Tid) {
        if tid + 1 > self.threads.len() {
            self.threads.resize_with(tid + 1, Default::default);
        }
        let mut thread = &mut self.threads[tid];
        thread.present = true;
        if thread.time == 0 {
            thread.time = self.max_time;
        }
        self.priority_queue.push(StrideNode {
            stride: thread.stride,
            tid: tid,
        });
        // println!("{:#?}", self.priority_queue);
    }

    fn pop(&mut self) -> Option<Tid> {
        let top = self.priority_queue.pop();
        if let Some(node) = top {
            // println!("old: {}, {}", self.current, self.threads[self.current].stride);
            let tid = node.tid;
            if !self.threads[tid].present {
                return self.pop();
            }
            // println!("{:#?}", self.priority_queue.into_sorted_vec());
            let thread = &mut self.threads[tid];
            // let old_stride = thread.stride;
            thread.stride = thread
                .stride
                .overflowing_add(self.BIG_STRIDE / thread.priority)
                .0;
            thread.present = false;
            self.current = tid;
            // println!("{}, {}, {}", self.current, thread.priority, thread.stride);
            // println!("{:#?}", self.priority_queue);

            // println!("priority: {}", thread.priority);
            // println!("old: {}, {}", self.current, old_stride);
            // println!("new: {}, {}", self.current, self.threads[self.current].stride);
            return Some(tid);
        } else {
            return None;
        }
    }

    fn tick(&mut self) -> bool {
        let time = &mut self.threads[self.current].time;
        if *time > 0 {
            *time -= 1;
        }
        *time == 0
    }

    fn exit(&mut self, tid: Tid) {
        self.threads[tid].present = false;
        self.current = 0;
    }

    fn set_priority(&mut self, priority: usize) {
        self.threads[self.current].priority = priority as u32;
    }
}

impl StrideScheduler {
    pub fn new(max_time: usize) -> Self {
        Self {
            threads: Vec::default(),
            priority_queue: BinaryHeap::default(),
            max_time: max_time,
            BIG_STRIDE: 0x7fffffff,
            current: 0,
        }
    }
}
