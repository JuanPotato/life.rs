# life.rs

[game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implementation in rust, using the standard grid algorithm (as opposed to something like [this](https://www.refsmmat.com/posts/2016-01-25-conway-game-of-life.html)).

usage:
-   `./gol` for random start, (almost) infinite iterations
-   `./gol N` for random start, N iterations


edit the WIDTH, HEIGHT, ALIVE_CHANCE, and SLEEP_TIME_MS constants at the top of main.rs if you want a different size, number of starting cells, or delay between generations.


![random grid gif](https://user-images.githubusercontent.com/15344581/52524646-2e4d1c00-2c97-11e9-9fd0-188a5013478e.gif)

![glider gun gif](https://user-images.githubusercontent.com/15344581/52524579-5d16c280-2c96-11e9-91c0-8dad3fe36852.gif)
