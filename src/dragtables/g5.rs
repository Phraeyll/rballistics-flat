use of::OrderedFloat;

use macros::FloatMap;

use std::collections::BTreeMap;

pub fn init() -> FloatMap<f64> {
    float_map!{
        0.00 => 0.1710,
        0.05 => 0.1719,
        0.10 => 0.1727,
        0.15 => 0.1732,
        0.20 => 0.1734,
        0.25 => 0.1730,
        0.30 => 0.1718,
        0.35 => 0.1696,
        0.40 => 0.1668,
        0.45 => 0.1637,
        0.50 => 0.1603,
        0.55 => 0.1566,
        0.60 => 0.1529,
        0.65 => 0.1497,
        0.70 => 0.1473,
        0.75 => 0.1463,
        0.80 => 0.1489,
        0.85 => 0.1583,
        0.875 => 0.1672,
        0.90 => 0.1815,
        0.925 => 0.2051,
        0.95 => 0.2413,
        0.975 => 0.2884,
        1.0 => 0.3379,
        1.025 => 0.3785,
        1.05 => 0.4032,
        1.075 => 0.4147,
        1.10 => 0.4201,
        1.15 => 0.4278,
        1.20 => 0.4338,
        1.25 => 0.4373,
        1.30 => 0.4392,
        1.35 => 0.4403,
        1.40 => 0.4406,
        1.45 => 0.4401,
        1.50 => 0.4386,
        1.55 => 0.4362,
        1.60 => 0.4328,
        1.65 => 0.4286,
        1.70 => 0.4237,
        1.75 => 0.4182,
        1.80 => 0.4121,
        1.85 => 0.4057,
        1.90 => 0.3991,
        1.95 => 0.3925 + 0.0001, // Clippy is dumb
        2.00 => 0.3861,
        2.05 => 0.3800,
        2.10 => 0.3741,
        2.15 => 0.3684,
        2.20 => 0.3630,
        2.25 => 0.3578,
        2.30 => 0.3529,
        2.35 => 0.3481,
        2.40 => 0.3435,
        2.45 => 0.3391,
        2.50 => 0.3349,
        2.60 => 0.3269,
        2.70 => 0.3194,
        2.80 => 0.3125,
        2.90 => 0.3060,
        3.00 => 0.2999,
        3.10 => 0.2942,
        3.20 => 0.2889,
        3.30 => 0.2838,
        3.40 => 0.2790,
        3.50 => 0.2745,
        3.60 => 0.2703,
        3.70 => 0.2662,
        3.80 => 0.2624,
        3.90 => 0.2588,
        4.00 => 0.2553,
        4.20 => 0.2488,
        4.40 => 0.2429,
        4.60 => 0.2376,
        4.80 => 0.2326,
        5.00 => 0.2280,
    }
}
