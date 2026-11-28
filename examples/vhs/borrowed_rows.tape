# Records examples/apps/borrowed_rows into target/borrowed_rows.gif. Install
# https://github.com/charmbracelet/vhs, then run this from the repository root:
#
#   just record borrowed_rows

Output target/borrowed_rows.gif

Set Shell bash
Set FontSize 18
Set Width 900
Set Height 460
Set Padding 12

Hide
Type "cargo run --quiet -p borrowed_rows" Enter
Sleep 5s
Show

Sleep 1s
Down@200ms 5
Type "G" Sleep 1500ms

# Cloning: every key press copies all 200,000 rows, and the draw time jumps.
Type "c" Sleep 1500ms
Up@400ms 3
Sleep 1s

# Back to borrowing.
Type "c" Sleep 1500ms
Up@200ms 3
Type "g" Sleep 1500ms

Type "q"
Sleep 1s
