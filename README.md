RPG dice thrower for command line. Author: Anton A. Truttse (Dargot) <dargot@yandex.ru>

# USAGE:
    dnddice.exe [FLAGS] [OPTIONS] [dicecodes]...

## FLAGS:
    -c, --command-line       Command-line mode
        --debug              Activate debug mode
    -h, --help               Prints help information
        --help-dice-codes    Show help about dice codes' format description
        --help-methods       Show help about generation methods. See --help-method=METHOD to see help about METHOD
        --help-tags          Show tags' list
    -l                       Log mode (-l, -ll, etc.)
        --max                Show maximum
        --mean               Show mean
        --median             Show median
        --min                Show minimum
        --mode               Show mode
        --no-help            No help messages
        --numbers-only       Show only numbers
        --probabilities      Show probabilities
        --show-method        Show method description
        --show-number        Show repetition number
        --stat               Show all statistics
        --sum                Show sum
    -V, --version            Prints version information
    -v                       Verbose mode (-v, -vv, etc.)

## OPTIONS:
    -C, --crop <crop>                       Number of greatest dices to be dropped [default: 0]
    -d, --dice <dice>                       Dice sides [default: 6]
    -n, --dice-num <dices-num>              Number of dices [default: 1]
    -D, --drop <drop>                       Number of lowest dices to be dropped [default: 0]
        --find-tags <find-tags>             Show help about generation method by tags (for example: "DnD,ordered"). See
                                            --help-tags to see tags' list [default: ]
        --help-method <help-method>         Show help about generation method [default: ]
        --log-file <log-file>               Log file [default: dnddice.log]
    -m, --method <method>                   Stat generation method (adnd1, adnd2, etc.) See --help-methods for full list
                                            [default: ]
    -p, --parameters <method-parameters>    Method parameters [default: ]
        --minus <minus>                     Result minus [default: 0]
    -N, --repetitions-num <num>             Number of repetitions [default: 1]
        --plus <plus>                       Result plus [default: 0]
    -r, --reroll <reroll>                   Reroll dices' results [default: ]
        --round-digits <round-digits>       Round probabilities to number of digits [default: 2]

## ARGS:
    <dicecodes>...    Dice codes (2d8plus1, 4d6drop1, 2d4-1d6/1d3 etc.) See --help-dice-codes for format description

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
