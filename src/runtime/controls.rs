use java_runtime::DeviceProfile;
use winit::keyboard::KeyCode;

pub(super) const CONTROL_PANEL_HEIGHT: usize = 112;

#[derive(Clone, Copy)]
struct ControlButton {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    key_code: i32,
    label: &'static str,
    kind: ControlKind,
}

#[derive(Clone, Copy)]
enum ControlKind {
    Label,
    Up,
    Down,
    Left,
    Right,
}

fn control_buttons(profile: DeviceProfile) -> [ControlButton; 11] {
    let keys = profile.key_layout();
    let (left, right) = match profile {
        DeviceProfile::Siemens => ("A", "B"),
        DeviceProfile::Motorola => ("L", "R"),
        _ => ("L", "R"),
    };
    [
        ControlButton {
            x: 8,
            y: 6,
            width: 48,
            height: 22,
            key_code: keys.soft_left,
            label: left,
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 184,
            y: 6,
            width: 48,
            height: 22,
            key_code: keys.soft_right,
            label: right,
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 42,
            y: 28,
            width: 28,
            height: 28,
            key_code: keys.up,
            label: "",
            kind: ControlKind::Up,
        },
        ControlButton {
            x: 14,
            y: 56,
            width: 28,
            height: 28,
            key_code: keys.left,
            label: "",
            kind: ControlKind::Left,
        },
        ControlButton {
            x: 42,
            y: 56,
            width: 28,
            height: 28,
            key_code: keys.fire,
            label: "OK",
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 70,
            y: 56,
            width: 28,
            height: 28,
            key_code: keys.right,
            label: "",
            kind: ControlKind::Right,
        },
        ControlButton {
            x: 42,
            y: 84,
            width: 28,
            height: 28,
            key_code: keys.down,
            label: "",
            kind: ControlKind::Down,
        },
        ControlButton {
            x: 142,
            y: 38,
            width: 32,
            height: 28,
            key_code: b'1' as i32,
            label: "1",
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 198,
            y: 38,
            width: 32,
            height: 28,
            key_code: b'3' as i32,
            label: "3",
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 142,
            y: 76,
            width: 32,
            height: 28,
            key_code: b'7' as i32,
            label: "7",
            kind: ControlKind::Label,
        },
        ControlButton {
            x: 198,
            y: 76,
            width: 32,
            height: 28,
            key_code: b'9' as i32,
            label: "9",
            kind: ControlKind::Label,
        },
    ]
}

fn keypad_mappings() -> &'static [(KeyCode, i32)] {
    &[
        (KeyCode::Digit0, b'0' as i32),
        (KeyCode::Numpad0, b'0' as i32),
        (KeyCode::Digit1, b'1' as i32),
        (KeyCode::Numpad1, b'1' as i32),
        (KeyCode::Digit2, b'2' as i32),
        (KeyCode::Numpad2, b'2' as i32),
        (KeyCode::Digit3, b'3' as i32),
        (KeyCode::Numpad3, b'3' as i32),
        (KeyCode::Digit4, b'4' as i32),
        (KeyCode::Numpad4, b'4' as i32),
        (KeyCode::Digit5, b'5' as i32),
        (KeyCode::Numpad5, b'5' as i32),
        (KeyCode::Digit6, b'6' as i32),
        (KeyCode::Numpad6, b'6' as i32),
        (KeyCode::Digit7, b'7' as i32),
        (KeyCode::Numpad7, b'7' as i32),
        (KeyCode::Digit8, b'8' as i32),
        (KeyCode::Numpad8, b'8' as i32),
        (KeyCode::Digit9, b'9' as i32),
        (KeyCode::Numpad9, b'9' as i32),
        (KeyCode::NumpadMultiply, b'*' as i32),
        (KeyCode::NumpadDivide, b'#' as i32),
        (KeyCode::Minus, b'#' as i32),
    ]
}

pub(super) fn keyboard_mappings(profile: DeviceProfile) -> Vec<(KeyCode, i32)> {
    let keys = profile.key_layout();
    let mut mappings = vec![
        (KeyCode::ArrowUp, keys.up),
        (KeyCode::KeyW, keys.up),
        (KeyCode::ArrowDown, keys.down),
        (KeyCode::KeyS, keys.down),
        (KeyCode::ArrowLeft, keys.left),
        (KeyCode::KeyA, keys.left),
        (KeyCode::ArrowRight, keys.right),
        (KeyCode::KeyD, keys.right),
        (KeyCode::Enter, keys.fire),
        (KeyCode::Space, keys.fire),
        (KeyCode::NumpadEnter, keys.fire),
        (KeyCode::KeyC, keys.fire),
        (KeyCode::KeyQ, keys.soft_left),
        (KeyCode::F1, keys.soft_left),
        (KeyCode::KeyZ, keys.soft_left),
        (KeyCode::KeyJ, keys.soft_left),
        (KeyCode::KeyE, keys.soft_right),
        (KeyCode::F2, keys.soft_right),
        (KeyCode::KeyX, keys.soft_right),
        (KeyCode::KeyK, keys.soft_right),
    ];
    if let Some(clear) = keys.clear {
        mappings.push((KeyCode::Backspace, clear));
    }
    mappings.extend_from_slice(keypad_mappings());
    mappings
}

