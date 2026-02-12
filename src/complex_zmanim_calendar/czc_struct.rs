use crate::{
    util::geolocation::GeoLocation,
    zmanim_calculator::{
        self,
        ZmanOffset::{self, Degrees, Minutes, MinutesZmaniyos},
    },
};

use chrono::{DateTime, TimeDelta};
use chrono_tz::Tz;
use pastey::paste;

/// Struct to store a 4-dimensional location and settings, to simplify getting
/// many *zmanim* for the same location. Has premade methods for many common
/// (and uncommon) *zmanim*. (see `impl` block)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexZmanimCalendar {
    /// Location at which to calculate *zmanim*
    pub geo_location: GeoLocation,

    /// Day for which to calculate *zmanim*. Time does not (should not) affect
    /// the calculations
    pub date: DateTime<Tz>,

    /// When to account for elevation. See [`UseElevation`]
    pub use_elevation: UseElevation,
}

/// Where unspecified, when sunrise and sunset are mentioned in the calculation
/// of other *zmanim*, they will be sea-level or elevation-based depending on
/// the value of `use_elevation`
///
/// **Elevation-based *zmanim* (even sunrise and sunset) should not be used
/// *lekula* without the guidance of a *posek***. See the documentation of
/// [`zmanim_calculator`] for more details.
///
/// Many of the methods return `None` if the sun does not reach the specified
/// position below the horizon at this location and date (common in polar
/// regions).
impl ComplexZmanimCalendar {
    // Basics
    /// Returns *alos hashachar* (dawn) based on either declination of the sun
    /// below the horizon, a fixed time offset, or a minutes *zmaniyos*
    /// (temporal minutes) offset before sunrise
    #[must_use]
    pub fn alos(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        let use_elevation = self.use_elevation.to_bool(false);
        zmanim_calculator::alos(&self.date, &self.geo_location, use_elevation, offset)
    }

    /// Returns sea level sunrise
    #[must_use]
    pub fn sea_level_sunrise(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::hanetz(&self.date, &self.geo_location, false)
    }

    /// Returns sea level sunset
    #[must_use]
    pub fn sea_level_sunset(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::shkia(&self.date, &self.geo_location, false)
    }

    /// Returns elevation-adjusted sunrise
    #[must_use]
    pub fn elevation_sunrise(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::hanetz(&self.date, &self.geo_location, true)
    }

    /// Returns elevation-adjusted sunset
    #[must_use]
    pub fn elevation_sunset(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::shkia(&self.date, &self.geo_location, true)
    }

    /// Returns *hanetz*, or sunrise. Will be elevation-adjusted or not
    /// depending on `use_elevation`
    #[must_use]
    pub fn hanetz(&self) -> Option<DateTime<Tz>> {
        let use_elevation = self.use_elevation.to_bool(true);
        zmanim_calculator::hanetz(&self.date, &self.geo_location, use_elevation)
    }

    /// Returns *shkia*, or sunset. Will be elevation-adjusted or not depending
    /// on `use_elevation`
    #[must_use]
    pub fn shkia(&self) -> Option<DateTime<Tz>> {
        let use_elevation = self.use_elevation.to_bool(true);
        zmanim_calculator::shkia(&self.date, &self.geo_location, use_elevation)
    }

