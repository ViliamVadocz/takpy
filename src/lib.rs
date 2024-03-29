use std::hash::{Hash, Hasher};

use fast_tak::Symmetry;
use pyo3::{exceptions::PyValueError, prelude::*};

macro_rules! game {
    ($name:ident, $size:literal, $half_komi:literal) => {
        mod $name {
            use super::*;

            #[pyclass]
            #[derive(Clone, Default)]
            pub struct Game(pub fast_tak::Game<$size, $half_komi>);

            #[pymethods]
            impl Game {
                fn __repr__(&self) -> String {
                    fast_tak::takparse::Tps::from(self.0.clone()).to_string()
                }

                fn __eq__(&self, other: Game) -> bool {
                    self.0 == other.0
                }

                fn __hash__(&self) -> u64 {
                    let mut s = std::hash::DefaultHasher::new();
                    self.0.hash(&mut s);
                    s.finish()
                }

                #[getter]
                fn half_komi(&self) -> i8 {
                    $half_komi
                }

                #[getter]
                fn size(&self) -> usize {
                    $size
                }

                /// Get the moves possible in the current position.
                fn possible_moves(&self) -> Vec<Move> {
                    let mut moves = Vec::with_capacity(128);
                    self.0.possible_moves(&mut moves);
                    moves.into_iter().map(Move).collect()
                }

                /// Play a move.
                fn play(&mut self, my_move: Move) -> Result<(), PlayError> {
                    self.0.play(my_move.0).map_err(Into::into)
                }

                /// Check whether the game has ended and who as won.
                fn result(&self) -> GameResult {
                    self.0.result().into()
                }

                /// Get the board.
                fn board(&self) -> Vec<Vec<Option<(Piece, Vec<Color>)>>> {
                    // TODO: Can we avoid all these ugly allocations?
                    self.0
                        .board
                        .iter()
                        .map(|row| {
                            row.map(|stack| {
                                stack.top().map(|(piece, _color)| {
                                    (
                                        piece.into(),
                                        stack.colors().into_iter().map(Into::into).collect(),
                                    )
                                })
                            })
                            .collect()
                        })
                        .collect()
                }

                #[getter]
                fn to_move(&self) -> Color {
                    self.0.to_move.into()
                }

                #[getter]
                fn white_reserves(&self) -> (u8, u8) {
                    (self.0.white_reserves.stones, self.0.white_reserves.caps)
                }

                #[getter]
                fn black_reserves(&self) -> (u8, u8) {
                    (self.0.black_reserves.stones, self.0.black_reserves.caps)
                }

                #[getter]
                fn ply(&self) -> u16 {
                    self.0.ply
                }

                #[getter]
                fn reversible_plies(&self) -> u16 {
                    self.0.reversible_plies
                }

                fn clone(&self) -> Game {
                    Clone::clone(self)
                }

                /// Clone the game and play a move in it, returning the newly created game.
                fn clone_and_play(&self, my_move: Move) -> Result<Game, PlayError> {
                    let mut clone = Clone::clone(self);
                    clone.0.play(my_move.0)?;
                    Ok(clone)
                }

                /// Return all the symmetries of this position.
                fn symmetries(&self) -> [Self; 8] {
                    self.0.symmetries().map(Self)
                }

                /// Return the position in the canonical orientation.
                fn canonical(&self) -> Self {
                    Self(self.0.clone().canonical())
                }
            }
        }
    };
}

game!(size_3, 3, 0);
game!(size_4, 4, 0);
game!(size_5, 5, 0);
game!(size_6, 6, 0);
game!(size_7, 7, 0);
game!(size_8, 8, 0);
game!(size_3_half_komi_4, 3, 4);
game!(size_4_half_komi_4, 4, 4);
game!(size_5_half_komi_4, 5, 4);
game!(size_6_half_komi_4, 6, 4);
game!(size_7_half_komi_4, 7, 4);
game!(size_8_half_komi_4, 8, 4);

/// Create a new game with the given size and half-komi.
#[pyfunction]
#[pyo3(signature = (size, half_komi=0))]
fn new_game(py: Python, size: usize, half_komi: i8) -> PyResult<PyObject> {
    match (size, half_komi) {
        (3, 0) => Ok(size_3::Game::default().into_py(py)),
        (4, 0) => Ok(size_4::Game::default().into_py(py)),
        (5, 0) => Ok(size_5::Game::default().into_py(py)),
        (6, 0) => Ok(size_6::Game::default().into_py(py)),
        (7, 0) => Ok(size_7::Game::default().into_py(py)),
        (8, 0) => Ok(size_8::Game::default().into_py(py)),
        (3, 4) => Ok(size_3_half_komi_4::Game::default().into_py(py)),
        (4, 4) => Ok(size_4_half_komi_4::Game::default().into_py(py)),
        (5, 4) => Ok(size_5_half_komi_4::Game::default().into_py(py)),
        (6, 4) => Ok(size_6_half_komi_4::Game::default().into_py(py)),
        (7, 4) => Ok(size_7_half_komi_4::Game::default().into_py(py)),
        (8, 4) => Ok(size_8_half_komi_4::Game::default().into_py(py)),
        _ => Err(PyValueError::new_err("Unsupported size or komi")),
    }
}

