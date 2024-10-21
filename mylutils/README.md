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

