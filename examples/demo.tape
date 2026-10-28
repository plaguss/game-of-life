Output examples/lifeterm.gif
Require ./target/release/lifeterm

Set Width 1600
Set Height 900
Set FontSize 18
Set TypingSpeed 100ms

Hide
Type "./target/release/lifeterm"
Enter
Sleep 500ms
Show

# Startup banner, then help.
Sleep 2s
Enter
Type "h"
Sleep 2s
Escape

# FPS: 12 -> 23.
Type "s"
Sleep 400ms
Enter
Sleep 300ms
Right
Right
Sleep 400ms
Escape

# Theme: Nord -> Matrix.
Down
Enter
Sleep 300ms
Enter
Sleep 400ms
Escape

# Pattern: R-pentomino -> Random Soup (wrap to the last entry).
Down
Enter
Sleep 300ms
Up
Enter
Sleep 400ms
Escape

# Rule: select Highlife, then return to Conway's rules for the finale.
Down
Enter
Sleep 300ms
Down
Enter
Sleep 400ms
Up
Enter
Sleep 300ms
Escape

# Enable Auto and Shadow.
Down
Enter
Sleep 400ms
Down
Enter
Sleep 600ms

# Reseed Random Soup so the finale starts with a fresh board.
Up 3
Enter
Up
Enter
Escape
Escape
Sleep 5s
Type "q"
Sleep 300ms