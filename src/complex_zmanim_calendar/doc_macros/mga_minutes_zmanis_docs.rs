macro_rules! shaah_mga_minutes_zmanis_doc {
    ($minutes:expr, $fraction:expr) => {
        concat!(
            "Returns a *shaah zmanis* (temporal hour) according to the opinion
            of the *Magen Avraham* (MGA) based on *alos* and *tzais* being ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before and after *hanetz* and *shkia*, respectively.
            This calculation divides the day based on the opinion of the MGA
            that the day runs from *alos* to *tzais*. This day is split into 12
            equal parts with each part being a *shaah zmanis*."
        )
    };
}

macro_rules! szks_mga_minutes_zmanis_doc {
    ($minutes:expr, $fraction:expr) => {
        concat!(
            "Returns the latest *zman krias shema* (time to recite *Shema* in
            the morning) according to the opinion of the *Magen Avraham* (MGA)
            based on *alos* and *tzais* being ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before and after *hanetz* and *shkia*, respectively.
            This time is 3 *shaos zmaniyos* (temporal hours) after dawn based on
            the opinion of the MGA that the day is calculated from *alos* to
            *tzais*."
        )
    };
}

macro_rules! szt_mga_minutes_zmanis_doc {
    ($minutes:expr, $fraction:expr) => {
        concat!(
            "Returns the latest *zman tefila* (time to recite the morning
            prayers) according to the opinion of the *Magen Avraham* (MGA)
            based on *alos* and *tzais* being ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before and after *hanetz* and *shkia*, respectively.
            This time is 4 *shaos zmaniyos* (temporal hours) after dawn based on
            the opinion of the MGA that the day is calculated from *alos* to
            *tzais*."
        )
    };
}

macro_rules! sz_biur_chametz_mga_minutes_zmanis_doc {
    ($minutes:expr, $fraction:expr) => {
        concat!(
            "Returns the latest time for burning *chametz* on *Erev Pesach*
            according to the opinion of the *Magen Avraham* (MGA)
            based on *alos* and *tzais* being ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before and after *hanetz* and *shkia*, respectively.
            This time is 5 *shaos zmaniyos* (temporal hours) after dawn based on
            the opinion of the MGA that the day is calculated from *alos* to
            *tzais*. Since this library does not implement a calendar, this
            method will return the *zman* any day of the year."
        )
    };
}

macro_rules! plag_mga_minutes_zmanis_lechumra_doc {
    ($minutes:expr, $fraction:expr) => {
        concat!(
            "This method should only be used *lechumra* and returns the time of
            *plag hamincha* according to the opinion of the *Magen Avraham*
            (MGA) based on *alos* and *tzais* being ",
            stringify!($minutes),
            " minutes of [the GRA's *shaos
            zmaniyos*](crate::complex_zmanim_calendar::ComplexZmanimCalendar::shaah_zmanis_gra),
            or ",
            $fraction,
            " of the day before and after *hanetz* and *shkia*, respectively.
            This time is 10.75 *shaos zmaniyos* (temporal hours) after dawn
            based on the opinion of the MGA that the day is calculated from
            *alos* to *tzais*. Since *plag* by this calculation can occur after
            sunset, it should only be used *lechumra*. For more information on
            this see the documentation on [*plag
            hamincha*](zmanim_calculator::plag_hamincha)."
        )
    };
}
