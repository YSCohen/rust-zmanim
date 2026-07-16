# 0.3.0
Many (not all) of the changes from KosherJava 3.0, and many other fixes and improvements

### Breaking API changes

1. `GeoLocation` fields now private; new validating constructor
    - Fields `latitude`/`longitude`/`elevation`/`timezone` changed from `pub` to `pub(crate)`; struct-literal construction no longer compiles.
    - `GeoLocation::new(lat, long, elev, tz) -> Result<Self, GeoLocationError>` validates: latitude in [-90,90], longitude in [-180,180], elevation finite number >= 0
    - New `GeoLocationError` enum
    - Added getters

2. `ComplexZmanimCalendar` fields now private; new constructor
    - `ComplexZmanimCalendar::new(geo_location, date, use_elevation)`
    - Added getters and setters for location, date & `use_elevation`
        - date/location setters clear the cache (see below)

3. functions throughout library take `Date`, not `&Date` (`Date` is `Copy`)

4. functions changed:
    - `tzeis_geonim_4_37_degrees` -> `tzeis_geonim_4_42_degrees`
    - `tzeis_geonim_4_61_degrees` -> `tzeis_geonim_4_66_degrees`

5. functions renamed:
    - `chatzos` -> `chatzos_hayom` (in `zmanim_calculator` and CZC)
    - `fixed_local_chatzos` -> `fixed_local_chatzos_hayom` (in `zmanim_calculator` and CZC)
    - `sof_zman_shema_mga_18_degrees_to_fixed_local_chatzos` -> `..._mga_alos_18_to_fixed_local_chatzos`
    - `sof_zman_shema_mga_16_1_degrees_to_fixed_local_chatzos` -> `..._mga_alos_16_1_to_fixed_local_chatzos`
    - `sof_zman_shema_alos_16_1_degrees_to_sunset` -> `..._alos_16_1_to_sunset`
    - `plag_alos_16_1_degrees_to_sunset` -> `plag_alos_16_1_to_sunset`
    - `sof_zman_shema_alos_16_1_degrees_to_tzeis_geonim_7_083_degrees` -> `..._alos_16_1_to_tzeis_7_083`
    - `plag_alos_16_1_degrees_to_tzeis_geonim_7_083_degrees` -> `plag_alos_16_1_to_tzeis_7_083`
    - removed `tzeis_geonim_5_88_degrees`

### New functionality

- `solar_azimuth`, `solar_elevation` in `astronomical_calculator` and CZC
- `polar_sunrise_ben_ish_chai`, `polar_sunset_ben_ish_chai`, `polar_plag_ben_ish_chai` in CZC
    - using new `astronomical_calculator::time_at_azimuth`
- `chatzos_hayom_as_half_day` in `zmanim_calculator` and CZC
- `half_day_based_shaah_zmanis`, `half_day_based_zman` in `zmanim_calculator`
- `shaah_zmanis_alos_16_1_to_tzeis_7_083` in CZC
- `percent_of_shaah_zmanis_from_degrees(degrees, sunset)` in CZC
- Zman registry: `ALL_ZMANIM`, `ZmanEntry`, `ZmanKind`, `ZmanValue`, `find_zman` in `complex_zmanim_calendar` (and prelude), enumerating every zero-arg zman accessor for name-based lookup/iteration
- [no API change] Per-instance lazy caching for `ComplexZmanimCalendar`

### Internal correctness / accuracy fixes

- Date-varying apparent solar radius. See `zenith_adjustments`
- Slight correction to constants
- More correct date anchoring in edge cases
- Fixed border bug in time normalization
- Better antimeridian adjustment algorithm from KJ 3
- More precise math in `zmanim_calculator`
- Removed all `unwrap()` panics
- Antimeridian handling factored out into `antimeridian_adjusted_date` and now also applied inside `utc_sun_rise_set` when adjusting the zenith (previously only used in the Julian-day path)
- `solar_midnight` now correctly propagates `None` from `date_time_from_time_of_day` (was constructing then wrapping)
- Extra refinement pass in `utc_solar_noon_midnight`, matching KosherJava 3

### Misc

- Lots of refactoring, deduplication, cleanup, &c
- Much more/better docs
- More examples
- Much more generated tests, which are much more comprehensive
    - generated from latest KosherJava commit
