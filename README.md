# circle_guesser

Simple game for guessing the centre of a circle, using 3 points on the circumference.

## Controls
 - Left Mouse Button: Guess the centre
 - Right Mouse Button/C: Restart
 - Left Arrow: fewer points
 - Right Arrow: more points
 
## Installation
 - Go [here](https://github.com/Epacnoss/circle_guesser/actions/workflows/build.yml), and find the top-most thing with a tick.
 - Click `CG_Windows` at the bottom to download the executable. It should be in the Artifacts section.
 - Extract the zip and run the executable.
 - If Windows complains, click `More Info` and then `Run Anyway`.

## User Experience
 - Two windows should open - one of them should be a square, with 3 green dots. This is the main game window, and the other is for displaying messages like scores.
 - The 3 green dots show the 3 positions on the circumference, and you can use a greater or fewer number of points with the left and right arrow keys.
 - Click on the window to make a guess, and a Red circle will show the actual Centre, a Grey outline will show the circumference, and a Blue circle will show where you clicked.
 - Press C or Right Click to try again. If you changed the number of hints, that will be preserved.
 - NB: If you resize the Window, a couple of invariants break so it will restart the current round.
