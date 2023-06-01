use std::{
    process,
    thread::sleep,
    time::{Duration, SystemTime},
};

use libc::{c_long, rusage, suseconds_t, time_t, timeval, RUSAGE_SELF};

const SLEEP_SECONDS: Duration = Duration::from_secs(3);
const NUM_MULS: usize = 1000_000_000;
const NUM_MALLOCS: usize = 100000000;
const MALLOC_SIZE: usize = 1000;

struct ProfileTimes {
    process: u32,
    real_time: SystemTime,
    ru_utime: timeval,
    ru_stime: timeval,
}

fn profile_start() -> ProfileTimes {
    let mut usage = rusage {
        ru_utime: timeval {
            tv_sec: 0 as time_t,
            tv_usec: 0 as suseconds_t,
        },
        ru_stime: timeval {
            tv_sec: 0 as time_t,
            tv_usec: 0 as suseconds_t,
        },
        ru_maxrss: 0 as c_long,
        ru_ixrss: 0 as c_long,
        ru_idrss: 0 as c_long,
        ru_isrss: 0 as c_long,
        ru_minflt: 0 as c_long,
        ru_majflt: 0 as c_long,
        ru_nswap: 0 as c_long,
        ru_inblock: 0 as c_long,
        ru_oublock: 0 as c_long,
        ru_msgsnd: 0 as c_long,
        ru_msgrcv: 0 as c_long,
        ru_nsignals: 0 as c_long,
        ru_nvcsw: 0 as c_long,
        ru_nivcsw: 0 as c_long,
    };

    unsafe {
        libc::getrusage(RUSAGE_SELF, &mut usage);
    }

    let profile = ProfileTimes {
        process: process::id(),
        real_time: SystemTime::now(),
        ru_utime: usage.ru_utime,
        ru_stime: usage.ru_stime,
    };

    profile
}

fn profile_log(profile_time: &ProfileTimes) {
    let mut usage = rusage {
        ru_utime: timeval {
            tv_sec: 0 as time_t,
            tv_usec: 0 as suseconds_t,
        },
        ru_stime: timeval {
            tv_sec: 0 as time_t,
            tv_usec: 0 as suseconds_t,
        },
        ru_maxrss: 0 as c_long,
        ru_ixrss: 0 as c_long,
        ru_idrss: 0 as c_long,
        ru_isrss: 0 as c_long,
        ru_minflt: 0 as c_long,
        ru_majflt: 0 as c_long,
        ru_nswap: 0 as c_long,
        ru_inblock: 0 as c_long,
        ru_oublock: 0 as c_long,
        ru_msgsnd: 0 as c_long,
        ru_msgrcv: 0 as c_long,
        ru_nsignals: 0 as c_long,
        ru_nvcsw: 0 as c_long,
        ru_nivcsw: 0 as c_long,
    };

    unsafe {
        libc::getrusage(RUSAGE_SELF, &mut usage);
    }

    let user_sec = usage.ru_utime.tv_sec - profile_time.ru_utime.tv_sec;
    let user_mill = usage.ru_utime.tv_usec - profile_time.ru_utime.tv_usec;
    let user_time = Duration::from_secs(user_sec as u64) + Duration::from_micros(user_mill as u64);

    let kernel_sec = usage.ru_stime.tv_sec - profile_time.ru_stime.tv_sec;
    let kernel_mill = usage.ru_stime.tv_usec - profile_time.ru_stime.tv_usec;
    let kernel_time =
        Duration::from_secs(kernel_sec as u64) + Duration::from_micros(kernel_mill as u64);

    let real_elapsed_time = profile_time.real_time.elapsed().unwrap().as_secs_f64();
    println!(
        "[pid: {}] real: {}, user: {}, kernel: {}",
        profile_time.process,
        real_elapsed_time,
        user_time.as_secs_f64(),
        kernel_time.as_secs_f64()
    );
}

fn main() {
    let mut x: f32 = 1.0;
    let profile_times = profile_start();

    for _ in 0..NUM_MULS {
        x *= 1.1;
    }

    profile_log(&profile_times);

    let profile_times = profile_start();
    unsafe {
        let mut p;
        for _ in 0..NUM_MALLOCS {
            p = libc::malloc(MALLOC_SIZE);
        }
    }

    profile_log(&profile_times);

    let profile_times = profile_start();
    sleep(SLEEP_SECONDS);
    profile_log(&profile_times);
}
