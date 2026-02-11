macro_rules! shaah_mga_degrees_basedon_doc {
    ($degrees:expr, $minutes:expr) => {
        concat!(
            "Returns a *shaah zmanis* (solar hour) according to the opinion of
            the *Magen Avraham* (MGA) based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This calculation divides the day based on
            the opinion of the *Magen Avraham* (MGA) that the day runs from *alos*
            to *tzais*. This day is split into 12 equal parts with each part being
            a *shaah zmanis*. This calculation is based on the position of the
            sun ",
            stringify!($minutes),
            " minutes before sunrise / after sunset in Jerusalem around the
            equinox / equilux, which calculates to ",
            stringify!($degrees),
            "&deg; below geometric zenith."
        )
    };
}

macro_rules! shaah_mga_degrees_lechumra_basedon_doc {
    ($degrees:expr, $minutes:expr) => {
        concat!(
            "Returns a *shaah zmanis* (solar hour) according to the opinion of
            the *Magen Avraham* (MGA) based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This calculation divides the day based on
            the opinion of the *Magen Avraham* (MGA) that the day runs from *alos*
            to *tzais*. This day is split into 12 equal parts with each part being
            a *shaah zmanis*. This calculation is based on the position of the
            sun ",
            stringify!($minutes),
            " minutes before sunrise / after sunset in Jerusalem around the
            equinox / equilux, which calculates to ",
            stringify!($degrees),
            "&deg; below geometric zenith. Since *zmanim* that use this method
            are extremely late or early and at a point when the sky is a long
            time past the 18&deg; point where the darkest point is reached,
            *zmanim* that use this should only be used *lechumra*, such as
            delaying the start of nighttime *mitzvos*."
        )
    };
}

macro_rules! shaah_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns a *shaah zmanis* (solar hour) according to the opinion of
            the *Magen Avraham* (MGA) based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This calculation divides the day based on
            the opinion of the *Magen Avraham* (MGA) that the day runs from *alos*
            to *tzais*. This day is split into 12 equal parts with each part being
            a *shaah zmanis*."
        )
    };
}

macro_rules! szks_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the latest *zman krias shema* (time to recite *Shema* in
            the morning) according to the opinion of the *Magen Avraham* (MGA)
            based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 3 *shaos zmaniyos* (temporal
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*."
        )
    };
}

macro_rules! szt_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the latest *zman tefila* (time to recite the morning
            prayers) according to the opinion of the *Magen Avraham* (MGA) based
            on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 4 *shaos zmaniyos* (temporal
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*."
        )
    };
}
macro_rules! sz_biur_chametz_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the latest time for burning *chametz* on *Erev Pesach*
            according to the opinion of the *Magen Avraham* (MGA) based on
            *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 5 *shaos zmaniyos* (temporal
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*. Since this library does not
            implement a calendar, this method will return the *zman* any day of
            the year."
        )
    };
}

macro_rules! mg_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the time of *mincha gedola* (the earliest time to pray
            *mincha* in the afternoon) according to the *Magen Avraham* based on
            *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 6.5 *shaos zmaniyos* (solar
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*. For more information on this see
            the documentation on [*mincha
            gedola*](crate::zmanim_calculator::mincha_gedola)."
        )
    };
}

macro_rules! slmk_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the time of *samuch lemincha ketana* (the time that eating
            or other activity can't begin prior to praying *mincha*) according
            to the *Magen Avraham* based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is half a *shaah zmanis* before
            *mincha ketana*, or 9 *shaos zmaniyos* (temporal hours) after *alos*
            based on the opinion of the MGA that the day is calculated from
            *alos* to *tzais*. For more information on this see the
            documentation on [*samuch lemincha
            ketana*](crate::zmanim_calculator::samuch_lemincha_ketana)."
        )
    };
}

macro_rules! mk_mga_degrees_doc {
    ($degrees:expr) => {
        concat!(
            "Returns the time of *mincha ketana* according to the *Magen
            Avraham* based on *alos* and *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 9.5 *shaos zmaniyos* (solar
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*. For more information on this see
            the documentation on [*mincha
            ketana*](zmanim_calculator::mincha_ketana)."
        )
    };
}

macro_rules! plag_mga_degrees_lechumra_doc {
    ($degrees:expr) => {
        concat!(
            "This method should only be used *lechumra* and returns the time of
            *plag hamincha* according to the *Magen Avraham* based on *alos* and
            *tzais* being ",
            stringify!($degrees),
            "&deg; below the horizon. This time is 10.75 *shaos zmaniyos* (solar
            hours) after *alos* based on the opinion of the MGA that the day is
            calculated from *alos* to *tzais*. Since *plag* by this calculation
            can occur after sunset, it should only be used *lechumra*. For more
            information on this see the documentation on [*plag
            hamincha*](zmanim_calculator::plag_hamincha)."
        )
    };
}
