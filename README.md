RPG dice thrower for command line. Author: Anton A. Truttse (Dargot) <dargot@yandex.ru>

# USAGE:
    dnddice.exe [FLAGS] [OPTIONS] [dicecodes]...

## FLAGS:
        --debug              Activate debug mode
    -h, --help               Prints help information
        --help-dice-codes    Show help about dice codes' format description
        --help-methods       Show help about generation methods. See --help-method=METHOD to see help about METHOD
        --max                Show maximum
        --mean               Show mean
        --median             Show median
        --min                Show minimum
        --mode               Show mode
        --no-help            No help messages
        --numbers-only       Show only numbers
        --probabilities      Show probabilities
        --stat               Show all statistics
        --sum                Show sum
    -V, --version            Prints version information
    -v                       Verbose mode (-v, -vv, etc.)

## OPTIONS:
    -C, --crop                           Number of greatest dices to be dropped
    -d, --dice                           Dice sides [default: 6]
    -n, --dice-num                       Number of dices [default: 1]
    -D, --drop                           Number of lowest dices to be dropped
        --help-method                    Show help about generation method
    -m, --method                         Stat generation method (adnd1, adnd2, etc.) See --help-methods for full list
        --minus                          Result minus
    -N, --repetitions-num                Number of repetitions
        --plus                           Result plus
        --round-digits                   Round probabilities to number of digits [default: 2]

## ARGS:
    <dicecodes>...    Dice codes (2d8plus1, 4d6drop1 etc.) See --help-dice-codes for format description

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
