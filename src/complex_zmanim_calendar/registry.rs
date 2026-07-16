//! A registry enumerating every zero-argument *zman* accessor on
//! [`ComplexZmanimCalendar`], for callers (such as a CLI) that need to look up
//! *zmanim* by name or iterate over all of them.
//!
//! [`ALL_ZMANIM`] is a compile-time table of [`ZmanEntry`] values. Each entry
//! pairs a *zman*'s method name with a function pointer that invokes the
//! corresponding accessor. Because those function pointers call the real
//! methods, a renamed or removed method is a compile error. A companion test
//! (`tests/registry_completeness.rs`) guards against *forgetting* to list a
//! method here.

use super::ComplexZmanimCalendar;
use jiff::{SignedDuration, Zoned};

/// Whether a *zman* is an instant in time or a duration.
///
/// Most *zmanim* are instants ([`Time`](ZmanKind::Time)). The *shaah zmanis*
/// ("temporal hour") *zmanim* are durations ([`Duration`](ZmanKind::Duration)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZmanKind {
    /// An instant in time, returned as a [`Zoned`].
    Time,
    /// A length of time, returned as a [`SignedDuration`].
    Duration,
}

/// The computed value of a *zman*: either an instant or a duration.
#[derive(Debug, Clone)]
pub enum ZmanValue {
    /// An instant in time (e.g. sunrise, *tzeis*).
    Time(Zoned),
    /// A length of time (a *shaah zmanis*).
    Duration(SignedDuration),
}

/// A single entry in the [`ALL_ZMANIM`] registry, describing one *zman*
/// accessor by name and providing a function to compute it.
#[derive(Debug, Clone, Copy)]
pub struct ZmanEntry {
    /// The name of the accessor method, e.g. `"sof_zman_shema_gra"`.
    pub name: &'static str,
    /// Whether this *zman* is an instant or a duration.
    pub kind: ZmanKind,
    /// Computes the *zman* for the given calendar, returning [`None`] when the
    /// underlying solar event does not occur (e.g. in polar regions).
    pub compute: fn(&ComplexZmanimCalendar) -> Option<ZmanValue>,
}

/// Looks up a [`ZmanEntry`] by its exact method name.
///
/// Returns [`None`] if no *zman* has that name.
#[must_use]
pub fn find_zman(name: &str) -> Option<&'static ZmanEntry> {
    ALL_ZMANIM.iter().find(|entry| entry.name == name)
}

/// Expands a grouped list of method names into the [`ALL_ZMANIM`] table. Each
/// name becomes a [`ZmanEntry`] whose `compute` closure calls the method of the
/// same name, so a stale name fails to compile.
macro_rules! zman_registry {
    ( times: [ $($t:ident),* $(,)? ], durations: [ $($d:ident),* $(,)? ] $(,)? ) => {
        /// Every zero-argument *zman* accessor on [`ComplexZmanimCalendar`].
        ///
        /// Time *zmanim* come first, then duration *zmanim*. Within each group
        /// the order follows the source order of the accessor definitions.
        pub const ALL_ZMANIM: &[ZmanEntry] = &[
            $(
                ZmanEntry {
                    name: stringify!($t),
                    kind: ZmanKind::Time,
                    compute: |czc| czc.$t().map(ZmanValue::Time),
                },
            )*
            $(
                ZmanEntry {
                    name: stringify!($d),
                    kind: ZmanKind::Duration,
                    compute: |czc| czc.$d().map(ZmanValue::Duration),
                },
            )*
        ];
    };
}

