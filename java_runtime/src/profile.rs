use alloc::string::String;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyLayout {
    pub up: i32,
    pub down: i32,
    pub left: i32,
    pub right: i32,
    pub fire: i32,
    pub soft_left: i32,
    pub soft_right: i32,
    pub clear: Option<i32>,
}

impl KeyLayout {
    pub const NOKIA: Self = Self {
        up: -1,
        down: -2,
        left: -3,
        right: -4,
        fire: -5,
        soft_left: -6,
        soft_right: -7,
        clear: Some(-8),
    };

    pub const SIEMENS: Self = Self {
        up: -59,
        down: -60,
        left: -61,
        right: -62,
        fire: -26,
        soft_left: -1,
        soft_right: -4,
        clear: None,
    };

    pub const MOTOROLA: Self = Self {
        up: -1,
        down: -6,
        left: -2,
        right: -5,
        fire: -20,
        soft_left: -21,
        soft_right: -22,
        clear: None,
    };

    pub fn game_action(self, key_code: i32) -> i32 {
        if key_code == self.up || key_code == b'2' as i32 {
            1
        } else if key_code == self.left || key_code == b'4' as i32 {
            2
        } else if key_code == self.right || key_code == b'6' as i32 {
            5
        } else if key_code == self.down || key_code == b'8' as i32 {
            6
        } else if key_code == self.fire || key_code == b'5' as i32 {
            8
        } else if key_code == self.soft_left || key_code == b'1' as i32 {
            9
        } else if key_code == self.soft_right || key_code == b'3' as i32 {
            10
        } else if key_code == b'7' as i32 {
            11
        } else if key_code == b'9' as i32 {
            12
        } else {
            0
        }
    }

    pub fn key_code_for_action(self, game_action: i32) -> i32 {
        match game_action {
            1 => self.up,
            2 => self.left,
            5 => self.right,
            6 => self.down,
            8 => self.fire,
            9 => self.soft_left,
            10 => self.soft_right,
            11 => b'7' as i32,
            12 => b'9' as i32,
            _ => 0,
        }
    }

