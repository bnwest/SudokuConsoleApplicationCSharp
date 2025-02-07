// Lessons learned:
//
// 1. String processing is a no fly zone.  strings are are an array of potentially
// variable length utf-8 characters.  Indexing into a string does not make sense.
// My workaround was to create an array of characters, where indexing works.
//
// 2. Indices must use type "usize".  Rust is very unforgiving.
//
// 3. types are not required at variable declaration.  rust compiler can decide
// from context what the type is, making the type implicit.  for loop variables
// get context from for statement which may not be enough context for integer types.
//
// 4. variable must be initialized at declaration.  might be required since types are not?
//
// 5. rust compiler warns on constant variable not being all CAPs,
// expressions have "unneeded" parens, etc aka stuff that does not matter.
//
// 6. arrays are declared ass backwards ... aka inside out.
// [[uside; 3]; 9] => a 9 row x 3 column array.
//
// 7. Extremely non-obvious when a variable gets "moved" (ending its lifetime
// before going out of scope).
//
// 8. Simple structs do not get Copy/Clone traits and must be explicitly derived
// or implemented.
//
// 9. private struct methods can not be unit tested (without some thunk code)
// since they are not visible outside of the struct???
//
// 10. rust does not support OO inheritance or polymorphism.
// rust supports structs with methods.
//
// 11. test driven development has first class support.
//

#[derive(Debug)] // adding so pretty print will work ... {:#?} for pretty-print
pub struct SudokuGame {
    // private data member(s)
    db_game: String,
}

impl SudokuGame {
    fn create() -> SudokuGame {
        // randomly create a game
        return SudokuGame {
            // db_game is a 81 character string, with the solved numbers present.
            db_game: String::from(
                // found hidden triple (3 6 7) at cell(5, 4) and cell(5, 6) and cell(5, 7), for row 5
                // "4....8.....753...8.9..6.41353....2.7.........7.6....81954.1..3.3...751.....9....5",
                // 4..  ..8  ...
                // ..7  53.  ..8
                // .9.  .6.  413
                //
                // 53.  ...  2.7
                // ...  ...  ...
                // 7.6  ...  .81
                //
                // 954  .1.  .3.
                // 3..  .75  1..
                // ...  9..  ..5

                // https://hodoku.sourceforge.net/en/tech_fishb.php
                // row x-wing
                // found x-wing exclusion for fish candidate 5 from cell(4, 5), given base set cells(2, 5) cells(2, 8) cells(5, 5) cells(5, 8).
                // ".41729.3.769..34.2.3264.7194.39..17.6.7..49.319537..24214567398376.9.541958431267",
                // column x-wing
                // found x-wing exclusion for fish candidate 1 from cell(5, 3), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(2, 4), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(5, 4), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(2, 7), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(5, 7), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(2, 8), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(5, 8), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(2, 9), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // found x-wing exclusion for fish candidate 1 from cell(5, 9), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
                // "98..62753.65..3...327.5...679..3.5...5...9...832.45..9673591428249.87..5518.2...7",

                // https://www.livesudoku.com/en/tutorial-xy-wing.php
                // xy wing
                // found xy wing: pivot cell(7, 1) pair(7, 9) wing1 cell(2, 1) pair(3, 7) wing2 cell(9, 3) pair(3, 9), (x, y) is (7, 9), z is 3
                //     remove 3 from cell(2, 3)
                //     remove 3 from cell(3, 3)
                // found xy wing: pivot cell(9, 8) pair(3, 9) wing1 cell(5, 8) pair(4, 9) wing2 cell(8, 9) pair(3, 4), (x, y) is (3, 9), z is 4
                //     remove 4 from cell(6, 9)
                //     remove 4 from cell(5, 9)
                "5..23.867.8.9....22..8.......1.9.25..273651....518237...2..3.816.8.19.2.14..28..5",
                // found xy wing: pivot cell(5, 5) pair(6, 9) wing1 cell(5, 2) pair(6, 7) wing2 cell(8, 5) pair(7, 9), (x, y) is (6, 9), z is 7
                //    remove 7 from cell(8, 2)
                // "647915382395284761.2.3..549....3...6...5.1...9...4.....86.5..235.26.3....39.28675",

                // https://www.sudoku9x9.com/techniques/xywing/
                // "81524736939..5842.4.2..9.586849712359..525816521..39742...965..7.9......1......9.",
                // found xy wing: pivot cell(6, 2) pair(5, 7) wing1 cell(5, 1) pair(7, 8) wing2 cell(6, 5) pair(5, 8), (x, y) is (5, 7), z is 8
                //     remove 8 from cell(5, 5)
                //     remove 8 from cell(6, 3)
                // "2859.643....342....435.89269.42..1.8.216..3.43..4.1.92..276........25...5.7...2..",
                // found xy wing: pivot cell(8, 5) pair(4, 8) wing1 cell(6, 5) pair(3, 4) wing2 cell(7, 6) pair(3, 8), (x, y) is (4, 8), z is 3
                //     remove 3 from cell(5, 6)
                //     remove 3 from cell(9, 5)
                //     remove 3 from cell(6, 6)
                // "...3672.13.2.14..771......3.8....324.4..2...527....189.21.5.....379.65125........",

                // found naked triple (2 4 6) for row 2
                // found naked triple (2 3 6) for row 8
                // found naked triple (2 7 9) for grid(3, 3)
                // "69..71.....8..3..1...5.9.....3...65.97.3.5.12.51...3.....7.8...8..4..1.....93..85"

                // found naked quad (3 4 6 7) for row 1
                // found naked quad (3 4 5 6) for grid(1, 1)
                // "....19..5......1.89....32...46..2.....78.59.....4..86...19....32.9......7..25...."

                // https://www.sudokuwiki.org/
                // hidden triples?
                // "000000000231090000065003100008924000100050006000136700009300570000010843000000000"
                // hidden quads
                // found hidden quad (2 3 7 8) at cell(2, 1) and cell(2, 2) and cell(2, 3) and cell(2, 9), for row 2
                // xxx found hidden quad (2 6 8 9) at cell(2, 2) and cell(5, 2) and cell(7, 2) and cell(8, 2), for column 2
                // xxx found hidden quad (1 6 8 9) at cell(1, 7) and cell(2, 7) and cell(3, 7) and cell(3, 9), for grid(1, 3)
                // "650000024000609000040000000570400061000501000310002085000000010000203000130000098"
                // xxx found hidden quad (2 3 7 8) at cell(1, 2) and cell(1, 5) and cell(1, 6) and cell(1, 7), for row 1
                // xxx found hidden quad (3 4 5 9) at cell(8, 2) and cell(8, 3) and cell(8, 4) and cell(8, 6), for row 8
                // xxx found hidden quad (3 4 7 8) at cell(3, 3) and cell(6, 3) and cell(8, 3) and cell(9, 3), for column 3
                // xxx found hidden quad (3 4 6 7) at cell(2, 4) and cell(2, 6) and cell(3, 4) and cell(3, 6), for grid(1, 2)
                // xxx found hidden quad (3 5 7 9) at cell(1, 7) and cell(2, 7) and cell(3, 7) and cell(3, 9), for grid(1, 3)
                // xxx found hidden quad (3 4 7 8) at cell(4, 2) and cell(5, 1) and cell(6, 2) and cell(6, 3), for grid(2, 1)
                // xxx found hidden quad (1 4 6 9) at cell(4, 4) and cell(4, 6) and cell(6, 4) and cell(6, 6), for grid(2, 2)
                // xxx found hidden quad (1 5 8 9) at cell(4, 7) and cell(4, 9) and cell(5, 9) and cell(6, 7), for grid(2, 3)
                // xxx found hidden quad (2 3 5 9) at cell(8, 4) and cell(8, 6) and cell(9, 4) and cell(9, 5), for grid(3, 2)
                // "000500000425090001800010020500000000019000460000000002090040003200060807000001600"

                // type 1 and 2 locked candidate in row and column
                // "563700000002000947040100000030050209020000080409010050000004010254000600000006495"

                // 2 x naked triples (grid,row)
                // "5..8.......8..91...69..4...8.61....47...9...39....75.2...9..43...26..9.......3..7"

                // hidden pair (4 6) at cell(7, 1) and cell(7, 3), for row 7
                // "3.2........7....9.......1.........7..6..4.........2.35...9.7........3..6.1....4.."

                // failed to solve - naked quad ( 2 3 5 8 ) for grid(2,3), exclude based on colors)
                // ".....7.....2...6......3......96..........5........4.1747....9..1...........85.3.."

                // "........547.......85..42...64.58......79.41......73.96...85..34.......673........"
                // "46...1.....2.96....3.....68.......37...6.7...51.......84.....5....71.9.....3...24"
                // "1....89...5..9..32.9.7..6..83.9...2.....4...6.....53..5..1.94...6..5...84...8..1."
                // "...7..1...81..2..595...1..7.......73..3...8..54.......2..6...594..8..76...7..4..."
                // "73...46...94..28...2..........43.2.7.........3.2.78..........5...89..73...61...84"

                // Simple Sudoku
                // http://www.angusj.com/sudoku/
                // hard
                // 2 x naked triples (grid,row)
                // hidden pair ( 6 8 ) at cell(6,5) and cell(6,8), row 6 (at start)
                // hidden triple ( 3 4 7 ) for row 8 (at start)
                // "5..8.......8..91...69..4...8.61....47...9...39....75.2...9..43...26..9.......3..7"
                // "........547.......85..42...64.58......79.41......73.96...85..34.......673........"
                // found hidden triple ( 4 5 8 ) for column 3 (at start)
                // "46...1.....2.96....3.....68.......37...6.7...51.......84.....5....71.9.....3...24"
                // found hidden triple ( 4 7 8 ) for grid(1,1)
                // "1....89...5..9..32.9.7..6..83.9...2.....4...6.....53..5..1.94...6..5...84...8..1."
                // "...7..1...81..2..595...1..7.......73..3...8..54.......2..6...594..8..76...7..4..."
                // "73...46...94..28...2..........43.2.7.........3.2.78..........5...89..73...61...84"

                // found hidden triple (3 6 7) at cell(5, 1) and cell(5, 4) and cell(5, 9), for row 5
                // "4....961...56...79.1.42.3...51.6.................1.83...7.83.6.53...67...692....3"
                // found hidden triple (3 6 7) at cell(5, 4) and cell(5, 6) and cell(5, 7), for row 5
                // "4....8.....753...8.9..6.41353....2.7.........7.6....81954.1..3.3...751.....9....5"

                // https://kjell.haxx.se/sudoku/ -- 26 is hard, 25 is harder, etc
                // 26
                // "4....5.83...........28...9.7.....469.6.41..2.85..6.3..9.......7..32.........9.8.5"
                // 25
                // ".9.....5....2.....16.....4.6.2...87...5..623.....9......69..1...287..3.5......4.6"
                // ".1.8....5.......23.......1.2...7349.....2.3........1..3......6..654.82.....15.8.4"
                // "3....918..8.......2....85.9..8.6...3...58.6...37...........4..85.1..7.3...9.....6"
                // ".58...3.1....5...7.3...8...61.57..2.5..1..9...2.8..4....1........9.8.........423."
                // ".4..2.1..1..9....6....8.7.2.9...36.5.2.....1...8..4...6....7.9..7.....8...91....7"
                // "..2.3.......8521..5.7....3........27.....4....9.61...3....7.3...8...5....43.9.68."
                // 24
                // ".8.6..53...49.....79....1....64...8...............87..8...71..5..3.9..6..2..8..9."
                // "......7.2...6.9.......1293.....37.145.....3..6.4.....9.8.4....6.7..81....4......."
                // "..1.2...5...863.2.....9..7..6.....4..9.....51.8.74...24......6.7..2........5...3."
                // "...8..5..3....1..4..9...71..582......9.1.76.3.1..........5..28.........79...43..."
                // "7.......9........5.6...2.....59.....4..2...6.836....2....1.34.6..9..7..11...46..."
                // "2...3.4.17.....9..8....5....54......1..8....5.....234741.7..58...3........2......"
                // "....3......8..1.2.2....9......5..9.6.2......4..738...2..2.7..3.936..5...4..1....."
                // 23
                // ".758....9...5.....19..42.7...6...31..8.2.5..7.......9...........5......8...731..."
                // "2..5...........79......46..9.1.3...24....6...657..8..3...9.......6...15..1.2....."
                // ".....19..3.7....268.........5.28469........1.2......8..9....5.1...83.....6.1....."
                // "...1........2...568.7.....9....128...3.56..........6..1.96...2......874..5....9.."
                // "9.....8.....876...5...4..........2.3.4........7.23.1...3...89422....3........9.5."
                // "...38.....3.....69....9....8.7....5.4...7.........284..24..6.....6...9.8.8..25..."
                // "..2.....7.36....5....25..8.5..682...2.....46........3.1.......8...916......32...."
                // ".1.....2.7.6.3...4...1..98...5...4.7......8.5..18.......7.5.....9...7...18.....7."
                // "...5..42757...96...2...3...8..6.2.............3.....7.6.....1..4.23.8.......27..."
                // "4......2..8.9.5.1.6..7...9...1..83....2....76.6...9......4..8........5...392....."
                // "5.....9....6..7453.....4.......5......8196..........6791...5.......82..4....1...5"
                // "..19..2.4.............5....1.5.......7.4..832..3...7.5.82....5....53.....9.2..6.."
                // 22
                // "...9...254.........1..4...6....816...69....875...2.....8.2..1.......4...7..5....."
                // ".......46.26....1..7..26....92..87......1...5...4.......1....2......96.35....7..."
                // "5....74.............7..8...9..61.7.........38..2....6.....4.6.7.4...1...138..2..."
                // "..2.........6793....6...1..7...9...5...8...46.......32...1....3...2.4...8..9..5.."
                // "7....1...3.....7........59..5.......9..813..7.....2......95..7..24.....5....4.83."
                // ".261..5.4..42.9.........31.6...3......74....9...............1..31...........126.7"
                // "4...........8..2.93.1....5.....4..1.5.....3..69.52.....5.79...8........72...3...."
                // 20
                // ".1....6...4...6..1.....5...8..9....3..5........6.13..7......9...72.4..........58."
                // 18
                // "..5.4....................12.4.3.2.....61.......9...8......7.5.81........3...64..."
                // 17 L1
                // "....6....5.7...........9.2..4...6...........5......7.8.93....1........4....85.2.."
                // "9.6..7.....7...3.8........5.............96....3...1..4...8......5.4.....1......6."
                // "..1.....5.....5..7..9.8......3...8.....4........7.29..5............1..6.72......."
                // "......3........59..6.84.....2...7..43..9........5.........1.8......6...75........"
                // "....4..9..........8.3.........1.8....49....2.7....3......5..3.7........8..2.1...."
                // "..1.........9....745...3..........1.....8..4..68.7.....2......8...1........4.5..."
                // 17 L2
                // "5....1...9.......8....6.7.........19...........3.7......68.......7...3.....9.5..2"
                // "........369.1.....5.........17.....8....6..1...3..5.........69.......7....28....."
                // "8.......5...14.....3.6..........9....1.....6...2..8......3.......5..2..9.6.....8."
                // "..2.4...8....6........9......81...........46........5.69.3.....5......9....2....3"
                // "53...............1......829.89.....5...7...6..............18...6...9....4......7."
                // "..............9..5.83.7....5....2..........7.......83.9...64.......3....2..5....9"
                // ".........8..9.3.....7.....1....4.5.......76..3.........46.7........2..83........9"
                // 17 L3
                // "...2...4........6..39.1....4..6.......2...9.....8..1......93.......5....2......7."
                // "....42........7...3.5.....9..9.............2...68..1...2.......4......7....5.1..6"
                // "......92.6...38...4........8.......5.1..........2......2..8.........4..3.97.5...."
                // ".........4....9..........65...56....9.....4.8....1....2.1...3.......37.4..5......"
                // failed to solve - Simple Sudoku could not solve
                // ".....8..1.6..3......2....7..........1.7.....8...69.....4....3...9....6.4.....1..."
                // ".53.6..........7..2.......4.......651..2.9...........89....12.......4.......5...."
                // 17 L4 - I Like
                // "3..2..........8.6.......49....13...2.96..........5......4..6...5.......1.......8."
                // "..............248.7..6.......4....1..58....9.........3......7..3.......6..2.84..."
                // ".9...8..........2.1...34.........4....25......67...8....5.........2...6.3...4...."
                // ".....8.......7.9.62.........79...4.......1........23....7.4.........3.1...6....2."
                // failed to solve - exclude based on multiple colors
                // "..21..6........95..3..7...............1.....7...6.5...59...........3...86.4......"
                // "....29.8...3..........8.....8..........7..3.1...1....51.53...............9..4..2."
                // "....7....9.......1..5.8....12.4............7........83.......3....1.2..9..89....."
                // ".9..........3........2..13.6.2.......7....9.4......7....8......2.3....6.....94..."
                // failed to solve - exclude based on colors
                // "....8....47.....1.....362....3..2......7...4.........9.4.......91............36.."
                // "....1..2.........8.63......5...2...9..4...3.....9.....4...........3.76..1......5."
                // "5.....7.3...1.4......8.....3....2....8.....9........1..9........18.....6....75..."

                // good one -- real grinder
                // "5..7.6.....8....43.......1...1.4................6.27...2....9.....83....7........"

                // failed to solve - naked quad ( 2 3 5 8 ) for grid(2,3), exclude based on colors)
                // ".....7.....2...6......3......96..........5........4.1747....9..1...........85.3.."
                // "4......9.82..1.......7..6.........81.....3......6.9...1.5..........4...8..9......"
                // ".....7....4......3...8.2.........7...6..5....2.....8..8....9..6.......457.3......"
                // "...46.8....9..........8.3.......1...68............7.2...7..9..1.4..3............7"
                // failed to solve - "exclude based on multiple colors"
                // "........4..6.....5.1...2.9.5.............13.....9..8..8..45......3...12.........."
                // "..9...3.6.........1..57.....2..5.....3..........14..7..6.9.3.........45.........."
                // "3.7..5.....5.1..........9...9...8......6...3....2....7.6....8......7....4..9....."
                // ".....5....4......36.......7.9....51.4..73........6.....51...2.....62............."
                // "3.2........7....9.......1.........7..6..4.........2.35...9.7........3..6.1....4.."
                // "...9.5...2.........1.8.....7......8.......19.3...2.....9........8.5....4....3...2"
                // "...1...24....4...7.6........1....8.....5....9....2.........15.......86..7.4......"
                // ".....59..1.6......4.....5...38.........6........417....9...2...........4...7....1"
                // ".....9.......1..2.5..3...........3.4.26.8.....1...........2..6.39.4............1."
                // good one
                // ".2...........4..65.7......8.3............17.29.6...........2.....4....9....8.3..."

                // xy-wing
                // found xy wing: pivot cell(8, 1) pair(1, 5) wing1 cell(8, 9) pair(2, 5) wing2 cell(9, 2) pair(1, 2), (x, y) is (1, 5), z is 2
                //     remove 2 from cell(8, 3)
                //     remove 2 from cell(9, 9)
                // found xy wing: pivot cell(9, 2) pair(1, 2) wing1 cell(2, 2) pair(2, 5) wing2 cell(8, 1) pair(1, 5), (x, y) is (1, 2), z is 5
                //     remove 5 from cell(2, 1)
                // "......1.6...8....3..19.........3...4..9......2.87......3..6...........9......5.8."

                // failed to solve x-wing
                // found x-wing exclusion for fish candidate 3 from cell(3, 6), given base set cells(3, 1) cells(3, 4) cells(4, 1) cells(4, 4).
                // found x-wing exclusion for fish candidate 3 from cell(4, 6), given base set cells(3, 1) cells(3, 4) cells(4, 1) cells(4, 4).
                // found x-wing exclusion for fish candidate 3 from cell(3, 8), given base set cells(3, 1) cells(3, 4) cells(4, 1) cells(4, 4).
                // found x-wing exclusion for fish candidate 3 from cell(3, 9), given base set cells(3, 1) cells(3, 4) cells(4, 1) cells(4, 4).
                // "1..46.......8......2....7............97..2..........414......6......927...3......"
                // "2.69.............8.......5.....85.7.....1....7.....9...........58..3..1....6..2.."
                // "..8.5...........2..4.....317.9..8...5...........3....4.1.2..........95........9.."
                // 17 l5
                // failed to solve - naked quad ( 1 3 4 7 ) for column 3 at start, Simple Sudoku fails to solve
                // "...7.....8.6.....1...29........3..2........79..5..4...........592........8...1..."
                // "1....6.5........3.....2..9.6.....7.....94........35........8..2..3.......94......"
                // "...8..2...39..4.........6.....29....3....7...14..............4.....1..3.6..5....."
                // ".....9...1.5..7.......48..6...5...4........8.2..3......89............1.3..4......"
                // "...2.1..9.....5...3.........51..9.........3........4.....8...214.36.....7........"
                // "5...............6....94.7....8........97..........1.3.16...5.........8.73.....4.."

                // http://www.angusj.com/sudoku/ -- Advanced Puzzle Pack 2 -- puzzle001.ss to puzzle046.ss
                // hidden pair ( 1 6 ) at cell(2,4) and cell(3,4), column 4
                // "2...578.37.....4...43.......7.54....32.876.49....13.5.......61...1.....44.736...2"
                // "7..42..6..435..2...1..3.59......4321....9....1643......71.6..3...9..361..2..41..9"
                // ".8..7..9...5..2...2.....4.8..7..61.5..2.3.6..1.82..9..3.1.....9...4..7...2..9..3."
                // "6.94..1...87.1......38.7....16.2...8..5...6..8...6.97....3.68......7.43...1..57.6"
                // "...9.61....3......2.9.5..48.5..1.3.7..8...4..1.7.3..2.73..2.5.4......7....43.5..."
                // "54..219..........5..695..1298...62.............27...6979..385..6..........856..71"
                // ".....7.21...6.8.......5..8693.7..4....6.4.5....4..9.6367..3.......1.6...14.9....."
                // "...8.6.4.5..7..31..43...8....5.....19..2.1..66.....9....2...63..69..2..8.5.1.7..."
                // "9.....3.1...21......8..6....659.82...4.....6...94.318....6..5......85...5.7.....3"
                // "65..4328..2.8.65.........648...24......198......76...878.........54.9.3..3458..26"
                // ".98..4..6.7..6....6.29.....9....6.21...4.8...56.1....9.....74.2....1..5.2..5..68."
                // "...1.....8...652.....47...8359.27.6..4.....5..2.54.7939...16.....428...7.....4..."
                // "5...9..6....5..1.7..4..1.9...14.765...........381.57...4.3..5..1.7..4....2..7...4"
                // ".32.9...5.....48....763...46....9.8...3...4...2.5....39...673....64.....2...8.15."
                // "..6..3.214.....6...98.7....6.31.........3.........54.6....4.16...1.....224.7..5.."
                // ".65...19.8..1.53..3.......575..2.......548.......1..245.......3..34.6..9.42...68."
                // "2.39.54......4.573...3.......7.....69..583..74.....8.......9...875.3......94.73.5"
                // "...5......469.853..8..32...82...41...94.2.86...18...43...24..5..721.568......3..."
                // "..2.4...13.5.69...47.1........7..13...14.27...63..8........1.78...68.9.38...7.5.."
                // "38..2.19...7...528..5..9.........8.4...4.7...5.2.........3..4..631...2...28.6..75"
                // ".3..2...67.....4..2.1.76....231...6.....5.....9...735....28.6.5..6.....19...1..4."
                // "4....268.8....1..2..9..6...29..8.4....4...3....6.4..18...7..1..6..9....4.416....7"
                // "2....5.986..8792.....2..7..3.54...8.4.......3.7...85.2..1..4.....8712..652.9....7"
                // ".46.9....3....8.7...8....32.2.1..6..8...3...9..1..9.5.41....8...8.6....4....7.12."
                // "2..59..43...........8327....76.....45.3.1.2.68.....93....1746...........69..53..1"
                // "..9..8.4.8.....3.7.3.9.2...49..7.2...7.....8...2.1..34...1.5.2.3.5.....8.6.3..5.."
                // ".....5.16174..2...65......9.....8.3...2.4.8...6.3.....4......68...5..27121.8....."
                // "..4.....9.2.5.......6.91.4......2.6.64..1..87.9.8......3.64.7.......7.5.1.....8.."
                // "..794...1....32..6...1...7...5.2..6996.....5882..9.4...4...1...6..27....5...697.."
                // ".2.1...3.45..32....3.7.54..1...78.....5...7.....25...1..24.7.1....58..94.8...1.6."
                // "..651..9.4.3..8.6..89..64......65.....41398.....24......56..12..3.4..5.9.4..526.."
                // ".974.6............1...5..9.4..9.2.5.2.18.59.7.5.3.1..6.4..1...5............6.931."
                // "8...65.9....2...57.2.8.....2.....7.5.6.....8.3.4.....9.....3.2.74...8....8.94...1"
                // ".54.7...8.2.8.5....7....35.7...286.....9.7.....241...9.98....1....3.4.8.5...8.96."
                // "......5..2..6....798..154..6..1..9.5..9.2.1..7.8..4..2..657..498....2..1..4......"
                // ".2.6..7.5.......1.9..17..8.5..2....8.1.9.7.5.7....6..2.8..94..3.5.......2.9..1.6."
                // ".53...1.44.....97.1..9....8.4.1........742........5.3.6....1..7.87.....12.1...64."
                // "5...6..7.431.7.......1..2....7..65...8..1..6...29..1....4..7.......5.723.5..2...9"
                // ".57.2..86..........1..8.47....4...595...9...373...6....78.5..9..........19..3.82."
                // "..8...1....5..2.647....1...5...4..9..39...42..7..3...6...8....921.4..6....6...5.."
                // ".5192..........32..2...3..59.658...1.386.295.2...198.47..2...1..13..........3467."
                // "1....4..2..3.7..4..8..6..7....7....654.....218....2....1..3..6..6..1.4..4..6....9"
                // "......1...1.....49..9.27.655..38.7.6.........2.7.64..317.83.9..95.....8...2......"
                // "7.9.3.8.....2.....4....8.6525...4...3..672..4...3...8794.8....3.....3.....3.1.7.8"
                // "4..2..89..96.8.2.1.......54...42.....5.9.1.8.....65...17.......6.2.4.51..39..2..8"
                // ".......5..5.9..3..3..754..8..8.4.6..17..2..45..5.7.2..5..867..4..9..5.8..6......."
            ),
        };
    }
}

