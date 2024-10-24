use pyo3::prelude::*;
use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::Path;


//
// READ TEXT FILE, RETURN LIST OF STRINGS
//
#[pyfunction]
fn read_txt<'a>(file_path: &str) -> Vec<String> {
    let lines = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");
    lines.lines().map(str::to_string).collect()
}

//
// READ CSV FILE, RETURN LIST OF LIST OF STRINGS
//
#[pyfunction]
fn read_csv<'a>(file_path: &str) -> Vec<Vec<String>> {
    let lines = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");
    let lines_vec: Vec<String> = lines.lines().map(str::to_string).collect();
    let lines_vec_iter = lines_vec.iter();
    let mut result: Vec<Vec<String>> = Vec::new();
    for line in lines_vec_iter {
        let cols: Vec<String> = line.split(",").map(str::to_string).collect();
        result.push(cols);
    }
    result
}

//
// READ AND MARSHALL JSON
// 

//
// READ AND MARSHALL YAML
//

//
// READ /PROC/STAT, RETURN DICTIONARY
//
// source: https://www.linuxhowtos.org/System/procstat.htm
//
// Various pieces of information about kernel activity are available in the
// /proc/stat file.
// All of the numbers reported in this file are aggregates since the system first booted.
// For a quick look, simply cat the file:
// > cat /proc/stat
// cpu  2255 34 2290 22625563 6290 127 456
// cpu0 1132 34 1441 11311718 3675 127 438
// cpu1 1123 0 849 11313845 2614 0 18
// intr 114930548 113199788 3 0 5 263 0 4 [... lots more numbers ...]
// ctxt 1990473
// btime 1062191376
// processes 2915
// procs_running 1
// procs_blocked 0
//
// The very first "cpu" line aggregates the numbers in all of the other "cpuN" lines.
// These numbers identify the amount of time the CPU has spent performing different kinds of work. Time units are in USER_HZ or Jiffies (typically hundredths of a second).
//
// The meanings of the columns are as follows, from left to right:
// user: normal processes executing in user mode
// nice: niced processes executing in user mode
// system: processes executing in kernel mode
// idle: twiddling thumbs
// iowait: waiting for I/O to complete
// irq: servicing interrupts
// softirq: servicing softirqs
// The "intr" line gives counts of interrupts serviced since boot time, for each
// of the possible system interrupts. The first column is the total of all interrupts serviced; each subsequent column is the total for that particular interrupt.
//
#[pyfunction]
fn read_proc_stat_cpu<'a>() -> HashMap<String, i32> {

    let mut proc_file_lines = Vec::new();
    for line in fs::read_to_string("/proc/stat").unwrap().lines() {
        proc_file_lines.push(line.to_string())
    }

    let cols = proc_file_lines[0].split_whitespace().collect::<Vec<&str>>();
    
    let user = cols[1].parse::<i32>().unwrap();
    let nice = cols[2].parse::<i32>().unwrap();
    let system = cols[3].parse::<i32>().unwrap();
    let idle = cols[4].parse::<i32>().unwrap();
    let iowait = cols[5].parse::<i32>().unwrap();
    let irq = cols[6].parse::<i32>().unwrap();
    let softirq = cols[7].parse::<i32>().unwrap();
    let total = user + nice + system + idle + iowait + irq + softirq;

    let mut cpu_stats = HashMap::new();
    cpu_stats.insert(String::from("total_time"),total);
    cpu_stats.insert(String::from("system_time"),system);
    cpu_stats.insert(String::from("user_time"), user);
    cpu_stats.insert(String::from("idle_time"), idle);
    cpu_stats.insert(String::from("nice_time"), nice);
    cpu_stats.insert(String::from("iowait_time"), iowait);
    cpu_stats.insert(String::from("irq_time"), irq);
    cpu_stats.insert(String::from("softirq_time"), softirq);

    cpu_stats
}

