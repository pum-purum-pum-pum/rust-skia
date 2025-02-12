use self::config::InvertStyle;
use crate::{prelude::*, scalar, ColorFilter};
use sb::SkHighContrastConfig;
use skia_bindings as sb;

pub mod config {
    pub use skia_bindings::SkHighContrastConfig_InvertStyle as InvertStyle;
    #[test]
    fn invert_style_naming() {
        let _ = InvertStyle::InvertLightness;
    }
}

#[derive(Clone, PartialEq)]
pub struct HighContrastConfig {
    pub grayscale: bool,
    pub invert_style: InvertStyle,
    pub contrast: scalar,
}

impl NativeTransmutable<SkHighContrastConfig> for HighContrastConfig {}
#[test]
fn high_contrast_config_layout() {
    HighContrastConfig::test_layout();
}

impl Default for HighContrastConfig {
    fn default() -> Self {
        Self {
            grayscale: false,
            invert_style: InvertStyle::NoInvert,
            contrast: 0.0,
        }
    }
}

impl HighContrastConfig {
    pub fn new(grayscale: bool, invert_style: InvertStyle, contrast: scalar) -> Self {
        Self {
            grayscale,
            invert_style,
            contrast,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.contrast >= -1.0 && self.contrast <= 1.0
    }
}

impl ColorFilter {
    pub fn high_contrast(config: &HighContrastConfig) -> Option<Self> {
        new(config)
    }
}

pub fn new(config: &HighContrastConfig) -> Option<ColorFilter> {
    ColorFilter::from_ptr(unsafe { sb::C_SkHighContrastFilter_Make(config.native()) })
}
