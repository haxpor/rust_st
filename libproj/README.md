# Brief

A library type project for testing to produce a shared library file that can
be consumed by C/C++ program.

# Consumption

Firstly build the cargo project via `cargo build`.
It will produce `target/debug/liblibproj.so` file (for Linux) that we can use
it as linking with C/C++ program.

## Consumed by C program

Use the following code as a C driver program, name it `consume.c`.

```c
#include <stdio.h>

extern int add();

int main() {
	printf("%d", add());
}
```

then build it with

```shell
gcc -llibproj -L<path-of-shared-library> consume.c
```

## Consumed by C++ program

Use the following code as a C++ driver program, name it `consume.cpp`

```c++
#include <iostream>

extern "C" int add();

int main()
{
	std::cout << add() << std::endl;
	return 0;
}
```

then build it with

```shell
g++ -llibproj -L<path-of-shared-library> consume.cpp
```

## Note

> For both cases, you might need to do `export LD_LIBRARY_PATH=<path-of-shared-library>` for which such shared library file `liblibproj.so` locates.