pub(super) fn draw_control_panel(buffer: &mut [u32], width: usize, top: usize, height: usize, pressed_key: Option<i32>, profile: DeviceProfile) {
    fill_buffer_rect(buffer, width, 0, top, width, height, 0x14181d);
    fill_buffer_rect(buffer, width, 0, top, width, 2, 0x2d3540);

    for button in control_buttons(profile) {
        let y = top + button.y;
        let pressed = pressed_key == Some(button.key_code);
        let fill = if pressed { 0x8fc7ff } else { 0xeef3f7 };
        let border = if pressed { 0x2e79bf } else { 0x6f7a86 };
        fill_buffer_rect(buffer, width, button.x, y, button.width, button.height, border);
        fill_buffer_rect(
            buffer,
            width,
            button.x + 2,
            y + 2,
            button.width.saturating_sub(4),
            button.height.saturating_sub(4),
            fill,
        );

        match button.kind {
            ControlKind::Label => draw_centered_text(buffer, width, button.x, y, button.width, button.height, button.label, 0x111820),
            ControlKind::Up => draw_triangle_up(buffer, width, button.x, y, button.width, button.height, 0x111820),
            ControlKind::Down => draw_triangle_down(buffer, width, button.x, y, button.width, button.height, 0x111820),
            ControlKind::Left => draw_triangle_left(buffer, width, button.x, y, button.width, button.height, 0x111820),
            ControlKind::Right => draw_triangle_right(buffer, width, button.x, y, button.width, button.height, 0x111820),
        }
    }
}

pub(super) fn normalize_mouse_pos(x: f32, y: f32, width: usize, height: usize) -> (usize, usize) {
    let mut x = x.max(0.0) as usize;
    let mut y = y.max(0.0) as usize;
    if x >= width && x < width * 2 {
        x /= 2;
    }
    if y >= height && y < height * 2 {
        y /= 2;
    }

    (x.min(width.saturating_sub(1)), y.min(height.saturating_sub(1)))
}

pub(super) fn control_at((x, y): (usize, usize), game_height: usize, profile: DeviceProfile) -> Option<i32> {
    if y < game_height {
        return None;
    }

    let local_y = y - game_height;
    control_buttons(profile).iter().find_map(|button| {
        let hit_x = x >= button.x && x < button.x + button.width;
        let hit_y = local_y >= button.y && local_y < button.y + button.height;
        if hit_x && hit_y { Some(button.key_code) } else { None }
    })
}

fn fill_buffer_rect(buffer: &mut [u32], width: usize, x: usize, y: usize, rect_width: usize, rect_height: usize, color: u32) {
    if width == 0 {
        return;
    }

    let height = buffer.len() / width;
    let max_x = (x + rect_width).min(width);
    let max_y = (y + rect_height).min(height);
    for py in y.min(height)..max_y {
        let row = py * width;
        for px in x.min(width)..max_x {
            buffer[row + px] = color;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_centered_text(buffer: &mut [u32], width: usize, x: usize, y: usize, rect_width: usize, rect_height: usize, text: &str, color: u32) {
    let scale = 2usize;
    let glyph_width = 3usize * scale;
    let glyph_height = 5usize * scale;
    let spacing = scale;
    let text_width = text.len().saturating_mul(glyph_width + spacing).saturating_sub(spacing);
    let start_x = x + rect_width.saturating_sub(text_width) / 2;
    let start_y = y + rect_height.saturating_sub(glyph_height) / 2;

    for (index, ch) in text.chars().enumerate() {
        let glyph_x = start_x + index * (glyph_width + spacing);
        draw_glyph(buffer, width, glyph_x, start_y, ch, scale, color);
    }
}

fn draw_glyph(buffer: &mut [u32], width: usize, x: usize, y: usize, ch: char, scale: usize, color: u32) {
    let Some(rows) = glyph_rows(ch) else {
        return;
    };

    for (row, bits) in rows.iter().enumerate() {
        for col in 0..3 {
            if bits & (1 << (2 - col)) != 0 {
                fill_buffer_rect(buffer, width, x + col * scale, y + row * scale, scale, scale, color);
            }
        }
    }
}

fn glyph_rows(ch: char) -> Option<[u8; 5]> {
    Some(match ch {
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
        'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'O' => [0b111, 0b101, 0b101, 0b101, 0b111],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        _ => return None,
    })
}

fn draw_triangle_up(buffer: &mut [u32], width: usize, x: usize, y: usize, rect_width: usize, _rect_height: usize, color: u32) {
    let cx = x + rect_width / 2;
    let top = y + 8;
    for row in 0..9 {
        let half = row / 2;
        fill_buffer_rect(buffer, width, cx.saturating_sub(half), top + row, half * 2 + 1, 1, color);
    }
}

fn draw_triangle_down(buffer: &mut [u32], width: usize, x: usize, y: usize, rect_width: usize, rect_height: usize, color: u32) {
    let cx = x + rect_width / 2;
    let top = y + rect_height.saturating_sub(17);
    for row in 0..9 {
        let half = (8 - row) / 2;
        fill_buffer_rect(buffer, width, cx.saturating_sub(half), top + row, half * 2 + 1, 1, color);
    }
}

fn draw_triangle_left(buffer: &mut [u32], width: usize, x: usize, y: usize, _rect_width: usize, rect_height: usize, color: u32) {
    let left = x + 8;
    let cy = y + rect_height / 2;
    for col in 0..9 {
        let half = col / 2;
        fill_buffer_rect(buffer, width, left + col, cy.saturating_sub(half), 1, half * 2 + 1, color);
    }
}

fn draw_triangle_right(buffer: &mut [u32], width: usize, x: usize, y: usize, rect_width: usize, rect_height: usize, color: u32) {
    let left = x + rect_width.saturating_sub(17);
    let cy = y + rect_height / 2;
    for col in 0..9 {
        let half = (8 - col) / 2;
        fill_buffer_rect(buffer, width, left + col, cy.saturating_sub(half), 1, half * 2 + 1, color);
    }
}
