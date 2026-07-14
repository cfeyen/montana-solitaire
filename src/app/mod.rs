use std::num::NonZero;

pub mod card;

use iced::{Font, Color, widget::{Column, button, button::Style as ButtonStyle, column, container, row, text}};

use card::{Card, DEFAULT_BOARD, Face};

const ONE: NonZero<usize> = unsafe { NonZero::new_unchecked(1) };
const PADDING: f32 = 4.0;
const FONT_SIZE: f32 = 50.0;

pub const FONT: Font = Font {
    family: iced::font::Family::Name("Noto Serif"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub fn start() -> iced::Result {
    iced::application(Solitaire::default, Solitaire::update, Solitaire::view)
        .font(include_bytes!("../../assets/NotoSerif-VariableFont_wdth,wght.ttf"))
        .default_font(FONT)
        .style(|_, _| iced::theme::Style { background_color: Color::BLACK, text_color: Color::BLACK })
        .run()
}

pub struct Solitaire {
    board: [[Option<Card>; 13]; 4],
    history: Vec<Move>,
    rounds: NonZero<usize>,
    moves: usize,
    undos: usize,
    two_row: Option<usize>,
    scale: f32,
}

impl Default for Solitaire {
    fn default() -> Self {
        let mut this = Self {
            board: DEFAULT_BOARD,
            history: Vec::new(),
            rounds: ONE,
            moves: 0,
            undos: 0,
            two_row: None,
            scale: 1.0,
        };

        this.shuffle();

        this
    }
}

impl Solitaire {
    fn shuffle(&mut self) {
        let mut rng = fastrand::Rng::new();

        let mut cards = self.board.clone()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        rng.shuffle(&mut cards);

        for r in 0..4 {
            for c in 0..13 {
                self.board[r][c] = cards[r * 13 + c];
            }
        }
    }

    fn reshuffle(&mut self) {
        self.rounds = self.rounds.saturating_add(1);
        self.history = Vec::new();

        let mut fixed_len = [0; 4];

        let mut redistributable = Vec::new();

        for r in 0..4 {
            for c in 0..13 {
                if c == 0 {
                    if !self.board[r][c].is_some_and(|card| card.face == Face::N2) {
                        break;
                    } else {
                        fixed_len[r] += 1;
                    }
                } else if self.board[r][c].is_some() && self.board[r][c - 1].is_some_and(|card| card.next() == self.board[r][c]) {
                    fixed_len[r] += 1;
                } else {
                    break
                }
            }
        }

        for r in 0..4 {
            for c in 0..13 {
                if c >= fixed_len[r] {
                    redistributable.push(self.board[r][c]);
                }
            }
        }

        let mut rng = fastrand::Rng::new();

        rng.shuffle(&mut redistributable);

        for r in 0..4 {
            for c in 0..13 {
                if c >= fixed_len[r] {
                    self.board[r][c] = redistributable.pop().unwrap();
                }
            }
        }
    }

    fn get_next_pos(&mut self, card: Option<Card>) -> Option<(usize, usize)> {
        let next = card?.next()?;

        for r in 0..4 {
            for c in 0..13 {
                if self.board[r][c] == Some(next) {
                    return Some((r, c));
                }
            }
        }

        None
    }

    fn select(&mut self, row: usize, col: usize) {
        if let Some(two_row) = self.two_row {
            if col > 0 && matches!(self.board[row][col], Some(Card { suite: _, face: Face::N2 })) {
                self.board[two_row][0] = std::mem::take(&mut self.board[row][col]);
                self.history.push(Move { r1: two_row as _, c1: 0, r2: row as _, c2: col as _ });
                self.moves += 1;
            }

            self.two_row = None;
        } else if self.board[row][col].is_none() {
            if col == 0 {
                self.two_row = Some(row);
            } else if let Some((next_row, next_col)) = self.get_next_pos(self.board[row][col - 1]) {
                self.board[row][col] = std::mem::take(&mut self.board[next_row][next_col]);
                self.history.push(Move { r1: row as _, c1: col as _, r2: next_row as _, c2: next_col as _ });
                self.moves += 1;
            }
        }
    }

    fn undo(&mut self) {
        if let Some(last_move) = self.history.pop() {
            self.two_row = None;

            self.board[last_move.r2 as usize][last_move.c2 as usize] = std::mem::take(&mut self.board[last_move.r1 as usize][last_move.c1 as usize]);
            self.moves -= 1;
            self.undos += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectCard { row: usize, col: usize },
    Reshuffle,
    Reset,
    Undo,
}

impl Solitaire {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            row![
                container(button(text("Reset").size(FONT_SIZE)).on_press(Message::Reset)).padding(PADDING).center_x(iced::Fill),
                container(text(format!("Rounds: {}", self.rounds)).size(FONT_SIZE).color(Color::WHITE)).padding(PADDING).center_x(iced::Fill),
                container(text(format!("Moves: {}", self.moves)).size(FONT_SIZE).color(Color::WHITE)).padding(PADDING).center_x(iced::Fill),
                container(text(format!("Undos: {}", self.undos)).size(FONT_SIZE).color(Color::WHITE)).padding(PADDING).center_x(iced::Fill),
                container(button(text("Reshuffle").size(FONT_SIZE)).on_press(Message::Reshuffle)).padding(PADDING).center_x(iced::Fill),
                container(button(text("Undo").size(FONT_SIZE)).on_press(Message::Undo)).padding(PADDING).center_x(iced::Fill),
            ],
            self.board()
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Reset => *self = Self::default(),
            Message::Reshuffle => self.reshuffle(),
            Message::SelectCard { row, col } => self.select(row, col),
            Message::Undo => self.undo(),
        }
    }

    fn board(&self) -> Column<'_, Message> {
        column(self.board.iter().enumerate().map(|(row_num, r)| {
            row(r.iter().enumerate().map(|(col_num, c)| {
                container(
                    button(
                        container(text(c.as_ref().map_or("", |c| c.face.as_str())).size(FONT_SIZE))
                            .width(self.scale)
                            .height(self.scale * 1.4)
                            .center(iced::Fill)
                    )
                    .style(move |_, _| ButtonStyle::default().with_background(c.as_ref().map_or(if let Some(two_row) = self.two_row && col_num == 0 && row_num == two_row {
                        Color::from_rgb8(0xFF, 0xFF, 0x00)
                    } else {
                        Color::WHITE
                    }, |c| c.suite.to_color())))
                    .on_press(Message::SelectCard { row: row_num, col: col_num })
                )
                .padding(PADDING)
                .into()
            })).into()
        }))
    }
}

struct Move {
    r1: u8,
    c1: u8,
    r2: u8,
    c2: u8,
}
