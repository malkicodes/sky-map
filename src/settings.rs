#[derive(Clone, Debug, Default)]
pub struct DisplaySettings {
    names: NameSetting,
    grid: GridSetting,
    constellations: ConstellationSetting,

    zen_mode: bool,
}

impl DisplaySettings {
    pub fn names(&self) -> NameSetting {
        self.names
    }

    pub fn cycle_names(&mut self) {
        self.names = match self.names {
            NameSetting::Proper => NameSetting::BayerFlamsteed,
            NameSetting::BayerFlamsteed => NameSetting::HD,
            NameSetting::HD => NameSetting::Hidden,
            NameSetting::Hidden => NameSetting::Proper,
        }
    }

    pub fn grid(&self) -> GridSetting {
        self.grid
    }

    pub fn cycle_grid(&mut self) {
        self.grid = match self.grid {
            GridSetting::MajorMinor => GridSetting::Major,
            GridSetting::Major => GridSetting::Horizon,
            GridSetting::Horizon => GridSetting::None,
            GridSetting::None => GridSetting::MajorMinor,
        }
    }

    pub fn constellations(&self) -> ConstellationSetting {
        self.constellations
    }

    pub fn cycle_constellations(&mut self) {
        self.constellations = match self.constellations {
            ConstellationSetting::All => ConstellationSetting::Hover,
            ConstellationSetting::Hover => ConstellationSetting::None,
            ConstellationSetting::None => ConstellationSetting::All,
        }
    }

    pub fn zen_mode(&self) -> bool {
        self.zen_mode
    }

    pub fn toggle_zen_mode(&mut self) {
        self.zen_mode = !self.zen_mode
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum NameSetting {
    #[default]
    Proper,
    BayerFlamsteed,
    HD,
    Hidden,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum GridSetting {
    None,
    Horizon,
    Major,
    #[default]
    MajorMinor,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ConstellationSetting {
    All,
    #[default]
    Hover,
    None,
}
