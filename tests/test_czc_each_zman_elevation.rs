//! Test every method in
//! [rust_zmanim::complex_zmanim_calendar::ComplexZmanimCalendar], with
//! elevation-adjusted calculations.

mod test_helper;

// not yet implemented...
// #[test]
// pub fn test_chatzos_halayla() {
//     let czc = test_helper::czc_test_elevation_sample();
//     assert_eq!(
//         czc.chatzos_halayla()
//             .unwrap()
//             .format("%Y-%m-%d %H:%M:%S %Z")
//             .to_string(),
//         "2017-10-18 00:25:10 IDT"
//     )
// }

#[test]
pub fn test_alos_60_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_60_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:40:47 IDT"
    )
}

#[test]
pub fn test_alos_72_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:31:58 IDT"
    )
}

#[test]
pub fn test_tzais_72_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:17:55 IDT"
    )
}

#[test]
pub fn test_alos_90_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:10:47 IDT"
    )
}

#[test]
pub fn test_alos_90_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:14:45 IDT"
    )
}

#[test]
pub fn test_tzais_90_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:35:08 IDT"
    )
}

#[test]
pub fn test_alos_96_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:09:01 IDT"
    )
}

#[test]
pub fn test_tzais_96_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:40:52 IDT"
    )
}

#[test]
pub fn test_tzais_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:49:05 IDT"
    )
}

#[test]
pub fn test_alos_96_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:04:47 IDT"
    )
}

#[test]
pub fn test_tzais_96_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:45:05 IDT"
    )
}

#[test]
pub fn test_alos_120_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 04:46:04 IDT"
    )
}

#[test]
pub fn test_tzais_120_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 20:03:48 IDT"
    )
}

#[test]
pub fn test_plag_mga_120_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_120_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:28:12 IDT"
    )
}

#[test]
pub fn test_plag_mga_120_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:32:23 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:56:52 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:58:27 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:47:52 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:49:51 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_96_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:44:52 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:46:59 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_3_hrs_before_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_3_hrs_before_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:25:16 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_120_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:32:52 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:06:13 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:07:17 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_90_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:00:13 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:01:33 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_96_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:58:13 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:59:38 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_mga_120_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_mga_120_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:50:13 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_2_hrs_before_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_2_hrs_before_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:25:16 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_30_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:55:16 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:59:37 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_ahavat_shalom() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:57:13 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_gra_greater_than_30_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_gra_greater_than_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:55:16 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_ahavat_shalom() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_ketana_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:39:40 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_ketana_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:27:42 IDT"
    )
}

#[test]
pub fn test_plag_mga_60_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_60_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:44:53 IDT"
    )
}

#[test]
pub fn test_plag_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:54:23 IDT"
    )
}

#[test]
pub fn test_plag_mga_90_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_90_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:08:38 IDT"
    )
}

#[test]
pub fn test_plag_mga_96_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_96_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:13:23 IDT"
    )
}

#[test]
pub fn test_plag_mga_96_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_96_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:10:03 IDT"
    )
}

#[test]
pub fn test_plag_mga_90_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_90_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:05:30 IDT"
    )
}

#[test]
pub fn test_plag_mga_72_minutes_zmanis() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_mga_72_minutes_zmanis()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:51:53 IDT"
    )
}

#[test]
pub fn test_plag_ahavat_shalom() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_ahavat_shalom()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:59:35 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_58_5_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_58_5_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:07:35 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_rt_2_stars() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.bein_hashmashos_rt_2_stars()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:32:59 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_18_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_18_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:51:05 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_16_875_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_16_875_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:52:13 IDT"
    )
}

#[test]
pub fn test_bein_hashmashos_yereim_13_5_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.bein_hashmashos_yereim_13_5_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:55:35 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:51:14 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:57:40 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:43:44 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_ketana_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 16:03:01 IDT"
    )
}

#[test]
pub fn test_plag_ateret_torah() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_ateret_torah()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:26:03 IDT"
    )
}

#[test]
pub fn test_fixed_local_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:39:51 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_gra() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:27:35 IDT"
    )
}

#[test]
pub fn test_sof_zman_biur_chametz_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_biur_chametz_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 11:15:35 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 08:55:19 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:04:19 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_gra_sunrise_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_gra_sunrise_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:40:19 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:40:10 IDT"
    )
}

#[test]
pub fn test_mincha_gedola_gra_fixed_local_chatzos_30_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_gedola_gra_fixed_local_chatzos_30_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 13:09:51 IDT"
    )
}

#[test]
pub fn test_mincha_ketana_gra_fixed_local_chatzos_to_sunset() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.mincha_ketana_gra_fixed_local_chatzos_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:51:54 IDT"
    )
}

#[test]
pub fn test_plag_gra_fixed_local_chatzos_to_sunset() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.plag_gra_fixed_local_chatzos_to_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 17:00:30 IDT"
    )
}

#[test]
pub fn test_samuch_lemincha_ketana_gra() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.samuch_lemincha_ketana_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:17:01 IDT"
    )
}

#[test]
pub fn test_samuch_lemincha_ketana_mga_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.samuch_lemincha_ketana_mga_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 15:53:01 IDT"
    )
}

#[test]
pub fn test_chatzos() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.chatzos()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 12:25:16 IDT"
    )
}

#[test]
pub fn test_tzais_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.tzais_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 19:21:05 IDT"
    )
}

#[test]
pub fn test_alos_72_minutes() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.alos_72_minutes()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 05:28:47 IDT"
    )
}

#[test]
pub fn test_sof_zman_tefila_gra() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_tefila_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 10:30:13 IDT"
    )
}

#[test]
pub fn test_sof_zman_shema_gra() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sof_zman_shema_gra()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 09:32:52 IDT"
    )
}

#[test]
pub fn test_elevation_sunset() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.elevation_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:09:05 IDT"
    )
}

#[test]
pub fn test_elevation_sunrise() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.elevation_sunrise()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 06:40:47 IDT"
    )
}

#[test]
pub fn test_sea_level_sunrise() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sea_level_sunrise()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 06:44:31 IDT"
    )
}

#[test]
pub fn test_sea_level_sunset() {
    let czc = test_helper::czc_test_elevation_sample();
    assert_eq!(
        czc.sea_level_sunset()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
        "2017-10-17 18:05:22 IDT"
    )
}