#[pyfunction]
#[pyo3(signature = (size, tps, half_komi=0))]
fn game_from_tps(py: Python, size: usize, tps: &str, half_komi: i8) -> PyResult<PyObject> {
    let tps: fast_tak::takparse::Tps = tps.parse().map_err(Into::<ParseTpsError>::into)?;
    match (size, half_komi) {
        (3, 0) => Ok(size_3::Game(tps.into()).into_py(py)),
        (4, 0) => Ok(size_4::Game(tps.into()).into_py(py)),
        (5, 0) => Ok(size_5::Game(tps.into()).into_py(py)),
        (6, 0) => Ok(size_6::Game(tps.into()).into_py(py)),
        (7, 0) => Ok(size_7::Game(tps.into()).into_py(py)),
        (8, 0) => Ok(size_8::Game(tps.into()).into_py(py)),
        (3, 4) => Ok(size_3_half_komi_4::Game(tps.into()).into_py(py)),
        (4, 4) => Ok(size_4_half_komi_4::Game(tps.into()).into_py(py)),
        (5, 4) => Ok(size_5_half_komi_4::Game(tps.into()).into_py(py)),
        (6, 4) => Ok(size_6_half_komi_4::Game(tps.into()).into_py(py)),
        (7, 4) => Ok(size_7_half_komi_4::Game(tps.into()).into_py(py)),
        (8, 4) => Ok(size_8_half_komi_4::Game(tps.into()).into_py(py)),
        _ => Err(PyValueError::new_err("Unsupported size or komi")),
    }
}

#[pyclass]
struct ParseTpsError(fast_tak::takparse::ParseTpsError);

impl From<ParseTpsError> for PyErr {
    fn from(error: ParseTpsError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<fast_tak::takparse::ParseTpsError> for ParseTpsError {
    fn from(error: fast_tak::takparse::ParseTpsError) -> Self {
        Self(error)
    }
}

#[pyclass]
#[derive(Clone)]
struct Move(fast_tak::takparse::Move);

#[pymethods]
impl Move {
    fn __repr__(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: Move) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let mut s = std::hash::DefaultHasher::new();
        self.0.hash(&mut s);
        s.finish()
    }

    #[new]
    fn new(s: &str) -> PyResult<Self> {
        Ok(Self(s.parse().map_err(Into::<ParseMoveError>::into)?))
    }

    #[getter]
    fn square(&self) -> (u8, u8) {
        let square = self.0.square();
        (square.row(), square.column())
    }

    #[getter]
    fn kind(&self) -> MoveKind {
        self.0.kind().into()
    }

    #[getter]
    fn piece(&self) -> Option<Piece> {
        match self.0.kind() {
            fast_tak::takparse::MoveKind::Place(piece) => Some(piece.into()),
            _ => None,
        }
    }

    #[getter]
    fn direction(&self) -> Option<Direction> {
        match self.0.kind() {
            fast_tak::takparse::MoveKind::Spread(direction, ..) => Some(direction.into()),
            _ => None,
        }
    }

    fn drop_counts(&self) -> Option<Vec<u32>> {
        match self.0.kind() {
            fast_tak::takparse::MoveKind::Spread(.., pattern) => {
                Some(pattern.into_iter().collect())
            }
            _ => None,
        }
    }

    fn symmetries(&self, size: usize) -> PyResult<[Self; 8]> {
        Ok(match size {
            3 => Symmetry::<3>::symmetries(&self.0),
            4 => Symmetry::<4>::symmetries(&self.0),
            5 => Symmetry::<5>::symmetries(&self.0),
            6 => Symmetry::<6>::symmetries(&self.0),
            7 => Symmetry::<7>::symmetries(&self.0),
            8 => Symmetry::<8>::symmetries(&self.0),
            _ => return Err(PyValueError::new_err("Unsupported size")),
        }
        .map(Self))
    }
}

#[pyclass]
enum MoveKind {
    Place,
    Spread,
}

impl From<fast_tak::takparse::MoveKind> for MoveKind {
    fn from(value: fast_tak::takparse::MoveKind) -> Self {
        use fast_tak::takparse::MoveKind;
        match value {
            MoveKind::Spread(..) => Self::Spread,
            MoveKind::Place(..) => Self::Place,
        }
    }
}

#[pyclass]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<fast_tak::takparse::Direction> for Direction {
    fn from(value: fast_tak::takparse::Direction) -> Self {
        use fast_tak::takparse::Direction;
        match value {
            Direction::Up => Self::Up,
            Direction::Down => Self::Down,
            Direction::Left => Self::Left,
            Direction::Right => Self::Right,
        }
    }
}

