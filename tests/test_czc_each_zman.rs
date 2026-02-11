//! Test every method in
//! [rust_zmanim::complex_zmanim_calendar::ComplexZmanimCalendar]. All expected
//! times were taken from KosherJava's `ComprehensiveZmanimCalendar.java`. I
//! created the tests by doing a lot of strange regexes on the output of a Java
//! method that printed results from every method in
//! `ComprehensiveZmanimCalendar`, so the order of the tests does not correspond
//! to my order in `complex_zmanim_calendar.rs`. The tests only check to the
//! second, because for the fractional seconds I would have had to update each
//! one to the greater precision (!= accuracy) over KosherJava offered by this
//! library

mod test_helper;
use rust_zmanim::prelude::ZmanOffset;

#[test]
pub fn test_alos_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_19_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:13:59 IDT"
    )
}

#[test]
pub fn test_tzais_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_19_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:34:15 IDT"
    )
}

#[test]
pub fn test_alos_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_18_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:22:27 IDT"
    )
}

#[test]
pub fn test_tzais_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_18_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:25:47 IDT"
    )
}

#[test]
pub fn test_alos_26_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_26_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 04:44:44 IDT"
    )
}

#[test]
pub fn test_tzais_26_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_26_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 20:03:26 IDT"
    )
}

#[test]
pub fn test_alos_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:31:24 IDT"
    )
}

#[test]
pub fn test_tzais_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:16:51 IDT"
    )
}

#[test]
pub fn test_alos_60_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_60_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:43:43 IDT"
    )
}

#[test]
pub fn test_tzais_60_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_60_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:04:36 IDT"
    )
}

#[test]
pub fn test_alos_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:35:37 IDT"
    )
}

#[test]
pub fn test_tzais_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:12:41 IDT"
    )
}

#[test]
pub fn test_alos_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:13:43 IDT"
    )
}

#[test]
pub fn test_tzais_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:34:36 IDT"
    )
}

#[test]
pub fn test_alos_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:18:36 IDT"
    )
}

#[test]
pub fn test_tzais_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:29:42 IDT"
    )
}

#[test]
pub fn test_alos_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:12:55 IDT"
    )
}

#[test]
pub fn test_tzais_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:35:23 IDT"
    )
}

#[test]
pub fn test_tzais_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:44:36 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_3_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_3_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:18:45 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_3_7_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_3_7_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:18:16 IDT"
    )
}

#[test]
pub fn test_alos_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:07:43 IDT"
    )
}

#[test]
pub fn test_tzais_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:40:36 IDT"
    )
}

#[test]
pub fn test_alos_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 04:43:43 IDT"
    )
}

#[test]
pub fn test_tzais_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 20:04:36 IDT"
    )
}

#[test]
pub fn test_alos_120_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 04:50:14 IDT"
    )
}

#[test]
pub fn test_tzais_120_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:58:05 IDT"
    )
}

#[test]
pub fn test_plag_120_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:23:30 IDT"
    )
}

#[test]
pub fn test_plag_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:28:40 IDT"
    )
}

#[test]
pub fn test_alos_19_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_19_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:17:45 IDT"
    )
}

#[test]
pub fn test_misheyakir_12_85_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_12_85_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:46:42 IDT"
    )
}

#[test]
pub fn test_misheyakir_11_5_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_11_5_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:53:04 IDT"
    )
}

#[test]
pub fn test_misheyakir_11_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_11_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:55:26 IDT"
    )
}

#[test]
pub fn test_misheyakir_10_2_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_10_2_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:59:13 IDT"
    )
}

#[test]
pub fn test_misheyakir_7_65_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_7_65_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 06:11:16 IDT"
    )
}

#[test]
pub fn test_misheyakir_9_5_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.misheyakir_9_5_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 06:02:31 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_19_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:49:03 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:57:46 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_18_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:53:17 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:57:56 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:59:53 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:48:56 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:51:23 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:45:56 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:48:32 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_3_hrs_before_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_3_hrs_before_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:24:28 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:33:56 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_alos_16_1_degrees_to_sunset() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_alos_16_1_degrees_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:39:42 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_alos_16_1_degrees_to_tzais_geonim_7_083_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:47:08 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_7_083_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_7_083_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:34:20 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_19_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:00:44 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:06:33 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_18_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:03:34 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:06:40 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:07:58 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:00:40 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:02:18 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:58:40 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:00:25 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:50:40 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_2_hrs_before_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_2_hrs_before_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:24:28 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_30_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:54:28 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:58:31 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:58:31 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_ahavat_shalom() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:56:26 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_gra_greater_than_30_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_gra_greater_than_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:54:28 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:24:53 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_ahavat_shalom() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:38:53 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:24:45 IDT"
    )
}

