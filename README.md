# diagonal-loop

## Usage

```console
$ ./target/debug/diagonal-loop --help
Usage: diagonal-loop <X> <Y> [Z]

Arguments:
  <X>  the length of x-axis
  <Y>  the length of y-axis
  [Z]  the length of z-axis (for a 3-dim array)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### 2-dimensional array

```console
$ ./target/release/diagonal-loop 3 4
=== two_dimensional_diagonal_loop ===

|(0,0)|(0,1)|(0,2)|(0,3)|
|(1,0)|(1,1)|(1,2)|(1,3)|
|(2,0)|(2,1)|(2,2)|(2,3)|

|(0,0)|
|(1,0)|(0,1)|
|(2,0)|(1,1)|(0,2)|
|(2,1)|(1,2)|(0,3)|
|(2,2)|(1,3)|
|(2,3)|

=== two_dimensional_diagonal_loop ===
```

### 3-dimensional array

```console
$ ./target/debug/diagonal-loop 3 3 3
=== three_dimensional_diagonal_loop ===

z = 0
|(0,0,0)|(0,1,0)|(0,2,0)|
|(1,0,0)|(1,1,0)|(1,2,0)|
|(2,0,0)|(2,1,0)|(2,2,0)|

z = 1
|(0,0,1)|(0,1,1)|(0,2,1)|
|(1,0,1)|(1,1,1)|(1,2,1)|
|(2,0,1)|(2,1,1)|(2,2,1)|

z = 2
|(0,0,2)|(0,1,2)|(0,2,2)|
|(1,0,2)|(1,1,2)|(1,2,2)|
|(2,0,2)|(2,1,2)|(2,2,2)|

|(0,0,0)|
|(1,0,0)|(0,1,0)|(0,0,1)|
|(2,0,0)|(1,1,0)|(0,2,0)|(1,0,1)|(0,1,1)|(0,0,2)|
|(2,1,0)|(1,2,0)|(2,0,1)|(1,1,1)|(0,2,1)|(1,0,2)|(0,1,2)|
|(2,2,0)|(2,1,1)|(1,2,1)|(2,0,2)|(1,1,2)|(0,2,2)|
|(2,2,1)|(2,1,2)|(1,2,2)|
|(2,2,2)|

=== three_dimensional_diagonal_loop ===
```