    pub fn key_name(self, key_code: i32) -> &'static str {
        if key_code == self.up {
            "UP"
        } else if key_code == self.down {
            "DOWN"
        } else if key_code == self.left {
            "LEFT"
        } else if key_code == self.right {
            "RIGHT"
        } else if key_code == self.fire {
            "FIRE"
        } else if key_code == self.soft_left {
            "SOFT1"
        } else if key_code == self.soft_right {
            "SOFT2"
        } else if self.clear == Some(key_code) {
            "CLEAR"
        } else {
            match key_code {
                35 => "#",
                42 => "*",
                48 => "0",
                49 => "1",
                50 => "2",
                51 => "3",
                52 => "4",
                53 => "5",
                54 => "6",
                55 => "7",
                56 => "8",
                57 => "9",
                _ => "",
            }
        }
    }

    pub fn game_canvas_mask(self, key_code: i32) -> Option<i32> {
        if key_code == self.up {
            Some(0x0002)
        } else if key_code == self.left {
            Some(0x0004)
        } else if key_code == self.right {
            Some(0x0020)
        } else if key_code == self.down {
            Some(0x0040)
        } else if key_code == self.fire {
            Some(0x0100)
        } else if key_code == self.soft_left || key_code == b'1' as i32 {
            Some(0x0200)
        } else if key_code == self.soft_right || key_code == b'3' as i32 {
            Some(0x0400)
        } else if key_code == b'7' as i32 {
            Some(0x0800)
        } else if key_code == b'9' as i32 {
            Some(0x1000)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceProfile {
    Generic,
    Nokia,
    SonyEricsson,
    Siemens,
    Samsung,
    Motorola,
    Lg,
    Vodafone,
    Sprint,
}

impl DeviceProfile {
    pub fn from_env_and_text(env: Option<&str>, text: &str) -> Self {
        if let Some(env) = env {
            let trimmed = env.trim();
            if !trimmed.is_empty() {
                return Self::parse(trimmed);
            }
        }
        Self::parse(text)
    }

    pub fn parse(text: &str) -> Self {
        let lower = text.to_ascii_lowercase();
        if lower.contains("siemens") {
            return Self::Siemens;
        }
        if lower.contains("samsung") {
            return Self::Samsung;
        }
        if lower.contains("motorola") || lower.contains("motomagx") {
            return Self::Motorola;
        }
        if lower.contains("vodafone") {
            return Self::Vodafone;
        }
        if lower.contains("sprint") {
            return Self::Sprint;
        }
        if lower.contains("/lg") || lower.contains("lge") || lower.contains("_lg_") {
            return Self::Lg;
        }
        let se = lower.contains("sonyericsson")
            || lower.contains("sony_ericsson")
            || lower.contains("-se")
            || lower.contains("_se")
            || lower.contains("k800")
            || lower.contains("mascot")
            || lower.contains("jp-8");
        let nokia = lower.contains("nokia") || lower.contains("s60") || lower.contains("n73") || lower.contains("n95") || lower.contains("s40");
        if se && !nokia {
            return Self::SonyEricsson;
        }
        if nokia && !se {
            return Self::Nokia;
        }
        if se && nokia {
            if lower.contains("k800") || lower.contains("sony") {
                return Self::SonyEricsson;
            }
            return Self::Nokia;
        }
        Self::Generic
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "Generic",
            Self::Nokia => "Nokia",
            Self::SonyEricsson => "SonyEricsson",
            Self::Siemens => "Siemens",
            Self::Samsung => "Samsung",
            Self::Motorola => "Motorola",
            Self::Lg => "Lg",
            Self::Vodafone => "Vodafone",
            Self::Sprint => "Sprint",
        }
    }

    pub fn platform_name(self) -> &'static str {
        match self {
            Self::Generic => "RustJava",
            Self::Nokia => "NokiaN73",
            Self::SonyEricsson => "SonyEricssonK800i",
            Self::Siemens => "SIE-C75",
            Self::Samsung => "Samsung-SGH-D900",
            Self::Motorola => "Motorola-E1000",
            Self::Lg => "LG-KU990",
            Self::Vodafone => "Vodafone",
            Self::Sprint => "Sprint",
        }
    }

    pub fn key_layout(self) -> KeyLayout {
        match self {
            Self::Siemens => KeyLayout::SIEMENS,
            Self::Motorola => KeyLayout::MOTOROLA,
            _ => KeyLayout::NOKIA,
        }
    }

    pub fn keyboard_hint(self) -> &'static str {
        match self {
            Self::Siemens => "Arrows/WASD: move, Enter/Space: fire, Q: A, E: B (Siemens)",
            Self::Motorola => "Arrows/WASD: move, Enter/Space: fire, Q: left soft, E: right soft (Motorola)",
            _ => "Arrows/WASD: d-pad, Enter/Space: fire, Q: left soft, E: right soft, 0-9: keypad",
        }
    }

    pub fn detect_from_parts<'a>(text: &str, api_classes: impl IntoIterator<Item = &'a str>) -> Self {
        let mut scores = VendorScores::default();
        let hinted = Self::parse(text);
        if hinted != Self::Generic {
            scores.add(hinted, 8);
        }
        for class in api_classes {
            if let Some(vendor) = vendor_package(class) {
                scores.add(vendor.to_profile(), 3);
            }
        }
        scores.winner()
    }

    pub fn allows_class(self, name: &str) -> bool {
        let vendor = vendor_package(name);
        match vendor {
            None => true,
            Some(VendorPackage::Nokia) => matches!(self, Self::Generic | Self::Nokia | Self::SonyEricsson),
            Some(VendorPackage::Mascot | VendorPackage::SonyEricsson) => matches!(self, Self::Generic | Self::SonyEricsson | Self::Nokia),
            Some(VendorPackage::Siemens) => matches!(self, Self::Generic | Self::Siemens),
            Some(VendorPackage::Samsung) => matches!(self, Self::Generic | Self::Samsung),
            Some(VendorPackage::Motorola) => matches!(self, Self::Generic | Self::Motorola),
            Some(VendorPackage::Lg) => matches!(self, Self::Generic | Self::Lg),
            Some(VendorPackage::Vodafone) => matches!(self, Self::Generic | Self::Vodafone),
            Some(VendorPackage::Sprint) => matches!(self, Self::Generic | Self::Sprint),
        }
    }
}

impl core::fmt::Display for DeviceProfile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Default)]
struct VendorScores {
    nokia: u32,
    sony: u32,
    siemens: u32,
    samsung: u32,
    motorola: u32,
    lg: u32,
    vodafone: u32,
    sprint: u32,
}

