# bincut

`bincut` is a command-line tool to extract byte-range from input.

* You can specify a range with offset and size.
* You can pipe input/output.
* `bincut` doesn't corrupt your terminal. If you try to output not UTF-8 data
  to tty, it aborts with error.

## Usage

```sh
$ bincut --offset=10 --size=20 input.bin > output.bin  # extract range 10..30
$ bincut --offset=10 input.bin > output.bin            # extract range 10..
$ bincut --size=20 input.bin > output.bin              # extract range 0..20

$ some_cmd | bincut --offset=10 --size=20 > output.bin  # you can pipe I/O
```
