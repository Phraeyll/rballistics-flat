use crate::util::*;

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const MAX_ANGLE: Angle = Angle::Radians(FRAC_PI_4);

fn max_angle() -> Numeric {
    MAX_ANGLE.to_radians().to_num()
}

struct IterFindElevation<'s> {
    sim: &'s mut super::Simulation<'s>,
    pitch_adjustment: Angle,
    elevation: Length,
    zero_distance: Length,
    zero_offset: Length,
    count: u64,
}
impl IterFindElevation<'_> {
    fn pitch_adjustment(&self) -> Numeric {
        self.pitch_adjustment.to_radians().to_num()
    }
    fn elevation(&self) -> Numeric {
        self.elevation.to_meters().to_num()
    }
    fn zero_distance(&self) -> Numeric {
        self.zero_distance.to_meters().to_num()
    }
    fn zero_offset(&self) -> Numeric {
        self.zero_offset.to_meters().to_num()
    }
}

impl<'s> Iterator for IterFindElevation<'s> {
    type Item = (Angle, Length);
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // Keep previous value to check if pitch changes
        let &mut Self {
            sim: &mut super::Simulation { muzzle_pitch, .. },
            count,
            elevation,
            ..
        } = self;
        // Change direction if
        // above(positive drop) and going   up(positive angle) or
        // below(negative drop) and going down(negative angle)
        if self.pitch_adjustment().is_sign_positive() ^ (self.elevation() < self.zero_offset()) {
            self.pitch_adjustment = Angle::Radians(self.pitch_adjustment() * -1.0);
        }
        // Always reduce angle on next iteration, converging towards either max(45) or min(0) degrees
        self.pitch_adjustment = Angle::Radians(self.pitch_adjustment() / 2.0);

        // Increment/decrement pitch before iteration below
        self.sim.muzzle_pitch =
            Angle::Radians(self.sim.muzzle_pitch.to_radians().to_num() + self.pitch_adjustment());

        // Used for debug output only
        let deg = self.sim.muzzle_pitch.to_degrees().to_num();

        if self.sim.muzzle_pitch.to_radians().to_num() > max_angle() {
            // This only can happen on second iteration, starting at 45 degrees
            // If switched to 45/2 degrees, algorithm will converge to either 45 or 0
            // Switched back to starting at 45 degrees to allow quick break if possible
            dbg!((MAX_ANGLE.to_degrees(), count, deg));
            None
        } else if self.sim.muzzle_pitch.to_radians().to_num() == muzzle_pitch.to_radians().to_num()
        {
            // This only can happen on second iteration, starting at 45 degrees
            // If switched to 45/2 degrees, algorithm will converge to either 45 or 0
            // Switched back to starting at 45 degrees to allow quick break if possible
            dbg!((
                muzzle_pitch,
                self.pitch_adjustment,
                self.sim.muzzle_pitch,
                elevation,
                count,
                deg
            ));
            None
        } else if let Some(p) = self
            // Find height in meters relative to zero, given pitch
            .sim
            .iter()
            .find(|p| p.relative_position().x >= self.zero_distance())
        {
            self.elevation = Length::Meters(p.relative_position().y);
            Some((self.sim.muzzle_pitch, self.elevation))
        } else {
            // Terminal velocity reached
            dbg!(count);
            None
        }
    }
}

// I would expect lifetime elision to work here
// but it currently does not
//
// impl super::Simulation<'_>
//
// note: ...so that the expression is assignable:
//           expected model::point_mass::zero::IterFindElevation<'_>
//              found model::point_mass::zero::IterFindElevation<'_>
// note: ...so that the expression is assignable:
//           expected &mut model::point_mass::Simulation<'_>
//              found &mut model::point_mass::Simulation<'_>
//
impl<'s> super::Simulation<'s> {
    fn find_elevation(
        &'s mut self,
        zero_distance: Length,
        zero_offset: Length,
    ) -> IterFindElevation {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        // Start with maximum angle to allow for zeroing at longer distances
        IterFindElevation {
            sim: self,
            pitch_adjustment: Angle::Radians(FRAC_PI_2),
            elevation: Length::Meters(-1.0),
            zero_distance,
            zero_offset,
            count: 0u64,
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub fn zero(
        &'s mut self,
        zero_distance: Length,
        zero_offset: Length,
        zero_tolerance: Length,
    ) -> Result<Angle, &str> {
        self.find_elevation(zero_distance, zero_offset)
            .find(|&(_, elevation)| {
                let elevation = elevation.to_meters().to_num();
                let zero_offset = zero_offset.to_meters().to_num();
                let zero_tolerance = zero_tolerance.to_meters().to_num();

                elevation > (zero_offset - zero_tolerance)
                    && elevation < (zero_offset + zero_tolerance)
            })
            .map(|(pitch, _)| Ok(pitch))
            .expect("Cannot zero for this range")
    }
}
