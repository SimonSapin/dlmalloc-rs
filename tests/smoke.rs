#![feature(allocator_api)]

extern crate dlmalloc;
extern crate rand;

use std::heap::{Layout, Alloc, System};

use dlmalloc::Dlmalloc;
use rand::Rng;

#[test]
fn smoke() {
    let mut a = Dlmalloc::new();
    unsafe {
        let layout = Layout::new::<u8>();
        let ptr = a.alloc(layout.clone()).unwrap_or_else(|e| System.oom(e));
        *ptr = 9;
        assert_eq!(*ptr, 9);
        a.dealloc(ptr, layout.clone());

        let ptr = a.alloc(layout.clone()).unwrap_or_else(|e| System.oom(e));
        *ptr = 10;
        assert_eq!(*ptr, 10);
        a.dealloc(ptr, layout.clone());
    }
}

#[test]
fn stress() {
    let mut a = Dlmalloc::new();
    let mut rng = rand::thread_rng();
    let mut ptrs = Vec::new();
    unsafe {
        for _ in 0..1_000_000 {
            let free =
                ptrs.len() > 0 &&
                ((ptrs.len() < 10_000 && rng.gen_weighted_bool(3)) || rng.gen());
            if free {
                let idx = rng.gen_range(0, ptrs.len());
                let (ptr, layout): (_, Layout) = ptrs.swap_remove(idx);
                println!("free({})", layout.size());
                a.dealloc(ptr, layout);
                continue
            }

            let size = if rng.gen() {
                rng.gen_range(1, 128)
            } else {
                // rng.gen_range(1, 128)
                rng.gen_range(1, 128 * 1024)
            };
            let align = if rng.gen_weighted_bool(10) {
                1 << rng.gen_range(3, 8)
            } else {
                8
            };
            let layout = Layout::from_size_align(size, align).unwrap();

            println!("malloc({})", layout.size());
            let ptr = a.alloc(layout.clone()).unwrap_or_else(|e| System.oom(e));
            for i in 0..layout.size() {
                *ptr.offset(i as isize) = 0xce;
            }
            ptrs.push((ptr, layout));
        }
    }
}
