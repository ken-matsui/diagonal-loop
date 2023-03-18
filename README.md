# diagonal-loop

Example code of diagonal loops for 2 & 3-dimensional arrays

## Usage

```console
$ cargo run -- --help
Example code of diagonal loops for 2 & 3-dimensional arrays

Usage: diagonal-loop [OPTIONS] <X> <Y> [Z]

Arguments:
  <X>  the length of x-axis
  <Y>  the length of y-axis
  [Z]  the length of z-axis (for a 3-dim array)

Options:
  -r, --report         Show general information about diagonals
      --no-elem        Do not show elements
      --bottom-up      Loop diagonals bottom-up (default: top-down)
      --block <BLOCK>  Split by blocks (BLOCK*BLOCK)
  -h, --help           Print help
  -V, --version        Print version
```

## Examples

### 2-dimensional array

#### Simple Case

```console
$ cargo run -- 3 4
=== two_dimensional_diagonal_loop ===

|(0,0)|(0,1)|(0,2)|(0,3)|
|(1,0)|(1,1)|(1,2)|(1,3)|
|(2,0)|(2,1)|(2,2)|(2,3)|

top-down:
|(0,0)|
|(0,1)|(1,0)|
|(0,2)|(1,1)|(2,0)|
|(0,3)|(1,2)|(2,1)|
|(1,3)|(2,2)|
|(2,3)|

=== two_dimensional_diagonal_loop ===
```

#### Bottom-up loop

```console
$ cargo run -- 3 4 --bottom-up
=== two_dimensional_diagonal_loop ===

|(0,0)|(0,1)|(0,2)|(0,3)|
|(1,0)|(1,1)|(1,2)|(1,3)|
|(2,0)|(2,1)|(2,2)|(2,3)|

bottom-up:
|(0,0)|
|(1,0)|(0,1)|
|(2,0)|(1,1)|(0,2)|
|(2,1)|(1,2)|(0,3)|
|(2,2)|(1,3)|
|(2,3)|

=== two_dimensional_diagonal_loop ===
```

#### Use blocks

```console
$ cargo run -- 4 4 --block 2
=== two_dimensional_diagonal_loop ===

|(0,0)|(0,1)|(0,2)|(0,3)|
|(1,0)|(1,1)|(1,2)|(1,3)|
|(2,0)|(2,1)|(2,2)|(2,3)|
|(3,0)|(3,1)|(3,2)|(3,3)|

top-down:
|(0,0)..=(1,1)|
|(0,2)..=(1,3)|(2,0)..=(3,1)|
|(2,2)..=(3,3)|
|

=== two_dimensional_diagonal_loop ===
```

#### With `--report`

```console
$ cargo run -- 3 4 --report
=== two_dimensional_diagonal_loop ===

|(0,0)|(0,1)|(0,2)|(0,3)|
|(1,0)|(1,1)|(1,2)|(1,3)|
|(2,0)|(2,1)|(2,2)|(2,3)|

top-down:
|(0,0)|
|(0,1)|(1,0)|
|(0,2)|(1,1)|(2,0)|
|(0,3)|(1,2)|(2,1)|
|(1,3)|(2,2)|
|(2,3)|

=== Report ===

num of diagonals: 6
max diagonal len (middle): 3
ave diagonal len: 2.00

=== two_dimensional_diagonal_loop ===
```

#### With `--no-elem`

```console
$ cargo run -- 123 283  --report --no-elem
=== two_dimensional_diagonal_loop ===

=== Report ===

num of diagonals: 405
max diagonal len (middle): 123
ave diagonal len: 85.95

=== two_dimensional_diagonal_loop ===
```

### 3-dimensional array

#### Simple Case

```console
$ cargo run -- 3 3 3
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

top-down:
|(0,0,0)|
|(0,0,1)|(0,1,0)|(1,0,0)|
|(0,0,2)|(0,1,1)|(0,2,0)|(1,0,1)|(1,1,0)|(2,0,0)|
|(0,1,2)|(0,2,1)|(1,0,2)|(1,1,1)|(1,2,0)|(2,0,1)|(2,1,0)|
|(0,2,2)|(1,1,2)|(1,2,1)|(2,0,2)|(2,1,1)|(2,2,0)|
|(1,2,2)|(2,1,2)|(2,2,1)|
|(2,2,2)|

=== three_dimensional_diagonal_loop ===
```

#### Bottom-up loop

```console
$ cargo run -- 3 3 3 --bottom-up
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

bottom-up:
|(0,0,0)|
|(1,0,0)|(0,1,0)|(0,0,1)|
|(2,0,0)|(1,1,0)|(0,2,0)|(1,0,1)|(0,1,1)|(0,0,2)|
|(2,1,0)|(1,2,0)|(2,0,1)|(1,1,1)|(0,2,1)|(1,0,2)|(0,1,2)|
|(2,2,0)|(2,1,1)|(1,2,1)|(2,0,2)|(1,1,2)|(0,2,2)|
|(2,2,1)|(2,1,2)|(1,2,2)|
|(2,2,2)|

=== three_dimensional_diagonal_loop ===
```

#### With `--report`

```console
$ cargo run -- 3 3 3 --report
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

top-down:
|(0,0,0)|
|(0,0,1)|(0,1,0)|(1,0,0)|
|(0,0,2)|(0,1,1)|(0,2,0)|(1,0,1)|(1,1,0)|(2,0,0)|
|(0,1,2)|(0,2,1)|(1,0,2)|(1,1,1)|(1,2,0)|(2,0,1)|(2,1,0)|
|(0,2,2)|(1,1,2)|(1,2,1)|(2,0,2)|(2,1,1)|(2,2,0)|
|(1,2,2)|(2,1,2)|(2,2,1)|
|(2,2,2)|

=== Report ===

num of diagonals: 7
max diagonal len (middle): 7
ave diagonal len: 3.86

=== three_dimensional_diagonal_loop ===
```

#### With `--no-elem`

```console
$ cargo run -- 123 283 326 --report --no-elem
=== three_dimensional_diagonal_loop ===

=== Report ===

num of diagonals: 730
max diagonal len (middle): 33209
ave diagonal len: 15544.84

=== three_dimensional_diagonal_loop ===
```