    /// Returns the latest *zman krias shema* (time to recite *Shema* in the
    /// morning) according to the opinion of the *Magen Avraham* (MGA) based on
    /// *alos* and *tzais* being given offset from sunrise and sunset,
    /// respectively.
    #[must_use]
    pub fn sof_zman_shema_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_shema(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns the latest *zman tefila* (time to recite *shacharis* in the
    /// morning) according to the opinion of the *Magen Avraham* (MGA) based on
    /// *alos* and *tzais* being the given offset from sunrise and sunset,
    /// respectively.
    #[must_use]
    pub fn sof_zman_tefila_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_tefila(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns the latest *zman biur chametz* (the latest time for burning
    /// *chametz* on *Erev Pesach*) according to the opinion of the *Magen
    /// Avraham* (MGA) based on *alos* and *tzais* being the given offset
    /// from sunrise and sunset, respectively.
    #[must_use]
    pub fn sof_zman_biur_chametz_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_biur_chametz(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns Astronomical *chatzos* (noon)
    #[must_use]
    pub fn chatzos(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::chatzos(&self.date, &self.geo_location)
    }

    /// Returns Astronomical *chatzos halayla* (midnight)
    #[must_use]
    pub fn chatzos_halayla(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::chatzos_halayla(&self.date, &self.geo_location)
    }

    /// Returns *mincha gedola* according to the opinion of the *Magen Avraham*
    /// (MGA) based on *alos* and *tzais* being the given offset from sunrise
    /// and sunset, respectively.
    #[must_use]
    pub fn mincha_gedola_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_gedola(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns *samuch lemincha ketana* according to the opinion of the *Magen
    /// Avraham* (MGA) based on *alos* and *tzais* being the given offset
    /// from sunrise and sunset, respectively.
    #[must_use]
    pub fn samuch_lemincha_ketana_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::samuch_lemincha_ketana(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns *mincha ketana* according to the opinion of the *Magen Avraham*
    /// (MGA) based on *alos* and *tzais* being the given offset from sunrise
    /// and sunset, respectively.
    #[must_use]
    pub fn mincha_ketana_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_ketana(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns *plag hamincha* according to the opinion of the *Magen Avraham*
    /// (MGA) based on *alos* and *tzais* being the given offset from sunrise
    /// and sunset, respectively.
    #[must_use]
    pub fn plag_mga(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    /// Returns *mincha gedola* calculated as 30 minutes after *chatzos* and not
    /// 1/2 of a *shaah zmanis* after *chatzos* as calculated by
    /// [`zmanim_calculator::mincha_gedola`]. See
    /// [`zmanim_calculator::mincha_gedola_30_minutes`] for more details
    #[must_use]
    pub fn mincha_gedola_30_minutes(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::mincha_gedola_30_minutes(&self.date, &self.geo_location)
    }

    /// Returns *tzais* (nightfall) based on either declination of the sun below
    /// the horizon, a fixed time offset, or a minutes *zmaniyos* (temporal
    /// minutes) offset after sunset
    #[must_use]
    pub fn tzais(&self, offset: &ZmanOffset) -> Option<DateTime<Tz>> {
        let use_elevation = self.use_elevation.to_bool(false);
        zmanim_calculator::tzais(&self.date, &self.geo_location, use_elevation, offset)
    }

    /// Returns *shaah zmanis* (temporal hour) according to the opinion of the
    /// *Magen Avraham* (MGA) based on *alos* and *tzais* being the given
    /// offset from sunrise and sunset, respectively.
    #[must_use]
    pub fn shaah_zmanis_mga(&self, offset: &ZmanOffset) -> Option<TimeDelta> {
        Some(zmanim_calculator::shaah_zmanis(
            &self.alos(offset)?,
            &self.tzais(offset)?,
        ))
    }

    // GRA
    /// Returns the latest *zman shema* (time to recite *Shema* in the morning)
    /// that is 3 *shaos zmaniyos* (solar hours) after
    /// sunrise according the GRA. The day is
    /// calculated from sunrise to sunset.
    #[must_use]
    pub fn sof_zman_shema_gra(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_shema(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    /// Returns the latest *zman tefila* (time to recite *shacharis* in the
    /// morning) that is 4 *shaos zmaniyos* (solar hours) after
    /// sunrise according GRA.
    /// The day is calculated from sunrise to sunset
    #[must_use]
    pub fn sof_zman_tefila_gra(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_tefila(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    /// Returns the latest time for burning *chametz* on *Erev
    /// Pesach* according to the opinion of the GRA. This time is 5 hours into
    /// the day based on the opinion of the GRA that the day is calculated from
    /// sunrise to sunset. Since this library does not implement a calendar,
    /// this method will return the *zman* any day of the year.
    #[must_use]
    pub fn sof_zman_biur_chametz_gra(&self) -> Option<DateTime<Tz>> {
        Some(self.hanetz()? + (self.shaah_zmanis_gra()? * 5))
    }

    /// Returns *mincha gedola* calculated as 6.5 * *shaos zmaniyos*
    /// (solar hours) after sunrise, according to the GRA.
    #[must_use]
    pub fn mincha_gedola_gra(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_gedola(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    /// Returns the later of
    /// [`mincha_gedola_gra`](ComplexZmanimCalendar::mincha_gedola_gra)
    /// and
    /// [`mincha_gedola_30_minutes`](ComplexZmanimCalendar::mincha_gedola_30_minutes)
    /// . In the winter when 1/2 of a
    /// [GRA *shaah zmanis*](ComplexZmanimCalendar::shaah_zmanis_gra)
    /// is less than 30 minutes
    /// [`mincha_gedola_30_minutes`](ComplexZmanimCalendar::mincha_gedola_30_minutes)
    /// will be returned, otherwise
    /// [`mincha_gedola_gra`](ComplexZmanimCalendar::mincha_gedola_gra)
    ///  will be returned
    #[must_use]
    pub fn mincha_gedola_gra_greater_than_30_minutes(&self) -> Option<DateTime<Tz>> {
        let mg_30 = self.mincha_gedola_30_minutes()?;
        let mg_gra = self.mincha_gedola_gra()?;
        Some(if mg_30 > mg_gra { mg_30 } else { mg_gra })
    }

    /// A method for calculating *samuch lemincha ketana*, / near *mincha
    /// ketana* time that is half an hour before [*mincha
    /// ketana*](ComplexZmanimCalendar::mincha_ketana_gra) or is 9 *shaos
    /// zmaniyos* (solar hours) after sunrise, calculated according to the GRA
    /// using a day starting at sunrise and ending at sunset. This is the
    /// time that eating or other activity can't begin prior to praying
    /// *mincha*. See the *Mechaber* and *Mishna Berurah* 232 and 249:2.
    #[must_use]
    pub fn samuch_lemincha_ketana_gra(&self) -> Option<DateTime<Tz>> {
        Some(self.hanetz()? + (self.shaah_zmanis_gra()? * 9))
    }

    /// Returns *mincha ketana* calculated as 9.5 * *shaos zmaniyos*
    /// (solar hours) after sunrise, according to the GRA.
    #[must_use]
    pub fn mincha_ketana_gra(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_ketana(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    /// Returns *plag hamincha* calculated as 10.75 * *shaos zmaniyos*
    /// (solar hours) after sunrise, according to the GRA.
    #[must_use]
    pub fn plag_gra(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    /// Returns a *shaah zmanis* according to the opinion of the GRA.
    /// This calculation divides the day based on the opinion of the *GRA* that
    /// the day runs from from sunrise to sunset. The day is split into 12 equal
    /// parts with each one being a *shaah zmanis*
    #[must_use]
    pub fn shaah_zmanis_gra(&self) -> Option<TimeDelta> {
        Some(zmanim_calculator::shaah_zmanis(
            &self.hanetz()?,
            &self.shkia()?,
        ))
    }

    // Baal HaTanya
    /// Returns the *Baal Hatanya*'s *alos* (dawn) calculated as the time when
    /// the sun is 16.9&deg; below the eastern geometric horizon before sunrise.
    /// It is based on the calculation that the time between dawn and
    /// [*hanetz amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya)
    /// (sunrise) is 72 minutes, the time that is takes to walk 4 *mil* at
    /// 18 minutes a *mil* (*Rambam* and others). The sun's position at 72
    /// minutes before *hanetz amiti* (sunrise) in Jerusalem around the
    /// equinox / equilux is 16.9&deg; below geometric zenith.
    #[must_use]
    pub fn alos_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(16.9))
    }

    /// **Note: *hanetz amiti* is used only for
    /// calculating certain other *zmanim*. For practical purposes, daytime
    /// *mitzvos* like *shofar* and *lulav* should not be done until after the
    /// normal time for [*hanetz*](ComplexZmanimCalendar::hanetz).**
    ///
    /// Returns the *Baal Hatanya*'s *hanetz amiti* (sunrise) without elevation
    /// adjustment. This forms the base for the *Baal Hatanya*'s dawn-based
    /// calculations that are calculated as a dip below the horizon before
    /// sunrise. According to the *Baal Hatanya*, *hanetz amiti*, or true
    /// (halachic) sunrise, is when the top of the sun's disk is visible at an
    /// elevation similar to the mountains of *Eretz Yisrael*. The time is
    /// calculated as the point at which the center of the sun's disk is
    /// 1.583&deg; below the horizon. This degree-based calculation can be found
    /// in Rabbi Shalom DovBer Levine's commentary on The *Baal Hatanya*'s
    /// *Seder Hachnasas Shabbos*. From an elevation of 546 meters, the top of
    /// *Har Hacarmel*, the sun disappears when it is 1&deg; 35' or 1.583&deg;
    /// below the sea level horizon. This in turn is based on the *Gemara
    /// Shabbos* 35a. There are other opinions brought down by Rabbi Levine,
    /// including Rabbi Yosef Yitzchok Feigelstock who calculates it as the
    /// degrees below the horizon 4 minutes after sunset in *Yerushalayim* (on
    /// the equinox). That is brought down as 1.583&deg;. This is identical to
    /// the 1&deg; 35' *zman* and is probably a typo and should be 1.683&deg;.
    /// These calculations are used by most *Chabad* calendars that use the
    /// *Baal Hatanya*'s *zmanim*.
    #[must_use]
    pub fn hanetz_amiti_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(1.583))
    }

    /// **Note: *shkiah amiti* is used only for calculating certain other
    /// *zmanim*. For practical purposes, all daytime *mitzvos* should be
    /// completed before the normal time for
    /// [*shkiah*](ComplexZmanimCalendar::shkia).**
    ///
    /// Returns the *Baal Hatanya*'s *shkiah amiti* (sunset) without elevation
    /// adjustment. This forms the base for the *Baal Hatanya*'s dusk-based
    /// calculations that are calculated as a dip below the horizon after
    /// sunset. According to the *Baal Hatanya*, *shkiah amiti*, true (halachic)
    /// sunset, is when the top of the sun's disk disappears from view at an
    /// elevation similar to the mountains of *Eretz Yisrael*. This time is
    /// calculated as the point at which the center of the sun's disk is 1.583
    /// degrees below the horizon.
    #[must_use]
    pub fn shkia_amiti_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(1.583))
    }

    /// Returns the *Baal Hatanya*'s a *shaah zmanis* (temporal hour). This
    /// forms the base for the *Baal Hatanya*'s day based calculations that are
    /// calculated as a 1.583&deg; dip below the horizon after sunset. According
    /// to the *Baal Hatanya*, *shkiah amiti*, true (halachic) sunset, is when
    /// the top of the sun's disk disappears from view at an elevation similar
    /// to the mountains of *Eretz Yisrael*. This time is calculated as the
    /// point at which the center of the sun's disk is 1.583 degrees below the
    /// horizon. The calculations are based on a day from [*hanetz
    /// amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya) to
    /// [*shkiah amiti*](ComplexZmanimCalendar::shkia_amiti_baal_hatanya). The
    /// day is split into 12 equal parts with each one being a *shaah
    /// zmanis*.
    #[must_use]
    pub fn shaah_zmanis_baal_hatanya(&self) -> Option<TimeDelta> {
        self.shaah_zmanis_mga(&Degrees(1.583))
    }

    /// Returns the the *Baal Hatanya*'s *sof *zman* krias shema*
    /// (latest time to recite *Shema* in the morning). This time is 3
    /// of the [*Baal Hatanya*'s temporal
    /// hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after [*hanetz
    /// amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya) (sunrise)
    /// based on the opinion of the *Baal Hatanya* that the day is
    /// calculated from sunrise to sunset.
    #[must_use]
    pub fn sof_zman_shema_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.sof_zman_shema_mga(&Degrees(1.583))
    }

    /// Returns the the *Baal Hatanya*'s *sof *zman* tefila* (latest
    /// time to recite the morning prayers). This time is 4 of the [*Baal
    /// Hatanya*'s temporal
    /// hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after [*hanetz
    /// amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya) (sunrise)
    /// based on the opinion of the *Baal Hatanya* that the day is
    /// calculated from sunrise to sunset.
    #[must_use]
    pub fn sof_zman_tefila_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.sof_zman_tefila_mga(&Degrees(1.583))
    }

    /// Returns the latest time for burning *chametz* on *Erev
    /// Pesach* according to the opinion of the Baal Hatanya. This time is 5
    /// of the [*Baal Hatanya*'s temporal
    /// hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after [*hanetz
    /// amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya) (sunrise)
    /// based on the opinion of the *Baal Hatanya* that the day is
    /// calculated from sunrise to sunset. Since this library does not
    /// implement a calendar, this method will return the *zman* any day of
    /// the year.
    #[must_use]
    pub fn sof_zman_biur_chametz_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.sof_zman_biur_chametz_mga(&Degrees(1.583))
    }

    /// Returns the the *Baal Hatanya*'s *mincha gedola*. *Mincha
    /// gedola* is the earliest time one can pray *mincha*. The *Rambam* is
    /// of the opinion that it is better to delay *mincha* until *mincha
    /// ketana* while the *Rash*, *Tur*, GRA and others are of the opinion
    /// that *mincha* can be prayed *lechatchila* starting at *mincha
    /// gedola*. This is calculated as 6.5 of the [*Baal Hatanya*'s temporal
    /// hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after [*hanetz
    /// amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya) (sunrise).
    /// This calculation is based on the opinion of the *Baal Hatanya* that
    /// the day is calculated from sunrise to sunset.
    #[must_use]
    pub fn mincha_gedola_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.mincha_gedola_mga(&Degrees(1.583))
    }

    /// Returns the later of
    /// [`mincha_gedola_baal_hatanya`](ComplexZmanimCalendar::mincha_gedola_baal_hatanya)
    /// and
    /// [`mincha_gedola_30_minutes`](ComplexZmanimCalendar::mincha_gedola_30_minutes)
    /// . In the winter when 1/2 of a
    /// [*Baal Hatanya shaah
    /// zmanis*](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya)
    /// is less than 30 minutes
    /// [`mincha_gedola_30_minutes`](ComplexZmanimCalendar::mincha_gedola_30_minutes)
    /// will be returned, otherwise
    /// [`mincha_gedola_baal_hatanya`](ComplexZmanimCalendar::mincha_gedola_baal_hatanya)
    ///  will be returned
    #[must_use]
    pub fn mincha_gedola_baal_hatanya_greater_than_30_minutes(&self) -> Option<DateTime<Tz>> {
        let mg_30 = self.mincha_gedola_30_minutes()?;
        let mg_bht = self.mincha_gedola_baal_hatanya()?;
        Some(if mg_30 > mg_bht { mg_30 } else { mg_bht })
    }

    /// Returns the *Baal Hatanya*'s *mincha ketana*. This is the
    /// preferred earliest time to pray *mincha* in the opinion of the *Rambam*
    /// and others. For more information on this see the documentation on
    /// *mincha gedola*. This is calculated as 9.5 of the [*Baal Hatanya*'s
    /// temporal hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after
    /// [*hanetz amiti*](ComplexZmanimCalendar::hanetz_amiti_baal_hatanya)
    /// (sunrise). This calculation is calculated based on the opinion of
    /// the *Baal Hatanya* that the day is calculated from sunrise to
    /// sunset.
    #[must_use]
    pub fn mincha_ketana_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.mincha_ketana_mga(&Degrees(1.583))
    }

    /// Returns the *Baal Hatanya*'s *plag hamincha*. This is
    /// calculated as 10.75 of the [*Baal Hatanya*'s temporal
    /// hours](ComplexZmanimCalendar::shaah_zmanis_baal_hatanya) after *hanetz
    /// amiti* (sunrise). This calculation is calculated based on the
    /// opinion of the *Baal Hatanya* that the day is calculated from
    /// sunrise to sunset.
    #[must_use]
    pub fn plag_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.plag_mga(&Degrees(1.583))
    }

    /// Returns *tzais* (nightfall) when the sun is 6&deg; below
    /// the western geometric horizon (90&deg;) after sunset. This calculation
    /// is based on the position of the sun 24 minutes after sunset in Jerusalem
    /// around the equinox / equilux, which is 6&deg; below geometric zenith.
    #[must_use]
    pub fn tzais_baal_hatanya(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(6.0))
    }

    // Rav Moshe Feinstein
    /// Returns fixed local *chatzos*. See
    /// [`zmanim_calculator::fixed_local_chatzos`] for more details
    #[must_use]
    pub fn fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        zmanim_calculator::fixed_local_chatzos(&self.date, &self.geo_location)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman krias shema* (latest time to recite *Shema* in the morning)
    /// according to the opinion of the *Magen Avraham* (MGA) that the day is
    /// calculated from dawn to nightfall, but calculated using the first half
    /// of the day only. The half a day starts at *alos* defined as 18&deg; and
    /// ends at fixed local *chatzos*. *Sof Zman Shema* is 3 *shaos
    /// zmaniyos* (solar hours) after *alos* or half of this half-day.
    #[must_use]
    pub fn sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.alos_18_degrees()?;
        let offset = (self.fixed_local_chatzos()? - alos) / 2;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman krias shema* (latest time to recite *Shema* in the morning)
    /// according to the opinion of the *Magen Avraham* (MGA) that the day is
    /// calculated from dawn to nightfall, but calculated using the first half
    /// of the day only. The half a day starts at *alos* defined as 16.1&deg;
    /// and ends at fixed local *chatzos*. *Sof Zman Shema* is 3 *shaos
    /// zmaniyos* (solar hours) after this *alos* or half of this half-day.
    #[must_use]
    pub fn sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.alos_16_1_degrees()?;
        let offset = (self.fixed_local_chatzos()? - alos) / 2;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman krias shema* (latest time to recite *Shema* in the morning)
    /// according to the opinion of the *Magen Avraham* (MGA) that the day is
    /// calculated from dawn to nightfall, but calculated using the first half
    /// of the day only. The half a day starts at *alos* defined as 90 minutes
    /// before sunrise and ends at fixed local *chatzos*. *Sof Zman Shema* is 3
    /// *shaos zmaniyos* (solar hours) after this *alos* or half of this
    /// half-day.
    #[must_use]
    pub fn sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.alos_90_minutes()?;
        let offset = (self.fixed_local_chatzos()? - alos) / 2;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman krias shema* (latest time to recite *Shema* in the morning)
    /// according to the opinion of the *Magen Avraham* (MGA) that the day is
    /// calculated from dawn to nightfall, but calculated using the first half
    /// of the day only. The half a day starts at *alos* defined as 72 minutes
    /// before sunrise and ends at fixed local *chatzos*. *Sof Zman Shema* is 3
    /// *shaos zmaniyos* (solar hours) after this *alos* or half of this
    /// half-day.
    #[must_use]
    pub fn sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.alos_72_minutes()?;
        let offset = (self.fixed_local_chatzos()? - alos) / 2;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman krias shema* (latest time to recite *Shema* in the morning)
    /// according to the opinion of the GRA that the day is calculated from
    /// sunrise to sunset, but calculated using the first half of the day only.
    /// The half a day starts at sunrise and ends at fixed local *chatzos*. *Sof
    /// zman Shema* is 3 *shaos zmaniyos* (solar hours) after sunrise or half of
    /// this half-day.
    #[must_use]
    pub fn sof_zman_shema_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.hanetz()?;
        let offset = (self.fixed_local_chatzos()? - alos) / 2;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// *sof zman tefila* (the latest time to recite the morning
    /// prayers) according to the opinion of the GRA that the day is calculated
    /// from sunrise to sunset, but calculated using the first half of the day
    /// only. The half a day starts at sunrise and ends at fixed local
    /// *chatzos*. *Sof zman tefila* is 4 *shaos zmaniyos* (solar hours) after
    /// sunrise or 2/3 of this half-day.
    #[must_use]
    pub fn sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<DateTime<Tz>> {
        let alos = self.hanetz()?;
        let offset = ((self.fixed_local_chatzos()? - alos) * 2) / 3;
        Some(alos + offset)
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// mincha gedola, the earliest time one can pray mincha calculated
    /// according to the GRA that is 30 minutes after fixed local *chatzos*.
    #[must_use]
    pub fn mincha_gedola_gra_fixed_local_chatzos_30_minutes(&self) -> Option<DateTime<Tz>> {
        Some(self.fixed_local_chatzos()? + TimeDelta::minutes(30))
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// mincha ketana (the preferred time to recite the mincha prayers according
    /// to the opinion of the Rambam and others) calculated according to the GRA
    /// that is 3.5 *shaos zmaniyos* (solar hours) after fixed local *chatzos*.
    #[must_use]
    pub fn mincha_ketana_gra_fixed_local_chatzos_to_sunset(&self) -> Option<DateTime<Tz>> {
        let chatzos = self.fixed_local_chatzos()?;
        let half_shaah = (self.shkia()? - chatzos) / 12;
        Some(chatzos + (half_shaah * 7))
    }

    /// Returns Rav Moshe Feinstein's opinion of the calculation of
    /// plag hamincha. This method returns plag hamincha calculated according to
    /// the GRA that the day ends at sunset and is 4.75 *shaos zmaniyos* (solar
    /// hours) after fixed local *chatzos*.
    #[must_use]
    pub fn plag_gra_fixed_local_chatzos_to_sunset(&self) -> Option<DateTime<Tz>> {
        let chatzos = self.fixed_local_chatzos()?;
        // (19/24) == (4.75/6)
        let quarter_shaah = (self.shkia()? - chatzos) / 24;
        Some(chatzos + (quarter_shaah * 19))
    }

    /// Method to return *tzais* (dusk) calculated as 50 minutes after
    /// sunset. This method returns
    /// *tzais* (nightfall) based on the opinion of Rabbi Moshe Feinstein
    /// for the New York area. This time should not be used for latitudes
    /// other than ones similar to the latitude of the NY area.
    #[must_use]
    pub fn tzais_50_minutes(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(50.0))
    }

    // Ahavat Shalom
    /// Returns the time of *mincha gedola* based on the opinion of
    /// Rabbi Yaakov Moshe Hillel as published in the luach of the Bais Horaah
    /// of Yeshivat Chevrat Ahavat Shalom that *mincha gedola* is calculated as
    /// half a *shaah zmanis* after *chatzos* with *shaos zmaniyos* calculated
    /// based on a day starting 72 minutes before sunrise (alos 16.1&deg;) and
    /// ending 13.5 minutes after sunset (*tzais* 3.7&deg;). *Mincha gedola* is
    /// the earliest time to pray *mincha*. The later of this time or 30
    /// clock minutes after *chatzos* is returned. See
    /// [`mincha_gedola_gra_greater_than_30_minutes`](ComplexZmanimCalendar::mincha_gedola_gra_greater_than_30_minutes)
    /// (though that calculation is based on *mincha gedola* GRA). For more
    /// information about *mincha gedola* see the documentation on [*mincha
    /// gedola*](zmanim_calculator::mincha_gedola).
    #[must_use]
    pub fn mincha_gedola_ahavat_shalom(&self) -> Option<DateTime<Tz>> {
        let mg_as = self.chatzos()? + (self.shaah_zmanis_alos_16_1_to_tzais_3_7()? / 2);
        let mg_30 = self.mincha_gedola_30_minutes()?;
        Some(if mg_30 > mg_as { mg_30 } else { mg_as })
    }

    /// Returns the time of *mincha ketana* based on the opinion of
    /// Rabbi Yaakov Moshe Hillel as published in the luach of the Bais Horaah
    /// of Yeshivat Chevrat Ahavat Shalom that *mincha ketana* is calculated as
    /// 2.5 *shaos zmaniyos* before *tzais* 3.8&deg; with *shaos zmaniyos*
    /// calculated based on a day starting at *alos* 16.1&deg; and ending at
    /// *tzais* 3.8&deg;. *Mincha ketana* is the preferred earliest time to
    /// pray *mincha* according to the opinion of the *Rambam* and others.
    /// For more information on this see the documentation on [*mincha
    /// gedola*](zmanim_calculator::mincha_gedola).
    #[must_use]
    pub fn mincha_ketana_ahavat_shalom(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_ketana(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_3_8_degrees()?,
        ))
    }

    /// Returns the time of *plag hamincha* based on the opinion of
    /// Rabbi Yaakov Moshe Hillel as published in the luach of the Bais Horaah
    /// of Yeshivat Chevrat Ahavat Shalom that that *plag hamincha* is
    /// calculated as 1.25 *shaos zmaniyos* before *tzais* 3.8&deg; with *shaos
    /// zmaniyos* calculated based on a day starting at *alos* 16.1&deg; and
    /// ending at *tzais* 3.8&deg;.
    #[must_use]
    pub fn plag_ahavat_shalom(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_3_8_degrees()?,
        ))
    }

    /// Returns a *shaah zmanis* (temporal hour) used by some *zmanim*
    /// according to the opinion of Rabbi Yaakov Moshe Hillel as published in
    /// the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom that is
    /// based on a day starting 72 minutes before sunrise in degrees (*alos*
    /// 16.1&deg;) and ending 14 minutes after sunset in degrees (*tzais*
    /// 3.8&deg;). This day is split into 12 equal parts with each part
    /// being a *shaah zmanis*. Note that with this system, *chatzos*
    /// (midday) will not be the point that the sun is halfway across the
    /// sky. These *shaos zmaniyos* are used for *Mincha Ketana* and *Plag
    /// Hamincha*. The 14 minutes are based on 3/4 of an 18 minute *mil*,
    /// with half a minute added for Rav Yosi.
    #[must_use]
    pub fn shaah_zmanis_alos_16_1_to_tzais_3_8(&self) -> Option<TimeDelta> {
        Some(zmanim_calculator::shaah_zmanis(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_3_8_degrees()?,
        ))
    }

    /// Returns a *shaah zmanis* (temporal hour) used by some *zmanim*
    /// according to the opinion of Rabbi Yaakov Moshe Hillel as published in
    /// the luach of the Bais Horaah of Yeshivat Chevrat Ahavat Shalom that is
    /// based on a day starting 72 minutes before sunrise in degrees (*alos*
    /// 16.1&deg;) and ending 13.5 minutes after sunset in degrees (*tzais*
    /// 3.7&deg;). This day is split into 12 equal parts with each part
    /// being a *shaah zmanis*. Note that with this system, *chatzos*
    /// (midday) will not be the point that the sun is halfway across the
    /// sky. These *shaos zmaniyos* are used for *mincha gedola* calculation.
    #[must_use]
    pub fn shaah_zmanis_alos_16_1_to_tzais_3_7(&self) -> Option<TimeDelta> {
        Some(zmanim_calculator::shaah_zmanis(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_3_7_degrees()?,
        ))
    }

    // Ateret Torah
    /// Returns the latest *zman krias shema* (time to recite
    /// *Shema* in the morning) based on the calculation of *Chacham* Yosef
    /// Harari-Raful of Yeshivat Ateret Torah, that the day starts 1/10th of the
    /// day before sunrise and is usually calculated as ending 40 minutes after
    /// sunset. *Shaos zmaniyos* are calculated based on this day and added to
    /// *alos*to reach this time. This time is 3 *shaos zmaniyos* (temporal
    /// hours) after *alos*72 zmaniyos. Note: Based on this calculation
    /// *chatzos* will not be at midday.
    #[must_use]
    pub fn sof_zman_shema_ateret_torah(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_shema(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    /// Returns the latest *zman tefila* (time to recite the morning
    /// prayers) based on the calculation of *Chacham* Yosef Harari-Raful of
    /// Yeshivat Ateret Torah, that the day starts 1/10th of the day before
    /// sunrise and is usually calculated as ending 40 minutes after sunset.
    /// *Shaos zmaniyos* are calculated based on this day and added to *alos*to
    /// reach this time. This time is 4 * *shaos zmaniyos* (temporal hours)
    /// after *alos*72 *zmaniyos*. Note: Based on this calculation *chatzos*
    /// will not be at midday.
    #[must_use]
    pub fn sof_zman_tefila_ateret_torah(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_tefila(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    /// Returns the time of *mincha gedola* based on the calculation of
    /// *Chacham* Yosef Harari-Raful of Yeshivat Ateret Torah, that the day
    /// starts 1/10th of the day before sunrise and is usually calculated as
    /// ending 40 minutes after sunset. The *Rambam* is of the opinion that it
    /// is better to delay *mincha* until *mincha ketana* while the Ra"sh, Tur,
    /// GRA and others are of the opinion that mincha can be prayed lechatchila
    /// starting at mincha gedola. For more information on this see the
    /// documentation on [*mincha gedola*](zmanim_calculator::mincha_gedola).
    /// This is calculated as 6.5 solar hours after *alos*. The calculation used
    /// is 6.5 * [`ComplexZmanimCalendar::shaah_zmanis_ateret_torah`] after
    /// *alos*.
    #[must_use]
    pub fn mincha_gedola_ateret_torah(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_gedola(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    /// Returns the time of *mincha ketana* based on the calculation of
    /// *Chacham* Yosef Harari-Raful of Yeshivat Ateret Torah, that the day
    /// starts 1/10th of the day before sunrise and is usually calculated as
    /// ending 40 minutes after sunset. This is the preferred earliest time to
    /// pray *mincha* according to the opinion of the *Rambam* and others. For
    /// more information on this see the documentation on [*mincha
    /// ketana*](zmanim_calculator::mincha_ketana). This is calculated as 9.5
    /// solar hours after *alos*. The calculation used is 9.5 *
    /// [`ComplexZmanimCalendar::shaah_zmanis_ateret_torah`] after *alos*.
    #[must_use]
    pub fn mincha_ketana_ateret_torah(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::mincha_ketana(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    /// Returns the time of *plag hamincha* based on the calculation
    /// of *Chacham* Yosef Harari-Raful of Yeshivat Ateret Torah, that the day
    /// starts 1/10th of the day before sunrise and is usually calculated as
    /// ending 40 minutes after sunset. *Shaos zmaniyos* are calculated based on
    /// this day and added to *alos* to reach this time. This time is 10.75
    /// *shaos zmaniyos* (temporal hours) after dawn.
    #[must_use]
    pub fn plag_ateret_torah(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    /// Returns *tzais* calculated as 40 minutes after sunset. Please note that
    /// *Chacham* Yosef Harari-Raful of Yeshivat Ateret Torah who uses this
    /// time, does so only for calculating various other zmanei hayom such as
    /// *Sof Zman Krias Shema* and *Plag Hamincha*. His calendars do not publish
    /// a *zman* for *Tzais*. It should also be noted that *Chacham*
    /// Harari-Raful provided a 25 minute *zman* for Israel. This API uses
    /// 40 minutes year round in any place on the globe.
    #[must_use]
    pub fn tzais_ateret_torah(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(40.0))
    }

    /// Returns a shaah zmanis (temporal hour) according to the opinion
    /// of the *Chacham* Yosef Harari-Raful of Yeshivat Ateret Torah calculated
    /// with *alos* being 1/10th of sunrise to sunset day, or 72 minutes
    /// *zmaniyos* of such a day before sunrise, and *tzais* is usually
    /// calculated as 40 minutes after sunset. This day is split into 12
    /// equal parts with each part being a shaah zmanis. Note that with this
    /// system, *chatzos* (midday) will not be the point that the sun is
    /// halfway across the sky.
    #[must_use]
    pub fn shaah_zmanis_ateret_torah(&self) -> Option<TimeDelta> {
        Some(zmanim_calculator::shaah_zmanis(
            &self.alos_72_minutes_zmanis()?,
            &self.tzais_ateret_torah()?,
        ))
    }

    // Shach
    /// Returns the latest *zman krias shema* (time to recite *Shema* in the
    /// morning) calculated as 3 hours (regular clock hours and not *shaos
    /// zmaniyos*) before *chatzos*. Generally known as part of the "Komarno"
    /// *zmanim* after Rav Yitzchak Eizik of Komarno, a proponent of this
    /// calculation, it actually predates him a lot. It is the opinion of the
    /// *Shach* in the *Nekudas Hakesef* (*Yoreh Deah* 184), Rav Moshe Lifshitz
    /// in his commentary *Lechem Mishneh* on *Brachos* 1:2. It is next brought
    /// down about 100 years later by the *Yaavetz* (in his *siddur*, *Mor
    /// Uktziah Orach Chaim* 1, *Lechem Shamayim*, *Brachos* 1:2 and *She'elos
    /// Yaavetz* vol. 1 no. 40), Rav Yitzchak Eizik of Komarno in the *Ma'aseh
    /// Oreg* on *Mishnayos Brachos* 11:2, *Shevus Yaakov*, *Chasan Sofer* and
    /// others. See *Yisrael Vehazmanim* vol. 1 7:3, page 55 - 62
    #[must_use]
    pub fn sof_zman_shema_3_hrs_before_chatzos(&self) -> Option<DateTime<Tz>> {
        Some(self.chatzos()? - TimeDelta::hours(3))
    }

    /// Returns the latest *zman* tefila (time to recite the morning prayers)
    /// calculated as 2 hours before *chatzos*. This is based on the opinions
    /// that calculate sof *zman* krias shema as [3 hours before
    /// *chatzos*](ComplexZmanimCalendar::sof_zman_shema_3_hrs_before_chatzos).
    #[must_use]
    pub fn sof_zman_tefila_2_hrs_before_chatzos(&self) -> Option<DateTime<Tz>> {
        Some(self.chatzos()? - TimeDelta::hours(2))
    }

    zmanim_for_offset!(
        _16_1_degrees,
        |_| Some(Degrees(16.1)),
        [
            alos => alos_degrees_basedon_doc!(16.1, 72),
            tzais => tzais_degrees_basedon_doc!(16.1, 72),
            shaah_zmanis_mga => shaah_mga_degrees_basedon_doc!(16.1, 72),
            sof_zman_shema_mga => szks_mga_degrees_doc!(16.1),
            sof_zman_tefila_mga => szt_mga_degrees_doc!(16.1),
            sof_zman_biur_chametz_mga => sz_biur_chametz_mga_degrees_doc!(16.1),
            mincha_gedola_mga => mg_mga_degrees_doc!(16.1),
            samuch_lemincha_ketana_mga => slmk_mga_degrees_doc!(16.1),
            mincha_ketana_mga => mk_mga_degrees_doc!(16.1),
            plag_mga => plag_mga_degrees_lechumra_doc!(16.1),
        ]
    );
    // *alos* 16.1 degrees to sunset
    /// Returns the latest *zman krias shema* (time to recite
    /// *Shema* in the morning) based on the opinion that the day starts at
    /// *alos* 16.1&deg; and ends at sea level sunset (no adjustment for
    /// elevation is made). This is the opinion of the *Chidushei UKlalos
    /// HaRazah* and the *Menora Hatehora* as mentioned by *Yisrael
    /// Vehazmanim* vol 1, sec. 7, ch. 3 no. 16. Three *shaos zmaniyos* are
    /// calculated based on this day and added to *alos* to reach this time.
    /// This time is 3 *shaos zmaniyos* (solar hours) after dawn based on
    /// the opinion that the day is calculated from a *alos* 16.1&deg; to
    /// sea level sunset.
    #[must_use]
    pub fn sof_zman_shema_alos_16_1_degrees_to_sunset(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_shema(
            &self.alos_16_1_degrees()?,
            &self.sea_level_sunset()?,
        ))
    }

    // seems like this method was removed from KJ
    /// This method should be used *lechumra* only and returns the time of *plag
    /// hamincha* based on the opinion that the day starts at *alos* 16.1&deg;
    /// and ends at sunset. 10.75 *shaos zmaniyos* are calculated based on
    /// this day and added to *alos*to reach this time. This time is 10.75
    /// *shaos zmaniyos* (temporal hours) after dawn based on the opinion
    /// that the day is calculated from a dawn of 16.1 degrees before
    /// sunrise to sunset. This returns the time of 10.75 * the calculated
    /// *shaah zmanis* after dawn. Since plag by this calculation can occur
    /// after sunset, it should only be used *lechumra*.
    #[must_use]
    pub fn plag_alos_16_1_degrees_to_sunset(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.alos_16_1_degrees()?,
            &self.shkia()?,
        ))
    }

    // 16.1 degrees to *tzais* geonim 7.083 degrees
    /// Returns the latest *zman krias shema* (time to recite
    /// *Shema* in the morning) based on the opinion that the day starts at
    /// *alos* 16.1&deg; and ends at *tzais* 7.083&deg;. 3 *shaos zmaniyos*
    /// are calculated based on this day and added to *alos*to reach this
    /// time. This time is 3 *shaos zmaniyos* (temporal hours) after alos
    /// 16.1&deg; based on the opinion that the day is calculated from a
    /// *alos* 16.1&deg; to *tzais* 7.083&deg;.
    #[must_use]
    pub fn sof_zman_shema_alos_16_1_degrees_to_tzais_geonim_7_083_degrees(
        &self,
    ) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::sof_zman_shema(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_7_083_degrees()?,
        ))
    }

    /// Returns the time of *plag hamincha* based on the opinion
    /// that the day starts at *alos* 16.1&deg; and ends at *tzais*
    /// 7.083&deg;. 10.75 *shaos zmaniyos* are calculated based on this day
    /// and added to *alos* to reach this time. This time is 10.75 *shaos
    /// zmaniyos* (temporal hours) after dawn based on the opinion that the
    /// day is calculated from a dawn of 16.1 degrees before sunrise to
    /// *tzais* 7.083&deg;. This returns the time of 10.75 * the calculated
    /// *shaah zmanis* after dawn.
    #[must_use]
    pub fn plag_alos_16_1_degrees_to_tzais_geonim_7_083_degrees(&self) -> Option<DateTime<Tz>> {
        Some(zmanim_calculator::plag_hamincha(
            &self.alos_16_1_degrees()?,
            &self.tzais_geonim_7_083_degrees()?,
        ))
    }

    zmanim_for_offset!(
        _18_degrees,
        |_| Some(Degrees(18.0)),
        [
            alos => alos_degrees_doc!(18),
            tzais => tzais_degrees_doc!(18),
            shaah_zmanis_mga => shaah_mga_degrees_doc!(18),
            sof_zman_shema_mga => szks_mga_degrees_doc!(18),
            sof_zman_tefila_mga => szt_mga_degrees_doc!(18),
            plag_mga => plag_mga_degrees_lechumra_doc!(18),
        ]
    );

    // 19 degrees
    /// Returns *alos* (dawn) calculated when the sun is 19&deg;
    /// below the eastern geometric horizon before sunrise. This is the
    /// *Rambam*'s *alos* according to Rabbi Moshe Kosower's *Maaglei Tzedek*,
    /// page 88, *Ayeles Hashachar* Vol. I, page 12, *Yom Valayla Shel Torah*,
    /// Ch. 34, p. 222 and Rabbi Yaakov Shakow's *Luach Ikvei Hayom*.
    #[must_use]
    pub fn alos_19_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(19.0))
    }

    zmanim_for_offset!(
        _19_8_degrees,
        |_| Some(Degrees(19.8)),
        [
            alos => alos_degrees_basedon_doc!(19.8, 90),
            tzais => tzais_degrees_basedon_doc!(19.8, 90),
            shaah_zmanis_mga => shaah_mga_degrees_basedon_doc!(19.8, 90),
            sof_zman_shema_mga => szks_mga_degrees_doc!(19.8),
            sof_zman_tefila_mga => szt_mga_degrees_doc!(19.8),
            plag_mga => plag_mga_degrees_lechumra_doc!(19.8),
        ]
    );

    zmanim_for_offset!(
        _26_degrees,
        |_| Some(Degrees(26.0)),
        [
            alos => alos_degrees_basedon_lechumra_doc!(26, 120),
            tzais => tzais_degrees_basedon_doc!(26, 120),
            shaah_zmanis_mga => shaah_mga_degrees_lechumra_basedon_doc!(26, 120),
            plag_mga => plag_mga_degrees_lechumra_doc!(26),
        ]
    );

    zmanim_for_offset!(
        _60_minutes,
        |_| Some(Minutes(60.0)),
        [
            alos => "Returns *alos* (dawn) calculated as 60 minutes before sunrise. This is the time to walk the distance of 4 *mil* at 15 minutes a *mil*. This seems to be the opinion of the *Chavas Yair* in the *Mekor Chaim, Orach Chaim* Ch. 90, though the *Mekor Chaim* in Ch. 58 and in the *Chut Hashani* Ch. 97 states that a person walks 3 and a 1/3 *mil* in an hour, or an 18-minute *mil*. Also see the *Divrei Malkiel* Vol. 4, Ch. 20, page 34) who mentions the 15 minute *mil lechumra* by baking *matzos*. Also see the *Maharik* Ch. 173 where the questioner quoting the *Ra'avan* is of the opinion that the time to walk a *mil* is 15 minutes (5 *mil* in a little over an hour). There are many who believe that there is a *ta'us sofer* (scribeal error) in the *Ra'avan*, and it should 4 *mil* in a little over an hour, or an 18-minute *mil*. Time based offset calculations are based on the opinion of the *Rishonim* who stated that the time of the *neshef* (time between dawn and sunrise) does not vary by the time of year or location but purely depends on the time it takes to walk the distance of 4 *mil*.",
            tzais => "Returns *tzais hakochavim* (nightfall) based on the opinion of the *Chavas Yair* and *Divrei Malkiel* that the time to walk the distance of a mil is 15 minutes, for a total of 60 minutes for 4 *mil* after sunset. See detailed documentation explaining the 60 minute concept at [alos_60_minutes](ComplexZmanimCalendar::alos_60_minutes).",
            shaah_zmanis_mga => shaah_mga_minutes_doc!(60),
            plag_mga => plag_mga_minutes_doc!(60),
        ]
    );

    zmanim_for_offset!(
        _72_minutes,
        |_| Some(Minutes(72.0)),
        [
            alos => alos_minutes_basedon_doc!(72, 18),
            tzais => "Returns *tzais hakochavim* (nightfall) based on the opinion of *Rabbeinu Tam* that *tzais hakochavim* is calculated as 72 minutes after sunset, the time it takes to walk 4 *mil* at 18 minutes a mil. According to the *Machtzis Hashekel* in *Orach Chaim* 235:3, the *Pri Megadim* in *Orach Chaim* 261:2 (see the *Biur Halacha*) and others (see *Hazmanim Bahalacha* 17:3 and 17:5) the 72 minutes are standard clock minutes any time of the year in any location.",
            shaah_zmanis_mga => shaah_mga_minutes_doc!(72),
            sof_zman_shema_mga => szks_mga_minutes_doc!(72),
            sof_zman_tefila_mga => szt_mga_minutes_doc!(72),
            sof_zman_biur_chametz_mga => sz_biur_chametz_mga_minutes_doc!(72),
            mincha_gedola_mga => mg_mga_minutes_doc!(72),
            samuch_lemincha_ketana_mga => slmk_mga_minutes_doc!(72),
            mincha_ketana_mga => mk_mga_minutes_doc!(72),
            plag_mga => plag_mga_minutes_lechumra_doc!(72),
        ]
    );

    zmanim_for_offset!(
        _72_minutes_zmanis,
        |this: &ComplexZmanimCalendar| {
            Some(MinutesZmaniyos {
                minutes_zmaniyos: 72.0,
                shaah_zmanis: this.shaah_zmanis_gra()?,
            })
        },
        [
            alos => concat!(
                alos_minutes_zmanis_basedon_doc!(72, 18, "1/10th"),
                "\nThis calculation is used in the calendars published by the Hisachdus Harabanim D'Artzos Habris Ve'Canada."
            ),
            tzais => "Returns *tzais hakochavim* (nightfall) calculated as 72 minutes *zmaniyos*, or 1/10th of the day after sunset. This is the way that the *Minchas Cohen* in *Ma'amar* 2:4 calculates *Rebbeinu Tam*'s time of *tzeis*. It should be noted that this calculation results in the shortest time from sunset to *tzais* being during the winter solstice, the longest at the summer solstice and 72 clock minutes at the equinox. This does not match reality, since there is no direct relationship between the length of the day and twilight. The shortest twilight is during the equinox, the longest is during the summer solstice, and in the winter with the shortest daylight, the twilight period is longer than during the equinoxes.",
            shaah_zmanis_mga => shaah_mga_minutes_zmanis_doc!(72, "1/10th"),
            sof_zman_shema_mga => szks_mga_minutes_zmanis_doc!(72, "1/10th"),
            sof_zman_tefila_mga => szt_mga_minutes_zmanis_doc!(72, "1/10th"),
            sof_zman_biur_chametz_mga => sz_biur_chametz_mga_minutes_zmanis_doc!(72, "1/10th"),
            plag_mga => plag_mga_minutes_zmanis_lechumra_doc!(72, "1/10th"),
        ]
    );

    zmanim_for_offset!(
        _90_minutes,
        |_| Some(Minutes(90.0)),
        [
            alos => alos_minutes_basedon_doc!(90, 22.5),
            tzais => "Returns *tzais hakochavim* (dusk) calculated as 90 minutes after sunset. This method returns *tzais* based on the opinion of the *Magen Avraham* that the time to walk the distance of a *mil* according to the *Rambam*'s opinion is 18 minutes, for a total of 90 minutes based on the opinion of Ula who calculated *tzais* as 5 *mil* after *shkiah* (sunset). A similar calculation [tzais_19_8_degrees](ComplexZmanimCalendar::tzais_19_8_degrees) uses solar position calculations based on this time.",
            shaah_zmanis_mga => shaah_mga_minutes_doc!(90),
            sof_zman_shema_mga => szks_mga_minutes_doc!(90),
            sof_zman_tefila_mga => szt_mga_minutes_doc!(90),
            plag_mga => plag_mga_minutes_lechumra_doc!(90),
        ]
    );

    zmanim_for_offset!(
        _90_minutes_zmanis,
        |this: &ComplexZmanimCalendar| {
            Some(MinutesZmaniyos {
                minutes_zmaniyos: 90.0,
                shaah_zmanis: this.shaah_zmanis_gra()?,
            })
        },
        [
            alos => alos_minutes_zmanis_basedon_doc!(90, 22.5, "1/8th"),
            tzais => concat!(
                tzais_minutes_zmanis_basedon_doc!(90, 22.5, "1/8th"),
                " This time is known in Yiddish as the *achtel* (an eighth) *zman*."
            ),
            shaah_zmanis_mga => shaah_mga_minutes_zmanis_doc!(90, "1/8th"),
            sof_zman_shema_mga => szks_mga_minutes_zmanis_doc!(90, "1/8th"),
            sof_zman_tefila_mga => szt_mga_minutes_zmanis_doc!(90, "1/8th"),
            plag_mga => plag_mga_minutes_zmanis_lechumra_doc!(90, "1/8th"),
        ]
    );

    zmanim_for_offset!(
        _96_minutes,
        |_| Some(Minutes(96.0)),
        [
            alos => alos_minutes_basedon_doc!(96, 24),
            tzais => tzais_minutes_basedon_doc!(96, 24),
            shaah_zmanis_mga => shaah_mga_minutes_doc!(96),
            sof_zman_shema_mga => szks_mga_minutes_doc!(96),
            sof_zman_tefila_mga => szt_mga_minutes_doc!(96),
            plag_mga => plag_mga_minutes_lechumra_doc!(96),
        ]
    );

    zmanim_for_offset!(
        _96_minutes_zmanis,
        |this: &ComplexZmanimCalendar| {
            Some(MinutesZmaniyos {
                minutes_zmaniyos: 96.0,
                shaah_zmanis: this.shaah_zmanis_gra()?,
            })
        },
        [
            alos => alos_minutes_zmanis_basedon_doc!(96, 22.5, "1/7.5th"),
            tzais => tzais_minutes_zmanis_basedon_doc!(96, 22.5, "1/7.5th"),
            shaah_zmanis_mga => shaah_mga_minutes_zmanis_doc!(96, "1/7.5th"),
            sof_zman_shema_mga => szks_mga_minutes_zmanis_doc!(96, "1/7.5th"),
            sof_zman_tefila_mga => szt_mga_minutes_zmanis_doc!(96, "1/7.5th"),
            plag_mga => plag_mga_minutes_zmanis_lechumra_doc!(96, "1/7.5th"),
        ]
    );

    zmanim_for_offset!(
        _120_minutes,
        |_| Some(Minutes(120.0)),
        [
            alos => "This method should be used *lechumra* only and returns *alos* (dawn) calculated using 120 minutes before sea level sunrise (no adjustment for elevation is made) based on the time to walk the distance of 5 *mil* (Ula) at 24 minutes a *mil*. Time based offset calculations for *alos* are based on the* opinion of the Rishonim who stated that the time of the *neshef* (time between dawn and sunrise) does not vary by the time of year or location but purely depends on the time it takes to walk the distance of 5 *mil* (Ula). Since this time is extremely early, it should only be used *lechumra*, such as not eating after this time on a fast day, and **not** as the start time for *mitzvos* that can only be performed during the day.",
            tzais => "Returns *tzais hakochavim* (dusk) calculated as 120 minutes after sunset. For information on how this is calculated see the documentation on [alos_120_minutes](ComplexZmanimCalendar::alos_120_minutes).",
            shaah_zmanis_mga => shaah_mga_minutes_doc!(120),
            sof_zman_shema_mga => szks_mga_minutes_doc!(120),
            sof_zman_tefila_mga => szt_mga_minutes_doc!(120),
            plag_mga => plag_mga_minutes_lechumra_doc!(120),
        ]
    );

    zmanim_for_offset!(
        _120_minutes_zmanis,
        |this: &ComplexZmanimCalendar| {
            Some(MinutesZmaniyos {
                minutes_zmaniyos: 120.0,
                shaah_zmanis: this.shaah_zmanis_gra()?,
            })
        },
        [
            alos => "This method should be used *lechumra* only and method returns *alos* (dawn) calculated using 120 minutes *zmaniyos* or 1/6th of the day before sunrise. This is based on a 24-minute *mil* so the time for 5 mil is 120 minutes which is 1/6th of a day `(12 * 60) / 6 = 120`. The day is calculated from sunrise to sunset. The actual calculation used is `astronomical_calculator::sunrise(&self.date, &self.geo_location) - (&self.shaah_zmanis_gra()? * 2.0)`. Since this time is extremely early, it should only be used *lechumra*, such as not eating after this time on a fast day, and **not** as the start time for *mitzvos* that can only be performed during the day.",
            tzais => "This method should be used *lechumra* only and returns *tzais* (dusk) calculated using 120 minutes *zmaniyos* after sunset. Since the *zman* is extremely late and at a time when the sun is well below the 18&deg; point (scientifically the darkest point) in most places on the globe, it should only be used *lechumra*, such as delaying the start of nighttime mitzvos.",
            shaah_zmanis_mga => "Returns a *shaah zmanis* (temporal hour) calculated using a dip of 120 minutes. This calculation divides the day based on the opinion of the *Magen Avraham* (MGA) that the day runs from dawn to dusk. Dawn for this calculation is 120 minutes before sunrise and dusk is 120 minutes after sunset. This day is split into 12 equal parts with each part being a *shaah zmanis*. This is identical to 1/6th of the day from sunrise to sunset. Since *zmanim* that use this method are extremely late or early and at a point when the sky is a long time past the 18&deg; point where the darkest point is reached, *zmanim* that use this should only be used *lechumra* only, such as delaying the start of nighttime *mitzvos*.",
            plag_mga => plag_mga_minutes_zmanis_lechumra_doc!(120, "1/6th"),
        ]
    );

    // Other Misheyakir
    /// Returns *misheyakir* based on the position of the sun 12.85&deg; below
    /// geometric zenith (90&deg;). This is based on the position of the sun
    /// slightly later than 57 minutes before sunrise in Jerusalem around the
    /// equinox / equilux. This *zman* is mentioned for use ***bish'as
    /// hadchak*** in the Birur Halacha Tinyana and Tlisa'ah in Orach Chaim
    /// siman 18 as 12.85&deg;. Actual calculations show it to be slightly more
    /// than 12.9&deg;, but the Birur Halacha indicates that 12.85&deg; is a
    /// slight *chumra* (on a *bedieved* time) VS the 12.9&deg; that 57 minutes
    /// calculates as (a difference of about 14 seconds at the equinox/equilux
    /// in Jerusalem). The *zman* of 12.9&deg; is also mentioned in the Piskei
    /// Tshuvos siman 18, page 190 (where a typo indicates that this is the
    /// degree equivalent to 60 minutes before sunrise, when in fact at that
    /// point the sun is about 13.5&deg; below the horizon). The 57 minute based
    /// time is mentioned by the Minchas Yitzchak vol. 9, siman 9 as 15 minutes
    /// before *alos hashachar* (though he is not clear what location he refers
    /// to, and does not mention a degree-based conversion). The Kaf Hachaim
    /// vol.1 siman 18, no. 18 states that in Yerushalayim 60 fixed minutes are
    /// used year round. Calculations show that 60 fixed minutes in Yerushalayim
    /// ranges from 13.5&deg; at the spring equinox to 11.5&deg; at the summer
    /// solstice. 57 minutes range from 12.9&deg; at the winter equinox to
    /// 11&deg; at the summer solstice. Analysis of the difference between
    /// 12.85&deg; and 12.9&deg;, shows that the maximum difference occurs at
    /// the summer solstice. In Lakewood, NJ at a latitude of 40.096&deg;, the
    /// maximum difference throughout the year is 23 seconds. In the winter
    /// where there is the greatest need for very early *misheyakir* times, the
    /// difference is in the 16 second range. Going north to Montreal at
    /// latitude 45.5&deg;, the maximum is 29 seconds and is about 18 seconds in
    /// the winter. Moving farther north to the elevation of Vilnius at a
    /// latitude of 54.68&deg;, things change. Firstly, around the summer
    /// solstice it will not reach that far below the horizon. On the dates that
    /// both can be calculated, the maximum difference can be pretty high on one
    /// or two days of the year (around Jul 8), with about a week having over a
    /// two minute difference between the two. Even at the latitude of Vilna,
    /// from Dec - March, the difference is about 22 seconds.
    #[must_use]
    pub fn misheyakir_12_85_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(12.85))
    }

    /// Returns *misheyakir* based on the position of the sun when
    /// it is 11.5&deg; below geometric zenith (90&deg;). This calculation is
    /// used for calculating *misheyakir* according to some opinions. This
    /// calculation is based on the position of the sun 52 minutes before
    /// sunrise in Jerusalem around the equinox / equilux, which calculates
    /// to 11.5&deg; below geometric zenith.
    #[must_use]
    pub fn misheyakir_11_5_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(11.5))
    }

