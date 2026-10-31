# Records examples/apps/lazy_table into target/lazy_table.gif. Install
# https://github.com/charmbracelet/vhs, then run this from the repository root:
#
#   just record lazy_table

Output target/lazy_table.gif

Set Shell bash
Set FontSize 16
Set Width 1000
Set Height 520
Set Padding 12

Hide
Type "cargo run --quiet -p lazy_table" Enter
Sleep 4s
Show

Sleep 1500ms

Down@350ms 8
Sleep 1500ms

# The long scroll: twenty seconds of held ↓, a thousand entries.
Down@20ms 1000
Sleep 2s

PageDown@250ms 12
Sleep 2s

Type "G"
Sleep 3s
Type "g"
Sleep 3s

Type "q"
Sleep 1s
