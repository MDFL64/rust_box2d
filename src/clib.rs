
/*#[no_mangle]
extern "C" fn assert(x: i32) {
    if x==0 {
        panic!("assert failed");
    }
}*/

#[no_mangle]
unsafe extern "C" fn malloc(size: i32) -> i32 {
    let layout = std::alloc::Layout::from_size_align_unchecked(size as usize,0);
    return std::alloc::alloc_zeroed( layout ) as i32;
}

#[no_mangle]
unsafe extern "C" fn free(ptr: i32) {
    let layout = std::alloc::Layout::from_size_align_unchecked(0,0);
    std::alloc::dealloc(ptr as *mut u8, layout);
}

#[no_mangle]
unsafe extern "C" fn _Znwm(size: i32) -> i32 {
    malloc(size)
}

#[no_mangle]
unsafe extern "C" fn _ZdlPv(addr: i32) {
    free(addr);
}

#[no_mangle]
extern "C" fn __cxa_pure_virtual() {
    panic!("pure virtual call");
}

#[no_mangle]
extern "C" fn perf_now() -> f64 {
    return 0.0;
}

extern "C" {
    fn debug_info(x: i32) -> ();
}

/*
#[no_mangle]
unsafe extern "C" fn free(addr: i32) {
    let layout = std::alloc::Layout::from_size_align_unchecked(0,0);
    std::alloc::dealloc(addr as *mut u8, layout);
}
*/
struct B2Pair {
    id_a: i32,
    id_b: i32
}

type PairCompare = fn(*const B2Pair, *const B2Pair) -> bool;

#[no_mangle]
unsafe extern "C" fn box2d_sort_pairs(buffer: *mut B2Pair, count: i32, compare: PairCompare) {
    let array = std::slice::from_raw_parts_mut(buffer,count as usize);
    array.sort_unstable_by(|a,b| {
        let less = compare(a,b);
        if less {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Equal;
    });
    /*for entry in array {
        //debug_info(-2);
        debug_info(entry.id_b + entry.id_a*1000);
        //debug_info(entry.id_b);
    }*/
}
