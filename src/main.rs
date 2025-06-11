use std::time::Instant;
use std::hint::black_box;

use perf_event::ReadFormat;
use perf_event::events::Event;
use perf_event::events::x86::{Msr, MsrId};

fn main() {

    if let Ok(msr) = Msr::new(MsrId::APERF) {
        perf_counter_test("APERF", msr, 0);
    }
}

fn perf_counter_test(name: &'static str, event: impl Event, core: usize) {
    match perf_event::Builder::new(event)
        .one_cpu(core)
        .any_pid()
        .exclude_hv(false)
        .exclude_kernel(false)
        .pinned(true)
        .read_format(
            ReadFormat::TOTAL_TIME_ENABLED | ReadFormat::TOTAL_TIME_RUNNING | ReadFormat::GROUP,
        )
        .build()
    {
        Ok(mut group) => {
            let iterations = 500_000;
            let start = Instant::now();

            for _ in 0..iterations {
                if group.enable_group().is_ok() {
                    if let Ok(reading) = group.read_group() {
                        if let Some(counter) = reading.get(&group) {
                            black_box(counter.value());
                        } else {
                            panic!("couldn't read counter");
                        }
                    } else {
                        panic!("perf group read failed");
                    }
                } else {
                    panic!("failed to enable perf group")
                }
            }

            let latency = start.elapsed().as_nanos() / iterations;

            println!("counter: {name} latency: {latency} ns/iter");
        }
        Err(_) => {
            eprintln!("counter: {name} could not be initialized");
        }
    }
}