#[derive(Debug)] // adding so pretty print will work ... {:#?} for pretty-print
pub struct SudokuPuzzleCell {
    possibles: [bool; 9],
    found: usize,
}

// Replace
//     #[derive(Clone, Copy)]
// with an explicit implementation ...

// Copy trait is opt-in, meaning you still have to either derive the Copy trait
// implementation or make it yourself.

// Clone is actually a super-trait of Copy, so anything that implements Copy
// also must implement Clone.

// Traits are similar to a feature often called interfaces in other languages,
// although with some differences.

impl Copy for SudokuPuzzleCell {}

impl Clone for SudokuPuzzleCell {
    fn clone(&self) -> SudokuPuzzleCell {
        *self
    }
}

impl SudokuPuzzleCell {
    const NOT_FOUND: usize = 0xFFFFFFFF;

    fn new() -> SudokuPuzzleCell {
        // create an empty cell
        return SudokuPuzzleCell {
            possibles: [true; 9],
            found: SudokuPuzzleCell::NOT_FOUND,
        };
    }

    fn initialize(&mut self) {
        // create an empty cell
        self.possibles = [true; 9];
        self.found = SudokuPuzzleCell::NOT_FOUND;
    }

    fn set_possible(&mut self, i: usize, value: bool) {
        self.possibles[i] = value;
    }

    fn is_possible(&self, i: usize) -> bool {
        return self.possibles[i];
    }

    fn is_solved(&self) -> bool {
        let solved: bool = self.found != SudokuPuzzleCell::NOT_FOUND;
        return solved;
    }

    fn set_exclusive_possibles(&mut self, possibles: &[usize; 9]) {
        for i in 0..9 {
            self.possibles[i] = false;
        }
        for i in 0..9 {
            let value: usize = possibles[i];
            if value != SudokuPuzzleCell::NOT_FOUND {
                self.possibles[value] = true;
            }
        }
    }
}

#[derive(Copy, Clone)]
struct CellCoordinate {
    row: usize,
    column: usize,
}

#[derive(Debug)]
pub struct SudokuPuzzle {
    // rust only supports single dimensional arrays
    // => one needs build their own multi-dimensional array
    cells: [[SudokuPuzzleCell; 9]; 9],
}

impl SudokuPuzzle {
    fn initialize(&mut self) {
        for row in 0..9 {
            for column in 0..9 {
                self.cells[row][column].initialize();
            }
        }
    }

    fn create(game: &SudokuGame) -> SudokuPuzzle {
        let mut puzzle: SudokuPuzzle = SudokuPuzzle {
            cells: [[SudokuPuzzleCell::new(); 9]; 9],
        };
        puzzle.initialize();

        // cleanup / normalize the inout game strings
        // some have "+" at the en of every row (given 9 rows)
        // some use "0" versus "." to indicate unknown values

        let input_string_has_plus_signs: bool = game.db_game.chars().nth(9).unwrap() == '+';
        let cell_increment: usize = {
            if input_string_has_plus_signs {
                10
            } else {
                9
            }
        };

        let mut normalized_db_game: String = String::from("");

        for i in (0..81).step_by(cell_increment) {
            normalized_db_game.push_str(&game.db_game[i..i + 9]);
        }

        // lesson learned: rust does not support string processing since it is hard.
        // rust "char" is a potentially multi-byte UTF-8 character.
        // the "chars()" iterates over the UTF-8 characters.
        let mut normalized_db_game_char_array: [char; 81] = ['.'; 81];
        for (i, c) in normalized_db_game.chars().enumerate() {
            if '1' <= c && c <= '9' {
                normalized_db_game_char_array[i] = c;
            }
        }
        // println!("create: normalized_db_game {normalized_db_game}");

        // convert normalized input game string into a Sudoku puzzle

        let mut input_index: usize = 0;

        for row in 0..9 {
            let input_row = &normalized_db_game_char_array[input_index..input_index + 9];
            for column in 0..9 {
                let cell: char = input_row[column];
                // println!("create: row {row} column {column} cell {cell}.");
                if '1' <= cell && cell <= '9' {
                    // convert '1'..'9' into 0..8, for internal futzing.
                    let cell_digit: usize = cell as usize - '1' as usize;
                    for i in 0..9 {
                        puzzle.cells[row][column].set_possible(i, false);
                    }
                    // know that this cell has a solution,
                    // but will let find_naked_singles() discover it and clean up neighbors
                    puzzle.cells[row][column].set_possible(cell_digit, true);
                }
            }
            input_index += 9;
        }

        return puzzle;
    }

    pub fn display(&self) {
        // a row containing 9 cells:
        //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"
        //      "| 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |  | 1 2 3 | | 1 2 3 | | 1 2 3 |\n"
        //      "| 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |  | 4 5 6 | | 4 5 6 | | 4 5 6 |\n"
        //      "| 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |  | 7 8 9 | | 7 8 9 | | 7 8 9 |\n"
        //      "+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+\n"

        for i in 0..9 {
            println!("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            for k in (0..9).step_by(3) {
                for j in 0..9 {
                    let cell: &SudokuPuzzleCell = &self.cells[i][j];

                    let kk: usize = usize::try_from(k).unwrap();

                    let char1: char = {
                        if cell.is_possible(kk + 0) {
                            char::from_digit(k + 1, 10).expect("digit is digit")
                        } else {
                            ' '
                        }
                    };
                    let char2: char = {
                        if cell.is_possible(kk + 1) {
                            char::from_digit(k + 2, 10).expect("digit is digit")
                        } else {
                            ' '
                        }
                    };
                    let char3: char = {
                        if cell.is_possible(kk + 2) {
                            char::from_digit(k + 3, 10).expect("digit is digit")
                        } else {
                            ' '
                        }
                    };

                    print!("| {char1} {char2} {char3} | ");
                    if (j + 1) % 3 == 0 {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("+-------+ +-------+ +-------+  +-------+ +-------+ +-------+  +-------+ +-------+ +-------+");
            if (i + 1) % 3 == 0 {
                println!();
            }
        }
    }

    fn find_naked_singles(&mut self) -> bool {
        let mut found_singles: bool = false;
        let mut singles: [[usize; 9]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                let cell: SudokuPuzzleCell = self.cells[i][j];
                if !cell.is_solved() {
                    let mut num_possibilities: u8 = 0;
                    let mut last_found: usize = SudokuPuzzleCell::NOT_FOUND;
                    for k in 0..9 {
                        if cell.is_possible(k) {
                            last_found = k;
                            num_possibilities += 1;
                            if num_possibilities > 1 {
                                break;
                            }
                        }
                    }
                    let found_single: bool = num_possibilities == 1;
                    if found_single {
                        found_singles = true;
                        singles[i][j] = last_found;
                    } else {
                        singles[i][j] = SudokuPuzzleCell::NOT_FOUND;
                    }
                } else {
                    singles[i][j] = SudokuPuzzleCell::NOT_FOUND;
                }
            }
        }

        if found_singles {
            for i in 0..9 {
                for j in 0..9 {
                    let found_single: bool = singles[i][j] != SudokuPuzzleCell::NOT_FOUND;
                    if found_single {
                        let k = singles[i][j];
                        {
                            let ii = i + 1;
                            let jj = j + 1;
                            let kk = k + 1;
                            println!("found naked single {kk} at cell({ii}, {jj}).");
                        }
                        // mark cell as a single
                        self.solve_cell(i, j, k);
                    }
                }
            }
        }

        return found_singles;
    }

    fn find_horizontal_exclusions(
        &mut self,
        assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32,
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] = [CellCoordinate {
            row: SudokuPuzzleCell::NOT_FOUND,
            column: SudokuPuzzleCell::NOT_FOUND,
        }; 9];

        for row in 0..9 {
            for column in 0..9 {
                cells[column].row = row;
                cells[column].column = column;
            }
            let description: String = format!("for row {}", row + 1);
            // let puzzle: &mut SudokuPuzzle = self;
            num_changes += assist(self, &cells, &description);
        }

        return num_changes > 0;
    }

    fn find_vertical_exclusions(
        &mut self,
        assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32,
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] = [CellCoordinate {
            row: SudokuPuzzleCell::NOT_FOUND,
            column: SudokuPuzzleCell::NOT_FOUND,
        }; 9];

        for column in 0..9 {
            for row in 0..9 {
                cells[row].row = row;
                cells[row].column = column;
            }
            let description: String = format!("for column {}", column + 1);
            num_changes += assist(self, &cells, &description);
        }

        return num_changes > 0;
    }

