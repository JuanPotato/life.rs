# life.rs

[game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implementation in rust, using the standard grid algorithm (as opposed to something like [this](https://www.refsmmat.com/posts/2016-01-25-conway-game-of-life.html)).

usage:
-   `./gol` for random start, (almost) infinite iterations
-   `./gol N` for random start, N iterations


edit the SIZE, INIT, and SLEEP_TIME_MS constants at the top of main.rs if you want a different size, number of starting cells, or delay between generations.