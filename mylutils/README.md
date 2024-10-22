# mylutils V 0.2.0

This is a collection of file utilities. It is currently under active development and not quite ready for primetime.

I am planing to turn this in to an open source project, using [the linux foundation guidelines](https://www.linuxfoundation.org/resources/open-source-guides/starting-an-open-source-project).

The following are instructions to help you use the package:

__Functions available in mylutils__

- read_txt
- read_csv
- read_proc_stat_cpu


## How to pip install and run

__1. Pip install__

```
pip install mylutils
```

__2. run the examples/example.py__

```
import mylutils

for line in mylutils.read_txt("test.txt"):
    print(line)

```

__3. on Python's REPL, do a dir(mylutils)__

```
<<<<<<< HEAD
$ python
> import mylutils
> dir(mylutils)
```

## Run in virtual environment

__1. clone the repo__

```
$ git clone git@github.com:mayelespino/pyrust.git
$ cd pyrust/mylutils/
```

__2. create a virtual environment__ 

```
$ sudo python3 -m venv .env
$ source .env/bin/activate
```

__3. install maturin__

```
$ pip install maturin
```

__4. build mylutils in virtual env__

```
$ maturin dev
```

__5. run the examples/example.py__

```
import mylutils

for line in mylutils.read_txt("test.txt"):
    print(line)

```

# Possible uses of mylutils

Write Pythion varios functions or a class that calls mylutils functions. For example:

```
def is_cpu_idle(threshold=50):
    cpu_stats = mylutils.read_proc_stat_cpu()
    total_time = cpu_stats['total_time']
    idle  _time = cpu_stats['idle_time']
    return((idle_time/total_time)*100 < threshold)
```

When called on Pythons REPL:

```
> print(f"\nis_cpu_idle(99): {is_cpu_idle(99)}", )
> is_cpu_idle(99): True
```

Next write a salt diagnostics module or stand-alone script based on the functions above. For example:

```
checks_to_perform = []
def add_check(check, list):
    if check not in list:
        list.append(check)
    return


if !is_cpu_idle(50):
    if is_cpu_user(50):
        print("CPU is busy processing User processes.")
        add_check("check_pids_state", checks_to_perform)
    if is_cpu_system(50):
        print("CPU is busy processing System processes.")
        add_check("check_system_errors", checks_to_perform)
    if is_cpu_iowait(25):
        print("CPU is busy in IOWAIT.")
        add_check("check_network_usage", checks_to_perform)
        add_check("check_disk_usage", checks_to_perform)
    if is_memory_swap(20):
        print("Memmory swap space usage is high.")
        add_check("check_system_errors", checks_to_perform)

```

=======
git clone git@github.com:mayelespino/pyrust.git
```

<!--

https://www.linuxhowtos.org/System/procstat.htm


> cat /proc/stat
cpu  2255 34 2290 22625563 6290 127 456
cpu0 1132 34 1441 11311718 3675 127 438
cpu1 1123 0 849 11313845 2614 0 18
intr 114930548 113199788 3 0 5 263 0 4 [... lots more numbers ...]
ctxt 1990473
btime 1062191376
processes 2915
procs_running 1
procs_blocked 0
The very first "cpu" line aggregates the numbers in all of the other "cpuN" lines.

These numbers identify the amount of time the CPU has spent performing different kinds of work. Time units are in USER_HZ or Jiffies (typically hundredths of a second).

The meanings of the columns are as follows, from left to right:

user: normal processes executing in user mode
nice: niced processes executing in user mode
system: processes executing in kernel mode
idle: twiddling thumbs
iowait: waiting for I/O to complete
irq: servicing interrupts
softirq: servicing softirqs
The "intr" line gives counts of interrupts serviced since boot time, for each
of the possible system interrupts. The first column is the total of all interrupts serviced; each subsequent column is the total for that particular interrupt.

The "ctxt" line gives the total number of context switches across all CPUs.
The "btime" line gives the time at which the system booted, in seconds since
the Unix epoch.
The "processes" line gives the number of processes and threads created, which includes (but is not limited to) those created by calls to the fork() and clone() system calls.
The "procs_running" line gives the number of processes currently running on CPUs.
The "procs_blocked" line gives the number of processes currently blocked, waiting for I/O to complete.


2781784	40825	1838323	151127794	2469103	0	36081		158293910
user	nice	system	idle	    iowait	irq	softirq
2%	    0%	    1%	    95%	        2%		0%  0%
>>>>>>> 7d6e2e3 (added mylutils-social-git.png)
