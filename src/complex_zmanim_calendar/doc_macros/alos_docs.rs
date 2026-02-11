// Degrees
macro_rules! alos_degrees_basedon_doc {
    ($degrees:expr, $minutes:expr) => {
        concat!(
            "Returns *alos* (dawn) calculated when the sun is ",
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

macro_rules! alos_degrees_basedon_lechumra_doc {
    ($degrees:expr, $minutes:expr) => {
        concat!(
            "This method should only be used *lechumra* and returns *alos*
            (dawn) calculated when the sun is ",
            stringify!($degrees),
            "&deg; below the eastern geometric horizon before sunrise. This
            calculation is based on the position of the sun ",
            stringify!($minutes),
            " minutes before sunrise in Jerusalem around the equinox / equilux,
            which calculates to ",
            stringify!($degrees),
            "&deg; below geometric zenith. Since this time is extremely early,
            it should only be used *lechumra* only, such as not eating after
            this time on a fast day, and not as the start time for *mitzvos*
            that can only be performed during the day."
        )
    };
}

macro_rules! alos_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns *alos* (dawn) calculated when the sun is ",
            stringify!($degrees),
            "&deg; below the eastern geometric horizon before sunrise."
        )
    };
}

// Minutes
macro_rules! alos_minutes_basedon_doc {
    ($minutes:expr, $mil_len:expr) => {
        concat!(
            "Returns *alos* (dawn) calculated as ",
            stringify!($minutes),
            " minutes before sunrise. This time is based on the time to walk the
            distance of 4 *mil* at ",
            stringify!($mil_len),
            " minutes per *mil*. Time-based offset calculations for *alos* are
            based on the opinion of the *Rishonim* who stated that the time of
            the *Neshef* (twilight between dawn and sunrise) does not vary by
            the time of year or location but depends on the time it takes to
            walk the distance of 4 mil"
        )
    };
}

// Minutes zmaniyos
macro_rules! alos_minutes_zmanis_basedon_doc {
    ($minutes:expr, $mil_len:expr, $fraction:expr) => {
        concat!(
            "Returns *alos* (dawn) calculated as ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before sunrise. This time is based on the time to walk the
            distance of 4 *mil* at ",
            stringify!($mil_len),
            " minutes per *mil*."
        )
    };
}
