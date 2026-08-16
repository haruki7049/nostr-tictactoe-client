use thiserror::Error;

const LINE_LENGTH: usize = 3;
const COLUMN_LENGTH: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    inner: [Line; COLUMN_LENGTH],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            inner: [
                Line {
                    inner: [BoardState::None, BoardState::None, BoardState::None],
                },
                Line {
                    inner: [BoardState::None, BoardState::None, BoardState::None],
                },
                Line {
                    inner: [BoardState::None, BoardState::None, BoardState::None],
                },
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    inner: [BoardState; LINE_LENGTH],
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LineError {
    #[error("Invalid line: {0:#?}")]
    InvalidLine(usize),
}

impl Line {
    // Zero start
    pub fn set(&mut self, side: PlayerSide, position: usize) -> Result<(), LineError> {
        Self::is_valid_position(position)?;

        match side {
            PlayerSide::Nought => self.set_nought(position),
            PlayerSide::Cross => self.set_cross(position),
        }
    }

    fn set_nought(&mut self, position: usize) -> Result<(), LineError> {
        Self::is_valid_position(position)?;

        // Edit line state
        self.inner[position] = BoardState::Nought;

        Ok(())
    }

    fn set_cross(&mut self, position: usize) -> Result<(), LineError> {
        Self::is_valid_position(position)?;

        // Edit line state
        self.inner[position] = BoardState::Cross;

        Ok(())
    }

    fn is_valid_position(position: usize) -> Result<(), LineError> {
        if position >= LINE_LENGTH {
            return Err(LineError::InvalidLine(position));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSide {
    /// O
    Nought,

    /// X
    Cross,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Board {
    pub fn set(&mut self, side: PlayerSide, position: Position) -> Result<(), BoardError> {
        Self::is_valid_column(position.column)?;

        // Edit the column state
        self.inner[position.column].set(side, position.line)?;

        Ok(())
    }

    fn is_valid_column(column: usize) -> Result<(), BoardError> {
        if column >= COLUMN_LENGTH {
            return Err(BoardError::InvalidColumn(column));
        }

        Ok(())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BoardError {
    #[error("Invalid column: {0:#?}")]
    InvalidColumn(usize),

    #[error("From LineError: {0:#?}")]
    Line(#[from] LineError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardState {
    /// Empty
    None,

    /// O
    Nought,

    /// X
    Cross,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use super::{Board, BoardError, BoardState, Line, LineError, PlayerSide, Position};

        #[test]
        fn board_default_value() {
            let board: Board = Board::default();
            assert_eq!(
                board.inner,
                [
                    Line {
                        inner: [BoardState::None, BoardState::None, BoardState::None],
                    },
                    Line {
                        inner: [BoardState::None, BoardState::None, BoardState::None],
                    },
                    Line {
                        inner: [BoardState::None, BoardState::None, BoardState::None],
                    },
                ]
            );
        }

        #[test]
        fn board_set_nought() -> anyhow::Result<()> {
            let mut board: Board = Board::default();
            board.set(PlayerSide::Nought, Position { line: 0, column: 0 })?;

            assert_eq!(
                board.inner,
                [
                    Line {
                        inner: [BoardState::Nought, BoardState::None, BoardState::None],
                    },
                    Line {
                        inner: [BoardState::None, BoardState::None, BoardState::None],
                    },
                    Line {
                        inner: [BoardState::None, BoardState::None, BoardState::None],
                    },
                ]
            );

            Ok(())
        }

        #[test]
        fn board_set_on_invalid_position() -> anyhow::Result<()> {
            const INVALID_LINE: usize = 5;
            let mut board: Board = Board::default();
            let error: Result<(), BoardError> = board.set(
                PlayerSide::Nought,
                Position {
                    line: INVALID_LINE,
                    column: 0,
                },
            );

            assert!(error.is_err());
            assert_eq!(
                error,
                Err(BoardError::Line(LineError::InvalidLine(INVALID_LINE)))
            );

            Ok(())
        }
    }

    mod line {
        use super::{BoardState, Line, LineError, PlayerSide};

        #[test]
        fn line_set_nought() -> anyhow::Result<()> {
            let mut line = Line {
                inner: [BoardState::None, BoardState::None, BoardState::None],
            };
            line.set(PlayerSide::Nought, 0)?;

            assert_eq!(
                line,
                Line {
                    inner: [BoardState::Nought, BoardState::None, BoardState::None],
                }
            );

            Ok(())
        }

        #[test]
        fn line_set_cross() -> anyhow::Result<()> {
            let mut line = Line {
                inner: [BoardState::None, BoardState::None, BoardState::None],
            };
            line.set(PlayerSide::Cross, 0)?;

            assert_eq!(
                line,
                Line {
                    inner: [BoardState::Cross, BoardState::None, BoardState::None],
                }
            );

            Ok(())
        }

        #[test]
        fn line_set_on_invalid_position() -> anyhow::Result<()> {
            const INVALID_LINE: usize = 3;
            let mut line = Line {
                inner: [BoardState::None, BoardState::None, BoardState::None],
            };
            let error: Result<(), LineError> = line.set(PlayerSide::Nought, INVALID_LINE);

            assert!(error.is_err());
            assert_eq!(error, Err(LineError::InvalidLine(INVALID_LINE)));

            Ok(())
        }
    }
}
