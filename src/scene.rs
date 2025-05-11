use std::fs;
use std::path::PathBuf;
use iced::{Command, Element, Font, Length};
use iced::widget::{container, row, Space, Column, text};
use crate::Msg;


#[derive(Debug)]
pub struct MainScene {
    pub hexdata1: Vec<String>,
    pub hexdata2: Vec<String>,
    pub scroll_offset: f32,
}


impl MainScene {
    pub fn update_scene(&mut self, message: Msg) -> Command<Msg> {
        log::info!("update {message:?}");
        match message {
            Msg::EditData1(action) => log::info!("edit data1: {}", action.is_edit()),
            Msg::EditData2(action) => log::info!("edit data2: {}", action.is_edit()),
        }
        Command::none()
    }

    pub fn view_scene(&self) -> Element<Msg> {
        log::info!("view");

        container(
            row![
                render_lines(&self.hexdata1[self.scroll_offset as usize .. self.scroll_offset as usize + 20]),
                Space::with_width(Length::Fill),
                render_lines(&self.hexdata2[self.scroll_offset as usize .. self.scroll_offset as usize + 20]),
            ]
        )
            .padding(20)
            .into()
    }
}


pub fn load_data_file_hex(path: &PathBuf) -> Result<Vec<String>, String> {
    const COL_COUNT: usize = 16;

    let data: Vec<u8> = fs::read(path)
        .map_err(|e| format!("Failed to read data file at {path:?}: {e}"))?;

    let mut result: Vec<u8> = Vec::with_capacity(data.len() * 3 - 1);
    for (i, &b) in data.iter().enumerate() {
        let offset: usize = (b as usize) * 3;
        result.extend_from_slice(&HEX_TABLE[offset..offset + 2]);
        if i != data.len() - 1 {
            result.push(b' ');
        }
    }

    let mut lines: Vec<String> = Vec::with_capacity(result.len() / COL_COUNT + 1);
    let mut i: usize = 0;
    loop {
        let end: usize = result.len().min(i + COL_COUNT*2+1);
        let slice: &[u8] = &result[i..end];
        // SAFETY: All values pushed are valid ASCII, so this is safe.
        let string: &str = unsafe { std::str::from_utf8_unchecked(slice) };
        lines.push(string.to_string());

        if i + COL_COUNT*2+1 > result.len() {
            return Ok(lines)
        }
        i += COL_COUNT*2+1;
    }
}


const HEX_TABLE: &[u8; 767] = b"\
00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F \
10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F \
20 21 22 23 24 25 26 27 28 29 2A 2B 2C 2D 2E 2F \
30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F \
40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F \
50 51 52 53 54 55 56 57 58 59 5A 5B 5C 5D 5E 5F \
60 61 62 63 64 65 66 67 68 69 6A 6B 6C 6D 6E 6F \
70 71 72 73 74 75 76 77 78 79 7A 7B 7C 7D 7E 7F \
80 81 82 83 84 85 86 87 88 89 8A 8B 8C 8D 8E 8F \
90 91 92 93 94 95 96 97 98 99 9A 9B 9C 9D 9E 9F \
A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 AA AB AC AD AE AF \
B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 BA BB BC BD BE BF \
C0 C1 C2 C3 C4 C5 C6 C7 C8 C9 CA CB CC CD CE CF \
D0 D1 D2 D3 D4 D5 D6 D7 D8 D9 DA DB DC DD DE DF \
E0 E1 E2 E3 E4 E5 E6 E7 E8 E9 EA EB EC ED EE EF \
F0 F1 F2 F3 F4 F5 F6 F7 F8 F9 FA FB FC FD FE FF";


fn render_lines(lines: &[String]) -> Element<Msg> {
    lines.iter().fold(Column::new(), |col, line| col.push(
        text(line)
            .font(Font::MONOSPACE).size(16)
    ))
        .spacing(0)
        .padding(0)
        .into()
}