#[test]
pub fn test_plag_mga_60_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_60_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:41:10 IDT"
    )
}

#[test]
pub fn test_plag_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:50:40 IDT"
    )
}

#[test]
pub fn test_plag_mga_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:04:55 IDT"
    )
}

#[test]
pub fn test_plag_mga_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:09:40 IDT"
    )
}

#[test]
pub fn test_plag_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:05:32 IDT"
    )
}

#[test]
pub fn test_plag_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:01:03 IDT"
    )
}

#[test]
pub fn test_plag_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:47:34 IDT"
    )
}

#[test]
pub fn test_plag_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:50:52 IDT"
    )
}

#[test]
pub fn test_plag_mga_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_19_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:04:38 IDT"
    )
}

#[test]
pub fn test_plag_mga_26_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_26_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:27:44 IDT"
    )
}

#[test]
pub fn test_plag_mga_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga_18_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:57:56 IDT"
    )
}

#[test]
pub fn test_plag_alos_16_1_degrees_to_sunset() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_alos_16_1_degrees_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:46:08 IDT"
    )
}

#[test]
pub fn test_plag_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_alos_16_1_degrees_to_tzais_geonim_7_083_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:12:46 IDT"
    )
}

#[test]
pub fn test_plag_ahavat_shalom() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:58:49 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_13_24_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_13_24_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:03:23 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_58_5_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_58_5_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:03:06 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_13_5_minutes_before_7_083_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_13_5_minutes_before_7_083_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:20:50 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_2_stars() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_2_stars()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:29:31 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_18_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_18_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:46:36 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_3_05_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_3_05_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:45:56 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_16_875_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_16_875_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:47:43 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_2_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_2_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:47:09 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_13_5_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_13_5_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:51:06 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_2_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_2_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:50:31 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_5_95_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_5_95_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:28:57 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_4_61_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_4_61_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:22:36 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_4_37_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_4_37_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:21:27 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_5_88_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_5_88_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:28:38 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_4_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_4_8_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:23:30 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_6_45_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_6_45_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:31:20 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_7_67_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_7_67_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:37:06 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_8_5_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_8_5_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:41:02 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_9_3_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_9_3_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:44:49 IDT"
    )
}

#[test]
pub fn test_tzais_geonim_9_75_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_geonim_9_75_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:46:56 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:52:52 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:58:37 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:42:59 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:00:13 IDT"
    )
}

#[test]
pub fn test_plag_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:22:25 IDT"
    )
}

#[test]
pub fn test_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:39:03 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:27:25 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:15:25 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:16:04 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:15:20 IDT"
    )
}

#[test]
pub fn test_alos_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:27:38 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:32:08 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:29:28 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:26:49 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:52:49 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_baal_hatanya_greater_than_30_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_baal_hatanya_greater_than_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:54:28 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:44:50 IDT"
    )
}

#[test]
pub fn test_plag_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:56:30 IDT"
    )
}

#[test]
pub fn test_tzais_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_baal_hatanya()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:29:12 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:00:45 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:05:14 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:56:23 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:05:23 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_gra_sunrise_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_gra_sunrise_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:41:23 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:40:36 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_gra_fixed_local_chatzos_30_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_gra_fixed_local_chatzos_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 13:09:03 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_gra_fixed_local_chatzos_to_sunset() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_gra_fixed_local_chatzos_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:48:57 IDT"
    )
}

#[test]
pub fn test_plag_gra_fixed_local_chatzos_to_sunset() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_gra_fixed_local_chatzos_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:56:46 IDT"
    )
}

#[test]
pub fn test_tzais_50_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_50_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:54:36 IDT"
    )
}

#[test]
pub fn test_samuch_lemincha_ketana_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.samuch_lemincha_ketana_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:14:22 IDT"
    )
}