    fn find_grid_exclusions(
        &mut self,
        assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32,
    ) -> bool {
        let mut num_changes: u32 = 0;
        let mut cells: [CellCoordinate; 9] = [CellCoordinate {
            row: SudokuPuzzleCell::NOT_FOUND,
            column: SudokuPuzzleCell::NOT_FOUND,
        }; 9];

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;

                let mut grid_index: usize = 0;
                for row in starti..starti + 3 {
                    for column in startj..startj + 3 {
                        cells[grid_index].row = row;
                        cells[grid_index].column = column;
                        grid_index += 1;
                    }
                }
                let description: String = format!("for grid({}, {})", gridi + 1, gridj + 1);
                num_changes += assist(self, &cells, &description);
            }
        }

        return num_changes > 0;
    }

    fn find_exclusions(
        &mut self,
        assist: fn(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32,
    ) -> bool {
        let mut found_exclusions: bool = false;

        found_exclusions |= self.find_horizontal_exclusions(assist);
        found_exclusions |= self.find_vertical_exclusions(assist);
        found_exclusions |= self.find_grid_exclusions(assist);

        return found_exclusions;
    }

    pub fn find_hidden_singles(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_hidden_singles_assist);
    }

    fn find_hidden_singles_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes = 0;

        for k in 0..9 {
            let mut num_possibilities = 0;
            let mut last_found: usize = SudokuPuzzleCell::NOT_FOUND;
            for c in 0..9 {
                let row: usize = cells[c].row;
                let column: usize = cells[c].column;
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        num_possibilities += 1;
                        last_found = c;
                    }
                }
            }

            let got_hidden_single: bool = num_possibilities == 1;
            if got_hidden_single {
                let row: usize = cells[last_found].row;
                let column: usize = cells[last_found].column;
                println!(
                    "found hidden single {} in cell({}, {}) {}.",
                    k + 1,
                    row + 1,
                    column + 1,
                    description
                );
                puzzle.solve_cell(row, column, k);
                num_changes += 1;
            }
        }

        return num_changes;
    }

    fn find_locked_candidate_horizontal(&mut self) -> bool {
        let mut num_changes: u32 = 0;
        let mut candidate: [[[bool; 9]; 3]; 9] = [[[false; 9]; 3]; 9];

        for row in 0..9 {
            for k in 0..9 {
                candidate[row][0][k] = false;
                candidate[row][1][k] = false;
                candidate[row][2][k] = false;
            }
            for k in 0..9 {
                for column in 0..9 {
                    if !self.cells[row][column].is_solved() {
                        let gridj: usize = column / 3;
                        candidate[row][gridj][k] |= self.cells[row][column].is_possible(k);
                    }
                }
            }
        }

        //
        // Locked Candidate Type 1
        // if single is in the grid's row/column but is not in the grid's other row/column,
        // then we can exclude the single from the rest of the row/column.
        // IOW, a candidate for a grid is only in a single row/column and thus the grid
        // constraint forces the candidate out of the remainder of the row/column.
        //

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;
                for k in 0..9 {
                    let mut found_locked_candidate: bool = false;
                    let mut locked_row: usize = SudokuPuzzleCell::NOT_FOUND;

                    if candidate[starti][gridj][k]
                        && !candidate[starti + 1][gridj][k]
                        && !candidate[starti + 2][gridj][k]
                    {
                        found_locked_candidate = true;
                        locked_row = starti;
                    }
                    if !candidate[starti][gridj][k]
                        && candidate[starti + 1][gridj][k]
                        && !candidate[starti + 2][gridj][k]
                    {
                        found_locked_candidate = true;
                        locked_row = starti + 1;
                    }
                    if !candidate[starti][gridj][k]
                        && !candidate[starti + 1][gridj][k]
                        && candidate[starti + 2][gridj][k]
                    {
                        found_locked_candidate = true;
                        locked_row = starti + 2;
                    }

                    if found_locked_candidate {
                        println!(
                            "found type 1 locked candidate {0} in row cells({1},{2}..{3}), exclude the rest of the row",
                            k + 1,
                            locked_row + 1,
                            startj + 1,
                            startj + 3
                        );
                        for column in 0..9 {
                            let this_grid_column: usize = column / 3;
                            let grid_needing_exclusion: bool = this_grid_column != gridj;
                            if grid_needing_exclusion {
                                if self.cells[locked_row][column].is_possible(k) {
                                    num_changes += 1;
                                    self.cells[locked_row][column].set_possible(k, false);
                                    println!("    exclude {} from cell({}, {}).", k + 1, locked_row + 1, column + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        //
        // Locked Candidate Type 2
        // if the single is in the grid's row/column but not in the rest of the row/column,
        // then we can restrict the single from the rest of the grid.
        // IOW, a candidate for a row/column is only in a single grid and the the row/column
        // constraint forces the candidate out of the remainder of the grid.
        //

        for row in 0..9 {
            for k in 0..9 {
                let mut found_locked_candidate: bool = false;
                let mut startj: usize = SudokuPuzzleCell::NOT_FOUND;

                if candidate[row][0][k] && !candidate[row][1][k] && !candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 0;
                }
                if !candidate[row][0][k] && candidate[row][1][k] && !candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 3;
                }
                if !candidate[row][0][k] && !candidate[row][1][k] && candidate[row][2][k] {
                    found_locked_candidate = true;
                    startj = 6;
                }

                if found_locked_candidate {
                    let gridj: usize = startj / 3;
                    let starti: usize = row / 3 * 3;
                    let found_possibilities_to_remove: bool = (starti != row && candidate[starti][gridj][k]
                        || starti + 1 != row && candidate[starti + 1][gridj][k]
                        || starti + 2 != row && candidate[starti + 2][gridj][k]);
                    if found_possibilities_to_remove {
                        println!(
                            "found type 2 locked candidate {} in row cells({},{}..{}), exclude the rest of the grid",
                            k + 1,
                            row + 1,
                            startj + 1,
                            startj + 3
                        );
                        for i in starti..starti + 3 {
                            let row_needing_exclusion: bool = i != row;
                            if row_needing_exclusion {
                                for j in startj..startj + 3 {
                                    if self.cells[i][j].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[i][j].set_possible(k, false);
                                        println!("    exclude {} from cell({}, {}).", k + 1, i + 1, j + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    fn find_locked_candidate_vertical(&mut self) -> bool {
        let mut num_changes: u32 = 0;
        let mut candidate: [[[bool; 9]; 3]; 9] = [[[false; 9]; 3]; 9];

        for column in 0..9 {
            for k in 0..9 {
                candidate[column][0][k] = false;
                candidate[column][1][k] = false;
                candidate[column][2][k] = false;
            }
            for k in 0..9 {
                for row in 0..9 {
                    if !self.cells[row][column].is_solved() {
                        let gridi: usize = row / 3;
                        candidate[column][gridi][k] |= self.cells[row][column].is_possible(k);
                    }
                }
            }
        }

        //
        // Locked Candidate Type 1
        // if single is in the grid's row/column but is not in the grid's other row/column,
        // then we can exclude the single from the rest of the row/column
        //

        for gridi in 0..3 {
            for gridj in 0..3 {
                let starti: usize = gridi * 3;
                let startj: usize = gridj * 3;
                for k in 0..9 {
                    let mut found_locked_candidate: bool = false;
                    let mut locked_column: usize = SudokuPuzzleCell::NOT_FOUND;
                    // if ( Candidate[startj, gridi, k] && !Candidate[startj + 1, gridi, k] && !Candidate[startj + 2, gridi, k] )
                    if candidate[startj][gridi][k]
                        && !candidate[startj + 1][gridi][k]
                        && !candidate[startj + 2][gridi][k]
                    {
                        found_locked_candidate = true;
                        locked_column = startj;
                    }
                    if !candidate[startj][gridi][k]
                        && candidate[startj + 1][gridi][k]
                        && !candidate[startj + 2][gridi][k]
                    {
                        found_locked_candidate = true;
                        locked_column = startj + 1;
                    }
                    if !candidate[startj][gridi][k]
                        && !candidate[startj + 1][gridi][k]
                        && candidate[startj + 2][gridi][k]
                    {
                        found_locked_candidate = true;
                        locked_column = startj + 2;
                    }
                    if found_locked_candidate {
                        // row includes
                        //     candidate[locked_row][0][k]
                        //     candidate[locked_row][1][k]
                        //     candidate[locked_row][2][k]
                        let found_possibilities_to_remove: bool = (gridi != 0 && candidate[locked_column][0][k]
                            || gridi != 1 && candidate[locked_column][1][k]
                            || gridi != 2 && candidate[locked_column][2][k]);
                        if found_possibilities_to_remove {
                            println!(
                                "found type 1 locked candidate {} in column cells({}..{},{}), exclude the rest of the column",
                                k+1, starti+1, starti+3, locked_column+1
                            );
                            for row in 0..9 {
                                let this_grid_row: usize = row / 3;
                                let grid_needing_exclusion: bool = this_grid_row != gridi;
                                if grid_needing_exclusion {
                                    if self.cells[row][locked_column].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[row][locked_column].set_possible(k, false);
                                        println!(
                                            "    exclude {} from cell({}, {}).",
                                            k + 1,
                                            row + 1,
                                            locked_column + 1
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //
        // Locked Candidate Type 2
        // if the single is in the grid's row/column but not in the rest of the row/column,
        // then we can restrict the single from the rest of the grid
        //

        for column in 0..9 {
            for k in 0..9 {
                let mut found_locked_candidate = false;
                let mut starti: usize = SudokuPuzzleCell::NOT_FOUND;
                // Candidate[column, 0, k] && !Candidate[column, 1, k] && !Candidate[column, 2, k] )
                if candidate[column][0][k] && !candidate[column][1][k] && !candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 0;
                }
                if !candidate[column][0][k] && candidate[column][1][k] && !candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 3;
                }
                if !candidate[column][0][k] && !candidate[column][1][k] && candidate[column][2][k] {
                    found_locked_candidate = true;
                    starti = 6;
                }
                if found_locked_candidate {
                    let gridi = starti / 3;
                    let startj: usize = column / 3 * 3;
                    // grid includes
                    //     candidate[starti][gridj][k]
                    //     candidate[starti+1][gridj][k]
                    //     candidate[starti+2][gridj][k]
                    let found_possibilities_to_remove: bool = ((startj != column && candidate[startj][gridi][k])
                        || (startj + 1 != column && candidate[startj + 1][gridi][k])
                        || (startj + 2 != column && candidate[startj + 2][gridi][k]));
                    if found_possibilities_to_remove {
                        println!(
                            "found type 2 locked candidated {} in column cells({}..{},{}), exclude the rest of the grid",
                            k+1, starti+1, starti+3, column+1
                        );
                        for j in startj..startj + 3 {
                            let column_needing_exclusion: bool = j != column;
                            if column_needing_exclusion {
                                for i in starti..starti + 3 {
                                    if self.cells[i][j].is_possible(k) {
                                        num_changes += 1;
                                        self.cells[i][j].set_possible(k, false);
                                        println!("    excluding {} in cell({}, {}).", k + 1, i + 1, j + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    //
    // Locked Candidate Type 1
    // if single is in the grid's row/column but is not in the grid's other row/column,
    // then we can exclude the single from the rest of the row/column.
    // IOW, a candidate for a grid is only in a single row/column and thus the grid
    // constraint forces the candidate out of the remainder of the row/column.
    //
    // Locked Candidate Type 2
    // if the single is in the grid's row/column but not in the rest of the row/column,
    // then we can restrict the single from the rest of the grid.
    // IOW, a candidate for a row/column is only in a single grid and the the row/column
    // constraint forces the candidate out of the remainder of the grid.
    //

    fn find_locked_candidate(&mut self) -> bool {
        let mut did_something: bool = false;
        did_something |= self.find_locked_candidate_horizontal();
        did_something |= self.find_locked_candidate_vertical();
        return did_something;
    }

    fn find_naked_pairs_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut candidate_pairs: [(usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];
        let mut num_candidate_pairs: usize = 0;

        // File "iter()" and iter_mut" under ... WTF.
        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;

            let mut p1: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut p2: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut num_possibilities: u32 = 0;

            for k in 0..9 {
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        num_possibilities += 1;
                        if num_possibilities > 2 {
                            break;
                        }
                        if p1 == SudokuPuzzleCell::NOT_FOUND {
                            p1 = k;
                        } else {
                            p2 = k;
                        }
                    }
                }
            }

            if num_possibilities == 2 {
                // we have found a pair candidate
                candidate_pairs[num_candidate_pairs] = (p1, p2, c);
                num_candidate_pairs += 1;
            }
        }

        let mut num_naked_pairs: usize = 0;
        let mut naked_pairs: [(usize, usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];

        if num_candidate_pairs >= 2 {
            for c1 in 0_usize..num_candidate_pairs - 1 {
                for c2 in c1 + 1..num_candidate_pairs {
                    // determine if candidate_pairs[c1] and candidate_pairs[c2]
                    // are candidate pairs.  do they reference the same pair?

                    let mut p1: usize = SudokuPuzzleCell::NOT_FOUND;
                    let mut p2: usize = SudokuPuzzleCell::NOT_FOUND;
                    let mut possibles_for_pair: [bool; 9] = [false; 9];

                    (p1, p2, _) = candidate_pairs[c1];
                    possibles_for_pair[p1] = true;
                    possibles_for_pair[p2] = true;

                    (p1, p2, _) = candidate_pairs[c2];
                    possibles_for_pair[p1] = true;
                    possibles_for_pair[p2] = true;

                    let mut num_possibles_for_pairs: u32 = 0;
                    for k in 0..9 {
                        if possibles_for_pair[k] {
                            num_possibles_for_pairs += 1;
                            if num_possibles_for_pairs > 2 {
                                break;
                            }
                        }
                    }
                    if num_possibles_for_pairs == 2 {
                        // candidate_pairs[c1] and candidate_pairs[c2]
                        // have the same pair: (p1, p2).
                        naked_pairs[num_naked_pairs] = (
                            candidate_pairs[c1].0,
                            candidate_pairs[c1].1,
                            candidate_pairs[c1].2, // index into cells[]
                            candidate_pairs[c2].2,
                        );
                        num_naked_pairs += 1;
                    }
                }
            }
        }

        if num_naked_pairs > 0 {
            for t in 0..num_naked_pairs {
                let (p1, p2, c1, c2) = naked_pairs[t];

                let mut num_changes_for_pair: u32 = 0;
                for (c, cell) in cells.iter().enumerate() {
                    let cell_outside_of_pair: bool = (c != c1) && (c != c2);
                    if !cell_outside_of_pair {
                        continue;
                    }

                    let row: usize = cell.row;
                    let column: usize = cell.column;
                    if !puzzle.cells[row][column].is_solved() {
                        if puzzle.cells[row][column].is_possible(p1) {
                            puzzle.cells[row][column].set_possible(p1, false);
                            num_changes_for_pair += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(p2) {
                            puzzle.cells[row][column].set_possible(p2, false);
                            num_changes_for_pair += 1;
                            num_changes += 1;
                        }
                    }
                }
                if num_changes_for_pair > 0 {
                    println!("found naked pair ({}, {}) {}", p1 + 1, p2 + 1, description);
                } else {
                    println!("xxx found naked pair ({}, {}) {}", p1 + 1, p2 + 1, description);
                }
            }
        }

        return num_changes;
    }

    fn find_naked_pairs(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_naked_pairs_assist);
    }

    fn find_naked_triples_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut num_candidate_triples: usize = 0;
        let mut candidate_triples: [(usize, usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];

        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;

            let mut t1: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut t2: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut t3: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut num_possibilities: u32 = 0;

            for k in 0..9 {
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        num_possibilities += 1;
                        if num_possibilities > 3 {
                            break;
                        }
                        if t1 == SudokuPuzzleCell::NOT_FOUND {
                            t1 = k;
                        } else if t2 == SudokuPuzzleCell::NOT_FOUND {
                            t2 = k;
                        } else {
                            t3 = k;
                        }
                    }
                }
            }

            // (1, 2) (2, 3) (1, 3) is a naked triple
            if num_possibilities == 3 || num_possibilities == 2 {
                candidate_triples[num_candidate_triples] = (t1, t2, t3, c); // t3 could be NOT_FOUND
                num_candidate_triples += 1;
            }
        }

        let mut num_naked_triples: usize = 0;
        let mut naked_triples: [(usize, usize, usize, usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];

        if num_candidate_triples >= 3 {
            for c1 in 0_usize..num_candidate_triples {
                for c2 in c1 + 1..num_candidate_triples {
                    for c3 in c2 + 1..num_candidate_triples {
                        let mut possibles_for_triple: [bool; 9] = [false; 9];

                        let (t1, t2, t3, _) = candidate_triples[c1];
                        possibles_for_triple[t1] = true;
                        possibles_for_triple[t2] = true;
                        if t3 != SudokuPuzzleCell::NOT_FOUND {
                            possibles_for_triple[t3] = true;
                        }

                        let (t1, t2, t3, _) = candidate_triples[c2];
                        possibles_for_triple[t1] = true;
                        possibles_for_triple[t2] = true;
                        if t3 != SudokuPuzzleCell::NOT_FOUND {
                            possibles_for_triple[t3] = true;
                        }

                        let (t1, t2, t3, _) = candidate_triples[c3];
                        possibles_for_triple[t1] = true;
                        possibles_for_triple[t2] = true;
                        if t3 != SudokuPuzzleCell::NOT_FOUND {
                            possibles_for_triple[t3] = true;
                        }

                        let mut num_possibles_for_triple: u32 = 0;
                        let mut t1: usize = SudokuPuzzleCell::NOT_FOUND;
                        let mut t2: usize = SudokuPuzzleCell::NOT_FOUND;
                        let mut t3: usize = SudokuPuzzleCell::NOT_FOUND;
                        for k in 0..9 {
                            if possibles_for_triple[k] {
                                num_possibles_for_triple += 1;
                                if t1 == SudokuPuzzleCell::NOT_FOUND {
                                    t1 = k;
                                } else if t2 == SudokuPuzzleCell::NOT_FOUND {
                                    t2 = k;
                                } else if t3 == SudokuPuzzleCell::NOT_FOUND {
                                    t3 = k;
                                }
                                if num_possibles_for_triple > 3 {
                                    break;
                                }
                            }
                        }
                        if num_possibles_for_triple == 3 {
                            naked_triples[num_naked_triples] = (
                                t1,
                                t2,
                                t3,
                                candidate_triples[c1].3, // .3 => index into cells[]
                                candidate_triples[c2].3,
                                candidate_triples[c3].3,
                            );
                            num_naked_triples += 1;
                        }
                    }
                }
            }
        }

        if num_naked_triples > 0 {
            for t in 0..num_naked_triples {
                let (t1, t2, t3, c1, c2, c3) = naked_triples[t];
                let mut num_changes_for_triple = 0;
                for c in 0..9 {
                    let cell_outside_of_triple: bool = (c != c1) && (c != c2) && (c != c3);
                    if !cell_outside_of_triple {
                        continue;
                    }

                    let row: usize = cells[c].row;
                    let column: usize = cells[c].column;

                    if !puzzle.cells[row][column].is_solved() {
                        if puzzle.cells[row][column].is_possible(t1) {
                            puzzle.cells[row][column].set_possible(t1, false);
                            num_changes_for_triple += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(t2) {
                            puzzle.cells[row][column].set_possible(t2, false);
                            num_changes_for_triple += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(t3) {
                            puzzle.cells[row][column].set_possible(t3, false);
                            num_changes_for_triple += 1;
                            num_changes += 1;
                        }
                    }
                }
                if num_changes_for_triple > 0 {
                    println!("found naked triple ({} {} {}) {}", t1 + 1, t2 + 1, t3 + 1, description);
                } else {
                    println!(
                        "xxx found naked triple ({} {} {}) {}",
                        t1 + 1,
                        t2 + 1,
                        t3 + 1,
                        description
                    );
                }
            }
        }

        return num_changes;
    }

    fn find_naked_triples(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_naked_triples_assist);
    }

    fn find_naked_quads_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut num_candidate_quads: usize = 0;
        let mut candidate_quads: [(usize, usize, usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];

        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;

            let mut t1: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut t2: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut t3: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut t4: usize = SudokuPuzzleCell::NOT_FOUND;
            let mut num_possibilities: u32 = 0;

            for k in 0..9 {
                if !puzzle.cells[row][column].is_solved() {
                    if puzzle.cells[row][column].is_possible(k) {
                        num_possibilities += 1;
                        if num_possibilities > 4 {
                            break;
                        }
                        if t1 == SudokuPuzzleCell::NOT_FOUND {
                            t1 = k;
                        } else if t2 == SudokuPuzzleCell::NOT_FOUND {
                            t2 = k;
                        } else if t3 == SudokuPuzzleCell::NOT_FOUND {
                            t3 = k;
                        } else {
                            t4 = k;
                        }
                    }
                }
            }

            // (1, 2) (2, 3) (3, 4) (1, 4) is a naked quad
            if num_possibilities == 4 || num_possibilities == 3 || num_possibilities == 2 {
                candidate_quads[num_candidate_quads] = (t1, t2, t3, t4, c); // t3/t4 could be NOT_FOUND
                num_candidate_quads += 1;
            }
        }

        let mut num_naked_quads: usize = 0;
        let mut naked_quads: [(usize, usize, usize, usize, usize, usize, usize, usize); 9] = [(
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
            SudokuPuzzleCell::NOT_FOUND,
        ); 9];

        if num_candidate_quads >= 4 {
            for c1 in 0_usize..num_candidate_quads {
                for c2 in c1 + 1..num_candidate_quads {
                    for c3 in c2 + 1..num_candidate_quads {
                        for c4 in c3 + 1..num_candidate_quads {
                            let mut possibles_for_quad: [bool; 9] = [false; 9];

                            let (t1, t2, t3, t4, _) = candidate_quads[c1];
                            possibles_for_quad[t1] = true;
                            possibles_for_quad[t2] = true;
                            if t3 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t3] = true;
                            }
                            if t4 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t4] = true;
                            }

                            let (t1, t2, t3, t4, _) = candidate_quads[c2];
                            possibles_for_quad[t1] = true;
                            possibles_for_quad[t2] = true;
                            if t3 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t3] = true;
                            }
                            if t4 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t4] = true;
                            }

                            let (t1, t2, t3, t4, _) = candidate_quads[c3];
                            possibles_for_quad[t1] = true;
                            possibles_for_quad[t2] = true;
                            if t3 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t3] = true;
                            }
                            if t4 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t4] = true;
                            }

                            let (t1, t2, t3, t4, _) = candidate_quads[c4];
                            possibles_for_quad[t1] = true;
                            possibles_for_quad[t2] = true;
                            if t3 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t3] = true;
                            }
                            if t4 != SudokuPuzzleCell::NOT_FOUND {
                                possibles_for_quad[t4] = true;
                            }

                            let mut num_possibles_for_quad: u32 = 0;
                            let mut t1: usize = SudokuPuzzleCell::NOT_FOUND;
                            let mut t2: usize = SudokuPuzzleCell::NOT_FOUND;
                            let mut t3: usize = SudokuPuzzleCell::NOT_FOUND;
                            let mut t4: usize = SudokuPuzzleCell::NOT_FOUND;
                            for k in 0..9 {
                                if possibles_for_quad[k] {
                                    num_possibles_for_quad += 1;
                                    if t1 == SudokuPuzzleCell::NOT_FOUND {
                                        t1 = k;
                                    } else if t2 == SudokuPuzzleCell::NOT_FOUND {
                                        t2 = k;
                                    } else if t3 == SudokuPuzzleCell::NOT_FOUND {
                                        t3 = k;
                                    } else if t4 == SudokuPuzzleCell::NOT_FOUND {
                                        t4 = k;
                                    }
                                    if num_possibles_for_quad > 4 {
                                        break;
                                    }
                                }
                            }
                            if num_possibles_for_quad == 4 {
                                naked_quads[num_naked_quads] = (
                                    t1,
                                    t2,
                                    t3,
                                    t4,
                                    candidate_quads[c1].4, // .3 => index into cells[]
                                    candidate_quads[c2].4,
                                    candidate_quads[c3].4,
                                    candidate_quads[c4].4,
                                );
                                num_naked_quads += 1;
                            }
                        }
                    }
                }
            }
        }

        if num_naked_quads > 0 {
            for t in 0..num_naked_quads {
                let (t1, t2, t3, t4, c1, c2, c3, c4) = naked_quads[t];
                let mut num_changes_for_quad = 0;
                for c in 0..9 {
                    let cell_outside_of_quad: bool = (c != c1) && (c != c2) && (c != c3) && (c != c4);
                    if !cell_outside_of_quad {
                        continue;
                    }

                    let row: usize = cells[c].row;
                    let column: usize = cells[c].column;

                    if !puzzle.cells[row][column].is_solved() {
                        if puzzle.cells[row][column].is_possible(t1) {
                            puzzle.cells[row][column].set_possible(t1, false);
                            num_changes_for_quad += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(t2) {
                            puzzle.cells[row][column].set_possible(t2, false);
                            num_changes_for_quad += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(t3) {
                            puzzle.cells[row][column].set_possible(t3, false);
                            num_changes_for_quad += 1;
                            num_changes += 1;
                        }
                        if puzzle.cells[row][column].is_possible(t4) {
                            puzzle.cells[row][column].set_possible(t4, false);
                            num_changes_for_quad += 1;
                            num_changes += 1;
                        }
                    }
                }
                if num_changes_for_quad > 0 {
                    println!(
                        "found naked quad ({} {} {} {}) {}",
                        t1 + 1,
                        t2 + 1,
                        t3 + 1,
                        t4 + 1,
                        description
                    );
                } else {
                    println!(
                        "xxx found naked quad ({} {} {} {}) {}",
                        t1 + 1,
                        t2 + 1,
                        t3 + 1,
                        t4 + 1,
                        description
                    );
                }
            }
        }

        return num_changes;
    }

    fn find_naked_quads(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_naked_quads_assist);
    }

    fn find_hidden_pairs_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut num_possibles_pairs: [usize; 9] = [0; 9];
        let mut possible_pairs: [bool; 9] = [false; 9];
        let mut possible_pair_locations: [[usize; 2]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 2]; 9]; // 9 x 2 array

        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;
            if !puzzle.cells[row][column].is_solved() {
                for k in 0..9 {
                    if puzzle.cells[row][column].is_possible(k) {
                        if num_possibles_pairs[k] < 2 {
                            possible_pair_locations[k][num_possibles_pairs[k]] = c;
                        }
                        num_possibles_pairs[k] += 1;
                        possible_pairs[k] = num_possibles_pairs[k] == 2;
                    }
                }
            }
        }

        for p1 in 0..9 {
            if possible_pairs[p1] {
                for p2 in p1 + 1..9 {
                    if possible_pairs[p2] {
                        let share_same_two_cells: bool = (possible_pair_locations[p1][0]
                            == possible_pair_locations[p2][0]
                            && possible_pair_locations[p1][1] == possible_pair_locations[p2][1]);
                        if share_same_two_cells {
                            let c1: usize = possible_pair_locations[p1][0];
                            let c2: usize = possible_pair_locations[p1][1];

                            // found a hidden pair ( p1 p2 ) in cells c1 and c2

                            let row1: usize = cells[c1].row;
                            let column1: usize = cells[c1].column;
                            let row2: usize = cells[c2].row;
                            let column2: usize = cells[c2].column;

                            // for cells outside of c1 and c2, exclude p1 and p2 possibilities

                            let mut num_changes_this_pair: u32 = 0;
                            for k in 0..9 {
                                let not_in_pair: bool = k != p1 && k != p2;
                                if not_in_pair {
                                    if puzzle.cells[row1][column1].is_possible(k) {
                                        puzzle.cells[row1][column1].set_possible(k, false);
                                        num_changes_this_pair += 1;
                                        num_changes += 1;
                                    }
                                    if puzzle.cells[row2][column2].is_possible(k) {
                                        puzzle.cells[row2][column2].set_possible(k, false);
                                        num_changes_this_pair += 1;
                                        num_changes += 1;
                                    }
                                }
                            }
                            if num_changes_this_pair > 0 {
                                println!(
                                    "found hidden pair ({} {}) at cell({}, {}) and cell({}, {}), {}",
                                    p1 + 1,
                                    p2 + 1,
                                    row1 + 1,
                                    column1 + 1,
                                    row2 + 1,
                                    column2 + 1,
                                    description
                                );
                            } else {
                                println!(
                                    "xxx found hidden pair ({} {}) at cell({}, {}) and cell({}, {}), {}",
                                    p1 + 1,
                                    p2 + 1,
                                    row1 + 1,
                                    column1 + 1,
                                    row2 + 1,
                                    column2 + 1,
                                    description
                                );
                            }
                        }
                    }
                }
            }
        }

        return num_changes;
    }

    fn find_hidden_pairs(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_hidden_pairs_assist);
    }

    fn find_hidden_triples_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut num_possibles_triples: [usize; 9] = [0; 9];
        let mut possible_triples: [bool; 9] = [false; 9];
        let mut possible_triple_locations: [[usize; 3]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 3]; 9]; // 9 x 3 array

        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;
            if !puzzle.cells[row][column].is_solved() {
                for k in 0..9 {
                    if puzzle.cells[row][column].is_possible(k) {
                        if num_possibles_triples[k] < 3 {
                            possible_triple_locations[k][num_possibles_triples[k]] = c;
                        }
                        num_possibles_triples[k] += 1;
                        possible_triples[k] = (num_possibles_triples[k] == 2 || num_possibles_triples[k] == 3);
                    }
                }
            }
        }

        for p1 in 0..9 {
            if possible_triples[p1] {
                for p2 in p1 + 1..9 {
                    if possible_triples[p2] {
                        for p3 in p2 + 1..9 {
                            if possible_triples[p3] {
                                let mut triple_cells: [bool; 9] = [false; 9];

                                let c1: usize = possible_triple_locations[p1][0];
                                let c2: usize = possible_triple_locations[p1][1];
                                let c3: usize = possible_triple_locations[p1][2];
                                triple_cells[c1] = true;
                                triple_cells[c2] = true;
                                if num_possibles_triples[p1] == 3 {
                                    triple_cells[c3] = true;
                                }

                                let c1: usize = possible_triple_locations[p2][0];
                                let c2: usize = possible_triple_locations[p2][1];
                                let c3: usize = possible_triple_locations[p2][2];
                                triple_cells[c1] = true;
                                triple_cells[c2] = true;
                                if num_possibles_triples[p2] == 3 {
                                    triple_cells[c3] = true;
                                }

                                let c1: usize = possible_triple_locations[p3][0];
                                let c2: usize = possible_triple_locations[p3][1];
                                let c3: usize = possible_triple_locations[p3][2];
                                triple_cells[c1] = true;
                                triple_cells[c2] = true;
                                if num_possibles_triples[p3] == 3 {
                                    triple_cells[c3] = true;
                                }

                                let num_triple_cells: u32 = {
                                    let mut num = 0;
                                    for i in 0..9 {
                                        if triple_cells[i] {
                                            num += 1;
                                        }
                                    }
                                    num
                                };
                                let share_same_three_cells: bool = num_triple_cells == 3;

                                if share_same_three_cells {
                                    let mut c1: usize = SudokuPuzzleCell::NOT_FOUND;
                                    let mut c2: usize = SudokuPuzzleCell::NOT_FOUND;
                                    let mut c3: usize = SudokuPuzzleCell::NOT_FOUND;
                                    for c in 0..9 {
                                        if triple_cells[c] {
                                            if c1 == SudokuPuzzleCell::NOT_FOUND {
                                                c1 = c;
                                            } else if c2 == SudokuPuzzleCell::NOT_FOUND {
                                                c2 = c;
                                            } else if c3 == SudokuPuzzleCell::NOT_FOUND {
                                                c3 = c;
                                            }
                                        }
                                    }

                                    // found a hidden triple (p1, p2, p3) in cells c1 and c2 and c3

                                    let row1: usize = cells[c1].row;
                                    let column1: usize = cells[c1].column;
                                    let row2: usize = cells[c2].row;
                                    let column2: usize = cells[c2].column;
                                    let row3: usize = cells[c3].row;
                                    let column3: usize = cells[c3].column;

                                    // for cells outside of c1 and c2 and c3, exclude p1 and p2 and p3 posibilities

                                    let mut num_changes_this_triple: u32 = 0;
                                    for k in 0..9 {
                                        let not_in_triple: bool = k != p1 && k != p2 && k != p3;
                                        if not_in_triple {
                                            if puzzle.cells[row1][column1].is_possible(k) {
                                                puzzle.cells[row1][column1].set_possible(k, false);
                                                num_changes_this_triple += 1;
                                                num_changes += 1;
                                            }
                                            if puzzle.cells[row2][column2].is_possible(k) {
                                                puzzle.cells[row2][column2].set_possible(k, false);
                                                num_changes_this_triple += 1;
                                                num_changes += 1;
                                            }
                                            if puzzle.cells[row3][column3].is_possible(k) {
                                                puzzle.cells[row3][column3].set_possible(k, false);
                                                num_changes_this_triple += 1;
                                                num_changes += 1;
                                            }
                                        }
                                    }
                                    if num_changes_this_triple > 0 {
                                        println!(
                                            "found hidden triple ({} {} {}) at cell({}, {}) and cell({}, {}) and cell({}, {}), {}",
                                            p1+1, p2+1, p3+1,
                                            row1+1, column1+1,
                                            row2+1, column2+1,
                                            row3+1, column3+1,
                                            description
                                        );
                                    } else {
                                        println!(
                                            "xxx found hidden triple ({} {} {}) at cell({}, {}) and cell({}, {}) and cell({}, {}), {}",
                                            p1+1, p2+1, p3+1,
                                            row1+1, column1+1,
                                            row2+1, column2+1,
                                            row3+1, column3+1,
                                            description
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes;
    }

    fn find_hidden_triples(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_hidden_triples_assist);
    }

    fn find_hidden_quads_assist(puzzle: &mut SudokuPuzzle, cells: &[CellCoordinate; 9], description: &String) -> u32 {
        let mut num_changes: u32 = 0;

        let mut num_possibles_quads: [usize; 9] = [0; 9];
        let mut possible_quads: [bool; 9] = [false; 9];
        let mut possible_quad_locations: [[usize; 4]; 9] = [[SudokuPuzzleCell::NOT_FOUND; 4]; 9]; // 9 x 4 array

        for (c, cell) in cells.iter().enumerate() {
            let row: usize = cell.row;
            let column: usize = cell.column;
            if !puzzle.cells[row][column].is_solved() {
                for k in 0..9 {
                    if puzzle.cells[row][column].is_possible(k) {
                        if num_possibles_quads[k] < 4 {
                            possible_quad_locations[k][num_possibles_quads[k]] = c;
                        }
                        num_possibles_quads[k] += 1;
                        possible_quads[k] =
                            (num_possibles_quads[k] == 2 || num_possibles_quads[k] == 3 || num_possibles_quads[k] == 4);
                    }
                }
            }
        }

        for p1 in 0..9 {
            if possible_quads[p1] {
                for p2 in p1 + 1..9 {
                    if possible_quads[p2] {
                        for p3 in p2 + 1..9 {
                            if possible_quads[p3] {
                                for p4 in p3 + 1..9 {
                                    if possible_quads[p4] {
                                        let mut quad_cells: [bool; 9] = [false; 9];

                                        let c1: usize = possible_quad_locations[p1][0];
                                        let c2: usize = possible_quad_locations[p1][1];
                                        let c3: usize = possible_quad_locations[p1][2];
                                        let c4: usize = possible_quad_locations[p1][3];
                                        quad_cells[c1] = true;
                                        quad_cells[c2] = true;
                                        if num_possibles_quads[p1] == 3 {
                                            quad_cells[c3] = true;
                                        } else if num_possibles_quads[p1] == 4 {
                                            quad_cells[c3] = true;
                                            quad_cells[c4] = true;
                                        }

                                        let c1: usize = possible_quad_locations[p2][0];
                                        let c2: usize = possible_quad_locations[p2][1];
                                        let c3: usize = possible_quad_locations[p2][2];
                                        let c4: usize = possible_quad_locations[p2][3];
                                        quad_cells[c1] = true;
                                        quad_cells[c2] = true;
                                        if num_possibles_quads[p2] == 3 {
                                            quad_cells[c3] = true;
                                        } else if num_possibles_quads[p2] == 4 {
                                            quad_cells[c3] = true;
                                            quad_cells[c4] = true;
                                        }

                                        let c1: usize = possible_quad_locations[p3][0];
                                        let c2: usize = possible_quad_locations[p3][1];
                                        let c3: usize = possible_quad_locations[p3][2];
                                        let c4: usize = possible_quad_locations[p3][3];
                                        quad_cells[c1] = true;
                                        quad_cells[c2] = true;
                                        if num_possibles_quads[p3] == 3 {
                                            quad_cells[c3] = true;
                                        } else if num_possibles_quads[p3] == 4 {
                                            quad_cells[c3] = true;
                                            quad_cells[c4] = true;
                                        }

                                        let c1: usize = possible_quad_locations[p4][0];
                                        let c2: usize = possible_quad_locations[p4][1];
                                        let c3: usize = possible_quad_locations[p4][2];
                                        let c4: usize = possible_quad_locations[p4][3];
                                        quad_cells[c1] = true;
                                        quad_cells[c2] = true;
                                        if num_possibles_quads[p4] == 3 {
                                            quad_cells[c3] = true;
                                        } else if num_possibles_quads[p4] == 4 {
                                            quad_cells[c3] = true;
                                            quad_cells[c4] = true;
                                        }

                                        let num_quad_cells: u32 = {
                                            let mut num = 0;
                                            for i in 0..9 {
                                                if quad_cells[i] {
                                                    num += 1;
                                                }
                                            }
                                            num
                                        };
                                        let share_same_four_cells: bool = num_quad_cells == 4;

                                        if share_same_four_cells {
                                            let mut c1: usize = SudokuPuzzleCell::NOT_FOUND;
                                            let mut c2: usize = SudokuPuzzleCell::NOT_FOUND;
                                            let mut c3: usize = SudokuPuzzleCell::NOT_FOUND;
                                            let mut c4: usize = SudokuPuzzleCell::NOT_FOUND;
                                            for c in 0..9 {
                                                if quad_cells[c] {
                                                    if c1 == SudokuPuzzleCell::NOT_FOUND {
                                                        c1 = c;
                                                    } else if c2 == SudokuPuzzleCell::NOT_FOUND {
                                                        c2 = c;
                                                    } else if c3 == SudokuPuzzleCell::NOT_FOUND {
                                                        c3 = c;
                                                    } else if c4 == SudokuPuzzleCell::NOT_FOUND {
                                                        c4 = c;
                                                    }
                                                }
                                            }

                                            // found a hidden quad (p1, p2, p3, p4) in cells c1 and c2 and c3 and c4

                                            let row1: usize = cells[c1].row;
                                            let column1: usize = cells[c1].column;
                                            let row2: usize = cells[c2].row;
                                            let column2: usize = cells[c2].column;
                                            let row3: usize = cells[c3].row;
                                            let column3: usize = cells[c3].column;
                                            let row4: usize = cells[c4].row;
                                            let column4: usize = cells[c4].column;

                                            // for cells outside of c1 and c2 and c3 and c4, exclude p1 and p2 and p3 and p4 posibilities

                                            let mut num_changes_this_quad: u32 = 0;
                                            for k in 0..9 {
                                                let not_in_quad: bool = k != p1 && k != p2 && k != p3 && k != p4;
                                                if not_in_quad {
                                                    if puzzle.cells[row1][column1].is_possible(k) {
                                                        puzzle.cells[row1][column1].set_possible(k, false);
                                                        num_changes_this_quad += 1;
                                                        num_changes += 1;
                                                    }
                                                    if puzzle.cells[row2][column2].is_possible(k) {
                                                        puzzle.cells[row2][column2].set_possible(k, false);
                                                        num_changes_this_quad += 1;
                                                        num_changes += 1;
                                                    }
                                                    if puzzle.cells[row3][column3].is_possible(k) {
                                                        puzzle.cells[row3][column3].set_possible(k, false);
                                                        num_changes_this_quad += 1;
                                                        num_changes += 1;
                                                    }
                                                    if puzzle.cells[row4][column4].is_possible(k) {
                                                        puzzle.cells[row4][column4].set_possible(k, false);
                                                        num_changes_this_quad += 1;
                                                        num_changes += 1;
                                                    }
                                                }
                                            }
                                            if num_changes_this_quad > 0 {
                                                println!(
                                                    "found hidden quad ({} {} {} {}) at cell({}, {}) and cell({}, {}) and cell({}, {}) and cell({}, {}), {}",
                                                    p1+1, p2+1, p3+1, p4+1,
                                                    row1+1, column1+1,
                                                    row2+1, column2+1,
                                                    row3+1, column3+1,
                                                    row4+1, column4+1,
                                                    description
                                                );
                                            } else {
                                                println!(
                                                    "xxx found hidden quad ({} {} {} {}) at cell({}, {}) and cell({}, {}) and cell({}, {}) and cell({}, {}), {}",
                                                    p1+1, p2+1, p3+1, p4+1,
                                                    row1+1, column1+1,
                                                    row2+1, column2+1,
                                                    row3+1, column3+1,
                                                    row4+1, column4+1,
                                                    description
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return num_changes;
    }

    fn find_hidden_quads(&mut self) -> bool {
        return self.find_exclusions(SudokuPuzzle::find_hidden_quads_assist);
    }

    fn find_xwing_rows(&mut self) -> bool {
        let mut num_changes = 0;

        for fish_candidate in 0..9 {
            for row1 in 0..9 {
                let mut num_row1_possibles_for_fish: usize = 0;
                let mut row1_columns: [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
                for column in 0..9 {
                    if self.cells[row1][column].is_possible(fish_candidate) {
                        row1_columns[num_row1_possibles_for_fish] = column;
                        num_row1_possibles_for_fish += 1;
                    }
                }
                if num_row1_possibles_for_fish != 2 {
                    continue;
                }

                for row2 in row1 + 1..9 {
                    let mut num_row2_possibles_for_fish: usize = 0;
                    let mut row2_columns: [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
                    for column in 0..9 {
                        if self.cells[row2][column].is_possible(fish_candidate) {
                            row2_columns[num_row2_possibles_for_fish] = column;
                            num_row2_possibles_for_fish += 1;
                        }
                    }
                    if num_row2_possibles_for_fish != 2 {
                        continue;
                    }

                    if row1_columns[0] != row2_columns[0] || row1_columns[1] != row2_columns[1] {
                        continue;
                    }

                    let column1: usize = row1_columns[0];
                    let column2: usize = row1_columns[1];

                    let mut num_changes_this_xwing = 0;
                    // look for exclusions in cover sets, column1 and column2.
                    for row in 0..9 {
                        if row == row1 || row == row2 {
                            // skip base set rows.
                            continue;
                        }
                        if self.cells[row][column1].is_possible(fish_candidate) {
                            num_changes += 1;
                            num_changes_this_xwing += 1;
                            self.cells[row][column1].set_possible(fish_candidate, false);
                            println!(
                                "found x-wing exclusion for fish candidate {} from cell({}, {}), given base set cells({}, {}) cells({}, {}) cells({}, {}) cells({}, {}).",
                                fish_candidate + 1,
                                row + 1, column1 + 1,
                                row1 + 1, column1 + 1,
                                row1 + 1, column2 + 1,
                                row2 + 1, column1 + 1,
                                row2 + 1, column2 + 1,
                            );
                        }
                        if self.cells[row][column2].is_possible(fish_candidate) {
                            num_changes += 1;
                            num_changes_this_xwing += 1;
                            self.cells[row][column2].set_possible(fish_candidate, false);
                            println!(
                                "found x-wing exclusion for fish candidate {} from cell({}, {}), given base set cells({}, {}) cells({}, {}) cells({}, {}) cells({}, {}).",
                                fish_candidate + 1,
                                row + 1, column2 + 1,
                                row1 + 1, column1 + 1,
                                row1 + 1, column2 + 1,
                                row2 + 1, column1 + 1,
                                row2 + 1, column2 + 1,
                            );
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    fn find_xwing_columns(&mut self) -> bool {
        let mut num_changes = 0;

        for fish_candidate in 0..9 {
            for column1 in 0..9 {
                let mut num_column1_possibles_for_fish: usize = 0;
                let mut column1_rows: [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
                for row in 0..9 {
                    if self.cells[row][column1].is_possible(fish_candidate) {
                        column1_rows[num_column1_possibles_for_fish] = row;
                        num_column1_possibles_for_fish += 1;
                    }
                }
                if num_column1_possibles_for_fish != 2 {
                    continue;
                }

                for column2 in column1 + 1..9 {
                    let mut num_column2_possibles_for_fish: usize = 0;
                    let mut column2_rows: [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
                    for row in 0..9 {
                        if self.cells[row][column2].is_possible(fish_candidate) {
                            column2_rows[num_column2_possibles_for_fish] = row;
                            num_column2_possibles_for_fish += 1;
                        }
                    }
                    if num_column2_possibles_for_fish != 2 {
                        continue;
                    }

                    if column1_rows[0] != column2_rows[0] || column1_rows[1] != column2_rows[1] {
                        continue;
                    }

                    let row1: usize = column1_rows[0];
                    let row2: usize = column1_rows[1];

                    let mut num_changes_this_xwing = 0;
                    // look for exclusions in cover sets, column1 and column2.
                    for column in 0..9 {
                        if column == column1 || column == column2 {
                            // skip base set columns.
                            continue;
                        }
                        if self.cells[row1][column].is_possible(fish_candidate) {
                            num_changes += 1;
                            num_changes_this_xwing += 1;
                            self.cells[row1][column].set_possible(fish_candidate, false);
                            println!(
                                "found x-wing exclusion for fish candidate {} from cell({}, {}), given base set cells({}, {}) cells({}, {}) cells({}, {}) cells({}, {}).",
                                fish_candidate + 1,
                                row1 + 1, column + 1,
                                row1 + 1, column1 + 1,
                                row1 + 1, column2 + 1,
                                row2 + 1, column1 + 1,
                                row2 + 1, column2 + 1,
                            );
                        }
                        if self.cells[row2][column].is_possible(fish_candidate) {
                            num_changes += 1;
                            num_changes_this_xwing += 1;
                            self.cells[row2][column].set_possible(fish_candidate, false);
                            println!(
                                "found x-wing exclusion for fish candidate {} from cell({}, {}), given base set cells({}, {}) cells({}, {}) cells({}, {}) cells({}, {}).",
                                fish_candidate + 1,
                                row2 + 1, column + 1,
                                row1 + 1, column1 + 1,
                                row1 + 1, column2 + 1,
                                row2 + 1, column1 + 1,
                                row2 + 1, column2 + 1,
                            );
                        }
                    }
                }
            }
        }

        return num_changes > 0;
    }

    fn find_xwing(&mut self) -> bool {
        let mut found_xwing: bool = false;
        found_xwing |= self.find_xwing_rows();
        found_xwing |= self.find_xwing_columns();
        return found_xwing;
    }

    fn find_xywing(&mut self) -> bool {
        // a whole lot of squeeze, for just a little juice.

        let mut num_changes: u32 = 0;

        let mut pairs: Vec<(usize, usize, usize, usize)> = Vec::new();
        for row in 0..9 {
            for column in 0..9 {
                // determine cell(row, column) contains a pair
                let mut num_possibles = 0;
                let mut pair: Vec<usize> = Vec::new();
                for k in 0..9 {
                    if self.cells[row][column].is_possible(k) {
                        num_possibles += 1;
                        pair.push(k);
                    }
                }
                if num_possibles == 2 {
                    pairs.push((row, column, pair[0], pair[1]));
                }
            }
        }

        // must have at least 3 pairs to find a xy wing
        if pairs.len() < 3 {
            return false;
        }

        //
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        // | 1 2 3 | |       | | 1   3 |  |       | |       | | 1     |
        // |       | | 4     | |       |  |   5   | |       | |       |
        // |       | |       | |       |  |       | |     9 | | 7     |
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        // |       | |       | |       |  |   2   | |       | | 1     |
        // |   5   | |       | |       |  |     6 | |       | |     6 |
        // |       | |     9 | | 7     |  |       | |    8  | |       |
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        // | 1 2   | |       | |       |  |   2   | |     3 | |       |
        // |       | |     6 | |       |  |       | |       | | 4     |
        // |       | |       | |   8   |  | 7     | |       | |       |
        // +-------+ +-------+ +-------+  +-------+ +-------+ +-------+
        //
        // xy wing:
        //     cell(3, 1) - wing1, shares a row with the pivot
        //     cell(3, 4) - pivot
        //     cell(1, 6) - wing2, shares a grid with the pivot
        //
        // pivot intersects with wing1 and wing2.
        // wing1 and wing2 do not intersect.
        //
        // one wing contains the pivot's 2
        // the other wing contains the pivot's 7
        //
        // if pivot is 7, wing2 is 1.
        // if pivot is 2, wing1 is 1.
        // => the cells which intersect with both wing1 and wing2 can not be 1.
        // the intersect cells, cell(1, 1) and cell(1, 3), can exclude 1.
        //
        // wing1 intersects with wing2's grid => cells(3, 4..6) can not be 1
        // wing2 intersects with wing1's grid => cells(1, 1..3) can not be 1
        // if wing1 and wing2 do not intersect each other's grid,
        // wing1 row and wing2 column intersect cell(3, 6) .. and
        // wing1 column and wing2 row intersect cell(1, 1).
        //

        for pivot_index in 0..pairs.len() {
            let pivot: (usize, usize, usize, usize) = pairs[pivot_index];
            for wing1_index in 0..pairs.len() {
                if wing1_index == pivot_index {
                    continue;
                }
                let wing1: (usize, usize, usize, usize) = pairs[wing1_index];
                for wing2_index in wing1_index + 1..pairs.len() {
                    if wing2_index == pivot_index {
                        continue;
                    }
                    let wing2: (usize, usize, usize, usize) = pairs[wing2_index];

                    // determine if pivot, wing1 and wing2 pairs fit xy ing pattern
                    // pivot pair is (x, y) and wing1 pair is (x, z) and pair pair is (y, z)
                    // x is only one of wing1 and wing2
                    // y is only one of wing1 and wing2
                    // z is in both wing1 and wing2

                    let x: usize = pivot.2;
                    let y: usize = pivot.3;

                    let x_in_wing1: bool = (wing1.2 == x || wing1.3 == x);
                    let x_in_wing2: bool = (wing2.2 == x || wing2.3 == x);
                    let y_in_wing1: bool = (wing1.2 == y || wing1.3 == y);
                    let y_in_wing2: bool = (wing2.2 == y || wing2.3 == y);

                    let x_only_in_one_wing: bool = (x_in_wing1 && !x_in_wing2) || (!x_in_wing1 && x_in_wing2);
                    let y_only_in_one_wing: bool = (y_in_wing1 && !y_in_wing2) || (!y_in_wing1 && y_in_wing2);
                    if !(x_only_in_one_wing && y_only_in_one_wing) {
                        continue;
                    }

                    let mut z_in_wing1: usize = SudokuPuzzleCell::NOT_FOUND;
                    if wing1.2 != x && wing1.2 != y {
                        z_in_wing1 = wing1.2;
                    } else if wing1.3 != x && wing1.3 != y {
                        z_in_wing1 = wing1.3;
                    }
                    if z_in_wing1 == SudokuPuzzleCell::NOT_FOUND {
                        // can't happen ...
                        continue;
                    }

                    let mut z_in_wing2: usize = SudokuPuzzleCell::NOT_FOUND;
                    if wing2.2 != x && wing2.2 != y {
                        z_in_wing2 = wing2.2;
                    } else if wing2.3 != x && wing2.3 != y {
                        z_in_wing2 = wing2.3;
                    }
                    if z_in_wing2 == SudokuPuzzleCell::NOT_FOUND {
                        // can't happen ...
                        continue;
                    }

                    if z_in_wing1 != z_in_wing2 {
                        continue;
                    }

                    let z: usize = z_in_wing1;

                    // pivot and wing1 must share row/column/grid
                    // pivot and wing2 must share row/column/grid
                    // wing1 and wing2 must not share row/column/grid

                    let pivot_and_wing1_share_row: bool = pivot.0 == wing1.0;
                    let pivot_and_wing1_share_column: bool = pivot.1 == wing1.1;

                    let pivot_grid_first_row: usize = pivot.0 / 3 * 3;
                    let pivot_grid_first_column: usize = pivot.1 / 3 * 3;

                    let wing1_grid_first_row: usize = wing1.0 / 3 * 3;
                    let wing1_grid_first_column: usize = wing1.1 / 3 * 3;
                    let pivot_and_wing1_share_grid: bool = (pivot_grid_first_row == wing1_grid_first_row
                        && pivot_grid_first_column == wing1_grid_first_column);

                    let pivot_and_wing2_share_row: bool = pivot.0 == wing2.0;
                    let pivot_and_wing2_share_column: bool = pivot.1 == wing2.1;

                    let wing2_grid_first_row: usize = wing2.0 / 3 * 3;
                    let wing2_grid_first_column: usize = wing2.1 / 3 * 3;
                    let pivot_and_wing2_share_grid: bool = pivot_grid_first_row == wing2_grid_first_row
                        && pivot_grid_first_column == wing2_grid_first_column;

                    let wing1_and_wing2_share_row: bool = wing1.0 == wing2.0;
                    let wing1_and_wing2_share_column: bool = wing1.1 == wing2.1;

                    let wing1_and_wing2_share_grid: bool = wing1_grid_first_row == wing2_grid_first_row
                        && wing1_grid_first_column == wing2_grid_first_column;

                    let pivot_and_wing1_are_compliant: bool =
                        pivot_and_wing1_share_row || pivot_and_wing1_share_column || pivot_and_wing1_share_grid;
                    let pivot_and_wing2_are_compliant: bool =
                        pivot_and_wing2_share_row || pivot_and_wing2_share_column || pivot_and_wing2_share_grid;
                    let wing1_and_wing2_are_compliant: bool =
                        !wing1_and_wing2_share_row && !wing1_and_wing2_share_column && !wing1_and_wing2_share_grid;

                    if !pivot_and_wing1_are_compliant {
                        continue;
                    }
                    if !pivot_and_wing2_are_compliant {
                        continue;
                    }
                    if !wing1_and_wing2_are_compliant {
                        continue;
                    }

                    // expect xy wing like:
                    //     pivot cell(7, 1) pair (7, 9)
                    //     wing1 cell(2, 1) pair (3, 7)
                    //     wing2 cell(9, 3) pair (3, 9)
                    //	   (x, y) is (7, 9), z is 3

                    // locate the cells where wing1 and wing2 intersect
                    // wing1 row/column may intersect with wing2 grid - three cells
                    // wing2 row/column may intersect with wing1 grid - three cells
                    // wing1 column may intersect with wing2 row
                    // wing1 row may intersect with wing2 column

                    // use a set since the set does not allow duplicates
                    // and no explicit effort is made to manually keep things unique.
                    use std::collections::HashSet; // stand-in for python set.

                    let mut intercept_cells: HashSet<(/*row*/ usize, /*column*/ usize)> = HashSet::new();

                    let wing1_row_intercepts_wing2_grid: bool =
                        wing2_grid_first_row <= wing1.0 && wing1.0 < wing2_grid_first_row + 3;
                    if wing1_row_intercepts_wing2_grid {
                        intercept_cells.insert((wing1.0, wing2_grid_first_column));
                        intercept_cells.insert((wing1.0, wing2_grid_first_column + 1));
                        intercept_cells.insert((wing1.0, wing2_grid_first_column + 2));
                    }

                    let wing1_column_intercepts_wing2_grid: bool =
                        wing2_grid_first_column <= wing1.1 && wing1.1 < wing2_grid_first_column + 3;
                    if wing1_column_intercepts_wing2_grid {
                        intercept_cells.insert((wing2_grid_first_row, wing1.1));
                        intercept_cells.insert((wing2_grid_first_row + 1, wing1.1));
                        intercept_cells.insert((wing2_grid_first_row + 2, wing1.1));
                    }

                    let wing2_row_intercepts_wing1_grid: bool =
                        wing1_grid_first_row <= wing2.0 && wing2.0 < wing1_grid_first_row + 3;
                    if wing2_row_intercepts_wing1_grid {
                        intercept_cells.insert((wing2.0, wing1_grid_first_column));
                        intercept_cells.insert((wing2.0, wing1_grid_first_column + 1));
                        intercept_cells.insert((wing2.0, wing1_grid_first_column + 2));
                    }

                    let wing2_column_intercepts_wing1_grid: bool =
                        wing1_grid_first_column <= wing2.1 && wing2.1 < wing1_grid_first_column + 3;
                    if wing1_column_intercepts_wing2_grid {
                        intercept_cells.insert((wing1_grid_first_row, wing2.1));
                        intercept_cells.insert((wing1_grid_first_row + 1, wing2.1));
                        intercept_cells.insert((wing1_grid_first_row + 2, wing2.1));
                    }

                    // wing1_column_intersects_wing2_row
                    intercept_cells.insert((wing2.0, wing1.1));

                    // wing1_row_intersects_wing2_column
                    intercept_cells.insert((wing1.0, wing2.1));

                    //
                    // z can be excluded from the intersect cells
                    //

                    let mut num_changes_per_xy_wing_candidate = 0;
                    for cell in &intercept_cells {
                        let row: usize = cell.0;
                        let column: usize = cell.1;
                        if self.cells[row][column].is_possible(z) {
                            num_changes_per_xy_wing_candidate += 1;
                        }
                    }

                    if num_changes_per_xy_wing_candidate > 0 {
                        println!(
                            "found xy wing: pivot cell({}, {}) pair({}, {}) wing1 cell({}, {}) pair({}, {}) wing2 cell({}, {}) pair({}, {}), (x, y) is ({}, {}), z is {}",
                            pivot.0+1, pivot.1+1, pivot.2+1, pivot.3+1,
                            wing1.0+1, wing1.1+1, wing1.2+1, wing1.3+1,
                            wing2.0+1, wing2.1+1, wing2.2+1, wing2.3+1,
                            x+1, y+1, z+1,
                        );
                        for cell in intercept_cells {
                            let row: usize = cell.0;
                            let column: usize = cell.1;
                            if self.cells[row][column].is_possible(z) {
                                println!("    remove {} from cell({}, {})", z + 1, row + 1, column + 1);
                                self.cells[row][column].set_possible(z, false);
                                num_changes += 1;
                            }
                            // else {
                            //     println!("    xxx remove {} from cell({}, {})", z+1, row+1, column+1);
                            //     self.cells[row][column].set_possible(z, false);
                            // }
                        }
                    } else {
                        println!(
                            "xxx found xy wing: pivot cell({}, {}) pair({}, {}) wing1 cell({}, {}) pair({}, {}) wing2 cell({}, {}) pair({}, {}), (x, y) is ({}, {}), z is {}",
                            pivot.0+1, pivot.1+1, pivot.2+1, pivot.3+1,
                            wing1.0+1, wing1.1+1, wing1.2+1, wing1.3+1,
                            wing2.0+1, wing2.1+1, wing2.2+1, wing2.3+1,
                            x+1, y+1, z+1,
                        );
                    }
                }
            }
        }

        return num_changes > 0;
    }

    fn solve_cell(&mut self, i: usize, j: usize, k: usize) {
        self.cells[i][j].found = k;

        // other possibilities may need to be removed from cell
        let mut exclusive_possibles: [usize; 9] = [SudokuPuzzleCell::NOT_FOUND; 9];
        exclusive_possibles[0] = k;
        self.cells[i][j].set_exclusive_possibles(&exclusive_possibles);

        self.remove_neighbors(i, j, k);
    }

    fn remove_horizontal_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        for column in 0..9 {
            let is_cell_to_change: bool = column != j;
            if is_cell_to_change {
                // note:
                // let mut cell: SudokuPuzzleCell = self.cells[i][j];
                // creates a copy of the cell => any changes are lost in the ether!!!

                let is_change_needed: bool = self.cells[i][column].is_possible(k);
                if is_change_needed {
                    found_removal = true;
                    self.cells[i][column].set_possible(k, false);
                }
            }
        }

        return found_removal;
    }

    //
    // puzzle[i, j] solved with value k
    // remove vertical neighbors as a possibility for k
    //

    fn remove_vertical_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        for row in 0..9 {
            let is_cell_to_change: bool = row != i;
            if is_cell_to_change {
                let is_change_needed: bool = self.cells[row][j].is_possible(k);
                if is_change_needed {
                    found_removal = true;
                    self.cells[row][j].set_possible(k, false);
                }
            }
        }

        return found_removal;
    }

    //
    // puzzle[i, j] solved with value k
    // remove grid neighbors as a possibility for k
    //

    fn remove_grid_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        let starti: usize = i / 3 * 3;
        let startj: usize = j / 3 * 3;

        for ii in starti..starti + 3 {
            for jj in startj..startj + 3 {
                let is_cell_to_change: bool = ii != i || jj != j;
                if is_cell_to_change {
                    let is_change_needed: bool = self.cells[ii][jj].is_possible(k);
                    if is_change_needed {
                        found_removal = true;
                        self.cells[ii][jj].set_possible(k, false);
                    }
                }
            }
        }

        return found_removal;
    }

    fn remove_neighbors(&mut self, i: usize, j: usize, k: usize) -> bool {
        let mut found_removal: bool = false;

        found_removal |= self.remove_horizontal_neighbors(i, j, k);
        found_removal |= self.remove_vertical_neighbors(i, j, k);
        found_removal |= self.remove_grid_neighbors(i, j, k);

        return found_removal;
    }

    fn solve(&mut self) -> bool {

        //
        // Solve the puzzle using the easiest to hardest methods.
        // Restart after every change
        //
        // Easy
        // 1. naked singles
        // 2. hidden singles
        // Standard/Moderate
        // 3. locked candidates type 1 and 2
        // 4. naked pairs
        // Hard
        // 5. naked triples
        // 6. naked quads
        // Harder
        // 7. hidden pairs
        // 8. hidden triples
        // 9. hidden quads (exceptionally rare)
        // Expert
        // 10. x-wing (fish x 2)
        // 11. xy-wing
        //----- do not plan to implement the methods below -----
        // Expert
        // swordfish (fish x 3), jellyfish (fish x 4), coloring et al
        //

        loop {
            // Easy

            let did_something: bool = self.find_naked_singles();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_hidden_singles();
            if did_something {
                continue;
            }

            // Standard/Moderate

            let did_something: bool = self.find_locked_candidate();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_naked_pairs();
            if did_something {
                continue;
            }

            // Hard

            let did_something: bool = self.find_naked_triples();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_naked_quads();
            if did_something {
                continue;
            }

            // Harder

            let did_something: bool = self.find_hidden_pairs();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_hidden_triples();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_hidden_quads();
            if did_something {
                continue;
            }

            // Expert

            let did_something: bool = self.find_xwing();
            if did_something {
                continue;
            }

            let did_something: bool = self.find_xywing();
            if did_something {
                continue;
            }

            break;
        }

        return self.is_solved();
    }

    pub fn is_solved(&self) -> bool {
        let mut got_unsolved_cell: bool = false;
        for i in 0..9 {
            for j in 0..9 {
                if !self.cells[i][j].is_solved() {
                    got_unsolved_cell = true;
                }
            }
        }
        return !got_unsolved_cell;
    }
}

pub fn get_next_game() -> SudokuGame {
    return SudokuGame::create();
}

pub fn create_puzzle(game: &SudokuGame) -> SudokuPuzzle {
    let mut puzzle: SudokuPuzzle = SudokuPuzzle::create(game);

    // freshly created puzzle may have nake singles, so solve them first.
    puzzle.find_naked_singles();
    // println!("puzzle: {puzzle:#?}");

    return puzzle;
}

pub fn solve_puzzle(puzzle: &mut SudokuPuzzle) -> bool {
    let solved: bool = puzzle.solve();
    return solved;
}

//
// % cargo test
// will run the unit tests
//

//
// https://doc.rust-lang.org/book/ch11-03-test-organization.html
//
// Unit Tests
//
// The purpose of unit tests is to test each unit of code in isolation
// from the rest of the code to quickly pinpoint where code is and isn’t working
// as expected. You’ll put unit tests in the src directory in each file with the
// code that they’re testing. The convention is to create a module named tests
// in each file to contain the test functions and to annotate the module
// with cfg(test).
//
// Should unit tests really be put in the same file as the source?
// https://users.rust-lang.org/t/should-unit-tests-really-be-put-in-the-same-file-as-the-source/62153
//
//  The following is a fairly common way of doing this:
//
//     src/
//         ...
//         ops.rs
//         ops/
//             tests.rs
//

#[cfg(test)]
mod tests {
    // pull outer scope into inner scope.
    use super::*;

    #[test]
    fn test_create_sudoku_game() {
        let game: SudokuGame = get_next_game();
        assert_eq!(game.db_game.len(), 81);
    }

    #[test]
    fn test_create_sudoku_puzzle() {
        let game: SudokuGame = get_next_game();
        assert_eq!(game.db_game.len(), 81);

        let mut puzzle: SudokuPuzzle = SudokuPuzzle::create(&game);
        // proceed if no panic ...

        // initialize() takes the game back to naught.
        puzzle.initialize();
        for row in 0..9 {
            for column in 0..9 {
                let cell: &SudokuPuzzleCell = &puzzle.cells[row][column];
                assert_eq!(cell.found, SudokuPuzzleCell::NOT_FOUND);
                for i in 0..9 {
                    // need to call find_naked_singles() ...
                    assert_eq!(cell.possibles[i], true);
                }
            }
        }
    }

    fn create_empty_sudoku_game() -> SudokuGame {
        let empty_row: &str = ".........";
        let db_game: String = String::from(format!(
            "{empty_row}{empty_row}{empty_row}{empty_row}{empty_row}{empty_row}{empty_row}{empty_row}{empty_row}"
        ));
        let game: SudokuGame = SudokuGame { db_game: db_game };
        return game;
    }

    fn create_empty_sudoku_puzzle() -> SudokuPuzzle {
        let game: SudokuGame = create_empty_sudoku_game();
        let puzzle: SudokuPuzzle = SudokuPuzzle::create(&game);
        return puzzle;
    }

    #[test]
    fn test_create_sudoku_empty_game() {
        let game: SudokuGame = create_empty_sudoku_game();
        assert_eq!(game.db_game.len(), 81);
    }

    #[test]
    fn test_create_sudoku_empty_puzzle() {
        let puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        let mut num_unsolved_cells: u32 = 0;
        for row in 0..9 {
            for column in 0..9 {
                let cell: &SudokuPuzzleCell = &puzzle.cells[row][column];
                assert_eq!(cell.found, SudokuPuzzleCell::NOT_FOUND);
                num_unsolved_cells += 1;
                for k in 0..9 {
                    assert_eq!(cell.possibles[k], true);
                }
            }
        }
        assert_eq!(num_unsolved_cells, 81);
    }

    fn test_cell_solved_ness(puzzle: &SudokuPuzzle, row: usize, column: usize, solved_possible: usize) {
        // 4 is a naked single in cell(3, 7) in grid(2, 3).

        // check solved cell(3, 7) ...
        let cell: &SudokuPuzzleCell = &puzzle.cells[row][column];
        assert_eq!(cell.found, 3);
        for k in 0..9 {
            if k == solved_possible {
                assert_eq!(cell.possibles[k], true);
            } else {
                assert_eq!(cell.possibles[k], false);
            }
        }

        // check cell(3, 7) row neighbors ...
        for j in 0..9 {
            let cell: &SudokuPuzzleCell = &puzzle.cells[row][j];
            // check cell(column, j)
            if j == column {
                assert_eq!(cell.possibles[solved_possible], true);
            } else {
                assert_eq!(cell.possibles[solved_possible], false);
            }
        }

        // check cell(3, 7) column neighbors ...
        for i in 0..9 {
            let cell: &SudokuPuzzleCell = &puzzle.cells[i][column];
            // check cell(i, column)
            if i == row {
                assert_eq!(cell.possibles[solved_possible], true);
            } else {
                assert_eq!(cell.possibles[solved_possible], false);
            }
        }

        // check cell(3, 7) grid neighbors ...
        let gridi_start: usize = (row / 3) * 3;
        let gridj_start: usize = (column / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                let cell: &SudokuPuzzleCell = &puzzle.cells[i][j];
                if i == row && j == column {
                    assert_eq!(cell.possibles[solved_possible], true);
                } else {
                    assert_eq!(cell.possibles[solved_possible], false);
                }
            }
        }
    }

    #[test]
    fn test_find_naked_singles_negative() {
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // puzzle.display();
        let found = puzzle.find_naked_singles();
        assert_eq!(found, false);

        let mut num_unsolved_cells: u32 = 0;
        for row in 0..9 {
            for column in 0..9 {
                let cell: &SudokuPuzzleCell = &puzzle.cells[row][column];
                if cell.found == SudokuPuzzleCell::NOT_FOUND {
                    num_unsolved_cells += 1;
                }
            }
        }
        assert_eq!(num_unsolved_cells, 81);
    }

    #[test]
    fn test_find_naked_singles_positive() {
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();
        let row: usize = 2;
        let column: usize = 6;
        // => cell(3, 7)

        let naked_single_possible: usize = 3;
        {
            let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[row][column];
            cell.possibles = [false; 9];
            cell.possibles[naked_single_possible] = true;
        }
        // 4 is a naked single in cell(3, 7) in grid(2, 3).

        // puzzle.display();
        let found = puzzle.find_naked_singles();
        assert_eq!(found, true);

        test_cell_solved_ness(&puzzle, row, column, naked_single_possible);
    }

    #[test]
    fn test_find_hidden_singles_negative() {
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // puzzle.display();
        let found = puzzle.find_hidden_singles();
        assert_eq!(found, false);

        let mut num_unsolved_cells: u32 = 0;
        for row in 0..9 {
            for column in 0..9 {
                let cell: &SudokuPuzzleCell = &puzzle.cells[row][column];
                if cell.found == SudokuPuzzleCell::NOT_FOUND {
                    num_unsolved_cells += 1;
                }
            }
        }
        assert_eq!(num_unsolved_cells, 81);
    }

    #[test]
    fn test_find_hidden_singles_row_positive() {
        // found hidden single 4 in cell(3, 7) for row 3.
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();
        let row: usize = 2;
        let column: usize = 6;
        // => cell(3, 7)

        let hidden_single_possible: usize = 3;
        for j in 0..9 {
            let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[row][j];
            if j == column {
                cell.possibles[hidden_single_possible] = true;
            } else {
                cell.possibles[hidden_single_possible] = false;
            }
        }
        // 4 is a hidden single in cell(3, 7) in grid(2, 3).

        // puzzle.display();
        let found = puzzle.find_hidden_singles();
        assert_eq!(found, true);

        test_cell_solved_ness(&puzzle, row, column, hidden_single_possible);
    }

    #[test]
    fn test_find_hidden_singles_column_positive() {
        // found hidden single 4 in cell(3, 7) for column 7.
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();
        let row: usize = 2;
        let column: usize = 6;
        // => cell(3, 7)

        let hidden_single_possible: usize = 3;
        for i in 0..9 {
            let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[i][column];
            if i == row {
                cell.possibles[hidden_single_possible] = true;
            } else {
                cell.possibles[hidden_single_possible] = false;
            }
        }
        // 4 is a hidden single in cell(3, 7) in grid(2, 3).

        // puzzle.display();
        let found = puzzle.find_hidden_singles();
        assert_eq!(found, true);

        test_cell_solved_ness(&puzzle, row, column, hidden_single_possible);
    }

    #[test]
    fn test_find_hidden_singles_grid_positive() {
        // found hidden single 4 in cell(3, 7) for grid(1, 3).
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();
        let row: usize = 2;
        let column: usize = 6;
        // => cell(3, 7)

        let hidden_single_possible: usize = 3;
        let gridi_start: usize = (row / 3) * 3;
        let gridj_start: usize = (column / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[i][j];
                if i == row && j == column {
                    cell.possibles[hidden_single_possible] = true;
                } else {
                    cell.possibles[hidden_single_possible] = false;
                }
            }
        }
        // 4 is a hidden single in cell(3, 7) in grid(2, 3).

        // puzzle.display();
        let found = puzzle.find_hidden_singles();
        assert_eq!(found, true);

        test_cell_solved_ness(&puzzle, row, column, hidden_single_possible);
    }

    #[test]
    fn test_find_locked_candidate_type_1_horizontal() {
        // found type 1 locked candidate 1 in row cells(2,1..3)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // locked candidate 1 in cells(2,1) and cell(2,3)
        let locked_candidate: usize = 0;
        let row: usize = 1;
        let column1: usize = 0;
        let column2: usize = 2;

        // candidate 1 in cell(1,1..3) and cell(3,1..3) are not possible.
        let gridi_start: usize = (row / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            if i == row {
                continue;
            }
            for j in gridj_start..gridj_start + 3 {
                puzzle.cells[i][j].set_possible(locked_candidate, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_locked_candidate();
        assert_eq!(found, true);

        // candidate 1 in cell(1,4..9) should not possible.
        for j in 0..9 {
            let candidate_gridj_start: usize = (column1 / 3) * 3;
            let column_in_candidate_grid: bool = (candidate_gridj_start <= j && j < candidate_gridj_start + 3);
            if !column_in_candidate_grid {
                assert_eq!(puzzle.cells[row][j].is_possible(locked_candidate), false);
            }
        }
    }

    #[test]
    fn test_find_locked_candidate_type_1_vertical() {
        // found type 1 locked candidate 1 in row cells(1..3,6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // locked candidate 1 in cells(1,6) and cell(2,6)
        let locked_candidate: usize = 0;
        let row1: usize = 0;
        let row2: usize = 1;
        let column: usize = 5;

        // candidate 1 in cell(1..3,4) and cell(1..3,5) are not possible.
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column / 3) * 3;
        for j in gridj_start..gridj_start + 3 {
            if j == column {
                continue;
            }
            for i in gridi_start..gridi_start + 3 {
                puzzle.cells[i][j].set_possible(locked_candidate, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_locked_candidate();
        assert_eq!(found, true);

        // candidate 1 in cell(4..9,6) should not possible.
        for i in 0..9 {
            let candidate_gridi_start: usize = (row1 / 3) * 3;
            let row_in_candidate_grid: bool = (candidate_gridi_start <= i && i < candidate_gridi_start + 3);
            if !row_in_candidate_grid {
                assert_eq!(puzzle.cells[i][column].is_possible(locked_candidate), false);
            }
        }
    }

    #[test]
    fn test_find_locked_candidate_type_2_horizontal() {
        // found type 2 locked candidate 1 in row cells(2,1..3)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // locked candidate 1 in cells(2,1) and cell(2,3)
        let locked_candidate: usize = 0;
        let row: usize = 1;
        let column1: usize = 0;
        let column2: usize = 2;

        for j in 0..9 {
            let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[row][j];
            if j == column1 || j == column2 {
                cell.possibles[locked_candidate] = true;
            } else {
                cell.possibles[locked_candidate] = false;
            }
        }

        // puzzle.display();
        let found = puzzle.find_locked_candidate();
        assert_eq!(found, true);

        // candidate 1 in cell(1,1..3) and cell(3,1..3) should not possible.
        let gridi_start: usize = (row / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            if i == row {
                continue;
            }
            for j in gridj_start..gridj_start + 3 {
                assert_eq!(puzzle.cells[i][j].is_possible(locked_candidate), false);
            }
        }
    }

    #[test]
    fn test_find_locked_candidate_type_2_vertical() {
        // found type 2 locked candidate 1 in row cells(1..3,6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // locked candidate 1 in cells(1,6) and cell(2,6)
        let locked_candidate: usize = 0;
        let row1: usize = 0;
        let row2: usize = 1;
        let column: usize = 5;

        for i in 0..9 {
            let cell: &mut SudokuPuzzleCell = &mut puzzle.cells[i][column];
            if i == row1 || i == row2 {
                cell.possibles[locked_candidate] = true;
            } else {
                cell.possibles[locked_candidate] = false;
            }
        }

        // puzzle.display();
        let found = puzzle.find_locked_candidate();
        assert_eq!(found, true);

        // candidate 1 in cell(0..2,3) and cell(0..2,4) should not possible.
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column / 3) * 3;
        for j in gridj_start..gridj_start + 3 {
            if j == column {
                continue;
            }
            for i in gridi_start..gridi_start + 3 {
                assert_eq!(puzzle.cells[i][j].is_possible(locked_candidate), false);
            }
        }
    }

    #[test]
    fn test_find_naked_pairs_row() {
        // found naked pair (2, 6) for row 9
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 6 in cell(9,*)
        let candidate1: usize = 1;
        let candidate2: usize = 5;
        let row: usize = 8;
        let column1: usize = 0; // candidates 2 and 6 will be only possible in cell(9,1)
        let column2: usize = 8; // candidates 2 and 6 will be only possible in cell(9,9)

        // for row 9, all cells should have (2,6) as a possible
        // for cell(9,1) and cell(9,9) should only have (2,6) possible
        for j in 0..9 {
            puzzle.cells[row][j].set_possible(candidate1, true);
            puzzle.cells[row][j].set_possible(candidate2, true);
        }
        for j in 0..9 {
            if j == column1 || j == column2 {
                for k in 0..9 {
                    if k == candidate1 || k == candidate2 {
                        // do nothing. possible[k] is already set to true.
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_pairs();
        assert_eq!(found, true);

        // chek that (2,6) is now excluded from the other columns
        for j in 0..9 {
            if j != column1 && j != column2 {
                assert_eq!(puzzle.cells[row][j].is_possible(candidate1), false);
                assert_eq!(puzzle.cells[row][j].is_possible(candidate2), false);
            }
        }
    }

    #[test]
    fn test_find_naked_pairs_column() {
        // found naked pair (2, 6) for column 9
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 6 in cell(*,9)
        let candidate1: usize = 1;
        let candidate2: usize = 5;
        let column: usize = 8;
        let row1: usize = 0; // candidates 2 and 6 will be only possible in cell(1,9)
        let row2: usize = 8; // candidates 2 and 6 will be only possible in cell(9,9)

        // for column 9, all cells should have (2,6) as a possible
        // for cell(1,9) and cell(9,9) should only have (2,6) possible
        for i in 0..9 {
            puzzle.cells[i][column].set_possible(candidate1, true);
            puzzle.cells[i][column].set_possible(candidate2, true);
        }
        for i in 0..9 {
            if i == row1 || i == row2 {
                for k in 0..9 {
                    if k == candidate1 || k == candidate2 {
                        // do nothing. possible[k] is already set to true.
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_pairs();
        assert_eq!(found, true);

        // chek that (2,6) is now excluded from the other rows
        for i in 0..9 {
            if i != row1 && i != row2 {
                assert_eq!(puzzle.cells[i][column].is_possible(candidate1), false);
                assert_eq!(puzzle.cells[i][column].is_possible(candidate2), false);
            }
        }
    }

    #[test]
    fn test_find_naked_pairs_grid() {
        // found naked pair (2, 6) for grid(3,3)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 6 in cell(7..9,7..9)
        let candidate1: usize = 1;
        let candidate2: usize = 5;
        let column1: usize = 6;
        let column2: usize = 8;
        let row1: usize = 6; // candidates 2 and 6 will be only possible in cell(7,7)
        let row2: usize = 8; // candidates 2 and 6 will be only possible in cell(9,9)

        // for grid(3,3), all cells should have (2,6) as a possible
        // for cell(7,9) and cell(9,9) should only have (2,6) possible
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                puzzle.cells[i][j].set_possible(candidate1, true);
                puzzle.cells[i][j].set_possible(candidate1, true);
            }
        }
        for k in 0..9 {
            if k != candidate1 && k != candidate2 {
                puzzle.cells[row1][column1].set_possible(k, false);
                puzzle.cells[row2][column2].set_possible(k, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_pairs();
        assert_eq!(found, true);

        // chek that (2,6) is now excluded from the other grid cells
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    continue;
                }
                if i == row2 && j == column2 {
                    continue;
                }
                assert_eq!(puzzle.cells[i][j].is_possible(candidate1), false);
                assert_eq!(puzzle.cells[i][j].is_possible(candidate2), false);
            }
        }
        assert!(true);
    }

    #[test]
    fn test_find_naked_triples_row() {
        // naked triple (1 2 3) in row 7
        // (1 2) in cell(7,3), (2 3) in cel(7, 4), (1 2 3) in cell(7, 9)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 6 in row 7
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row: usize = 6;
        let column1: usize = 2;
        let column2: usize = 3;
        let column3: usize = 8;

        // for row 7, all non-candidate cells show have (1 2 3) as possible.
        // for cell(7,3), (1 2) are only possibles.
        // for cell(7,4), (2 3) are only possibles.
        // for cell(7,9), (1 2 3) are only possibles.
        for j in 0..9 {
            for k in 0..9 {
                if j == column1 {
                    if k == candidate1 || k == candidate2 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else if j == column2 {
                    if k == candidate2 || k == candidate3 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else if j == column3 {
                    if k == candidate1 || k == candidate2 || k == candidate3 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else {
                    puzzle.cells[row][j].set_possible(k, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_triples();
        assert_eq!(found, true);

        for j in 0..9 {
            if j == column1 || j == column2 || j == column3 {
                continue;
            } else {
                assert_eq!(puzzle.cells[row][j].is_possible(candidate1), false);
                assert_eq!(puzzle.cells[row][j].is_possible(candidate2), false);
                assert_eq!(puzzle.cells[row][j].is_possible(candidate3), false);
            }
        }
    }

    #[test]
    fn test_find_naked_triples_column() {
        // naked triple (1 2 3) in column 7
        //     (1 2)   in cell(3, 7)
        //     (2 3)   in cell(4, 7)
        //     (1 2 3) in cell(9, 7)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 3 in row 7
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row1: usize = 2;
        let row2: usize = 3;
        let row3: usize = 8;
        let column: usize = 6;

        // for column 7, all non-candidate cells show have (1 2 3) as possible.
        // for cell(3, 7), (1 2) are only possibles.
        // for cell(4, 7), (2 3) are only possibles.
        // for cell(9, 7), (1 2 3) are only possibles.
        for i in 0..9 {
            for k in 0..9 {
                if i == row1 {
                    if k == candidate1 || k == candidate2 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else if i == row2 {
                    if k == candidate2 || k == candidate3 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else if i == row3 {
                    if k == candidate1 || k == candidate2 || k == candidate3 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else {
                    puzzle.cells[i][column].set_possible(k, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_triples();
        assert_eq!(found, true);

        for i in 0..9 {
            if i == row1 || i == row2 || i == row3 {
                continue;
            } else {
                assert_eq!(puzzle.cells[i][column].is_possible(candidate1), false);
                assert_eq!(puzzle.cells[i][column].is_possible(candidate2), false);
                assert_eq!(puzzle.cells[i][column].is_possible(candidate3), false);
            }
        }
    }

    #[test]
    fn test_find_naked_triples_grid() {
        // naked triple (1 2 3) in grid(2, 2)
        //     (1 2) in cell(4, 4)
        //     (2 3) in cell(5, 5)
        //     (1 2 3) in cell(6, 6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 3 in grid(2, 2)
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row1: usize = 3;
        let row2: usize = 4;
        let row3: usize = 5;
        let column1: usize = 3;
        let column2: usize = 4;
        let column3: usize = 5;

        // for grid(2, 2), all non-candidate cells show have (1 2 3) as possible.
        // for cell(4, 4), (1 2) are only possibles.
        // for cell(5, 5), (2 3) are only possibles.
        // for cell(6, 6), (1 2 3) are only possibles.
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    for k in 0..9 {
                        if k == candidate1 || k == candidate2 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else if i == row2 && j == column2 {
                    for k in 0..9 {
                        if k == candidate2 || k == candidate3 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else if i == row3 && j == column3 {
                    for k in 0..9 {
                        if k == candidate1 || k == candidate2 || k == candidate3 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_triples();
        assert_eq!(found, true);

        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    continue;
                } else if i == row2 && j == column2 {
                    continue;
                } else if i == row3 && j == column3 {
                    continue;
                } else {
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate1), false);
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate2), false);
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate3), false);
                }
            }
        }
    }

    #[test]
    fn test_find_naked_quads_row() {
        // naked quad (1 2 3 4) in row 7
        //     (1 2)     in cell(7, 1)
        //     (2 3)     in cell(7, 4)
        //     (3 4)     in cell(7, 6)
        //     (1 2 3 4) in cell(7, 9)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in row 7
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row: usize = 6;
        let column1: usize = 0;
        let column2: usize = 3;
        let column3: usize = 5;
        let column4: usize = 8;

        // for row 7, all non-candidate cells show have (1 2 3 4) as possible.
        // for cell(7, 1), (1 2) are only possibles.
        // for cell(7, 4), (2 3) are only possibles.
        // for cell(7, 6), (3 4) are only possibles.
        // for cell(7, 9), (1 2 3 4) are only possibles.
        for j in 0..9 {
            for k in 0..9 {
                if j == column1 {
                    if k == candidate1 || k == candidate2 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else if j == column2 {
                    if k == candidate2 || k == candidate3 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else if j == column3 {
                    if k == candidate3 || k == candidate4 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else if j == column4 {
                    if k == candidate1 || k == candidate2 || k == candidate3 || k == candidate4 {
                        puzzle.cells[row][j].set_possible(k, true);
                    } else {
                        puzzle.cells[row][j].set_possible(k, false);
                    }
                } else {
                    puzzle.cells[row][j].set_possible(k, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_quads();
        assert_eq!(found, true);

        for j in 0..9 {
            for k in 0..9 {
                if j == column1 {
                    continue;
                } else if j == column2 {
                    continue;
                } else if j == column3 {
                    continue;
                } else if j == column4 {
                    continue;
                } else {
                    assert_eq!(puzzle.cells[row][j].is_possible(candidate1), false);
                    assert_eq!(puzzle.cells[row][j].is_possible(candidate2), false);
                    assert_eq!(puzzle.cells[row][j].is_possible(candidate3), false);
                    assert_eq!(puzzle.cells[row][j].is_possible(candidate4), false);
                }
            }
        }
    }

    #[test]
    fn test_find_naked_quads_column() {
        // naked quad (1 2 3 4) in column 7
        //     (1 2)     in cell(1, 7)
        //     (2 3)     in cell(4, 7)
        //     (3 4)     in cell(6, 7)
        //     (1 2 3 4) in cell(9, 7)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in column 7
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row1: usize = 0;
        let row2: usize = 3;
        let row3: usize = 5;
        let row4: usize = 8;
        let column: usize = 6;

        // for row 7, all non-candidate cells show have (1 2 3 4) as possible.
        // for cell(1, 7), (1 2) are only possibles.
        // for cell(4, 7), (2 3) are only possibles.
        // for cell(6, 7), (3 4) are only possibles.
        // for cell(9, 7), (1 2 3 4) are only possibles.
        for i in 0..9 {
            for k in 0..9 {
                if i == row1 {
                    if k == candidate1 || k == candidate2 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else if i == row2 {
                    if k == candidate2 || k == candidate3 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else if i == row3 {
                    if k == candidate3 || k == candidate4 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else if i == row4 {
                    if k == candidate1 || k == candidate2 || k == candidate3 || k == candidate4 {
                        puzzle.cells[i][column].set_possible(k, true);
                    } else {
                        puzzle.cells[i][column].set_possible(k, false);
                    }
                } else {
                    puzzle.cells[i][column].set_possible(k, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_quads();
        assert_eq!(found, true);

        for i in 0..9 {
            for k in 0..9 {
                if i == row1 {
                    continue;
                } else if i == row2 {
                    continue;
                } else if i == row3 {
                    continue;
                } else if i == row4 {
                    continue;
                } else {
                    assert_eq!(puzzle.cells[i][column].is_possible(candidate1), false);
                    assert_eq!(puzzle.cells[i][column].is_possible(candidate2), false);
                    assert_eq!(puzzle.cells[i][column].is_possible(candidate3), false);
                    assert_eq!(puzzle.cells[i][column].is_possible(candidate4), false);
                }
            }
        }
        assert!(true);
    }

    #[test]
    fn test_find_naked_quads_grid() {
        // naked quad (1 2 3 4) in grid(2, 2)
        //     (1 2)     in cell(4, 4)
        //     (2 3)     in cell(4, 6)
        //     (3 4)     in cell(6, 4)
        //     (1 2 3 4) in cell(6, 6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in grid(2, 2)
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row1: usize = 3;
        let row2: usize = 3;
        let row3: usize = 5;
        let row4: usize = 5;
        let column1: usize = 3;
        let column2: usize = 5;
        let column3: usize = 3;
        let column4: usize = 5;

        // for grid(2, 2), all non-candidate cells show have (1 2 3 4) as possible.
        // for cell(4, 4), (1 2) are only possibles.
        // for cell(4, 6), (2 3) are only possibles.
        // for cell(6, 4), (3 4) are only possibles.
        // for cell(6, 6), (1 2 3 4) are only possibles.
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    for k in 0..9 {
                        if k == candidate1 || k == candidate2 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else if i == row2 && j == column2 {
                    for k in 0..9 {
                        if k == candidate2 || k == candidate3 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else if i == row3 && j == column3 {
                    for k in 0..9 {
                        if k == candidate3 || k == candidate4 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else if i == row4 && j == column4 {
                    for k in 0..9 {
                        if k == candidate1 || k == candidate2 || k == candidate3 || k == candidate4 {
                            puzzle.cells[i][j].set_possible(k, true);
                        } else {
                            puzzle.cells[i][j].set_possible(k, false);
                        }
                    }
                } else {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                    puzzle.cells[i][j].set_possible(candidate4, true);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_naked_quads();
        assert_eq!(found, true);

        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    continue;
                } else if i == row2 && j == column2 {
                    continue;
                } else if i == row3 && j == column3 {
                    continue;
                } else if i == row4 && j == column4 {
                    continue;
                } else {
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate1), false);
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate2), false);
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate3), false);
                    assert_eq!(puzzle.cells[i][j].is_possible(candidate4), false);
                }
            }
        }
    }

    #[test]
    fn test_find_hidden_pairs_row() {
        // hidden pair (2 7) at cell(3, 4) and cell(3, 6), for row 3
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 7 in row 3
        let candidate1: usize = 1;
        let candidate2: usize = 6;
        let row: usize = 2;
        let column1: usize = 3;
        let column2: usize = 5;

        // for row 3, all non-candidate cells need to exclude candidates
        for j in 0..9 {
            if j == column1 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
            } else if j == column2 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
            } else {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_pairs();
        assert_eq!(found, true);

        // cell(3, 4) and cell(3, 6) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 {
                assert_eq!(puzzle.cells[row][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column2].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidden_pairs_column() {
        // hidden pair (2 7) at cell(4, 3) and cell(6, 3), for column 3
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 7 in row 3
        let candidate1: usize = 1;
        let candidate2: usize = 6;
        let row1: usize = 3;
        let row2: usize = 5;
        let column: usize = 2;

        // for column 3, all non-candidate cells need to exclude candidates
        for i in 0..9 {
            if i == row1 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
            } else if i == row2 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
            } else {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_pairs();
        assert_eq!(found, true);

        // cell(4, 3) and cell(6, 3) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 {
                assert_eq!(puzzle.cells[row1][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidden_pairs_grid() {
        // hidden pair (2 7) at cell(4, 4) and cell(6, 6), for grid(2, 2)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 2 and 7 in cell(4, 4) and cell(6, 6), for grid(2, 2)
        let candidate1: usize = 1;
        let candidate2: usize = 6;
        let row1: usize = 3;
        let row2: usize = 5;
        let column1: usize = 3;
        let column2: usize = 5;

        // grid (2, 2) is cell(4..6, 4..6).
        // for grid (2, 2), all non-candidate cells need to exclude candidates
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                } else if i == row2 && j == column2 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                } else {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, false);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_pairs();
        assert_eq!(found, true);

        // cell(4, 4) and cell(6, 6) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 {
                assert_eq!(puzzle.cells[row1][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column2].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidden_triples_row() {
        // found hidden triple (1 2 3) at cell(3, 1) and cell(3, 4) and cell(3, 9), for row 3
        //     (1 2)   in cell(3, 1)
        //     (2 3)   in cell(3, 4)
        //     (1 2 3) in cell(3, 9)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 3 in row 3
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row: usize = 2;
        let column1: usize = 0;
        let column2: usize = 3;
        let column3: usize = 8;

        // for row 3, all non-candidate cells need to exclude candidates
        for j in 0..9 {
            if j == column1 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, false);
            } else if j == column2 {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, true);
            } else if j == column3 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, true);
            } else {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, false);
                puzzle.cells[row][j].set_possible(candidate3, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_triples();
        assert_eq!(found, true);

        // cell(3, 1), cell(3, 4) and cell(3, 9) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 {
                assert_eq!(puzzle.cells[row][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column2].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column3].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidden_triples_column() {
        // found hidden triple (1 2 3) at cell(1, 3) and cell(4, 3) and cell(9, 3), for column 3
        //     (1 2)   in cell(1, 3)
        //     (2 3)   in cell(4, 3)
        //     (1 2 3) in cell(9, 3)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 3 in column 3
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row1: usize = 0;
        let row2: usize = 3;
        let row3: usize = 8;
        let column: usize = 2;

        // for column 3, all non-candidate cells need to exclude candidates
        for i in 0..9 {
            if i == row1 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, false);
            } else if i == row2 {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, true);
            } else if i == row3 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, true);
            } else {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, false);
                puzzle.cells[i][column].set_possible(candidate3, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_triples();
        assert_eq!(found, true);

        // cell(3, 1), cell(3, 4) and cell(3, 9) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 {
                assert_eq!(puzzle.cells[row1][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row3][column].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidden_triples_grid() {
        // found hidden triple (1 2 3) at cell(4, 4) and cell(5, 5) and cell(6, 6), for grid(2, 2)
        //     (1 2)   in cell(4, 4)
        //     (2 3)   in cell(5, 5)
        //     (1 2 3) in cell(6, 6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2 and 3 in column 3
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let row1: usize = 3;
        let row2: usize = 4;
        let row3: usize = 5;
        let column1: usize = 3;
        let column2: usize = 4;
        let column3: usize = 5;

        // for grid(2, 2), all non-candidate cells need to exclude candidates
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, false);
                } else if i == row2 && j == column2 {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                } else if i == row3 && j == column3 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                } else {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, false);
                    puzzle.cells[i][j].set_possible(candidate3, false);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_triples();
        assert_eq!(found, true);

        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 {
                assert_eq!(puzzle.cells[row1][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column2].is_possible(k), false);
                assert_eq!(puzzle.cells[row3][column3].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidded_quads_row() {
        // found hidden quad (1 2 3 4) at cell(3, 1) and cell(3, 4) and cell(3, 6) and cell(3, 9), for row 3
        //     (1 2)     in cell(3, 1)
        //     (2 3)     in cell(3, 4)
        //     (3 4)     in cell(3, 6)
        //     (1 2 3 4) in cell(3, 9)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in row 3
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row: usize = 2;
        let column1: usize = 0;
        let column2: usize = 3;
        let column3: usize = 5;
        let column4: usize = 8;

        // for row 3, all non-candidate cells need to exclude candidates
        for j in 0..9 {
            if j == column1 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, false);
                puzzle.cells[row][j].set_possible(candidate4, false);
            } else if j == column2 {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, true);
                puzzle.cells[row][j].set_possible(candidate4, false);
            } else if j == column3 {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, false);
                puzzle.cells[row][j].set_possible(candidate3, true);
                puzzle.cells[row][j].set_possible(candidate4, true);
            } else if j == column4 {
                puzzle.cells[row][j].set_possible(candidate1, true);
                puzzle.cells[row][j].set_possible(candidate2, true);
                puzzle.cells[row][j].set_possible(candidate3, true);
                puzzle.cells[row][j].set_possible(candidate4, true);
            } else {
                puzzle.cells[row][j].set_possible(candidate1, false);
                puzzle.cells[row][j].set_possible(candidate2, false);
                puzzle.cells[row][j].set_possible(candidate3, false);
                puzzle.cells[row][j].set_possible(candidate4, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_quads();
        assert_eq!(found, true);

        // cell(3, 1), cell(3, 4) and cell(3, 9) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 && k != candidate4 {
                assert_eq!(puzzle.cells[row][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column2].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column3].is_possible(k), false);
                assert_eq!(puzzle.cells[row][column4].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidded_quads_column() {
        // found hidden quad (1 2 3 4) in column 3
        //     (1 2)   in cell(1, 3)
        //     (2 3)   in cell(4, 3)
        //     (3 4)   in cell(6, 3)
        //     (1 2 3) in cell(9, 3)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in column 3
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row1: usize = 0;
        let row2: usize = 3;
        let row3: usize = 5;
        let row4: usize = 8;
        let column: usize = 2;

        // for column 3, all non-candidate cells need to exclude candidates
        for i in 0..9 {
            if i == row1 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, false);
                puzzle.cells[i][column].set_possible(candidate4, false);
            } else if i == row2 {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, true);
                puzzle.cells[i][column].set_possible(candidate4, false);
            } else if i == row3 {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, false);
                puzzle.cells[i][column].set_possible(candidate3, true);
                puzzle.cells[i][column].set_possible(candidate4, true);
            } else if i == row4 {
                puzzle.cells[i][column].set_possible(candidate1, true);
                puzzle.cells[i][column].set_possible(candidate2, true);
                puzzle.cells[i][column].set_possible(candidate3, true);
                puzzle.cells[i][column].set_possible(candidate4, true);
            } else {
                puzzle.cells[i][column].set_possible(candidate1, false);
                puzzle.cells[i][column].set_possible(candidate2, false);
                puzzle.cells[i][column].set_possible(candidate3, false);
                puzzle.cells[i][column].set_possible(candidate4, false);
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_quads();
        assert_eq!(found, true);

        // cell(3, 1), cell(3, 4) and cell(3, 9) need to exclude all non-candidates
        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 && k != candidate4 {
                assert_eq!(puzzle.cells[row1][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row3][column].is_possible(k), false);
                assert_eq!(puzzle.cells[row4][column].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_hidded_quads_grid() {
        // found hidden quad (1 2 3 4) for grid(2, 2)
        //     (1 2)     in cell(4, 4)
        //     (2 3)     in cell(4, 6)
        //     (3 4)     in cell(6, 4)
        //     (1 2 3 4) in cell(6, 6)
        let mut puzzle: SudokuPuzzle = create_empty_sudoku_puzzle();

        // candidates 1, 2, 3 and 4 in grid(2, 2)
        let candidate1: usize = 0;
        let candidate2: usize = 1;
        let candidate3: usize = 2;
        let candidate4: usize = 3;
        let row1: usize = 3;
        let row2: usize = 3;
        let row3: usize = 5;
        let row4: usize = 5;
        let column1: usize = 3;
        let column2: usize = 5;
        let column3: usize = 3;
        let column4: usize = 5;

        // for grid(2, 2), all non-candidate cells need to exclude candidates
        let gridi_start: usize = (row1 / 3) * 3;
        let gridj_start: usize = (column1 / 3) * 3;
        for i in gridi_start..gridi_start + 3 {
            for j in gridj_start..gridj_start + 3 {
                if i == row1 && j == column1 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, false);
                    puzzle.cells[i][j].set_possible(candidate4, false);
                } else if i == row2 && j == column2 {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                    puzzle.cells[i][j].set_possible(candidate4, false);
                } else if i == row3 && j == column3 {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, false);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                    puzzle.cells[i][j].set_possible(candidate4, true);
                } else if i == row4 && j == column4 {
                    puzzle.cells[i][j].set_possible(candidate1, true);
                    puzzle.cells[i][j].set_possible(candidate2, true);
                    puzzle.cells[i][j].set_possible(candidate3, true);
                    puzzle.cells[i][j].set_possible(candidate4, true);
                } else {
                    puzzle.cells[i][j].set_possible(candidate1, false);
                    puzzle.cells[i][j].set_possible(candidate2, false);
                    puzzle.cells[i][j].set_possible(candidate3, false);
                    puzzle.cells[i][j].set_possible(candidate4, false);
                }
            }
        }

        // puzzle.display();
        let found = puzzle.find_hidden_quads();
        assert_eq!(found, true);

        for k in 0..9 {
            if k != candidate1 && k != candidate2 && k != candidate3 && k != candidate4 {
                assert_eq!(puzzle.cells[row1][column1].is_possible(k), false);
                assert_eq!(puzzle.cells[row2][column2].is_possible(k), false);
                assert_eq!(puzzle.cells[row3][column3].is_possible(k), false);
                assert_eq!(puzzle.cells[row4][column4].is_possible(k), false);
            }
        }
    }

    #[test]
    fn test_find_xwing_rows() {
        // found x-wing exclusion for fish candidate 5 from cell(4, 5), given base set cells(2, 5) cells(2, 8) cells(5, 5) cells(5, 8).
        //
        //     fish candidate: 5
        //     base set:
        //         row 2: cell(2, 5) and cell(2, 8)
        //         row 5: cell(5, 5) and cell(5, 8)
        //     cover set:
        //         column 5: cell(4, 5)
        let game: SudokuGame = SudokuGame {
            db_game: String::from(".41729.3.769..34.2.3264.7194.39..17.6.7..49.319537..24214567398376.9.541958431267"),
        };
        let mut puzzle: SudokuPuzzle = create_puzzle(&game);

        let fish_candidate: usize = 4;

        // puzzle.display();
        let found = puzzle.find_xwing();
        assert_eq!(found, true);

        assert_eq!(puzzle.cells[3][4].is_possible(fish_candidate), false);
    }

    #[test]
    fn test_find_xwing_columns() {
        // found x-wing exclusion for fish candidate 1 from cell(5, 3), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(2, 4), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(5, 4), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(2, 7), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(5, 7), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(2, 8), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(5, 8), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(2, 9), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        // found x-wing exclusion for fish candidate 1 from cell(5, 9), given base set cells(2, 1) cells(2, 5) cells(5, 1) cells(5, 5).
        //
        //     fish candidate: 1
        //     base set:
        //         column 1: cell(2, 1) and cell(5, 1)
        //         column 5: cell(2, 5) and cell(5, 5)
        //     cover set:
        //         row 2: cell(2, 4)
        //         row 2: cell(2, 7)
        //         row 2: cell(2, 8)
        //         row 2: cell(2, 9)
        //         row 5: cell(5, 3)
        //         row 5: cell(5, 4)
        //         row 5: cell(5, 7)
        //         row 5: cell(5, 8)
        //         row 5: cell(5, 9)

        let game: SudokuGame = SudokuGame {
            db_game: String::from("98..62753.65..3...327.5...679..3.5...5...9...832.45..9673591428249.87..5518.2...7"),
        };
        let mut puzzle: SudokuPuzzle = create_puzzle(&game);

        let fish_candidate: usize = 0;

        // puzzle.display();
        let found = puzzle.find_xwing();
        assert_eq!(found, true);

        // check cover set
        assert_eq!(puzzle.cells[1][3].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[1][6].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[1][7].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[1][8].is_possible(fish_candidate), false);

        assert_eq!(puzzle.cells[4][2].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[4][3].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[4][6].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[4][7].is_possible(fish_candidate), false);
        assert_eq!(puzzle.cells[4][8].is_possible(fish_candidate), false);
    }

    #[test]
    fn test_find_xywing() {
        // found xy wing: pivot cell(7, 1) pair(7, 9) wing1 cell(2, 1) pair(3, 7) wing2 cell(9, 3) pair(3, 9), (x, y) is (7, 9), z is 3
        //     remove 3 from cell(2, 3)
        //     remove 3 from cell(3, 3)
        // found xy wing: pivot cell(9, 8) pair(3, 9) wing1 cell(5, 8) pair(4, 9) wing2 cell(8, 9) pair(3, 4), (x, y) is (3, 9), z is 4
        //     remove 4 from cell(5, 9)
        //     remove 4 from cell(4, 9)

        let game: SudokuGame = SudokuGame {
            db_game: String::from("5..23.867.8.9....22..8.......1.9.25..273651....518237...2..3.816.8.19.2.14..28..5"),
        };
        let mut puzzle: SudokuPuzzle = create_puzzle(&game);

        puzzle.find_locked_candidate(); // add in some exclusions, to find xy wings

        puzzle.display();
        let found = puzzle.find_xywing();
        assert_eq!(found, true);

        assert_eq!(puzzle.cells[1][2].is_possible(2), false);
        assert_eq!(puzzle.cells[2][2].is_possible(2), false);
        assert_eq!(puzzle.cells[3][8].is_possible(3), false);
        assert_eq!(puzzle.cells[4][8].is_possible(3), false);
    }
}