impl VendorScores {
    fn add(&mut self, profile: DeviceProfile, points: u32) {
        match profile {
            DeviceProfile::Generic => {}
            DeviceProfile::Nokia => self.nokia += points,
            DeviceProfile::SonyEricsson => self.sony += points,
            DeviceProfile::Siemens => self.siemens += points,
            DeviceProfile::Samsung => self.samsung += points,
            DeviceProfile::Motorola => self.motorola += points,
            DeviceProfile::Lg => self.lg += points,
            DeviceProfile::Vodafone => self.vodafone += points,
            DeviceProfile::Sprint => self.sprint += points,
        }
    }

    fn winner(self) -> DeviceProfile {
        [
            (self.nokia, DeviceProfile::Nokia),
            (self.sony, DeviceProfile::SonyEricsson),
            (self.siemens, DeviceProfile::Siemens),
            (self.samsung, DeviceProfile::Samsung),
            (self.motorola, DeviceProfile::Motorola),
            (self.lg, DeviceProfile::Lg),
            (self.vodafone, DeviceProfile::Vodafone),
            (self.sprint, DeviceProfile::Sprint),
        ]
        .into_iter()
        .max_by_key(|(score, _)| *score)
        .filter(|(score, _)| *score > 0)
        .map(|(_, profile)| profile)
        .unwrap_or(DeviceProfile::Generic)
    }
}

#[derive(Clone, Copy)]
enum VendorPackage {
    Nokia,
    Mascot,
    SonyEricsson,
    Siemens,
    Samsung,
    Motorola,
    Lg,
    Vodafone,
    Sprint,
}

impl VendorPackage {
    fn to_profile(self) -> DeviceProfile {
        match self {
            Self::Nokia => DeviceProfile::Nokia,
            Self::Mascot | Self::SonyEricsson => DeviceProfile::SonyEricsson,
            Self::Siemens => DeviceProfile::Siemens,
            Self::Samsung => DeviceProfile::Samsung,
            Self::Motorola => DeviceProfile::Motorola,
            Self::Lg => DeviceProfile::Lg,
            Self::Vodafone => DeviceProfile::Vodafone,
            Self::Sprint => DeviceProfile::Sprint,
        }
    }
}

fn vendor_package(name: &str) -> Option<VendorPackage> {
    if name.starts_with("com/nokia/") {
        Some(VendorPackage::Nokia)
    } else if name.starts_with("com/mascotcapsule/") {
        Some(VendorPackage::Mascot)
    } else if name.starts_with("com/sonyericsson/") {
        Some(VendorPackage::SonyEricsson)
    } else if name.starts_with("com/siemens/") {
        Some(VendorPackage::Siemens)
    } else if name.starts_with("com/samsung/") {
        Some(VendorPackage::Samsung)
    } else if name.starts_with("com/motorola/") {
        Some(VendorPackage::Motorola)
    } else if name.starts_with("com/lg/") || name.starts_with("com/lge/") {
        Some(VendorPackage::Lg)
    } else if name.starts_with("com/vodafone/") {
        Some(VendorPackage::Vodafone)
    } else if name.starts_with("com/sprintpcs/") {
        Some(VendorPackage::Sprint)
    } else {
        None
    }
}

pub fn detect_device_profile(jar_name: &str, extra: Option<&str>) -> DeviceProfile {
    let mut combined = String::from(jar_name);
    if let Some(extra) = extra {
        combined.push(' ');
        combined.push_str(extra);
    }
    DeviceProfile::parse(&combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jar_classes_override_generic_filename() {
        let profile = DeviceProfile::detect_from_parts("3d_treasuretowers.jar", ["com/mascotcapsule/micro3d/v3/Graphics3D"]);
        assert_eq!(profile, DeviceProfile::SonyEricsson);
    }

    #[test]
    fn filename_nokia_wins_without_vendor_classes() {
        let profile = DeviceProfile::detect_from_parts("corpus_nokia_s40_game_01.jar", ["javax/microedition/lcdui/Canvas"]);
        assert_eq!(profile, DeviceProfile::Nokia);
    }

    #[test]
    fn siemens_layout_does_not_use_nokia_up_code_for_dpad() {
        let layout = DeviceProfile::Siemens.key_layout();
        assert_eq!(layout.up, -59);
        assert_eq!(layout.game_action(-59), 1);
        assert_eq!(layout.game_action(-1), 9);
        assert_eq!(layout.key_code_for_action(1), -59);
    }

    #[test]
    fn nokia_layout_keeps_midp_negative_codes() {
        let layout = DeviceProfile::Generic.key_layout();
        assert_eq!(layout.fire, -5);
        assert_eq!(layout.game_action(-5), 8);
        assert_eq!(layout.key_name(-5), "FIRE");
    }
}
