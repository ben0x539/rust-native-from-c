#![crate_type = "dylib"] // could also compile with --crate-type=dylib instead

extern crate native;

use std::io::timer::sleep;
use std::cast::transmute;
use std::libc::c_int;

// just some simple toy feature that exercises the rust runtime
pub struct Counter(Receiver<int>);

impl Counter {
    pub fn new(count: int) -> Counter {
        let (tx, rx) = channel::<int>();

        spawn(proc() {
            for i in range(0, count) {
                sleep(1000);
                tx.send(i);
            }
        });

        return Counter(rx);
    }

    pub fn wait(&self) -> Option<int> { 
        let Counter(ref rx) = *self;
        return rx.recv_opt();
    }
}

// Exported C-ABI wrapper functions for the above
//
// Oh no: https://github.com/mozilla/rust/issues/10025 means we can't
// currently declare *exported* extern "C" functions as unsafe because
// *imported* extern "C" functions are always unsafe.
//
// So they're declared a safe functions here even though they do unsafe
// things...

// no_mangle is necessary to avoid weird mangled symbol names with hashes and
// version numbers and that sort of thing in them
#[no_mangle]
pub extern "C" fn counter_new(count: c_int) -> *mut Counter {
    unsafe {
        // being a bit unnecessarily verbose:
        // first create our Counter object, stored in a simple ~-allocation.
        let counter: ~Counter = ~Counter::new(count as int);
        // ... then transmute the ~-ptr to a C-style pointer as an unsafe,
        // unchecked cast.
        // This "consumes" the ~-ptr and unsafely inhibits its automatic
        // cleanup. It's only okay because a ~-ptr and a C-style pointer have
        // the same representation, and we do the inverse later.
        let raw_counter: *mut Counter = transmute(counter);
        return raw_counter;
    }
}

#[no_mangle]
pub extern "C" fn counter_wait(raw_counter: *mut Counter) -> c_int {
    unsafe {
        // deref the C-style pointer, call method on rust object, translate
        // Option<int> to a simple int by substituting -1 for a missing value.
        (*raw_counter).wait().unwrap_or(-1) as c_int
    }
}

#[no_mangle]
pub extern "C" fn counter_free(raw_counter: *mut Counter) {
    unsafe {
        // Treat the C-style pointer as a ~-ptr again so that it will be
        // properly disposed of when it goes out of scope.
        let _counter: ~Counter = transmute(raw_counter);
    }
}

// This is a wrapper that boots up the rust runtime, runs a C "main"-style
// function, and shuts down the rust runtime again. 
//
// Alternatively we could just provide the entry point ourselves, do that, and
// then call some C code, but then I wouldn't get away with calling this whole
// thing "calling rust from C programs" anymore.
#[no_mangle]
pub extern "C" fn run_with_runtime(argc: int, argv: **u8,
                                   main: extern "C" fn(int, **u8)) -> int {
    native::start(argc, argv, proc() main(argc, argv))
}

// these are ignored by rustc, unless invoked with --test, in which case rustc
// emits an executable test runner instead of a library.
#[test]
fn test_rust() {
    let c = Counter::new(5);

    loop {
        let i = match c.wait() { Some(i) => i, _ => break };

        println!("{}", i);
    }
}

#[test]
#[allow(unused_unsafe)]
fn test_c() {
    // unsafe block technically not needed since we had to declare the
    // exported functions as safe, but since they aren't, really, we
    // might as well be honest here.
    unsafe {
        let c = counter_new(5);
        let mut i;

        while {i = counter_wait(c); i} != -1 {
            println!("{}", i);
        }

        counter_free(c);
    }
}