//
// READ /PROC/MEMINFO, RETURN DICTIONARY
//
// cat /proc/meminfo
// MemTotal:       16346452 kB
// MemFree:         6952332 kB
// MemAvailable:   14266328 kB
// Buffers:          774420 kB
// Cached:          6329928 kB
// SwapCached:            0 kB
// Active:          2935724 kB
// Inactive:        5561148 kB
// Active(anon):    1285808 kB
// Inactive(anon):        0 kB
// Active(file):    1649916 kB
// Inactive(file):  5561148 kB
// Unevictable:       32396 kB
// Mlocked:           27580 kB
// SwapTotal:       4194300 kB
// SwapFree:        4194300 kB
// Zswap:                 0 kB
// Zswapped:              0 kB
// Dirty:               372 kB
//
#[pyfunction]
fn read_proc_meminfo<'a>() -> HashMap<String, String> {

    let mut proc_file_lines = Vec::new();
    for line in fs::read_to_string("/proc/meminfo").unwrap().lines() {
        proc_file_lines.push(line.to_string())
    }

    let mut meminfo = HashMap::new();
    for line in proc_file_lines {
        let cols = line.split(":").collect::<Vec<&str>>();
        meminfo.insert(cols[0].to_string(), cols[1].to_string());
    }

    meminfo
}

//
// READ /PROC/VMSTAT, RETURN DICTIONARY
//
// cat /proc/vmstat
// nr_free_pages 1740682
// nr_zone_inactive_anon 0
// nr_zone_active_anon 321625
// nr_zone_inactive_file 1390348
#[pyfunction]
fn read_proc_vmstat<'a>() -> HashMap<String, String> {

    let mut proc_file_lines = Vec::new();
    for line in fs::read_to_string("/proc/vmstat").unwrap().lines() {
        proc_file_lines.push(line.to_string())
    }

    let mut vmstat = HashMap::new();
    for line in proc_file_lines {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        vmstat.insert(cols[0].to_string(), cols[1].to_string());
    }

    vmstat
}


