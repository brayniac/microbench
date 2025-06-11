use std::time::Instant;
use std::hint::black_box;

fn main() {
    perf_counter_test("APERF", Msr::new(MsrId::APERF), 0);
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
        Ok(counter) => {
            let iterations = 500_000;
            let start = Instant::now();

            for _ in 0..iterations {
                black_box(counter.value());
            }

            let latency = start.elapsed().as_nanos() / iterations;

            println!("counter: {} latency: {} ns/iter", name, latency);
        }
        Err(e) => {
            Err(())
        }
    }
}