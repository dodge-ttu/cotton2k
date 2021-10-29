use crate::bindings::{
    dl, isw, iyear, maxk, maxl, nk, nl, wk, CO2EnrichmentFactor, Clim, DayEmerge, DayEndCO2,
    DayEndMulch, DayFinish, DayPlant, DayStart, DayStartCO2, DayStartMulch, DensityFactor,
    Elevation, Kday, LastDayWeatherData, Latitude, Longitude, MulchIndicator, MulchTranLW,
    MulchTranSW, PerPlantArea, PlantPopulation, PlantRowColumn, PlantRowLocation, RowSpace,
    SitePar, VarPar,
};
use crate::profile::{MulchType, Profile, WeatherRecord};
use chrono::Datelike;
use std::io::Read;
use std::path::Path;

pub fn read_profile(profile_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(profile_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut profile: Profile = toml::from_str(&contents)?;
    match profile.name {
        None => {
            profile.name = Some(String::from(
                profile_path.file_stem().unwrap().to_str().unwrap(),
            ));
        }
        Some(_) => {}
    }
    unsafe {
        Latitude = profile.latitude;
        Longitude = profile.longitude;
        Elevation = profile.elevation;
        iyear = profile.start_date.year();
        DayStart = profile.start_date.ordinal() as i32;
        DayFinish = profile.stop_date.ordinal() as i32;
        match profile.emerge_date {
            Some(date) => DayEmerge = date.ordinal() as i32,
            None => {}
        }
        match profile.plant_date {
            Some(date) => {
                DayPlant = date.ordinal() as i32;
            }
            None => {}
        }
        if profile.emerge_date.is_none() {
            // If the date of emergence has not been given, emergence will be simulated by the model. In this case, isw = 0, and a check is performed to make sure that the date of planting has been given.
            if profile.plant_date.is_none() {
                panic!(
                    "one of planting date or emergence date must be given in the profile file!!"
                );
            }
            isw = 0;
        } else if profile.emerge_date > profile.plant_date {
            // If the date of emergence has been given in the input: isw = 1 if simulation starts before emergence,
            isw = 1;
        } else {
            // or isw = 2 if simulation starts at emergence.
            isw = 2;
            Kday = 1;
        }
        // For advanced users only: if there is CO2 enrichment, read also CO2 factor, DOY dates
        match profile.co2_enrichment {
            Some(enrichment) => {
                CO2EnrichmentFactor = enrichment.factor;
                DayStartCO2 = enrichment.start_date.ordinal() as i32;
                DayEndCO2 = enrichment.stop_date.ordinal() as i32;
            }
            None => CO2EnrichmentFactor = 0.,
        }
        // If soil mulch is used, read relevant parameters.
        match profile.mulch {
            Some(mulch) => match mulch.indicator {
                MulchType::NoMulch => {}
                _ => {
                    MulchIndicator = mulch.indicator as i32;
                    MulchTranSW = mulch.sw_trans;
                    MulchTranLW = mulch.lw_trans;
                    DayStartMulch = mulch.start_date.ordinal() as i32;
                    match mulch.stop_date {
                        Some(date) => DayEndMulch = date.ordinal() as i32,
                        None => DayEndMulch = DayFinish,
                    }
                }
            },
            None => {
                MulchIndicator = MulchType::NoMulch as i32;
            }
        }
        SitePar[1] = profile.site.wind_blow_after_sunrise;
        SitePar[2] = profile.site.wind_max_after_noon;
        SitePar[3] = profile.site.wind_stop_after_sunset;
        SitePar[4] = profile.site.night_time_wind_factor;
        SitePar[7] = profile.site.cloud_type_correction_factor;
        SitePar[8] = profile.site.max_temperature_after_noon;
        SitePar[9] = profile.site.deep_soil_temperature.0;
        SitePar[10] = profile.site.deep_soil_temperature.1;
        SitePar[11] = profile.site.deep_soil_temperature.2;
        SitePar[12] = profile.site.dew_point_range.0;
        SitePar[13] = profile.site.dew_point_range.1;
        SitePar[14] = profile.site.dew_point_range.2;
        SitePar[15] = profile.site.albedo_range.1;
        SitePar[16] = profile.site.albedo_range.0;
        for pair in profile.cultivar_parameters.iter().enumerate() {
            VarPar[pair.0 + 1] = *pair.1;
        }
        RowSpace = if profile.skip_row_width > 0. {
            (profile.row_space + profile.skip_row_width) / 2.
        } else {
            profile.row_space
        };
        // PlantRowLocation is the distance from edge of slab, cm, of the plant row.
        PlantRowLocation = RowSpace / 2.;
        // Compute PlantPopulation - number of plants per hectar, and
        // PerPlantArea - the average surface area per plant, in dm2, and
        // the empirical plant density factor (DensityFactor). This factor will be used to express the effect of plant density on some plant growth rate functions.  Note that DensityFactor =1 for 5 plants per sq m (or 50000 per ha).
        PlantPopulation = profile.plants_per_meter / RowSpace * 1000000.;
        PerPlantArea = 1000000. / PlantPopulation;
        DensityFactor = (VarPar[1] * (5. - PlantPopulation / 10000.)).exp();
        // Define the numbers of rows and columns in the soil slab (nl, nk).
        // Define the depth, in cm, of consecutive nl layers.
        nl = maxl;
        nk = maxk;
        dl[0] = 2.;
        dl[1] = 2.;
        dl[2] = 2.;
        dl[3] = 4.;
        for i in 4..(maxl - 2) as usize {
            dl[i] = 5.;
        }
        dl[(maxl - 2) as usize] = 10.;
        dl[(maxl - 1) as usize] = 10.;
        //      The width of the slab columns is computed by dividing the row
        //  spacing by the number of columns. It is assumed that slab width is
        //  equal to the average row spacing, and column widths are uniform.
        //      Note: wk is an array - to enable the option of non-uniform
        //  column widths in the future.
        //      PlantRowColumn (the column including the plant row) is now computed
        //      from
        //  PlantRowLocation (the distance of the plant row from the edge of the
        //  slab).
        let mut sumwk = 0.; // sum of column widths
        PlantRowColumn = 0;
        for k in 0..nk {
            wk[k as usize] = RowSpace / nk as f64;
            sumwk = sumwk + wk[k as usize];
            if PlantRowColumn == 0 && sumwk > PlantRowLocation {
                PlantRowColumn = if (sumwk - PlantRowLocation) > (0.5 * wk[k as usize]) {
                    k - 1
                } else {
                    k
                };
            }
        }
    }
    let weather_path = if profile.weather_path.is_relative() {
        profile_path.parent().unwrap().join(profile.weather_path)
    } else {
        profile.weather_path
    };
    let mut rdr = csv::Reader::from_path(weather_path)?;
    let mut jdd: i32 = 0;
    for result in rdr.deserialize() {
        let record: WeatherRecord = result?;
        jdd = record.date.ordinal() as i32;
        let j = jdd - unsafe { DayStart };
        if j < 0 {
            continue;
        }
        unsafe {
            Clim[j as usize].nDay = jdd;
            // convert \frac{MJ}{m^2} to langleys
            Clim[j as usize].Rad = record.irradiation * 23.884;
            Clim[j as usize].Tmax = record.tmax;
            Clim[j as usize].Tmin = record.tmin;
            Clim[j as usize].Wind =
                if profile.site.average_wind_speed.is_some() && record.wind.is_none() {
                    profile.site.average_wind_speed.unwrap()
                } else {
                    record.wind.unwrap_or(0.)
                };
            Clim[j as usize].Tdew = record.tdew.unwrap_or(estimate_dew_point(
                record.tmax,
                profile.site.estimate_dew_point.0,
                profile.site.estimate_dew_point.1,
            ));
            Clim[j as usize].Rain = record.rain;
        }
    }
    unsafe {
        LastDayWeatherData = jdd;
    }
    Ok(())
}
/// estimates the approximate daily average dewpoint temperature when it is not available.
fn estimate_dew_point(maxt: f64, site5: f64, site6: f64) -> f64 {
    if maxt <= 20. {
        site5
    } else if maxt >= 40. {
        site6
    } else {
        ((40. - maxt) * site5 + (maxt - 20.) * site6) / 20.
    }
}
