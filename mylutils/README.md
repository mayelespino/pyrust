# mylutils

This is a collection of file utilities. It is currently under active development and not quite ready for primetime.

I am planing to turn this in to an open source project, using [the linux foundation guidelines](https://www.linuxfoundation.org/resources/open-source-guides/starting-an-open-source-project).

The following are instructions to help you use the package:

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