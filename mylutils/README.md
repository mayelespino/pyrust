# mylutils V 0.2.0

This is a collection of file utilities. It is currently under active development and not quite ready for prime-time.

The following are instructions to help you use the package:

 ## Functions now available in mylutils

- read_txt : Returns a list of strings, one line each.
- read_csv : Returns a list of lists of strings, one string on each column.
- read_proc_stat_cpu : Returns a dictionary with data from the first line from ```/proc/stat```, which corresponds to the consolidated CPU data.
- read_proc_meminfo : Returns a list of dictionaries with data from ```/proc/meminfo```.
- read_proc_vmstat : Returns a dictionary with data from the first line from ```/proc/vmstat```.
- read_proc_pid_stat : Returns a list of dictionaries with the data from  ```/proc/<PID>/stat``` file. One dictionary per pid.  Please read ```man 5 proc``` for more information.

## Functions to be implemented 
- read_proc_meminfo
- read_proc_pid_status


# How to pip install and run

## In the OS

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
$ python
> import mylutils
> dir(mylutils)
```

## In virtual environment

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

Write Python various functions or a class that calls mylutils functions. For example:

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
