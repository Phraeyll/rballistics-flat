use crate::of::OrderedFloat;

use crate::macros::FloatMap;
use crate::util::Numeric;

use std::collections::BTreeMap;

pub fn init() -> FloatMap<Numeric> {
    float_map!{
        0.00 => 0.2105,
        0.05 => 0.2105,
        0.10 => 0.2104,
        0.15 => 0.2104,
        0.20 => 0.2103,
        0.25 => 0.2103,
        0.30 => 0.2103,
        0.35 => 0.2103,
        0.40 => 0.2103,
        0.45 => 0.2102,
        0.50 => 0.2102,
        0.55 => 0.2102,
        0.60 => 0.2102,
        0.65 => 0.2102,
        0.70 => 0.2103,
        0.75 => 0.2103,
        0.80 => 0.2104,
        0.825 => 0.2104,
        0.85 => 0.2105,
        0.875 => 0.2106,
        0.90 => 0.2109,
        0.925 => 0.2183,
        0.95 => 0.2571,
        0.975 => 0.3358,
        1.0 => 0.4068,
        1.025 => 0.4378,
        1.05 => 0.4476,
        1.075 => 0.4493,
        1.10 => 0.4477,
        1.125 => 0.4450,
        1.15 => 0.4419,
        1.20 => 0.4353,
        1.25 => 0.4283,
        1.30 => 0.4208,
        1.35 => 0.4133,
        1.40 => 0.4059,
        1.45 => 0.3986,
        1.50 => 0.3915,
        1.55 => 0.3845,
        1.60 => 0.3777,
        1.65 => 0.3710,
        1.70 => 0.3645,
        1.75 => 0.3581,
        1.80 => 0.3519,
        1.85 => 0.3458,
        1.90 => 0.3400,
        1.95 => 0.3343,
        2.00 => 0.3288,
        2.05 => 0.3234,
        2.10 => 0.3182,
        2.15 => 0.3131,
        2.20 => 0.3081,
        2.25 => 0.3032,
        2.30 => 0.2983,
        2.35 => 0.2937,
        2.40 => 0.2891,
        2.45 => 0.2845,
        2.50 => 0.2802,
        2.60 => 0.2720,
        2.70 => 0.2642,
        2.80 => 0.2569,
        2.90 => 0.2499,
        3.00 => 0.2432,
        3.10 => 0.2368,
        3.20 => 0.2308,
        3.30 => 0.2251,
        3.40 => 0.2197,
        3.50 => 0.2147,
        3.60 => 0.2101,
        3.70 => 0.2058,
        3.80 => 0.2019,
        3.90 => 0.1983,
        4.00 => 0.1950,
        4.20 => 0.1890,
        4.40 => 0.1837,
        4.60 => 0.1791,
        4.80 => 0.1750,
        5.00 => 0.1713,
    }
}