#[test]
pub fn test_samuch_lemincha_ketana_mga_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.samuch_lemincha_ketana_mga_16_1_degrees()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:50:29 IDT"
    )
}

#[test]
pub fn test_samuch_lemincha_ketana_mga_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.samuch_lemincha_ketana_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:50:22 IDT"
    )
}

#[test]
pub fn test_plag_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:53:40 IDT"
    )
}

#[test]
pub fn test_chatzos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:24:28 IDT"
    )
}

#[test]
pub fn test_tzais_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:16:36 IDT"
    )
}

#[test]
pub fn test_alos_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:31:43 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:52:31 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:42:45 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:30:40 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:33:56 IDT"
    )
}

#[test]
pub fn test_shaah_zmanis_gra() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(czc.shaah_zmanis_gra().unwrap().num_milliseconds(), 3404427)
}

#[test]
pub fn test_shaah_zmanis_19_8_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_19_8_degrees().unwrap().num_milliseconds(),
        4301331
    )
}

#[test]
pub fn test_shaah_zmanis_16_1_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_16_1_degrees().unwrap().num_milliseconds(),
        4127245
    )
}

#[test]
pub fn test_shaah_zmanis_72_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_72_minutes_zmanis()
            .unwrap()
            .num_milliseconds(),
        4085312
    )
}

#[test]
pub fn test_shaah_zmanis_90_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_90_minutes_zmanis()
            .unwrap()
            .num_milliseconds(),
        4255534
    )
}

#[test]
pub fn test_shaah_zmanis_18_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_18_degrees().unwrap().num_milliseconds(),
        4216641
    )
}

#[test]
pub fn test_shaah_zmanis_26_degrees() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_26_degrees().unwrap().num_milliseconds(),
        4593504
    )
}

#[test]
pub fn test_shaah_zmanis_60_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_60_minutes().unwrap().num_milliseconds(),
        4004427
    )
}

#[test]
pub fn test_shaah_zmanis_72_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_72_minutes().unwrap().num_milliseconds(),
        4124427
    )
}

#[test]
pub fn test_shaah_zmanis_90_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_90_minutes().unwrap().num_milliseconds(),
        4304427
    )
}

#[test]
pub fn test_shaah_zmanis_96_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_96_minutes_zmanis()
            .unwrap()
            .num_milliseconds(),
        4312274
    )
}

#[test]
pub fn test_shaah_zmanis_ateret_torah() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_ateret_torah().unwrap().num_milliseconds(),
        3944870
    )
}

#[test]
pub fn test_shaah_zmanis_alos_16_1_to_tzais_3_8() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_alos_16_1_to_tzais_3_8()
            .unwrap()
            .num_milliseconds(),
        3836747
    )
}

#[test]
pub fn test_shaah_zmanis_alos_16_1_to_tzais_3_7() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_alos_16_1_to_tzais_3_7()
            .unwrap()
            .num_milliseconds(),
        3834368
    )
}

#[test]
pub fn test_shaah_zmanis_96_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_96_minutes().unwrap().num_milliseconds(),
        4364427
    )
}

#[test]
pub fn test_shaah_zmanis_120_minutes() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_120_minutes().unwrap().num_milliseconds(),
        4604427
    )
}

#[test]
pub fn test_shaah_zmanis_120_minutes_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_120_minutes_zmanis()
            .unwrap()
            .num_milliseconds(),
        4539236
    )
}

#[test]
pub fn test_shaah_zmanis_baal_hatanya() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis_baal_hatanya().unwrap().num_milliseconds(),
        3440274
    )
}

#[test]
pub fn test_alos() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.alos(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:13:59 IDT"
    )
}

#[test]
pub fn test_tzais() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.tzais(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:34:15 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_shema_mga(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:49:03 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:00:44 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_mga() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_gedola_mga(&ZmanOffset::Degrees(16.1))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:58:31 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_mga() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.mincha_ketana_mga(&ZmanOffset::Degrees(16.1))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:24:53 IDT"
    )
}

#[test]
pub fn test_plag_mga() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.plag_mga(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:04:38 IDT"
    )
}

#[test]
pub fn test_shaah_zmanis() {
    let czc = test_helper::czc_test_sample();
    assert_eq!(
        czc.shaah_zmanis(&ZmanOffset::Degrees(19.8))
            .unwrap()
            .num_milliseconds(),
        4301331
    )
}
