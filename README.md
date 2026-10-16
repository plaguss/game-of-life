# game-of-life

A simple game of life simulator

Use for an example: https://github.com/charmbracelet/vhs

[ ] Does it make sense to have toroidal wrap for the given size?
    [ ] What happens when the screen is to overcrowded? maybe better to just let die the out of bound cells?
[ ] Ensure we can resize the screen
    [ ] On resizing (and at start) put the initial pattern at the center. Also on resizing, center again
[ ] Handle FPS with arrows and +/- (show a pop up in the screen when modified like this)
[ ] Add a random soup of patterns, and every so often, include another one at random, also when it's been steady for a given amount of time.
[ ] Add notes in the Help, not just for how it works this, but for the patterns etc
[ ] Start the simulation with the pattern in the center of the grid. We'll need a notion for the center,
and on resizing or moving of the terminal, we must ensure it is kept, because we loose them (resizing should readjust the board and move the cells to the new place)