#[pyclass]
enum GameResult {
    Ongoing,
    WhiteWin,
    BlackWin,
    Draw,
}

impl From<fast_tak::GameResult> for GameResult {
    fn from(value: fast_tak::GameResult) -> Self {
        use fast_tak::takparse::Color;
        use fast_tak::GameResult;
        match value {
            GameResult::Ongoing => Self::Ongoing,
            GameResult::Winner {
                color: Color::White,
                ..
            } => Self::WhiteWin,
            GameResult::Winner {
                color: Color::Black,
                ..
            } => Self::BlackWin,
            GameResult::Draw { .. } => Self::Draw,
        }
    }
}

impl From<fast_tak::takparse::GameResult> for GameResult {
    fn from(value: fast_tak::takparse::GameResult) -> Self {
        use fast_tak::takparse::GameResult;
        match value {
            GameResult::White(..) => Self::WhiteWin,
            GameResult::Black(..) => Self::BlackWin,
            GameResult::Draw => Self::Draw,
        }
    }
}

#[pymethods]
impl GameResult {
    fn color(&self) -> Option<Color> {
        match self {
            Self::WhiteWin => Some(Color::White),
            Self::BlackWin => Some(Color::Black),
            _ => None,
        }
    }
}

#[pyclass]
enum Color {
    White,
    Black,
}

#[pymethods]
impl Color {
    fn next(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl From<fast_tak::takparse::Color> for Color {
    fn from(value: fast_tak::takparse::Color) -> Self {
        use fast_tak::takparse::Color;
        match value {
            Color::White => Self::White,
            Color::Black => Self::Black,
        }
    }
}

#[pyclass]
enum Piece {
    Flat,
    Wall,
    Cap,
}

impl From<fast_tak::takparse::Piece> for Piece {
    fn from(value: fast_tak::takparse::Piece) -> Self {
        use fast_tak::takparse::Piece;
        match value {
            Piece::Flat => Self::Flat,
            Piece::Wall => Self::Wall,
            Piece::Cap => Self::Cap,
        }
    }
}

#[pyclass]
struct PlayError(fast_tak::PlayError);

impl From<PlayError> for PyErr {
    fn from(error: PlayError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<fast_tak::PlayError> for PlayError {
    fn from(error: fast_tak::PlayError) -> Self {
        Self(error)
    }
}

#[pyclass]
struct ParseMoveError(fast_tak::takparse::ParseMoveError);

impl From<ParseMoveError> for PyErr {
    fn from(error: ParseMoveError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<fast_tak::takparse::ParseMoveError> for ParseMoveError {
    fn from(error: fast_tak::takparse::ParseMoveError) -> Self {
        Self(error)
    }
}

#[pyclass]
struct Ptn(fast_tak::takparse::Ptn);

#[pymethods]
impl Ptn {
    #[new]
    fn new(s: &str) -> PyResult<Self> {
        Ok(Self(s.parse().map_err(Into::<ParsePtnError>::into)?))
    }

    fn tags(&self) -> Vec<(String, String)> {
        self.0
            .tags()
            .iter()
            .map(|tag| (tag.key().to_owned(), tag.value().to_owned()))
            .collect()
    }

    fn moves(&self) -> Vec<Move> {
        self.0.moves().iter().map(|m| Move(*m)).collect()
    }

    fn comments(&self) -> Vec<Vec<String>> {
        self.0.comments().to_vec()
    }

    #[getter]
    fn result(&self) -> GameResult {
        self.0
            .result()
            .map(GameResult::from)
            .unwrap_or(GameResult::Ongoing)
    }
}

#[pyclass]
struct ParsePtnError(fast_tak::takparse::ParsePtnError);

impl From<ParsePtnError> for PyErr {
    fn from(error: ParsePtnError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

impl From<fast_tak::takparse::ParsePtnError> for ParsePtnError {
    fn from(error: fast_tak::takparse::ParsePtnError) -> Self {
        Self(error)
    }
}

#[pymodule]
fn takpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_game, m)?)?;
    m.add_function(wrap_pyfunction!(game_from_tps, m)?)?;
    m.add_class::<size_3::Game>()?; // export one of Game objects to help with type hints
    m.add_class::<Move>()?;
    m.add_class::<GameResult>()?;
    m.add_class::<Color>()?;
    m.add_class::<Piece>()?;
    m.add_class::<Direction>()?;
    m.add_class::<MoveKind>()?;
    m.add_class::<Ptn>()?;
    m.add_class::<PlayError>()?;
    m.add_class::<ParseMoveError>()?;
    m.add_class::<ParseTpsError>()?;
    m.add_class::<ParsePtnError>()?;
    Ok(())
}
