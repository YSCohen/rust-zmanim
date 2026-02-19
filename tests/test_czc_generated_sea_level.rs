mod test_helper;
use std::iter::zip;

#[test]
fn test_alos_19_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:30:07 EDT",
        "2017-10-17 05:13:59 IDT",
        "2017-10-17 05:29:42 PDT",
        "2017-10-17 04:15:28 JST",
        "None",
        "2017-10-17 05:40:07 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_19_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_19_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:53:34 EDT",
        "2017-10-17 19:34:15 IDT",
        "2017-10-17 19:49:43 PDT",
        "2017-10-17 18:37:32 JST",
        "None",
        "2017-10-17 20:45:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_19_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_18_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:39:33 EDT",
        "2017-10-17 05:22:27 IDT",
        "2017-10-17 05:38:23 PDT",
        "2017-10-17 04:24:20 JST",
        "None",
        "2017-10-17 05:47:48 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_18_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_18_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:44:09 EDT",
        "2017-10-17 19:25:47 IDT",
        "2017-10-17 19:41:02 PDT",
        "2017-10-17 18:28:40 JST",
        "None",
        "2017-10-17 20:37:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_18_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_26_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 04:57:25 EDT",
        "2017-10-17 04:44:44 IDT",
        "2017-10-17 04:59:40 PDT",
        "2017-10-17 03:44:47 JST",
        "None",
        "2017-10-17 05:13:31 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_26_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_26_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 20:26:11 EDT",
        "2017-10-17 20:03:26 IDT",
        "2017-10-17 20:19:41 PDT",
        "2017-10-17 19:08:09 JST",
        "None",
        "2017-10-17 21:12:04 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_26_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:49:30 EDT",
        "2017-10-17 05:31:24 IDT",
        "2017-10-17 05:47:34 PDT",
        "2017-10-17 04:33:42 JST",
        "2017-10-17 02:22:35 EDT",
        "2017-10-17 05:55:52 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:34:14 EDT",
        "2017-10-17 19:16:51 IDT",
        "2017-10-17 19:31:52 PDT",
        "2017-10-17 18:19:19 JST",
        "2017-10-17 21:32:57 EDT",
        "2017-10-17 20:29:38 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_60_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:09:51 EDT",
        "2017-10-17 05:43:43 IDT",
        "2017-10-17 06:01:45 PDT",
        "2017-10-17 04:49:21 JST",
        "None",
        "2017-10-17 06:00:05 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_60_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_60_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:13:58 EDT",
        "2017-10-17 19:04:36 IDT",
        "2017-10-17 19:17:45 PDT",
        "2017-10-17 18:03:45 JST",
        "None",
        "2017-10-17 20:25:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_60_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_72_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:03:26 EDT",
        "2017-10-17 05:35:37 IDT",
        "2017-10-17 05:54:09 PDT",
        "2017-10-17 04:41:55 JST",
        "None",
        "2017-10-17 05:45:34 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_72_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_72_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:20:23 EDT",
        "2017-10-17 19:12:41 IDT",
        "2017-10-17 19:25:21 PDT",
        "2017-10-17 18:11:11 JST",
        "None",
        "2017-10-17 20:39:51 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_72_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_90_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:39:51 EDT",
        "2017-10-17 05:13:43 IDT",
        "2017-10-17 05:31:45 PDT",
        "2017-10-17 04:19:21 JST",
        "None",
        "2017-10-17 05:30:05 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_90_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_90_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:43:58 EDT",
        "2017-10-17 19:34:36 IDT",
        "2017-10-17 19:47:45 PDT",
        "2017-10-17 18:33:45 JST",
        "None",
        "2017-10-17 20:55:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_90_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_90_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:46:50 EDT",
        "2017-10-17 05:18:36 IDT",
        "2017-10-17 05:37:15 PDT",
        "2017-10-17 04:25:03 JST",
        "None",
        "2017-10-17 05:26:56 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_90_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_90_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:36:59 EDT",
        "2017-10-17 19:29:42 IDT",
        "2017-10-17 19:42:15 PDT",
        "2017-10-17 18:28:03 JST",
        "None",
        "2017-10-17 20:58:28 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_90_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_96_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:41:18 EDT",
        "2017-10-17 05:12:55 IDT",
        "2017-10-17 05:31:37 PDT",
        "2017-10-17 04:19:26 JST",
        "None",
        "2017-10-17 05:20:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_96_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_96_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:42:31 EDT",
        "2017-10-17 19:35:23 IDT",
        "2017-10-17 19:47:53 PDT",
        "2017-10-17 18:33:40 JST",
        "None",
        "2017-10-17 21:04:41 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_96_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:53:58 EDT",
        "2017-10-17 18:44:36 IDT",
        "2017-10-17 18:57:45 PDT",
        "2017-10-17 17:43:45 JST",
        "None",
        "2017-10-17 20:05:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_3_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:29:44 EDT",
        "2017-10-17 18:18:45 IDT",
        "2017-10-17 18:32:17 PDT",
        "2017-10-17 17:18:34 JST",
        "2017-10-17 15:10:07 EDT",
        "2017-10-17 19:37:44 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_3_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_3_7_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:29:12 EDT",
        "2017-10-17 18:18:16 IDT",
        "2017-10-17 18:31:48 PDT",
        "2017-10-17 17:18:04 JST",
        "2017-10-17 15:06:14 EDT",
        "2017-10-17 19:37:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_3_7_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_96_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:33:51 EDT",
        "2017-10-17 05:07:43 IDT",
        "2017-10-17 05:25:45 PDT",
        "2017-10-17 04:13:21 JST",
        "None",
        "2017-10-17 05:24:05 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_96_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_96_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:49:58 EDT",
        "2017-10-17 19:40:36 IDT",
        "2017-10-17 19:53:45 PDT",
        "2017-10-17 18:39:45 JST",
        "None",
        "2017-10-17 21:01:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_96_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_120_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:09:51 EDT",
        "2017-10-17 04:43:43 IDT",
        "2017-10-17 05:01:45 PDT",
        "2017-10-17 03:49:21 JST",
        "None",
        "2017-10-17 05:00:05 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_120_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_120_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 20:13:58 EDT",
        "2017-10-17 20:04:36 IDT",
        "2017-10-17 20:17:45 PDT",
        "2017-10-17 19:03:45 JST",
        "None",
        "2017-10-17 21:25:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_120_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_120_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:19:10 EDT",
        "2017-10-17 04:50:14 IDT",
        "2017-10-17 05:09:05 PDT",
        "2017-10-17 03:56:57 JST",
        "None",
        "2017-10-17 04:55:53 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_120_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_120_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 20:04:40 EDT",
        "2017-10-17 19:58:05 IDT",
        "2017-10-17 20:10:25 PDT",
        "2017-10-17 18:56:09 JST",
        "None",
        "2017-10-17 21:29:32 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_120_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_7_083_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:47:03 EDT",
        "2017-10-17 18:34:20 IDT",
        "2017-10-17 18:48:16 PDT",
        "2017-10-17 17:34:52 JST",
        "2017-10-17 16:57:02 EDT",
        "2017-10-17 19:51:31 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_7_083_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_30_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:12:18 EDT",
        "2017-10-17 12:54:28 IDT",
        "2017-10-17 13:10:05 PDT",
        "2017-10-17 11:56:55 JST",
        "2017-10-17 12:34:32 EDT",
        "2017-10-17 13:42:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_30_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:27:14 EDT",
        "2017-10-17 10:06:40 IDT",
        "2017-10-17 10:23:05 PDT",
        "2017-10-17 09:10:09 JST",
        "None",
        "2017-10-17 10:44:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_72_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:29:05 EDT",
        "2017-10-17 10:07:58 IDT",
        "2017-10-17 10:24:33 PDT",
        "2017-10-17 09:11:40 JST",
        "None",
        "2017-10-17 10:43:39 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_72_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:24:24 EDT",
        "2017-10-17 10:06:33 IDT",
        "2017-10-17 10:22:20 PDT",
        "2017-10-17 09:08:55 JST",
        "2017-10-17 08:46:02 EDT",
        "2017-10-17 10:47:07 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:49:54 EDT",
        "2017-10-17 10:29:28 IDT",
        "2017-10-17 10:45:51 PDT",
        "2017-10-17 09:32:54 JST",
        "2017-10-17 11:40:07 EDT",
        "2017-10-17 11:07:27 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:09:55 EDT",
        "2017-10-17 12:52:49 IDT",
        "2017-10-17 13:08:13 PDT",
        "2017-10-17 11:54:58 JST",
        "2017-10-17 12:08:26 EDT",
        "2017-10-17 13:44:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 12:56:57 EDT",
        "2017-10-17 12:39:03 IDT",
        "2017-10-17 12:54:46 PDT",
        "2017-10-17 11:41:26 JST",
        "2017-10-17 12:19:10 EDT",
        "2017-10-17 13:27:12 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_120_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:32:25 EDT",
        "2017-10-17 18:23:30 IDT",
        "2017-10-17 18:36:32 PDT",
        "2017-10-17 17:22:29 JST",
        "None",
        "2017-10-17 19:46:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_120_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_120_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:39:48 EDT",
        "2017-10-17 18:28:40 IDT",
        "2017-10-17 18:42:20 PDT",
        "2017-10-17 17:28:30 JST",
        "None",
        "2017-10-17 19:42:42 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_120_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_19_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:34:19 EDT",
        "2017-10-17 05:17:45 IDT",
        "2017-10-17 05:33:33 PDT",
        "2017-10-17 04:19:25 JST",
        "None",
        "2017-10-17 05:43:32 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_19_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_12_85_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:06:30 EDT",
        "2017-10-17 05:46:42 IDT",
        "2017-10-17 06:03:16 PDT",
        "2017-10-17 04:49:43 JST",
        "2017-10-17 04:21:54 EDT",
        "2017-10-17 06:09:38 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_12_85_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_11_5_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:13:34 EDT",
        "2017-10-17 05:53:04 IDT",
        "2017-10-17 06:09:47 PDT",
        "2017-10-17 04:56:23 JST",
        "2017-10-17 05:02:03 EDT",
        "2017-10-17 06:15:20 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_11_5_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_11_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:16:11 EDT",
        "2017-10-17 05:55:26 IDT",
        "2017-10-17 06:12:13 PDT",
        "2017-10-17 04:58:51 JST",
        "2017-10-17 05:16:25 EDT",
        "2017-10-17 06:17:26 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_11_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_10_2_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:20:22 EDT",
        "2017-10-17 05:59:13 IDT",
        "2017-10-17 06:16:05 PDT",
        "2017-10-17 05:02:48 JST",
        "2017-10-17 05:39:04 EDT",
        "2017-10-17 06:20:48 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_10_2_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_7_65_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:33:46 EDT",
        "2017-10-17 06:11:16 IDT",
        "2017-10-17 06:28:27 PDT",
        "2017-10-17 05:15:24 JST",
        "2017-10-17 06:51:04 EDT",
        "2017-10-17 06:31:32 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_7_65_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_misheyakir_9_5_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 06:24:03 EDT",
        "2017-10-17 06:02:31 IDT",
        "2017-10-17 06:19:29 PDT",
        "2017-10-17 05:06:15 JST",
        "2017-10-17 05:58:44 EDT",
        "2017-10-17 06:23:45 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.misheyakir_9_5_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_19_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:05:59 EDT",
        "2017-10-17 08:49:03 IDT",
        "2017-10-17 09:04:42 PDT",
        "2017-10-17 07:50:59 JST",
        "None",
        "2017-10-17 09:26:27 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_19_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:15:41 EDT",
        "2017-10-17 08:57:46 IDT",
        "2017-10-17 09:13:38 PDT",
        "2017-10-17 08:00:06 JST",
        "2017-10-17 07:10:10 EDT",
        "2017-10-17 09:34:18 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_18_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:10:42 EDT",
        "2017-10-17 08:53:17 IDT",
        "2017-10-17 09:09:03 PDT",
        "2017-10-17 07:55:25 JST",
        "None",
        "2017-10-17 09:30:16 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_18_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:19:53 EDT",
        "2017-10-17 08:57:56 IDT",
        "2017-10-17 09:14:45 PDT",
        "2017-10-17 08:01:57 JST",
        "None",
        "2017-10-17 09:30:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_72_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:22:41 EDT",
        "2017-10-17 08:59:53 IDT",
        "2017-10-17 09:16:57 PDT",
        "2017-10-17 08:04:14 JST",
        "None",
        "2017-10-17 09:29:08 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_72_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_90_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:10:53 EDT",
        "2017-10-17 08:48:56 IDT",
        "2017-10-17 09:05:45 PDT",
        "2017-10-17 07:52:57 JST",
        "None",
        "2017-10-17 09:21:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_90_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_90_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:14:23 EDT",
        "2017-10-17 08:51:23 IDT",
        "2017-10-17 09:08:30 PDT",
        "2017-10-17 07:55:48 JST",
        "None",
        "2017-10-17 09:19:49 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_90_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_96_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:07:53 EDT",
        "2017-10-17 08:45:56 IDT",
        "2017-10-17 09:02:45 PDT",
        "2017-10-17 07:49:57 JST",
        "None",
        "2017-10-17 09:18:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_96_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_96_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:11:36 EDT",
        "2017-10-17 08:48:32 IDT",
        "2017-10-17 09:05:41 PDT",
        "2017-10-17 07:53:00 JST",
        "None",
        "2017-10-17 09:16:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_96_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_3_hrs_before_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:42:18 EDT",
        "2017-10-17 09:24:28 IDT",
        "2017-10-17 09:40:05 PDT",
        "2017-10-17 08:26:55 JST",
        "2017-10-17 09:04:32 EDT",
        "2017-10-17 10:12:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_3_hrs_before_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_120_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 08:55:53 EDT",
        "2017-10-17 08:33:56 IDT",
        "2017-10-17 08:50:45 PDT",
        "2017-10-17 07:37:57 JST",
        "None",
        "2017-10-17 09:06:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_120_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_alos_16_1_degrees_to_sunset() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 08:55:37 EDT",
        "2017-10-17 08:39:42 IDT",
        "2017-10-17 08:55:07 PDT",
        "2017-10-17 07:41:13 JST",
        "None",
        "2017-10-17 09:18:14 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_alos_16_1_degrees_to_sunset() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:03:53 EDT",
        "2017-10-17 08:47:08 IDT",
        "2017-10-17 09:02:44 PDT",
        "2017-10-17 07:49:00 JST",
        "2017-10-17 06:01:11 EDT",
        "2017-10-17 09:24:47 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_19_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:17:56 EDT",
        "2017-10-17 10:00:44 IDT",
        "2017-10-17 10:16:22 PDT",
        "2017-10-17 09:02:49 JST",
        "None",
        "2017-10-17 10:41:53 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_19_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_18_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:21:05 EDT",
        "2017-10-17 10:03:34 IDT",
        "2017-10-17 10:19:16 PDT",
        "2017-10-17 09:05:47 JST",
        "None",
        "2017-10-17 10:44:26 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_18_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_90_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:21:14 EDT",
        "2017-10-17 10:00:40 IDT",
        "2017-10-17 10:17:05 PDT",
        "2017-10-17 09:04:09 JST",
        "None",
        "2017-10-17 10:38:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_90_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_90_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:23:33 EDT",
        "2017-10-17 10:02:18 IDT",
        "2017-10-17 10:18:55 PDT",
        "2017-10-17 09:06:03 JST",
        "None",
        "2017-10-17 10:37:27 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_90_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_96_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:19:14 EDT",
        "2017-10-17 09:58:40 IDT",
        "2017-10-17 10:15:05 PDT",
        "2017-10-17 09:02:09 JST",
        "None",
        "2017-10-17 10:36:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_96_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_96_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:21:43 EDT",
        "2017-10-17 10:00:25 IDT",
        "2017-10-17 10:17:02 PDT",
        "2017-10-17 09:04:11 JST",
        "None",
        "2017-10-17 10:35:23 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_96_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_mga_120_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:11:14 EDT",
        "2017-10-17 09:50:40 IDT",
        "2017-10-17 10:07:05 PDT",
        "2017-10-17 08:54:09 JST",
        "None",
        "2017-10-17 10:28:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_mga_120_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_2_hrs_before_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:42:18 EDT",
        "2017-10-17 10:24:28 IDT",
        "2017-10-17 10:40:05 PDT",
        "2017-10-17 09:26:55 JST",
        "2017-10-17 10:04:32 EDT",
        "2017-10-17 11:12:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_2_hrs_before_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:15:35 EDT",
        "2017-10-17 12:58:31 IDT",
        "2017-10-17 13:13:55 PDT",
        "2017-10-17 12:00:39 JST",
        "None",
        "2017-10-17 13:49:45 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:16:13 EDT",
        "2017-10-17 12:58:31 IDT",
        "2017-10-17 13:14:04 PDT",
        "2017-10-17 12:00:55 JST",
        "2017-10-17 12:45:42 EDT",
        "2017-10-17 13:49:09 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_ahavat_shalom() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:13:58 EDT",
        "2017-10-17 12:56:26 IDT",
        "2017-10-17 13:11:56 PDT",
        "2017-10-17 11:58:45 JST",
        "2017-10-17 12:36:21 EDT",
        "2017-10-17 13:46:56 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_ahavat_shalom() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_gra_greater_than_30_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:12:18 EDT",
        "2017-10-17 12:54:28 IDT",
        "2017-10-17 13:10:05 PDT",
        "2017-10-17 11:56:55 JST",
        "None",
        "2017-10-17 13:43:45 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_gra_greater_than_30_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:42:24 EDT",
        "2017-10-17 16:24:53 IDT",
        "2017-10-17 16:40:08 PDT",
        "2017-10-17 15:27:19 JST",
        "2017-10-17 17:33:18 EDT",
        "2017-10-17 17:27:36 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_ahavat_shalom() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 15:51:21 EDT",
        "2017-10-17 15:38:53 IDT",
        "2017-10-17 15:52:58 PDT",
        "2017-10-17 14:39:13 JST",
        "2017-10-17 12:30:13 EDT",
        "2017-10-17 16:46:31 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_ahavat_shalom() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:37:37 EDT",
        "2017-10-17 16:24:45 IDT",
        "2017-10-17 16:38:55 PDT",
        "2017-10-17 15:25:15 JST",
        "None",
        "2017-10-17 17:32:04 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_60_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:52:18 EDT",
        "2017-10-17 17:41:10 IDT",
        "2017-10-17 17:54:50 PDT",
        "2017-10-17 16:41:00 JST",
        "None",
        "2017-10-17 18:55:12 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_60_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:01:48 EDT",
        "2017-10-17 17:50:40 IDT",
        "2017-10-17 18:04:20 PDT",
        "2017-10-17 16:50:30 JST",
        "None",
        "2017-10-17 19:04:42 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_90_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:16:03 EDT",
        "2017-10-17 18:04:55 IDT",
        "2017-10-17 18:18:35 PDT",
        "2017-10-17 17:04:45 JST",
        "None",
        "2017-10-17 19:18:57 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_90_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_96_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:20:48 EDT",
        "2017-10-17 18:09:40 IDT",
        "2017-10-17 18:23:20 PDT",
        "2017-10-17 17:09:30 JST",
        "None",
        "2017-10-17 19:23:42 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_96_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_96_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:14:54 EDT",
        "2017-10-17 18:05:32 IDT",
        "2017-10-17 18:18:42 PDT",
        "2017-10-17 17:04:41 JST",
        "None",
        "2017-10-17 19:26:21 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_96_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_90_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:10:31 EDT",
        "2017-10-17 18:01:03 IDT",
        "2017-10-17 18:14:14 PDT",
        "2017-10-17 17:00:14 JST",
        "None",
        "2017-10-17 19:21:26 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_90_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_72_minutes_zmanis() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:57:22 EDT",
        "2017-10-17 17:47:34 IDT",
        "2017-10-17 18:00:51 PDT",
        "2017-10-17 16:46:53 JST",
        "None",
        "2017-10-17 19:06:41 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_72_minutes_zmanis() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:08:19 EDT",
        "2017-10-17 17:50:52 IDT",
        "2017-10-17 18:06:00 PDT",
        "2017-10-17 16:53:19 JST",
        "2017-10-17 19:33:08 EDT",
        "2017-10-17 18:58:37 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_19_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:23:38 EDT",
        "2017-10-17 18:04:38 IDT",
        "2017-10-17 18:20:08 PDT",
        "2017-10-17 17:07:44 JST",
        "None",
        "2017-10-17 19:11:06 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_19_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_26_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:49:27 EDT",
        "2017-10-17 18:27:44 IDT",
        "2017-10-17 18:43:51 PDT",
        "2017-10-17 17:31:58 JST",
        "None",
        "2017-10-17 19:32:13 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_26_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_mga_18_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:16:10 EDT",
        "2017-10-17 17:57:56 IDT",
        "2017-10-17 18:13:15 PDT",
        "2017-10-17 17:00:43 JST",
        "None",
        "2017-10-17 19:05:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_mga_18_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_alos_16_1_degrees_to_sunset() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:56:25 EDT",
        "2017-10-17 16:46:08 IDT",
        "2017-10-17 16:59:37 PDT",
        "2017-10-17 15:45:37 JST",
        "None",
        "2017-10-17 18:01:00 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_alos_16_1_degrees_to_sunset() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:26:03 EDT",
        "2017-10-17 17:12:46 IDT",
        "2017-10-17 17:26:57 PDT",
        "2017-10-17 16:13:30 JST",
        "2017-10-17 15:25:56 EDT",
        "2017-10-17 18:24:29 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_alos_16_1_degrees_to_tzais_geonim_7_083_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_ahavat_shalom() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:10:33 EDT",
        "2017-10-17 16:58:49 IDT",
        "2017-10-17 17:12:37 PDT",
        "2017-10-17 15:58:54 JST",
        "2017-10-17 13:50:10 EDT",
        "2017-10-17 18:12:08 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_ahavat_shalom() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_rt_13_24_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:19:17 EDT",
        "2017-10-17 19:03:23 IDT",
        "2017-10-17 19:18:04 PDT",
        "2017-10-17 18:05:15 JST",
        "2017-10-17 19:51:46 EDT",
        "2017-10-17 20:17:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_rt_13_24_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_rt_58_5_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:12:28 EDT",
        "2017-10-17 19:03:06 IDT",
        "2017-10-17 19:16:15 PDT",
        "2017-10-17 18:02:15 JST",
        "None",
        "2017-10-17 20:23:49 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_rt_58_5_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_rt_13_5_minutes_before_7_083_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:33:33 EDT",
        "2017-10-17 18:20:50 IDT",
        "2017-10-17 18:34:46 PDT",
        "2017-10-17 17:21:22 JST",
        "2017-10-17 16:43:32 EDT",
        "2017-10-17 19:38:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_rt_13_5_minutes_before_7_083_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_rt_2_stars() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:41:41 EDT",
        "2017-10-17 18:29:31 IDT",
        "2017-10-17 18:43:19 PDT",
        "2017-10-17 17:29:50 JST",
        "None",
        "2017-10-17 19:47:32 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_rt_2_stars() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_18_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:55:58 EDT",
        "2017-10-17 17:46:36 IDT",
        "2017-10-17 17:59:45 PDT",
        "2017-10-17 16:45:45 JST",
        "None",
        "2017-10-17 19:07:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_18_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_3_05_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:53:07 EDT",
        "2017-10-17 17:45:56 IDT",
        "2017-10-17 17:58:35 PDT",
        "2017-10-17 16:44:11 JST",
        "None",
        "2017-10-17 19:09:06 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_3_05_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_16_875_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:57:06 EDT",
        "2017-10-17 17:47:43 IDT",
        "2017-10-17 18:00:53 PDT",
        "2017-10-17 16:46:53 JST",
        "None",
        "2017-10-17 19:08:27 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_16_875_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_2_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:54:28 EDT",
        "2017-10-17 17:47:09 IDT",
        "2017-10-17 17:59:49 PDT",
        "2017-10-17 16:45:27 JST",
        "None",
        "2017-10-17 19:10:09 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_2_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_13_5_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:00:28 EDT",
        "2017-10-17 17:51:06 IDT",
        "2017-10-17 18:04:15 PDT",
        "2017-10-17 16:50:15 JST",
        "None",
        "2017-10-17 19:11:49 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_13_5_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_bein_hashmashos_yereim_2_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:58:15 EDT",
        "2017-10-17 17:50:31 IDT",
        "2017-10-17 18:03:17 PDT",
        "2017-10-17 16:49:00 JST",
        "None",
        "2017-10-17 19:13:04 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.bein_hashmashos_yereim_2_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_5_95_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:41:05 EDT",
        "2017-10-17 18:28:57 IDT",
        "2017-10-17 18:42:46 PDT",
        "2017-10-17 17:29:15 JST",
        "2017-10-17 16:23:10 EDT",
        "2017-10-17 19:46:46 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_5_95_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_4_61_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:34:01 EDT",
        "2017-10-17 18:22:36 IDT",
        "2017-10-17 18:36:14 PDT",
        "2017-10-17 17:22:36 JST",
        "2017-10-17 15:39:36 EDT",
        "2017-10-17 19:41:08 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_4_61_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_4_37_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:32:45 EDT",
        "2017-10-17 18:21:27 IDT",
        "2017-10-17 18:35:04 PDT",
        "2017-10-17 17:21:24 JST",
        "2017-10-17 15:31:11 EDT",
        "2017-10-17 19:40:08 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_4_37_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_5_88_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:40:43 EDT",
        "2017-10-17 18:28:38 IDT",
        "2017-10-17 18:42:25 PDT",
        "2017-10-17 17:28:54 JST",
        "2017-10-17 16:21:00 EDT",
        "2017-10-17 19:46:28 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_5_88_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_4_8_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:35:01 EDT",
        "2017-10-17 18:23:30 IDT",
        "2017-10-17 18:37:10 PDT",
        "2017-10-17 17:23:32 JST",
        "2017-10-17 15:46:07 EDT",
        "2017-10-17 19:41:56 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_4_8_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_6_45_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:43:43 EDT",
        "2017-10-17 18:31:20 IDT",
        "2017-10-17 18:45:12 PDT",
        "2017-10-17 17:31:44 JST",
        "2017-10-17 16:38:21 EDT",
        "2017-10-17 19:48:52 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_6_45_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_7_67_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:50:08 EDT",
        "2017-10-17 18:37:06 IDT",
        "2017-10-17 18:51:07 PDT",
        "2017-10-17 17:37:46 JST",
        "2017-10-17 17:13:54 EDT",
        "2017-10-17 19:54:00 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_7_67_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_8_5_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:54:29 EDT",
        "2017-10-17 18:41:02 IDT",
        "2017-10-17 18:55:09 PDT",
        "2017-10-17 17:41:52 JST",
        "2017-10-17 17:37:18 EDT",
        "2017-10-17 19:57:29 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_8_5_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_9_3_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:58:41 EDT",
        "2017-10-17 18:44:49 IDT",
        "2017-10-17 18:59:01 PDT",
        "2017-10-17 17:45:49 JST",
        "2017-10-17 17:59:32 EDT",
        "2017-10-17 20:00:51 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_9_3_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_geonim_9_75_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:01:02 EDT",
        "2017-10-17 18:46:56 IDT",
        "2017-10-17 19:01:12 PDT",
        "2017-10-17 17:48:03 JST",
        "2017-10-17 18:11:59 EDT",
        "2017-10-17 20:02:45 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_geonim_9_75_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:16:04 EDT",
        "2017-10-17 08:52:52 IDT",
        "2017-10-17 09:10:03 PDT",
        "2017-10-17 07:57:22 JST",
        "None",
        "2017-10-17 09:20:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:20:17 EDT",
        "2017-10-17 09:58:37 IDT",
        "2017-10-17 10:15:21 PDT",
        "2017-10-17 09:02:32 JST",
        "None",
        "2017-10-17 10:32:09 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:00:49 EDT",
        "2017-10-17 12:42:59 IDT",
        "2017-10-17 12:58:36 PDT",
        "2017-10-17 11:45:25 JST",
        "None",
        "2017-10-17 13:31:16 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:13:27 EDT",
        "2017-10-17 16:00:13 IDT",
        "2017-10-17 16:14:30 PDT",
        "2017-10-17 15:00:52 JST",
        "None",
        "2017-10-17 17:06:12 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_ateret_torah() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:33:43 EDT",
        "2017-10-17 17:22:25 IDT",
        "2017-10-17 17:36:08 PDT",
        "2017-10-17 16:22:19 JST",
        "None",
        "2017-10-17 18:35:46 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_ateret_torah() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:45:19 EDT",
        "2017-10-17 05:27:38 IDT",
        "2017-10-17 05:43:42 PDT",
        "2017-10-17 04:29:46 JST",
        "2017-10-17 01:36:59 EDT",
        "2017-10-17 05:52:28 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:53:53 EDT",
        "2017-10-17 09:32:08 IDT",
        "2017-10-17 09:48:54 PDT",
        "2017-10-17 08:36:04 JST",
        "2017-10-17 11:28:48 EDT",
        "2017-10-17 10:04:50 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_baal_hatanya_greater_than_30_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:12:18 EDT",
        "2017-10-17 12:54:28 IDT",
        "2017-10-17 13:10:05 PDT",
        "2017-10-17 11:56:55 JST",
        "2017-10-17 12:34:32 EDT",
        "2017-10-17 13:44:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_baal_hatanya_greater_than_30_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 15:57:57 EDT",
        "2017-10-17 15:44:50 IDT",
        "2017-10-17 15:59:04 PDT",
        "2017-10-17 14:45:26 JST",
        "2017-10-17 12:42:24 EDT",
        "2017-10-17 16:51:54 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:07:57 EDT",
        "2017-10-17 16:56:30 IDT",
        "2017-10-17 17:10:15 PDT",
        "2017-10-17 15:56:28 JST",
        "2017-10-17 12:56:33 EDT",
        "2017-10-17 18:10:10 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_baal_hatanya() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 18:41:21 EDT",
        "2017-10-17 18:29:12 IDT",
        "2017-10-17 18:43:00 PDT",
        "2017-10-17 17:29:30 JST",
        "2017-10-17 16:24:42 EDT",
        "2017-10-17 19:46:58 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_baal_hatanya() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:18:15 EDT",
        "2017-10-17 09:00:45 IDT",
        "2017-10-17 09:16:34 PDT",
        "2017-10-17 08:02:53 JST",
        "None",
        "2017-10-17 09:37:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:23:13 EDT",
        "2017-10-17 09:05:14 IDT",
        "2017-10-17 09:21:10 PDT",
        "2017-10-17 08:07:34 JST",
        "2017-10-17 07:20:52 EDT",
        "2017-10-17 09:41:32 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:18:24 EDT",
        "2017-10-17 08:56:23 IDT",
        "2017-10-17 09:13:15 PDT",
        "2017-10-17 08:00:23 JST",
        "None",
        "2017-10-17 09:28:39 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_90_minutes_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:27:24 EDT",
        "2017-10-17 09:05:23 IDT",
        "2017-10-17 09:22:15 PDT",
        "2017-10-17 08:09:23 JST",
        "None",
        "2017-10-17 09:37:39 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_mga_72_minutes_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_gra_sunrise_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:03:24 EDT",
        "2017-10-17 09:41:23 IDT",
        "2017-10-17 09:58:15 PDT",
        "2017-10-17 08:45:23 JST",
        "None",
        "2017-10-17 10:13:39 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_gra_sunrise_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 11:01:15 EDT",
        "2017-10-17 10:40:36 IDT",
        "2017-10-17 10:57:05 PDT",
        "2017-10-17 09:44:04 JST",
        "None",
        "2017-10-17 11:18:10 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_gra_sunrise_to_fixed_local_chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_gedola_gra_fixed_local_chatzos_30_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 13:26:57 EDT",
        "2017-10-17 13:09:03 IDT",
        "2017-10-17 13:24:46 PDT",
        "2017-10-17 12:11:26 JST",
        "2017-10-17 12:49:10 EDT",
        "2017-10-17 13:57:12 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_gedola_gra_fixed_local_chatzos_30_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_mincha_ketana_gra_fixed_local_chatzos_to_sunset() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:01:53 EDT",
        "2017-10-17 15:48:57 IDT",
        "2017-10-17 16:03:10 PDT",
        "2017-10-17 14:49:27 JST",
        "None",
        "2017-10-17 16:56:06 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.mincha_ketana_gra_fixed_local_chatzos_to_sunset() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_plag_gra_fixed_local_chatzos_to_sunset() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 17:07:56 EDT",
        "2017-10-17 16:56:46 IDT",
        "2017-10-17 17:10:28 PDT",
        "2017-10-17 15:56:36 JST",
        "None",
        "2017-10-17 18:10:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.plag_gra_fixed_local_chatzos_to_sunset() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_50_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:03:58 EDT",
        "2017-10-17 18:54:36 IDT",
        "2017-10-17 19:07:45 PDT",
        "2017-10-17 17:53:45 JST",
        "None",
        "2017-10-17 20:15:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_50_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_samuch_lemincha_ketana_gra() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 15:27:57 EDT",
        "2017-10-17 15:14:22 IDT",
        "2017-10-17 15:28:45 PDT",
        "2017-10-17 14:15:09 JST",
        "None",
        "2017-10-17 16:19:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.samuch_lemincha_ketana_gra() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_samuch_lemincha_ketana_mga_16_1_degrees() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:08:03 EDT",
        "2017-10-17 15:50:29 IDT",
        "2017-10-17 16:05:48 PDT",
        "2017-10-17 14:52:55 JST",
        "2017-10-17 16:45:22 EDT",
        "2017-10-17 16:51:11 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.samuch_lemincha_ketana_mga_16_1_degrees() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_samuch_lemincha_ketana_mga_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 16:03:57 EDT",
        "2017-10-17 15:50:22 IDT",
        "2017-10-17 16:04:45 PDT",
        "2017-10-17 14:51:09 JST",
        "None",
        "2017-10-17 16:55:01 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.samuch_lemincha_ketana_mga_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_chatzos() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 12:42:18 EDT",
        "2017-10-17 12:24:28 IDT",
        "2017-10-17 12:40:05 PDT",
        "2017-10-17 11:26:55 JST",
        "2017-10-17 12:04:32 EDT",
        "2017-10-17 13:12:43 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.chatzos() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_tzais_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 19:25:58 EDT",
        "2017-10-17 19:16:36 IDT",
        "2017-10-17 19:29:45 PDT",
        "2017-10-17 18:15:45 JST",
        "None",
        "2017-10-17 20:37:19 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.tzais_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_alos_72_minutes() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 05:57:51 EDT",
        "2017-10-17 05:31:43 IDT",
        "2017-10-17 05:49:45 PDT",
        "2017-10-17 04:37:21 JST",
        "None",
        "2017-10-17 05:48:05 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.alos_72_minutes() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_tefila_gra() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 10:51:14 EDT",
        "2017-10-17 10:30:40 IDT",
        "2017-10-17 10:47:05 PDT",
        "2017-10-17 09:34:09 JST",
        "None",
        "2017-10-17 11:08:30 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_tefila_gra() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sof_zman_shema_gra() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-17 09:55:53 EDT",
        "2017-10-17 09:33:56 IDT",
        "2017-10-17 09:50:45 PDT",
        "2017-10-17 08:37:57 JST",
        "None",
        "2017-10-17 10:06:24 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.sof_zman_shema_gra() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_chatzos_halayla() {
    let cals = test_helper::basic_location_czcs(false);
    let expected_datetime_strs = [
        "2017-10-18 00:42:12 EDT",
        "2017-10-18 00:24:22 IDT",
        "2017-10-18 00:39:59 PDT",
        "2017-10-17 23:26:48 JST",
        "2017-10-18 00:04:26 EDT",
        "2017-10-18 01:12:36 +14",
    ];

    for (czc, edt) in zip(cals, expected_datetime_strs) {
        let result = match czc.chatzos_halayla() {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}
