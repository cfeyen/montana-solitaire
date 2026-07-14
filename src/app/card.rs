use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suite: Suite,
    pub face: Face,
}

impl Card {
    pub fn next(&self) -> Option<Self> {
        self.face.next().map(|f| Self { suite: self.suite, face: f})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suite {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl Suite {
    pub const fn to_color(&self) -> Color {
        match self {
            Self::Spade => Color::from_rgb8(0x88, 0x88, 0x88),
            Self::Club => Color::from_rgb8(0x00, 0xFF, 0x00),
            Self::Heart => Color::from_rgb8(0xFF, 0x00, 0x00),
            Self::Diamond => Color::from_rgb8(0x00, 0xFF, 0xFF),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    J,
    Q,
    K,
}

impl Face {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::N2 => "2",
            Self::N3 => "3",
            Self::N4 => "4",
            Self::N5 => "5",
            Self::N6 => "6",
            Self::N7 => "7",
            Self::N8 => "8",
            Self::N9 => "9",
            Self::N10 => "10",
            Self::J => "J",
            Self::Q => "Q",
            Self::K => "K",
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            Self::N2 => Some(Self::N3),
            Self::N3 => Some(Self::N4),
            Self::N4 => Some(Self::N5),
            Self::N5 => Some(Self::N6),
            Self::N6 => Some(Self::N7),
            Self::N7 => Some(Self::N8),
            Self::N8 => Some(Self::N9),
            Self::N9 => Some(Self::N10),
            Self::N10 => Some(Self::J),
            Self::J => Some(Self::Q),
            Self::Q => Some(Self::K),
            Self::K => None,
        }
    }
}

pub const DEFAULT_BOARD: [[Option<Card>; 13]; 4] = [
    [
        Some(Card { suite: Suite::Spade, face: Face::N2 }),
        Some(Card { suite: Suite::Spade, face: Face::N3 }),
        Some(Card { suite: Suite::Spade, face: Face::N4 }),
        Some(Card { suite: Suite::Spade, face: Face::N5 }),
        Some(Card { suite: Suite::Spade, face: Face::N6 }),
        Some(Card { suite: Suite::Spade, face: Face::N7 }),
        Some(Card { suite: Suite::Spade, face: Face::N8 }),
        Some(Card { suite: Suite::Spade, face: Face::N9 }),
        Some(Card { suite: Suite::Spade, face: Face::N10 }),
        Some(Card { suite: Suite::Spade, face: Face::J }),
        Some(Card { suite: Suite::Spade, face: Face::Q }),
        Some(Card { suite: Suite::Spade, face: Face::K }),
        None,
    ],
    [
        Some(Card { suite: Suite::Club, face: Face::N2 }),
        Some(Card { suite: Suite::Club, face: Face::N3 }),
        Some(Card { suite: Suite::Club, face: Face::N4 }),
        Some(Card { suite: Suite::Club, face: Face::N5 }),
        Some(Card { suite: Suite::Club, face: Face::N6 }),
        Some(Card { suite: Suite::Club, face: Face::N7 }),
        Some(Card { suite: Suite::Club, face: Face::N8 }),
        Some(Card { suite: Suite::Club, face: Face::N9 }),
        Some(Card { suite: Suite::Club, face: Face::N10 }),
        Some(Card { suite: Suite::Club, face: Face::J }),
        Some(Card { suite: Suite::Club, face: Face::Q }),
        Some(Card { suite: Suite::Club, face: Face::K }),
        None,
    ],
    [
        Some(Card { suite: Suite::Heart, face: Face::N2 }),
        Some(Card { suite: Suite::Heart, face: Face::N3 }),
        Some(Card { suite: Suite::Heart, face: Face::N4 }),
        Some(Card { suite: Suite::Heart, face: Face::N5 }),
        Some(Card { suite: Suite::Heart, face: Face::N6 }),
        Some(Card { suite: Suite::Heart, face: Face::N7 }),
        Some(Card { suite: Suite::Heart, face: Face::N8 }),
        Some(Card { suite: Suite::Heart, face: Face::N9 }),
        Some(Card { suite: Suite::Heart, face: Face::N10 }),
        Some(Card { suite: Suite::Heart, face: Face::J }),
        Some(Card { suite: Suite::Heart, face: Face::Q }),
        Some(Card { suite: Suite::Heart, face: Face::K }),
        None,
    ],
    [
        Some(Card { suite: Suite::Diamond, face: Face::N2 }),
        Some(Card { suite: Suite::Diamond, face: Face::N3 }),
        Some(Card { suite: Suite::Diamond, face: Face::N4 }),
        Some(Card { suite: Suite::Diamond, face: Face::N5 }),
        Some(Card { suite: Suite::Diamond, face: Face::N6 }),
        Some(Card { suite: Suite::Diamond, face: Face::N7 }),
        Some(Card { suite: Suite::Diamond, face: Face::N8 }),
        Some(Card { suite: Suite::Diamond, face: Face::N9 }),
        Some(Card { suite: Suite::Diamond, face: Face::N10 }),
        Some(Card { suite: Suite::Diamond, face: Face::J }),
        Some(Card { suite: Suite::Diamond, face: Face::Q }),
        Some(Card { suite: Suite::Diamond, face: Face::K }),
        None,
    ],
];