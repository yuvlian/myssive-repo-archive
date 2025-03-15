use crate::models::{GenericSdkRsp, HourlyWeather, OnGetWeatherDataRsp};
use axum::Json;
use chrono::{Duration, Timelike, Utc};
use rand::Rng;

pub async fn on_get_weather() -> Json<GenericSdkRsp<OnGetWeatherDataRsp>> {
    let hourly = (0..24)
        .map(|i| {
            let date_time = Utc::now() + Duration::hours(i);
            HourlyWeather {
                condition: 1,
                date: date_time.format("%Y-%m-%d").to_string(),
                hour: date_time.hour() as u8,
                temp: rand::thread_rng().gen_range(20..=30),
            }
        })
        .collect();

    Json(GenericSdkRsp::<OnGetWeatherDataRsp> {
        data: OnGetWeatherDataRsp {
            hourly,
            ..Default::default()
        },
        ..Default::default()
    })
}