zman_registry! {
    times: [
        // hand-written (Option<Zoned>)
        sea_level_sunrise,
        sea_level_sunset,
        elevation_sunrise,
        elevation_sunset,
        hanetz,
        shkia,
        chatzos_hayom,
        chatzos_hayom_as_half_day,
        chatzos_halayla,
        mincha_gedola_30_minutes,
        sof_zman_shema_gra,
        sof_zman_tefila_gra,
        sof_zman_biur_chametz_gra,
        mincha_gedola_gra,
        mincha_gedola_gra_greater_than_30_minutes,
        samuch_lemincha_ketana_gra,
        mincha_ketana_gra,
        plag_gra,
        alos_baal_hatanya,
        hanetz_amiti_baal_hatanya,
        shkia_amiti_baal_hatanya,
        sof_zman_shema_baal_hatanya,
        sof_zman_tefila_baal_hatanya,
        sof_zman_biur_chametz_baal_hatanya,
        mincha_gedola_baal_hatanya,
        mincha_gedola_baal_hatanya_greater_than_30_minutes,
        mincha_ketana_baal_hatanya,
        plag_baal_hatanya,
        tzeis_baal_hatanya,
        fixed_local_chatzos_hayom,
        sof_zman_shema_mga_alos_18_to_fixed_local_chatzos,
        sof_zman_shema_mga_alos_16_1_to_fixed_local_chatzos,
        sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos,
        sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos,
        sof_zman_shema_gra_sunrise_to_fixed_local_chatzos,
        sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos,
        mincha_gedola_gra_fixed_local_chatzos_30_minutes,
        mincha_ketana_gra_fixed_local_chatzos_to_sunset,
        plag_gra_fixed_local_chatzos_to_sunset,
        tzeis_50_minutes,
        mincha_gedola_ahavat_shalom,
        mincha_ketana_ahavat_shalom,
        plag_ahavat_shalom,
        sof_zman_shema_ateret_torah,
        sof_zman_tefila_ateret_torah,
        mincha_gedola_ateret_torah,
        mincha_ketana_ateret_torah,
        plag_ateret_torah,
        tzeis_ateret_torah,
        polar_sunrise_ben_ish_chai,
        polar_plag_ben_ish_chai,
        polar_sunset_ben_ish_chai,
        sof_zman_shema_3_hrs_before_chatzos,
        sof_zman_tefila_2_hrs_before_chatzos,
        sof_zman_shema_alos_16_1_to_sunset,
        plag_alos_16_1_to_sunset,
        sof_zman_shema_alos_16_1_to_tzeis_7_083,
        plag_alos_16_1_to_tzeis_7_083,
        alos_19_degrees,
        misheyakir_12_85_degrees,
        misheyakir_11_5_degrees,
        misheyakir_11_degrees,
        misheyakir_10_2_degrees,
        misheyakir_9_5_degrees,
        misheyakir_7_65_degrees,
        bein_hashmashos_yereim_18_minutes,
        bein_hashmashos_yereim_3_05_degrees,
        bein_hashmashos_yereim_16_875_minutes,
        bein_hashmashos_yereim_2_8_degrees,
        bein_hashmashos_yereim_13_5_minutes,
        bein_hashmashos_yereim_2_1_degrees,
        bein_hashmashos_rt_13_24_degrees,
        bein_hashmashos_rt_58_5_minutes,
        bein_hashmashos_rt_13_5_minutes_before_7_083_degrees,
        bein_hashmashos_rt_2_stars,
        tzeis_geonim_3_7_degrees,
        tzeis_geonim_3_8_degrees,
        tzeis_geonim_4_42_degrees,
        tzeis_geonim_4_66_degrees,
        tzeis_geonim_4_8_degrees,
        tzeis_geonim_5_95_degrees,
        tzeis_geonim_6_45_degrees,
        tzeis_geonim_7_083_degrees,
        tzeis_geonim_7_67_degrees,
        tzeis_geonim_8_5_degrees,
        tzeis_geonim_9_3_degrees,
        tzeis_geonim_9_75_degrees,
        // macro-generated (zmanim_for_offset!)
        alos_16_1_degrees,
        tzeis_16_1_degrees,
        sof_zman_shema_mga_16_1_degrees,
        sof_zman_tefila_mga_16_1_degrees,
        sof_zman_biur_chametz_mga_16_1_degrees,
        mincha_gedola_mga_16_1_degrees,
        samuch_lemincha_ketana_mga_16_1_degrees,
        mincha_ketana_mga_16_1_degrees,
        plag_mga_16_1_degrees,
        alos_18_degrees,
        tzeis_18_degrees,
        sof_zman_shema_mga_18_degrees,
        sof_zman_tefila_mga_18_degrees,
        plag_mga_18_degrees,
        alos_19_8_degrees,
        tzeis_19_8_degrees,
        sof_zman_shema_mga_19_8_degrees,
        sof_zman_tefila_mga_19_8_degrees,
        plag_mga_19_8_degrees,
        alos_26_degrees,
        tzeis_26_degrees,
        plag_mga_26_degrees,
        alos_60_minutes,
        tzeis_60_minutes,
        plag_mga_60_minutes,
        alos_72_minutes,
        tzeis_72_minutes,
        sof_zman_shema_mga_72_minutes,
        sof_zman_tefila_mga_72_minutes,
        sof_zman_biur_chametz_mga_72_minutes,
        mincha_gedola_mga_72_minutes,
        samuch_lemincha_ketana_mga_72_minutes,
        mincha_ketana_mga_72_minutes,
        plag_mga_72_minutes,
        alos_72_minutes_zmanis,
        tzeis_72_minutes_zmanis,
        sof_zman_shema_mga_72_minutes_zmanis,
        sof_zman_tefila_mga_72_minutes_zmanis,
        sof_zman_biur_chametz_mga_72_minutes_zmanis,
        plag_mga_72_minutes_zmanis,
        alos_90_minutes,
        tzeis_90_minutes,
        sof_zman_shema_mga_90_minutes,
        sof_zman_tefila_mga_90_minutes,
        plag_mga_90_minutes,
        alos_90_minutes_zmanis,
        tzeis_90_minutes_zmanis,
        sof_zman_shema_mga_90_minutes_zmanis,
        sof_zman_tefila_mga_90_minutes_zmanis,
        plag_mga_90_minutes_zmanis,
        alos_96_minutes,
        tzeis_96_minutes,
        sof_zman_shema_mga_96_minutes,
        sof_zman_tefila_mga_96_minutes,
        plag_mga_96_minutes,
        alos_96_minutes_zmanis,
        tzeis_96_minutes_zmanis,
        sof_zman_shema_mga_96_minutes_zmanis,
        sof_zman_tefila_mga_96_minutes_zmanis,
        plag_mga_96_minutes_zmanis,
        alos_120_minutes,
        tzeis_120_minutes,
        sof_zman_shema_mga_120_minutes,
        sof_zman_tefila_mga_120_minutes,
        plag_mga_120_minutes,
        alos_120_minutes_zmanis,
        tzeis_120_minutes_zmanis,
        plag_mga_120_minutes_zmanis,
    ],
    durations: [
        // hand-written (Option<SignedDuration>)
        shaah_zmanis_gra,
        shaah_zmanis_baal_hatanya,
        shaah_zmanis_alos_16_1_to_tzeis_3_8,
        shaah_zmanis_alos_16_1_to_tzeis_3_7,
        shaah_zmanis_ateret_torah,
        shaah_zmanis_alos_16_1_to_tzeis_7_083,
        // macro-generated (zmanim_for_offset!)
        shaah_zmanis_mga_16_1_degrees,
        shaah_zmanis_mga_18_degrees,
        shaah_zmanis_mga_19_8_degrees,
        shaah_zmanis_mga_26_degrees,
        shaah_zmanis_mga_60_minutes,
        shaah_zmanis_mga_72_minutes,
        shaah_zmanis_mga_72_minutes_zmanis,
        shaah_zmanis_mga_90_minutes,
        shaah_zmanis_mga_90_minutes_zmanis,
        shaah_zmanis_mga_96_minutes,
        shaah_zmanis_mga_96_minutes_zmanis,
        shaah_zmanis_mga_120_minutes,
        shaah_zmanis_mga_120_minutes_zmanis,
    ]
}
