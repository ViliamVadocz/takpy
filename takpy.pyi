from enum import Enum

Stack = tuple[Piece, list[Color]]
Board = list[list[None | Stack]]

def new_game(size: int, half_komi: int = 0) -> Game: ...
def game_from_tps(size: int, tps: str, half_komi: int = 0) -> Game: ...

class Game:
    half_komi: int
    size: int
    board: Board
    to_move: Color
    white_reserves: tuple[int, int]
    black_reserves: tuple[int, int]
    ply: int
    reversible_plies: int
    possible_moves: list[Move]
    result: GameResult

    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    def play(self, my_move: Move) -> None: ...
    def clone(self) -> Game: ...
    def clone_and_play(self, my_move: Move) -> Game: ...

class Move:
    square: tuple[int, int]
    kind: MoveKind
    piece: None | Piece
    direction: None | Direction
    drop_counts: None | list[int]

    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...
    @staticmethod
    def from_ptn(s: str) -> Move: ...

class MoveKind(Enum):
    Place = 0
    Spread = 1

class Direction(Enum):
    Up = 0
    Down = 1
    Left = 2
    Right = 3

class GameResult(Enum):
    Ongoing = 0
    WhiteWin = 1
    BlackWin = 2
    Draw = 3

    def color(self) -> Color | None: ...

class Color(Enum):
    White = 0
    Black = 1

    def next(self) -> Color: ...

class Piece(Enum):
    Flat = 0
    Wall = 1
    Cap = 2
