# i8080emu

## About

A work-in-progress emulator for the 8-bit [Intel 8080](https://en.wikipedia.org/wiki/Intel_8080)
microprocessor from the 1970s. To verify this emulator's correctness I'm
currently using a test suite created by Frank D. Cringle and modified by
Ian Bartholomew that you can find [here](https://altairclone.com/downloads/cpu_tests/).

## Build

You will need [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
installed on your machine to build the emulator from source. Once you've cloned
the repository, navigate to its root directory and run the following command:

```sh
cargo build --release
```

## Run

To run a program on the emulator, use the `run` command followed by the path
to the program:

```sh
i8080emu run /path-to-program/program.COM
```

## Test

You will need to download the tests before you can run them. If you have `curl`
installed on your machine, you can download the tests with the following
command:

```sh
curl \
-O https://altairclone.com/downloads/cpu_tests/8080PRE.COM \
-O https://altairclone.com/downloads/cpu_tests/TST8080.COM \
-O https://altairclone.com/downloads/cpu_tests/CPUTEST.COM \
-O https://altairclone.com/downloads/cpu_tests/8080EXM.COM
```

Then, run a test as outlined above:

```sh
i8080emu run 8080PRE.COM
```

The longest test suite is `8080EXM.COM`, which takes around 25 minutes to run.
The other suites should only take a few seconds each.

## Resources

### Documents

1. [Intel 8080 Microcomputer Systems User's Manual September 1975](https://www.nj7p.info/Manuals/PDFs/Intel/9800153B.pdf)
    - Also available [here](https://archive.org/details/intel8080microco00inte) and [here](https://mark-ogden.uk/files/intel/publications/98-153B%20Intel%208080%20Microcomputer%20Systems%20Users%20Manual-Sep75.pdf)
2. [Intel 8080 Assembly Language Programming Manual](https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
3. [Intel MCS-80/85™ Family User's Manual October 1979](https://archive.org/details/Mcs80_85FamilyUsersManual/page/n1/mode/2up)
    - Also available [here](https://drive.google.com/file/d/0B9rh9tVI0J5mMDQ5M2VkYzYtMWZkYS00YWVlLTg5MWEtNTgzN2ZjYTk3YWU3/view?resourcekey=0--8gZogrur8I4z7w4MMAwkg)
4. [8080/8085 Assembly Language Programming Manual](https://altairclone.com/downloads/manuals/8080-8085%20Programmers%20Manual.pdf)
5. [CP/M Plus Operating System Programming Guide](http://www.cpm.z80.de/manuals/cpm3-pgr.pdf)

### Sites

1. [altairclone.com](https://altairclone.com/downloads/) provides a variety of
downloads for many different things: manuals, ROMs, CPU tests, etc.
2. [mark-ogden.uk](https://mark-ogden.uk/files/intel/publications/) provides a
variety of publications related to Intel technologies.
3. [emulator101.com](http://web.archive.org/web/20240522090447/http://emulator101.com/)
is now defunct, but is available on the Wayback Machine and seems like a good
source of 8080 emulator information.

### Other Emulators

1. [8080](https://github.com/superzazu/8080) by superzazu (GitHub)
is an Intel 8080 emulator written in C.
2. [i8080-core](https://github.com/begoon/i8080-core) by begoon (GitHub) is an
Intel 8080 emulator written in C specifically for the KR580VM80A, which was a
Russian clone of the Intel 8080.
3. [i8080-javascript](https://github.com/chris-j-akers/i8080-javascript)
by chris-j-akers (GitHub) is an Intel 8080 emulator written in Javascript that
you can use online [here](https://8080.cakers.io/).
