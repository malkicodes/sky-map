#[derive(Clone, Debug, Default)]
pub struct DisplaySettings {
    pub(crate) names: NameSetting,
    pub(crate) grid: GridSetting,
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