// https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html
// /proc/<pid>/stat
// (1) pid  %d
// The process ID.
//
// (2) comm  %s
// The filename of the executable, in parentheses.
// This is visible whether or not the executable is
// swapped out.
//
// (3) state  %c
// One of the following characters, indicating process
// state:
//
// R  Running
//
// S  Sleeping in an interruptible wait
//
// D  Waiting in uninterruptible disk sleep
//
// Z  Zombie
//
// T  Stopped (on a signal) or (before Linux 2.6.33)
//    trace stopped
//
// t  Tracing stop (Linux 2.6.33 onward)
//
// W  Paging (only before Linux 2.6.0)
//
// X  Dead (from Linux 2.6.0 onward)
// 
// x  Dead (Linux 2.6.33 to 3.13 only)
//
// K  Wakekill (Linux 2.6.33 to 3.13 only)
//
// W  Waking (Linux 2.6.33 to 3.13 only)
//
// P  Parked (Linux 3.9 to 3.13 only)
//
// I  Idle (Linux 4.14 onward)
//
// (4) ppid  %d
// The PID of the parent of this process.
//
// (5) pgrp  %d
// The process group ID of the process.
//
// (6) session  %d
// The session ID of the process.
//
// (7) tty_nr  %d
// The controlling terminal of the process.  (The minor
// device number is contained in the combination of
// bits 31 to 20 and 7 to 0; the major device number is
// in bits 15 to 8.)
//
// (8) tpgid  %d
// The ID of the foreground process group of the con‐
// trolling terminal of the process.
//
// (9) flags  %u
// The kernel flags word of the process.  For bit mean‐
// ings, see the PF_* defines in the Linux kernel
// source file include/linux/sched.h.  Details depend
// on the kernel version.
//
// The format for this field was %lu before Linux 2.6.
//
// (10) minflt  %lu
// The number of minor faults the process has made
// which have not required loading a memory page from
// disk.
//
// (11) cminflt  %lu
// The number of minor faults that the process's
// waited-for children have made.
//
// (12) majflt  %lu
// The number of major faults the process has made
// which have required loading a memory page from disk.
//
// (13) cmajflt  %lu
// The number of major faults that the process's
// waited-for children have made.
//
// (14) utime  %lu
// Amount of time that this process has been scheduled
// in user mode, measured in clock ticks (divide by
// sysconf(_SC_CLK_TCK)).  This includes guest time,
// guest_time (time spent running a virtual CPU, see
// below), so that applications that are not aware of
// the guest time field do not lose that time from
// their calculations.
//
// (15) stime  %lu
// Amount of time that this process has been scheduled
// in kernel mode, measured in clock ticks (divide by
// sysconf(_SC_CLK_TCK)).
//
// (16) cutime  %ld
// Amount of time that this process's waited-for chil‐
// dren have been scheduled in user mode, measured in
// clock ticks (divide by sysconf(_SC_CLK_TCK)).  (See
// also times(2).)  This includes guest time,
// cguest_time (time spent running a virtual CPU, see
// below).
//
// (17) cstime  %ld
// Amount of time that this process's waited-for chil‐
// dren have been scheduled in kernel mode, measured in
// clock ticks (divide by sysconf(_SC_CLK_TCK)).
//
// (18) priority  %ld
// (Explanation for Linux 2.6) For processes running a
// real-time scheduling policy (policy below; see
// sched_setscheduler(2)), this is the negated schedul‐
// ing priority, minus one; that is, a number in the
// range -2 to -100, corresponding to real-time priori‐
// ties 1 to 99.  For processes running under a non-
// real-time scheduling policy, this is the raw nice
// value (setpriority(2)) as represented in the kernel.
// The kernel stores nice values as numbers in the
// range 0 (high) to 39 (low), corresponding to the
// user-visible nice range of -20 to 19.
//
// Before Linux 2.6, this was a scaled value based on
// the scheduler weighting given to this process.
//
// (19) nice  %ld
// The nice value (see setpriority(2)), a value in the
// range 19 (low priority) to -20 (high priority).
//
// (20) num_threads  %ld
// Number of threads in this process (since Linux 2.6).
// Before kernel 2.6, this field was hard coded to 0 as
// a placeholder for an earlier removed field.
//
// (21) itrealvalue  %ld
// The time in jiffies before the next SIGALRM is sent
// to the process due to an interval timer.  Since ker‐
// nel 2.6.17, this field is no longer maintained, and
// is hard coded as 0.
//
// (22) starttime  %llu
// The time the process started after system boot.  In
// kernels before Linux 2.6, this value was expressed
// in jiffies.  Since Linux 2.6, the value is expressed
// in clock ticks (divide by sysconf(_SC_CLK_TCK)).
//
// The format for this field was %lu before Linux 2.6.
//
// (23) vsize  %lu
// Virtual memory size in bytes.
//
// (24) rss  %ld
// Resident Set Size: number of pages the process has
// in real memory.  This is just the pages which count
// toward text, data, or stack space.  This does not
// include pages which have not been demand-loaded in,
// or which are swapped out.
//
// (25) rsslim  %lu
// Current soft limit in bytes on the rss of the
// process; see the description of RLIMIT_RSS in
// getrlimit(2).
//
// (26) startcode  %lu  [PT]
// The address above which program text can run.
//
// (27) endcode  %lu  [PT]
// The address below which program text can run.
//
// (28) startstack  %lu  [PT]
// The address of the start (i.e., bottom) of the
// stack.
//
// (29) kstkesp  %lu  [PT]
// The current value of ESP (stack pointer), as found
// in the kernel stack page for the process.
//
// (30) kstkeip  %lu  [PT]
// The current EIP (instruction pointer).
//
// (31) signal  %lu
// The bitmap of pending signals, displayed as a deci‐
// mal number.  Obsolete, because it does not provide
// information on real-time signals; use
// /proc/[pid]/status instead.
//
// (32) blocked  %lu
// The bitmap of blocked signals, displayed as a deci‐
// mal number.  Obsolete, because it does not provide
// information on real-time signals; use
// /proc/[pid]/status instead.
//
// (33) sigignore  %lu
// The bitmap of ignored signals, displayed as a deci‐
// mal number.  Obsolete, because it does not provide
// information on real-time signals; use
// /proc/[pid]/status instead.
//
// (34) sigcatch  %lu
// The bitmap of caught signals, displayed as a decimal
// number.  Obsolete, because it does not provide
// information on real-time signals; use
// /proc/[pid]/status instead.
//
// (35) wchan  %lu  [PT]
// This is the "channel" in which the process is wait‐
// ing.  It is the address of a location in the kernel
// where the process is sleeping.  The corresponding
// symbolic name can be found in /proc/[pid]/wchan.
//
// (36) nswap  %lu
// Number of pages swapped (not maintained).
//
// (37) cnswap  %lu
// Cumulative nswap for child processes (not main‐
// tained).
//
// (38) exit_signal  %d  (since Linux 2.1.22)
// Signal to be sent to parent when we die.
//
// (39) processor  %d  (since Linux 2.2.8)
// CPU number last executed on.
//
// (40) rt_priority  %u  (since Linux 2.5.19)
// Real-time scheduling priority, a number in the range
// 1 to 99 for processes scheduled under a real-time
// policy, or 0, for non-real-time processes (see
// sched_setscheduler(2)).
//
// (41) policy  %u  (since Linux 2.5.19)
// Scheduling policy (see sched_setscheduler(2)).
// Decode using the SCHED_* constants in linux/sched.h.
//
// The format for this field was %lu before Linux
// 2.6.22.
//
// (42) delayacct_blkio_ticks  %llu  (since Linux 2.6.18)
// Aggregated block I/O delays, measured in clock ticks
// (centiseconds).
//
// (43) guest_time  %lu  (since Linux 2.6.24)
// Guest time of the process (time spent running a vir‐
// tual CPU for a guest operating system), measured in
// clock ticks (divide by sysconf(_SC_CLK_TCK)).
//
// (44) cguest_time  %ld  (since Linux 2.6.24)
// Guest time of the process's children, measured in
// clock ticks (divide by sysconf(_SC_CLK_TCK)).
//
// (45) start_data  %lu  (since Linux 3.3)  [PT]
// Address above which program initialized and unini‐
// tialized (BSS) data are placed.
//
// (46) end_data  %lu  (since Linux 3.3)  [PT]
// Address below which program initialized and unini‐
// tialized (BSS) data are placed.
//
// (47) start_brk  %lu  (since Linux 3.3)  [PT]
// Address above which program heap can be expanded
// with brk(2).
//
// (48) arg_start  %lu  (since Linux 3.5)  [PT]
// Address above which program command-line arguments
// (argv) are placed.
//
// (49) arg_end  %lu  (since Linux 3.5)  [PT]
// Address below program command-line arguments (argv)
// are placed.
//
// (50) env_start  %lu  (since Linux 3.5)  [PT]
// Address above which program environment is placed.
//
// (51) env_end  %lu  (since Linux 3.5)  [PT]
// Address below which program environment is placed.
//
// (52) exit_code  %d  (since Linux 3.5)  [PT]
// The thread's exit status in the form reported by
// waitpid(2).
// 
// man 5 proc
//
#[pyfunction]
fn read_proc_pid_stat<'a>() -> Vec<HashMap<String, String>> {
    let mut stat_files: Vec<String> = Vec::new();

    for e in WalkDir::new("/proc").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_dir() {
	    let path: String = e.path().display().to_string();
            if path.matches('/').count() == 2 {
               stat_files.push(path + "/stat");
            }
        }
    }

    let mut stat_file_lines: Vec<String> = Vec::new();
    for file in stat_files {
        if Path::new(&file).exists() {
           let contents = fs::read_to_string(file).expect("Should have been able to read the file");
           stat_file_lines.push(contents)
        }
    }

    let mut stat_file_data: Vec<HashMap<String, String>> = Vec::new();
    for line in stat_file_lines {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert(String::from("pid"), cols[0].to_string());
        data.insert(String::from("comm"), cols[1].to_string());
        data.insert(String::from("state"), cols[2].to_string());
        data.insert(String::from("ppid"), cols[3].to_string());
        data.insert(String::from("pgrp"), cols[4].to_string());
        data.insert(String::from("session"), cols[5].to_string());
        data.insert(String::from("tty_nr"), cols[6].to_string());
        data.insert(String::from("tpgid"), cols[7].to_string());
        data.insert(String::from("flags"), cols[8].to_string());
        data.insert(String::from("format"), cols[9].to_string());
        data.insert(String::from("minflt"), cols[10].to_string());
        data.insert(String::from("cminflt"), cols[11].to_string());
        data.insert(String::from("majflt"), cols[12].to_string());
        data.insert(String::from("cmajflt"), cols[13].to_string());
        data.insert(String::from("utime"), cols[14].to_string());
        data.insert(String::from("stime"), cols[15].to_string());
        data.insert(String::from("cutime"), cols[16].to_string());
        data.insert(String::from("priority"), cols[17].to_string());
        data.insert(String::from("nice"), cols[18].to_string());
        data.insert(String::from("num_threads"), cols[19].to_string());
        data.insert(String::from("itrealvalue"), cols[20].to_string());
        data.insert(String::from("starttime"), cols[21].to_string());
        data.insert(String::from("vsize"), cols[22].to_string());
        data.insert(String::from("rss"), cols[23].to_string());
        data.insert(String::from("rsslim"), cols[24].to_string());
        data.insert(String::from("startcode"), cols[25].to_string());
        data.insert(String::from("endcode"), cols[26].to_string());
        data.insert(String::from("startstack"), cols[27].to_string());
        data.insert(String::from("kstkesp"), cols[28].to_string());
        data.insert(String::from("kstkeip"), cols[29].to_string());
        data.insert(String::from("signal"), cols[30].to_string());
        data.insert(String::from("blocked"), cols[31].to_string());
        data.insert(String::from("sigignore"), cols[32].to_string());
        data.insert(String::from("sigcatch"), cols[33].to_string());
        data.insert(String::from("wchan"), cols[34].to_string());
        data.insert(String::from("nswap"), cols[35].to_string());
        data.insert(String::from("cnswap"), cols[36].to_string());
        data.insert(String::from("exit_signal"), cols[37].to_string());
        data.insert(String::from("processor"), cols[38].to_string());
        data.insert(String::from("rt_priority"), cols[39].to_string());
        data.insert(String::from("policy"), cols[40].to_string());
        data.insert(String::from("delayacct_blkio_ticks"), cols[41].to_string());
        data.insert(String::from("guest_time"), cols[42].to_string());
        data.insert(String::from("cguest_time"), cols[43].to_string());
        data.insert(String::from("start_data"), cols[44].to_string());
        data.insert(String::from("end_data"), cols[45].to_string());
        data.insert(String::from("start_brk"), cols[46].to_string());
        data.insert(String::from("arg_start"), cols[47].to_string());
        data.insert(String::from("arg_end"), cols[48].to_string());
        data.insert(String::from("env_start"), cols[49].to_string());
        data.insert(String::from("env_end"), cols[50].to_string());
        data.insert(String::from("exit_code"), cols[51].to_string());
        stat_file_data.push(data);
    }

    stat_file_data

}






/////////////////////////////////////////////////////////////////////////////////////
/// A Python module implemented in Rust.
/////////////////////////////////////////////////////////////////////////////////////
#[pymodule]
fn mylutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_txt, m)?)?;
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;
    m.add_function(wrap_pyfunction!(read_proc_stat_cpu, m)?)?;
    m.add_function(wrap_pyfunction!(read_proc_meminfo, m)?)?;
    m.add_function(wrap_pyfunction!(read_proc_vmstat, m)?)?;
    m.add_function(wrap_pyfunction!(read_proc_pid_stat, m)?)?;
    Ok(())
}