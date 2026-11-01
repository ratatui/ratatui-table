# Records examples/apps/borders into target/borders.gif. Install
# https://github.com/charmbracelet/vhs, then run this from the repository root:
#
#   just record borders

Output target/borders.gif

Set Shell bash
Set FontSize 18
Set Width 900
Set Height 460
Set Padding 12

Hide
Type "cargo run --quiet -p borders" Enter
Sleep 3s
Show

Sleep 1s
Down@400ms 3
Sleep 1s

# Each preset in turn, down to no lines at all and back to the full grid.
Type "b" Sleep 1500ms
Type "b" Sleep 1500ms
Type "b" Sleep 1500ms
Type "b" Sleep 1500ms
Type "b" Sleep 1500ms
Type "b" Sleep 1500ms

# The same grid in every glyph set.
Type "s" Sleep 1500ms
Type "s" Sleep 1500ms
Type "s" Sleep 1500ms
Type "s" Sleep 1s

# Column lines without the frame, then row lines, then the gap widening and narrowing to 0.
Type "bb" Sleep 1500ms
Type "h" Sleep 1500ms
Type "+" Sleep 1s
Type "+" Sleep 1s
Type "-" Sleep 1s
Type "-" Sleep 1s
Type "-" Sleep 1500ms

Type "q"
Sleep 1s
