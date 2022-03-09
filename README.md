
# stat

`stat` is a small cli tool for calculating simple statistics from a vector. Additionall, the tool contains two ascii graph options to output a line and a histogram plots (thanks to https://github.com/orhanbalci/rasciigraph).

## installing

building locally requires the rust toolchain (https://rustup.rs/). 

```
cargo build --release
```

or install it to the cargo bin directory (make sure it is in your $PATH).

```
cargo install --path .
```

# examples

```
> cat iris.csv | awk -F',' '{print $1}' |stat -H
n          min        max        mean       median     mode       sd         var
150        4.3        7.9        5.8433332  5.8        5          0.8253013  0.68112224
```

or transpose the output

```
> cat tests/iris.csv | awk -F',' '{print $1}' |stat -Ht
N       149
min     4.3
max     7.9
mean    5.8433332
med     5.8
mode    5
stdev   0.8253013
var     0.68112224
```

quick histogram and line plots

```
> cat tests/input_med.csv |stat -Hh
 819 ┼                                ╭──╮
 778 ┤                             ╭──╯  ╰╮
 737 ┤                            ╭╯      │
 696 ┤                           ╭╯       ╰╮
 655 ┤                          ╭╯         ╰╮
 614 ┤                         ╭╯           ╰╮
 574 ┤                        ╭╯             ╰╮
 533 ┤                       ╭╯               ╰╮
 492 ┤                       │                 ╰╮
 451 ┤                      ╭╯                  ╰╮
 410 ┤                     ╭╯                    │
 369 ┤                    ╭╯                     ╰╮
 329 ┤                   ╭╯                       ╰╮
 288 ┤                  ╭╯                         │
 247 ┤                 ╭╯                          ╰╮
 206 ┤                ╭╯                            ╰─╮
 165 ┤              ╭─╯                               ╰─╮
 124 ┤             ╭╯                                   ╰╮
  84 ┤           ╭─╯                                     ╰─╮
  43 ┤       ╭───╯                                         ╰────╮
   2 ┼───────╯                                                  ╰──────────

> cat tests/input_med_sin.csv |stat -Hl
  89.52 ┤   ╭╮
  80.21 ┤  ╭╯│            ╭──╮            ╭─╮            ╭─╮            ╭─╮
  70.90 ┤ ╭╯ ╰╮           │  │           ╭╯ │           ╭╯ │           ╭╯ ╰╮
  61.60 ┤ │   │          ╭╯  │           │  ╰╮          │  ╰╮          │   │
  52.29 ┤ │   ╰╮         │   ╰╮         ╭╯   │         ╭╯   │          │   │
  42.98 ┤ │    │         │    │         │    │         │    │         ╭╯   ╰╮
  33.67 ┤╭╯    │        ╭╯    │         │    ╰╮        │    ╰╮        │     │
  24.36 ┤│     │        │     ╰╮       ╭╯     │        │     │        │     │
  15.05 ┤│     ╰╮       │      │       │      │       ╭╯     │        │     │
   5.74 ┤│      │       │      │       │      │       │      │       ╭╯     ╰╮
  -3.57 ┼╯      │       │      │       │      ╰╮      │      ╰╮      │       │
 -12.87 ┤       │      ╭╯      ╰╮     ╭╯       │      │       │      │       │
 -22.18 ┤       ╰╮     │        │     │        │     ╭╯       │     ╭╯       │
 -31.49 ┤        │     │        │     │        │     │        │     │        ╰
 -40.80 ┤        │     │        │     │        ╰╮    │        ╰╮    │
 -50.11 ┤        ╰╮   ╭╯        ╰╮   ╭╯         │    │         │    │
 -59.42 ┤         │   │          │   │          │   ╭╯         │   ╭╯
 -68.73 ┤         │   │          │   │          │   │          │   │
 -78.03 ┤         ╰╮ ╭╯          ╰╮  │          ╰╮  │          ╰╮  │
 -87.34 ┤          │ │            │ ╭╯           │ ╭╯           │ ╭╯
 -96.65 ┤          ╰─╯            ╰─╯            ╰─╯            ╰─╯
```

