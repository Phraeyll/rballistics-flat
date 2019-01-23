use crate::model::builder::{ScopeBuilder, SimulationBuilder};
use crate::model::core::Scope;
use crate::util::*;

impl Default for Scope {
    fn default() -> Self {
        Self {
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
            pitch: Angle::Radians(0.0),
            yaw: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
        }
    }
}

impl ScopeBuilder for SimulationBuilder {
    fn set_height(mut self, value: Numeric) -> Result<Self> {
        self.scope.height = Length::Inches(value);
        Ok(self)
    }
    fn set_offset(mut self, value: Numeric) -> Result<Self> {
        self.scope.offset = Length::Inches(value);
        Ok(self)
    }
    fn set_pitch(mut self, value: Numeric) -> Result<Self> {
        self.scope.pitch = Angle::Minutes(value);
        Ok(self)
    }
    fn set_yaw(mut self, value: Numeric) -> Result<Self> {
        self.scope.yaw = Angle::Minutes(value);
        Ok(self)
    }
    fn set_roll(mut self, value: Numeric) -> Result<Self> {
        self.scope.roll = Angle::Degrees(value);
        Ok(self)
    }
}
