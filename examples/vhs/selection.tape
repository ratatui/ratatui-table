# Records examples/apps/selection into target/selection.gif. Install
# https://github.com/charmbracelet/vhs, then run this from the repository root:
#
#   just record selection

Output target/selection.gif

Set Shell bash
Set FontSize 18
Set Width 900
Set Height 460
Set Padding 12

Hide
Type "cargo run --quiet -p selection" Enter
Sleep 3s
Show

Sleep 1s
Down@400ms 3
Right@400ms 2
Sleep 1s

# Row, then column, then cell, so each highlight style shows on its own.
Type "r" Sleep 1500ms
Type "c" Sleep 1500ms
Type "x" Sleep 1500ms

# The symbol column follows the selection, then lands on every column.
Type "p" Sleep 1500ms
Left@500ms 2
Sleep 1s
Type "p" Sleep 1500ms
Type "p" Sleep 1500ms
Type "p" Sleep 1500ms

# Without a selection, WhenSelected drops the symbol column entirely.
Type "s" Sleep 1s
Type "n" Sleep 2s

Type "q"
Sleep 1s