    /// Returns *misheyakir* based on the position of the sun when
    /// it is 11&deg; below geometric zenith (90&deg;). This calculation is used
    /// for calculating *misheyakir* according to some opinions. This
    /// calculation is based on the position of the sun 48 minutes before
    /// sunrise in Jerusalem around the equinox / equilux, which calculates
    /// to 11&deg; below geometric zenith.
    #[must_use]
    pub fn misheyakir_11_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(11.0))
    }

    /// Returns *misheyakir* based on the position of the sun when
    /// it is 10.2&deg; below geometric zenith (90&deg;). This calculation is
    /// used for calculating *misheyakir* according to some opinions. This
    /// calculation is based on the position of the sun 45 minutes before
    /// sunrise in Jerusalem around the equinox which calculates to 10.2&deg;
    /// below geometric zenith.
    #[must_use]
    pub fn misheyakir_10_2_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(10.2))
    }

    /// Returns *misheyakir* based on the position of the sun when
    /// it is 9.5&deg; below geometric zenith (90&deg;). This calculation is
    /// based on Rabbi Dovid Kronglass's Calculation of 45 minutes in
    /// Baltimore as mentioned in *Divrei Chachamim* No. 24 brought down by
    /// the *Birur Halacha*, *Tinyana*, Ch. 18. This calculates to 9.5&deg;.
    /// Also see Rabbi Yaakov Yitzchok Neiman in *Kovetz Eitz Chaim* Vol. 9,
    /// p. 202 that the *Vya'an Yosef* did not want to rely on times earlier
    /// than 45 minutes in New York. This *zman* is also used in the
    /// calendars published by Rabbi Hershel Edelstein. As mentioned in
    /// Yisroel Vehazmanim, Rabbi Edelstein who was given the 45 minute
    /// *zman* by Rabbi Bick. The calendars published by the *Edot
    /// Hamizrach* communities also use this *zman*. This also follows the
    /// opinion of Rabbi Shmuel Kamenetsky who provided the time of 36 and
    /// 45 minutes, but did not provide a degree-based time. Since this
    /// *zman* depends on the level of light, Rabbi Yaakov Shakow presented
    /// these degree-based times to Rabbi Shmuel Kamenetsky who agreed to
    /// them.
    #[must_use]
    pub fn misheyakir_9_5_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(9.5))
    }

    /// Returns *misheyakir* based on the position of the sun when
    /// it is 7.65&deg; below geometric zenith (90&deg;). The degrees are based
    /// on a 35/36 minute *zman* around the equinox / equilux, when the
    /// *neshef* (twilight) is the shortest. This time is based on Rabbi
    /// Moshe Feinstein who writes in *Ohr Hachaim* Vol. 4, Ch. 6 that
    /// *misheyakir* in New York is 35-40 minutes before sunrise, something
    /// that is a drop less than 8&deg;. Rabbi Yisroel Taplin in *Zmanei
    /// Yisrael* (page 117) notes that Rabbi Yaakov Kamenetsky stated that
    /// it is not less than 36 minutes before sunrise (maybe it is
    /// 40 minutes). *Sefer Yisrael Vehazmanim* (p. 7) quotes the *Tamar
    /// Yifrach* in the name of the Satmar Rov that one should be stringent
    /// not consider *misheyakir* before 36 minutes. This is also the
    /// accepted *minhag* in Lakewood that is used in the Yeshiva. This
    /// follows the opinion of Rabbi Shmuel Kamenetsky who provided the time
    /// of 35/36 minutes, but did not provide a degree-based time. Since
    /// this *zman* depends on the level of light, Rabbi Yaakov Shakow
    /// presented this degree-based calculations to Rabbi Shmuel Kamenetsky
    /// who agreed to them.
    #[must_use]
    pub fn misheyakir_7_65_degrees(&self) -> Option<DateTime<Tz>> {
        self.alos(&Degrees(7.65))
    }

    // Bein Hashmashos Yereim
    /// Returns the beginning of *bein hashmashos* (twilight)
    /// according to the Yereim (Rabbi Eliezer of Metz) calculated as 18 minutes
    /// or 3/4 of a 24-minute mil before sunset. According to the Yereim, bein
    /// hashmashos starts 3/4 of a mil before sunset and *tzais* or nightfall
    /// starts at sunset.
    #[must_use]
    pub fn bein_hashmashos_yereim_18_minutes(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(-18.0))
    }

    /// Returns the beginning of *bein hashmashos* (twilight)
    /// according to the Yereim (Rabbi Eliezer of Metz) calculated as the sun's
    /// position 3.05&deg; above the horizon around the equinox / equilux, its
    /// position 18 minutes or 3/4 of an 24-minute mil before sunset. According
    /// to the Yereim, *bein hashmashos* starts 3/4 of a mil before sunset and
    /// *tzais* or nightfall starts at sunset. Note that lechumra (of about 14
    /// seconds) a refraction value of 0.5166&deg; as opposed to the traditional
    /// 0.566&deg; is used. This is more inline with the actual refraction in
    /// Eretz Yisrael and is brought down by Rabbi Yedidya Manet in his
    /// Zmanei Halacha Lema'aseh (p. 11). That is the first source that I am
    /// aware of that calculates degree-based Yereim zmanim. The 0.5166&deg;
    /// refraction is also used by the Luach Itim Lebinah. Calculating the
    /// Yereim's *bein hashmashos* using 18-minute based degrees is also
    /// suggested in the upcoming 8th edition of the zmanim Kehilchasam. For
    /// more details, see the article The Yereim's Bein Hashmashos.
    #[must_use]
    pub fn bein_hashmashos_yereim_3_05_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(-3.05))
    }

    /// Returns the beginning of *bein hashmashos* (twilight)
    /// according to the Yereim (Rabbi Eliezer of Metz) calculated as 16.875
    /// minutes or 3/4 of a 22.5-minute mil before sunset. According to the
    /// Yereim, *bein hashmashos* starts 3/4 of a mil before sunset and *tzais*
    /// or nightfall starts at sunset.
    #[must_use]
    pub fn bein_hashmashos_yereim_16_875_minutes(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(-16.875))
    }

    /// Returns the beginning of *bein hashmashos* (twilight)
    /// according to the Yereim (Rabbi Eliezer of Metz) calculated as the sun's
    /// position 2.8&deg; above the horizon around the equinox / equilux, its
    /// position 16.875 minutes or 3/4 of an 18-minute mil before sunset.
    /// According to the Yereim, *bein hashmashos* starts 3/4 of a mil before
    /// sunset and *tzais* or nightfall starts at sunset. Details, including how
    /// the degrees were calculated can be seen in the documentation of
    /// [`bein_hashmashos_yereim_3_05_degrees`](ComplexZmanimCalendar::bein_hashmashos_yereim_3_05_degrees).
    #[must_use]
    pub fn bein_hashmashos_yereim_2_8_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(-2.8))
    }

    /// Returns the beginning of *bein hashmashos* (twilight)
    /// according to the Yereim (Rabbi Eliezer of Metz) calculated as 13.5
    /// minutes or 3/4 of an 18-minute mil before sunset. According to the
    /// Yereim, *bein hashmashos* starts 3/4 of a mil before sunset and *tzais*
    /// or nightfall starts at sunset.
    #[must_use]
    pub fn bein_hashmashos_yereim_13_5_minutes(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(-13.5))
    }

    /// Returns the beginning of *bein hashmashos* according to the
    /// Yereim (Rabbi Eliezer of Metz) calculated as the sun's position 2.1&deg;
    /// above the horizon around the equinox / equilux in Yerushalayim, its
    /// position 13.5 minutes or 3/4 of an 18-minute mil before sunset.
    /// According to the Yereim, *bein hashmashos* starts 3/4 of a mil before
    /// sunset and *tzais* or nightfall starts at sunset. Details, including how
    /// the degrees were calculated can be seen in the documentation of
    /// [`bein_hashmashos_yereim_3_05_degrees`](ComplexZmanimCalendar::bein_hashmashos_yereim_3_05_degrees).
    #[must_use]
    pub fn bein_hashmashos_yereim_2_1_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(-2.1))
    }

    // Bein Hashmashos Rabeinu Tam
    /// Method to return the beginning of *bein hashmashos* of Rabbeinu Tam
    /// calculated when the sun is 13.24&deg; below the western geometric
    /// horizon (90&deg;) after sunset. This calculation is based on the
    /// same calculation of [*bein hashmashos* Rabbeinu Tam
    /// 58.5](ComplexZmanimCalendar::bein_hashmashos_rt_58_5_minutes) minutes
    /// but uses a degree-based calculation instead of 58.5 exact minutes.
    /// This calculation is based on the position of the sun 58.5 minutes
    /// after sunset in Jerusalem around the equinox / equilux, which
    /// calculates to 13.24&deg; below geometric zenith. NOTE: As per
    /// Yisrael Vehazmanim Vol. III page 1028, No. 50, a dip of slightly
    /// less than 13&deg; should be used. Calculations show that the proper
    /// dip to be 13.2456&deg; (truncated to 13.24 that provides about 1.5
    /// second earlier (*lechumra*) time) below the horizon at that time. This
    /// makes a difference of 1 minute and 10 seconds in Jerusalem during the
    /// Equinox, and 1 minute 29 seconds during the solstice as compared to the
    /// proper 13.24&deg; versus 13&deg;. For NY during the solstice, the
    /// difference is 1 minute 56 seconds.
    #[must_use]
    pub fn bein_hashmashos_rt_13_24_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(13.24))
    }

    /// Returns the beginning of *bein hashmashos* of Rabbeinu Tam
    /// calculated as a 58.5-minute offset after sunset. *bein hashmashos* is
    /// 3/4 of a mil before *tzais* or 3 1/4 mil after sunset. With a mil
    /// calculated as 18 minutes, 3.25 * 18 = 58.5 minutes.
    #[must_use]
    pub fn bein_hashmashos_rt_58_5_minutes(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Minutes(58.5))
    }

    /// Returns the beginning of *bein hashmashos* based on the
    /// calculation of 13.5 minutes (3/4 of an 18-minute mil) before *shkiah*
    /// calculated as 7.083&deg;.
    #[must_use]
    pub fn bein_hashmashos_rt_13_5_minutes_before_7_083_degrees(&self) -> Option<DateTime<Tz>> {
        Some(self.tzais_geonim_7_083_degrees()? - TimeDelta::seconds(810))
    }

    /// Returns the beginning of *bein hashmashos* of Rabbeinu Tam
    /// calculated according to the opinion of the Divrei Yosef (see Yisrael
    /// Vehazmanim) calculated 5/18th (27.77%) of the time between alos
    /// (calculated as 19.8&deg; before sunrise) and sunrise. This is added to
    /// sunset to arrive at the time for *bein hashmashos* of Rabbeinu Tam.
    #[must_use]
    pub fn bein_hashmashos_rt_2_stars(&self) -> Option<DateTime<Tz>> {
        let offset_seconds =
            (self.hanetz()? - self.alos_19_8_degrees()?).as_seconds_f64() * (5.0 / 18.0);
        let offset_delta = TimeDelta::seconds(offset_seconds.trunc() as i64)
            + TimeDelta::nanoseconds((offset_seconds.fract() * 1_000_000_000.0) as i64);
        Some(self.shkia()? + offset_delta)
    }

    // Other Tzais
    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated at the sun's position at 3.7&deg;
    /// below the western horizon. This is a very early *zman* and should not be
    /// relied on without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_3_7_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(3.7))
    }

    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated at the sun's position at 3.8&deg;
    /// below the western horizon. This is a very early *zman* and should not be
    /// relied on without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_3_8_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(3.8))
    }

    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated as 3/4 of a *mil*, based on a
    /// 22.5-minute *mil*, or 16 7/8 minutes. It is the sun's position at
    /// 4.37&deg; below the western horizon. This is a very early *zman* and
    /// should not be relied on without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_4_37_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(4.37))
    }

    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated as 3/4 of a *mil* based on a
    /// 24-minute *mil*, or 18 minutes. It is the sun's position at 4.61&deg;
    /// below the western horizon. This is a very early *zman* and should
    /// not be relied on without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_4_61_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(4.61))
    }

    /// Returns the *tzais* (nightfall) based on the opinion of the
    /// *Geonim* calculated as 3/4 of a *mil* based on the sun's position at
    /// 4.8&deg; below the western horizon. This is based on Rabbi Leo Levi's
    /// calculations. This is a very early *zman* and should not be relied on
    /// without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_4_8_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(4.8))
    }

    /// Returns the *tzais* (nightfall) based on the opinion of the
    /// *Geonim* calculated as 3/4 of a 24-minute *mil*, based on a mil being 24
    /// minutes, and is calculated as 18 + 2 + 4 for a total of 24 minutes. It
    /// is the sun's position at 5.88&deg; below the western horizon. This is a
    /// very early *zman* and should not be relied on without Rabbinical
    /// guidance.
    #[must_use]
    pub fn tzais_geonim_5_88_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(5.88))
    }

    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated at the sun's position at 5.95&deg;
    /// below the western horizon. This is a very early *zman* and should not be
    /// relied on without Rabbinical guidance.
    #[must_use]
    pub fn tzais_geonim_5_95_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(5.95))
    }

    /// Returns the *tzais* (nightfall) based on the opinion of the
    /// *Geonim* as calculated by Rabbi Yechiel Michel Tucazinsky. It is based
    /// on of the position of the sun no later than 31 minutes after sunset
    /// in Jerusalem the height of the summer solstice and is 28 minutes
    /// after *shkiah* around the equinox / equilux. This computes to 6.45&deg;
    /// below the western horizon.
    #[must_use]
    pub fn tzais_geonim_6_45_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(6.45))
    }

    /// Returns the *tzais hakochavim* (nightfall) based on the
    /// opinion of the *Geonim* calculated when the sun's position 7.083&deg;
    /// (or 7&deg; 5′) below the western horizon. This is often referred to
    /// as 7&deg;5' or 7&deg; and 5 minutes. This calculation is based on
    /// the observation of 3 medium-sized stars by Dr. Baruch (Berthold)
    /// Cohn in his luach Tabellen enthaltend die Zeitangaben für den Beginn
    /// der Nacht und des Tages für die Breitengrade + 66 bis -38 published
    /// in Strasbourg, France in 1899. This calendar was very popular in
    /// Europe, and many other calendars based their time on it. Rav Dovid
    /// Tzvi Hoffman in his *Sh"Ut Melamed Leho'il* in an exchange of
    /// letters with Baruch Cohn in *Orach Chaim* 30 agreed to this *zman*
    /// (page 36), as did the *Sh"Ut Bnei Tziyon* and the *Tenuvas
    /// Sadeh*. It is very close to the time of the *Mekor Chesed* of the *Sefer
    /// chasidim*. It is close to the position of the sun 30 minutes after
    /// sunset in Jerusalem around the equinox / equilux, but not Exactly. The
    /// actual position of the sun 30 minutes after sunset in Jerusalem at the
    /// equilux is 7.205&deg; and 7.199&deg; at the equinox. See *Hazmanim
    /// Bahalacha* vol 2, pages 520-521 for more details.
    #[must_use]
    pub fn tzais_geonim_7_083_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(7.083))
    }

    /// Returns *tzais* (nightfall) based on the opinion of the
    /// Geonim calculated as 45 minutes after sunset during the summer solstice
    /// in New York, when the *neshef* (twilight) is the longest. The sun's
    /// position at this time computes to 7.75&deg; below the western horizon.
    /// See Igros Moshe Even Haezer 4, Ch. 4 (regarding *tzais* for *krias
    /// Shema*). It is also mentioned in Rabbi Heber's *Shaarei Zmanim* on in
    /// chapter 10 (page 87) and chapter 12 (page 108). Also see the time of 45
    /// minutes in Rabbi Simcha Bunim Cohen's The radiance of Shabbos as the
    /// earliest *zman* for New York. This *zman* is also listed in the *Divrei
    /// Shalom* Vol. III, chapter 75, and *Bais Av"i* Vol. III, chapter 117.
    /// This *zman* is also listed in the *Divrei Shalom* etc. chapter 177
    /// (FIXME - could not be located). Since this *zman* depends on the level
    /// of light, Rabbi Yaakov Shakow presented this degree-based calculation to
    /// Rabbi Rabbi Shmuel Kamenetsky who agreed to it.
    #[must_use]
    pub fn tzais_geonim_7_67_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(7.67))
    }

    /// Returns *tzais* (nightfall) when the sun is 8.5&deg; below the geometric
    /// horizon (90&deg;) after sunset, a time that Rabbi Meir Posen in his the
    /// *Ohr Meir* calculated that 3 small stars are visible, which is later
    /// than the required 3 medium stars. This calculation is based on the sun's
    /// position below the horizon 36 minutes after sunset in Jerusalem around
    /// the equinox / equilux.
    #[must_use]
    pub fn tzais_geonim_8_5_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(8.5))
    }

    /// Returns the *tzais* (nightfall) based on the calculations
    /// used in the *Luach Itim Lebinah* as the stringent time for *tzais*. It
    /// is calculated at the sun's position at 9.3&deg; below the western
    /// horizon.
    #[must_use]
    pub fn tzais_geonim_9_3_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(9.3))
    }

    /// Returns the *tzais* (nightfall) based on the opinion of the
    /// *Geonim* calculated as 60 minutes after sunset around the equinox /
    /// equilux, the day that a solar hour is 60 minutes in New York. The sun's
    /// position at this time computes to 9.75&deg; below the western horizon.
    /// This is the opinion of Rabbi Eliyahu Henkin. This also follows the
    /// opinion of Rabbi Shmuel Kamenetsky. Rabbi Yaakov Shakow presented
    /// these degree-based times to Rabbi Shmuel Kamenetsky who agreed to
    /// them.
    #[must_use]
    pub fn tzais_geonim_9_75_degrees(&self) -> Option<DateTime<Tz>> {
        self.tzais(&Degrees(9.75))
    }
}

/// When to use elevation for *zmanim* calculations. See the documentation of
/// [`zmanim_calculator`] for some discussion of this
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseElevation {
    /// Never use elevation
    No,
    /// Only use elevation directly for *hanetz* and *shkia*
    HanetzShkia,
    /// Always use elevation
    All,
}

impl UseElevation {
    /// Convert the `UseElevation` into a `bool` for
    /// [`zmanim_calculator`] functions. The param
    /// `hanetz_or_shkia` should be `true` if the calling function is
    /// calculating *hanetz* or *shkia*, and `false` otherwise
    #[must_use]
    pub const fn to_bool(&self, hanetz_or_shkia: bool) -> bool {
        match &self {
            Self::No => false,
            Self::HanetzShkia => hanetz_or_shkia,
            Self::All => true,
        }
    }
}
