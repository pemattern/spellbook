use phf::phf_map;
use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct Icon {
    pub str: &'static str,
    pub color: Color,
}

impl Icon {
    pub const fn new(icon: &'static str, color: Color) -> Self {
        Self { str: icon, color }
    }

    pub const EMPTY: Icon = Icon::new(" ", Color::White);
}

pub static APPLICATION_ICON_MAP: phf::Map<&'static str, Icon> = phf_map! {
    "Blender" => Icon::new("󰂫", Color::Yellow),
    "Chrome" => Icon::new("", Color::Red),
    "Chromium" => Icon::new("", Color::Cyan),
    "Code" => Icon::new("󰨞", Color::Blue),
    "Edge" => Icon::new("󰇩", Color::Cyan),
    "Firefox" => Icon::new("󰈹", Color::LightRed),
    "LibreOffice" => Icon::new("", Color::Gray),
    "LibreOffice Base" => Icon::new("", Color::Magenta),
    "LibreOffice Calc" => Icon::new("", Color::Green),
    "LibreOffice Draw" => Icon::new("", Color::Yellow),
    "LibreOffice Impress" => Icon::new("", Color::LightRed),
    "LibreOffice Math" => Icon::new("", Color::Red),
    "LibreOffice Writer" => Icon::new("", Color::Blue),
    "Neovim" => Icon::new("", Color::Green),
    "Opera" => Icon::new("", Color::Red),
    "Steam" => Icon::new("󰓓", Color::Cyan),
    "Thunderbird" => Icon::new("", Color::Blue),
    "Vim" => Icon::new("", Color::Green),
    "VLC" => Icon::new("󰕼", Color::LightRed),
};

pub static CATEGORY_ICON_MAP: phf::Map<&'static str, Icon> = phf_map! {
    "AudioVideo" => Icon::new("", Color::LightGreen),
    "Audio" => Icon::new("", Color::LightBlue),
    "Video" => Icon::new("󰕧", Color::LightCyan),
    "Development" => Icon::new("󰅩", Color::LightYellow),
    "Education" => Icon::new("", Color::Gray),
    "Game" => Icon::new("󱎓", Color::LightMagenta),
    "Graphics" => Icon::new("󰟽", Color::Magenta),
    "Network" => Icon::new("󰛳", Color::Blue),
    "Office" => Icon::new("󰈙", Color::White),
    "Settings" => Icon::new("", Color::Yellow),
    "System" => Icon::new("󰒓", Color::DarkGray),
    "Utility" => Icon::new("󱌢", Color::Gray),
};
