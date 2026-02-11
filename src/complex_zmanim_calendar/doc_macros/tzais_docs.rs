// Degrees
macro_rules! tzais_degrees_basedon_doc {
    ($degrees:expr, $minutes:expr) => {
        concat!(
            "Returns *tzais* (dusk) calculated when the sun is ",
            stringify!($degrees),
            "&deg; below the eastern geometric horizon before sunrise. This
            calculation is based on the position of the sun ",
            stringify!($minutes),
            " minutes before sunrise in Jerusalem around the equinox / equilux,
            which calculates to ",
            stringify!($degrees),
            "&deg; below geometric zenith."
        )
    };
}

macro_rules! tzais_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns *tzais* (dusk) calculated when the sun is ",
            stringify!($degrees),
            "&deg; below the eastern geometric horizon before sunrise."
        )
    };
}

// Minutes
macro_rules! tzais_minutes_basedon_doc {
    ($minutes:expr, $mil_len:expr) => {
        concat!(
            "Returns *tzais* (dusk) calculated as ",
            stringify!($minutes),
            " after sunset. This time is based on the time to walk the
            distance of 4 *mil* at ",
            stringify!($mil_len),
            " minutes per *mil*.",
        )
    };
}

// Minutes Zmanis
macro_rules! tzais_minutes_zmanis_basedon_doc {
    ($minutes:expr, $mil_len:expr, $fraction:expr) => {
        concat!(
            "Returns *tzais hakochavim* (dusk) calculated as ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day after sunset. This time is based on the time to walk the
            distance of 4 *mil* at ",
            stringify!($mil_len),
            " minutes per *mil*."
        )
    };
}
