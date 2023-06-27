# Iced Calc

Just a little practice project, building a calculator, with the iced-rs GUI libary (an Elm-architecture UI library for Rust)

No tutorial followed, learned as I went, so likely a bit rough.

## Design

    ---------
        6 * 0
            0
    ---------
    | | | | |
    | | | | |
    | | | | |
    | | | | |
    | | | | |
    | | | | |

So a main window with a display and buttons.

Process for design:
1. identify model
2. specify model
3. specify view
4. specify messages
5. specify update
6. update view
7. external connections (n/a for this)
8. test and refactoring

## Identify && Specify Model

- current number, current operation
- where it gets a little confusing is where you just specify a number and an operation, and press equals
- or just an operation and press equals. it appears to translate to current number, current operation, last entered number

so initial model might be:

    CalcState
        displayed: f64
        current_op: Operation
        last_entered: f64

## View

main layout is a column:

    current formula
    current result
    button rows

each row is a series of buttons:

    7 8 9 div
    4 5 6 mul
    1 2 3 add
    - 0 . sub
    equals

## How does math on a calculator work?

scen 1:

    press op (x) without number becomes 0 x 
    press equals becomes 0 x 0, doesnt repeat

scen 2:

    press number
    press equals just equals number

scen 3:

    press number
    press op (x)
    press equals becomes n x n = res
    press equals again becomes res x n = res2

scen 4:

    press number
    press op
    press number2
    press equals becomes n x n2 = res
    press equals again becomes res x n2 = res2

its almost like a very small queue...
    add number
    add op
    add number
    = calcs res, pops top, and adds res
    op calcs res, pops all, adds res, adds op
