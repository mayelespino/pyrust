use pyo3::prelude::*;
use std::fs;
use std::collections::HashMap;

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



// cat /proc/swaps
// Filename				Type		Size		Used		Priority
// /swap.img                               file		4194300		0		-2


// /proc/vmstat
// nr_free_pages 1740682
// nr_zone_inactive_anon 0
// nr_zone_active_anon 321625
// nr_zone_inactive_file 1390348



// /proc/<pid>/status
// Name:	cpuhp/1
// Umask:	0000
// State:	S (sleeping) <-----

/////////////////////////////////////////////////////////////////////////////////////
/// A Python module implemented in Rust.
/////////////////////////////////////////////////////////////////////////////////////
#[pymodule]
fn mylutils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_txt, m)?)?;
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;
    m.add_function(wrap_pyfunction!(read_proc_stat_cpu, m)?)?;
    Ok(())
}