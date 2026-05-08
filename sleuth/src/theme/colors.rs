//! Color palette for SLEUTH - paper-cream aesthetic

use iced::Color;

/// Paper-cream background - warm off-white for reading comfort
pub const PAPER: Color = Color::from_rgb(
    0xf6 as f32 / 255.0,
    0xf1 as f32 / 255.0,
    0xe6 as f32 / 255.0,
);

/// Darker paper for panels/sidebars
pub const PAPER_DARK: Color = Color::from_rgb(
    0xec as f32 / 255.0,
    0xe5 as f32 / 255.0,
    0xd5 as f32 / 255.0,
);

/// Even darker for borders/dividers
pub const PAPER_BORDER: Color = Color::from_rgb(
    0xd4 as f32 / 255.0,
    0xcc as f32 / 255.0,
    0xb8 as f32 / 255.0,
);

/// Primary text color - dark brown for readability
pub const TEXT_PRIMARY: Color = Color::from_rgb(
    0x2d as f32 / 255.0,
    0x2a as f32 / 255.0,
    0x26 as f32 / 255.0,
);

/// Secondary/muted text
pub const TEXT_SECONDARY: Color = Color::from_rgb(
    0x6b as f32 / 255.0,
    0x64 as f32 / 255.0,
    0x5a as f32 / 255.0,
);

// Diff colors - semantic highlighting

/// Inserted text background (green)
pub const INSERT_BG: Color = Color::from_rgb(
    0xd4 as f32 / 255.0,
    0xed as f32 / 255.0,
    0xda as f32 / 255.0,
);

/// Inserted text foreground (dark green)
pub const INSERT_FG: Color = Color::from_rgb(
    0x15 as f32 / 255.0,
    0x58 as f32 / 255.0,
    0x24 as f32 / 255.0,
);

/// Deleted text background (red)
pub const DELETE_BG: Color = Color::from_rgb(
    0xf8 as f32 / 255.0,
    0xd7 as f32 / 255.0,
    0xda as f32 / 255.0,
);

/// Deleted text foreground (dark red)
pub const DELETE_FG: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x1c as f32 / 255.0,
    0x24 as f32 / 255.0,
);

// Political party colors for blame gutter

/// Republican red
pub const PARTY_R: Color = Color::from_rgb(
    0xe9 as f32 / 255.0,
    0x14 as f32 / 255.0,
    0x1d as f32 / 255.0,
);

/// Democrat blue
pub const PARTY_D: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x15 as f32 / 255.0,
    0xbc as f32 / 255.0,
);

/// Independent purple
pub const PARTY_I: Color = Color::from_rgb(
    0x80 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x80 as f32 / 255.0,
);

// UI accent colors

/// Selection highlight
pub const SELECTION: Color = Color::from_rgb(
    0xc9 as f32 / 255.0,
    0xda as f32 / 255.0,
    0xf2 as f32 / 255.0,
);

/// Hover highlight
pub const HOVER: Color = Color::from_rgb(
    0xe8 as f32 / 255.0,
    0xe2 as f32 / 255.0,
    0xd4 as f32 / 255.0,
);

/// Link/action color
pub const ACCENT: Color = Color::from_rgb(
    0x1a as f32 / 255.0,
    0x56 as f32 / 255.0,
    0xdb as f32 / 255.0,
);

// Badge colors

/// Changed badge background (amber)
pub const BADGE_CHANGED: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0xec as f32 / 255.0,
    0xb3 as f32 / 255.0,
);

/// New badge background (green)
pub const BADGE_NEW: Color = Color::from_rgb(
    0xc8 as f32 / 255.0,
    0xe6 as f32 / 255.0,
    0xc9 as f32 / 255.0,
);

/// Deleted badge background (red)
pub const BADGE_DELETED: Color = Color::from_rgb(
    0xff as f32 / 255.0,
    0xcd as f32 / 255.0,
    0xd2 as f32 / 255.0,
);
