use serde::{Deserialize, Serialize};
use winit::{WindowAttributes, WindowBuilder};

use crate::monitor::{MonitorIdent, MonitorsAccess};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisplayConfig {
    /// Name of the application window.
    #[serde(default = "default_title")]
    pub title: String,
    /// Enables fullscreen mode on specific monitor when set.
    /// Defaults to `None`, which means fullscreen is off.
    #[serde(default)]
    pub fullscreen: Option<MonitorIdent>,
    /// Current window dimensions, measured in pixels (px).
    #[serde(default)]
    pub dimensions: Option<(u32, u32)>,
    /// Minimum window dimensions, measured in pixels (px).
    #[serde(default)]
    pub min_dimensions: Option<(u32, u32)>,
    /// Maximum window dimensions, measured in pixels (px).
    #[serde(default)]
    pub max_dimensions: Option<(u32, u32)>,
    #[serde(default = "default_visibility")]
    pub visibility: bool,
    /// Whether the window should always be on top of other windows.
    #[serde(default)]
    pub always_on_top: bool,
    /// Whether the window should have borders and bars.
    #[serde(default = "default_decorations")]
    pub decorations: bool,
    /// Whether the window should be maximized upon creation.
    #[serde(default)]
    pub maximized: bool,
    /// Enable multitouch on iOS.
    #[serde(default)]
    pub multitouch: bool,
    /// Whether the window is resizable or not.
    #[serde(default = "default_resizable")]
    pub resizable: bool,
    /// Whether the the window should be transparent. If this is true, writing
    /// colors with alpha values different than 1.0 will produce a transparent
    /// window.
    #[serde(default)]
    pub transparent: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            title: default_title(),
            fullscreen: None,
            dimensions: None,
            min_dimensions: None,
            max_dimensions: None,
            visibility: default_visibility(),
            always_on_top: false,
            decorations: default_decorations(),
            maximized: false,
            multitouch: false,
            resizable: default_resizable(),
            transparent: false
        }
    }
}

fn default_title() -> String {
    "Amethyst game".to_string()
}

fn default_decorations() -> bool {
    true
}

fn default_visibility() -> bool {
    true
}

fn default_resizable() -> bool {
    true
}

impl DisplayConfig {
    /// Creates a `winit::WindowBuilder` using the values set in the DisplayConfig
    ///
    /// The `MonitorsAccess` is needed to configure a fullscreen window.
    pub fn to_windowbuilder(self, monitors: &impl MonitorsAccess) -> WindowBuilder {
        let attrs = WindowAttributes {
            dimensions: self.dimensions.map(Into::into),
            max_dimensions: self.max_dimensions.map(Into::into),
            min_dimensions: self.min_dimensions.map(Into::into),
            title: self.title,
            maximized: self.maximized,
            visible: self.visibility,
            transparent: self.transparent,
            decorations: self.decorations,
            always_on_top: self.always_on_top,
            window_icon: None,
            fullscreen: self.fullscreen.map(|ident| ident.monitor_id(monitors)),
            resizable: self.resizable,
            multitouch: self.multitouch
        };

        let mut builder = WindowBuilder::new();
        builder.window = attrs;
        builder
    }
}
