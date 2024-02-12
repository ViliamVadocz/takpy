# takpy

Python wrapper for [`fast-tak`](https://crates.io/crates/fast-tak).

Create a new game using `takpy.new_game(size, half_komi)`.

```py
from takpy import new_game, GameResult
game = new_game(6, 4)

while game.result() == GameResult.Ongoing:
    moves = game.possible_moves()     
    # pick move
    game.play(move)
```

- `game.possible_moves()` gets a list of available moves in the current position.
- `game.play(move)` is used to play a move in the game.
- `game.result()` calculates the result of the game.

There are getters for `game.board`, `game.to_move`, `game.white_reserves`, `game.black_reserves`, `game.ply`, and `game.reversible_plies`.

You can find out more about how to use the library as part of my [Takbot Tutorial](https://viliamvadocz.github.io/takbot_tutorial/takpy.html).
