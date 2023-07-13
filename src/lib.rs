use pyo3::{exceptions::PyValueError, prelude::*};

macro_rules! game {
    ($name:ident, $size:literal, $half_komi:literal) => {
        mod $name {
            use super::*;

            #[pyclass]
            #[derive(Clone, Default)]
            pub struct Game(fast_tak::Game<$size, $half_komi>);

            #[pymethods]
            impl Game {
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

                // TODO: Can we avoid all these ugly allocations?
                #[getter]
                fn board(&self) -> Vec<Vec<Option<(Piece, Vec<Color>)>>> {
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
        (5, 4) => Ok(size_5_half_komi_4::Game::default().into_py(py)),
        (6, 4) => Ok(size_6_half_komi_4::Game::default().into_py(py)),
        (7, 4) => Ok(size_7_half_komi_4::Game::default().into_py(py)),
        (8, 4) => Ok(size_8_half_komi_4::Game::default().into_py(py)),
        _ => Err(PyValueError::new_err("Unsupported size or komi")),
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
        use fast_tak::takparse::Color::*;
        use fast_tak::GameResult::*;
        match value {
            Ongoing => Self::Ongoing,
            Winner { color: White, .. } => Self::WhiteWin,
            Winner { color: Black, .. } => Self::BlackWin,
            Draw { .. } => Self::Draw,
        }
    }
}

#[pyclass]
enum Color {
    White,
    Black,
}

impl From<fast_tak::takparse::Color> for Color {
    fn from(value: fast_tak::takparse::Color) -> Self {
        use fast_tak::takparse::Color::*;
        match value {
            White => Self::White,
            Black => Self::Black,
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
        use fast_tak::takparse::Piece::*;
        match value {
            Flat => Self::Flat,
            Wall => Self::Wall,
            Cap => Self::Cap,
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

#[pymodule]
fn takpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_game, m)?)?;
    m.add_class::<Move>()?;
    m.add_class::<GameResult>()?;
    m.add_class::<PlayError>()?;
    Ok(())
}
