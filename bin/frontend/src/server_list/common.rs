use chrono::prelude::*;
use goolog::*;
use reqwasm::http::Request;
use serde::de::DeserializeOwned;

use crate::globals::URL_ORIGIN;

pub async fn get<T>(
    last_update: &mut Option<DateTime<Utc>>,
    data_url: &str,
    latest_url: &str,
) -> Option<T>
where
    T: DeserializeOwned,
{
    let url_origin = URL_ORIGIN.get_or_init(|| {
        fatal!(
            "HttpGetter",
            "The `URL_ORIGIN` should have been set by the root component."
        )
    });
    let data_url = url_origin.to_owned() + data_url;
    let latest_url = url_origin.to_owned() + latest_url;

    let latest_update: DateTime<Utc> = Request::get(&latest_url)
        .send()
        .await
        .unwrap_or_else(|error| {
            fatal!("HttpGetter", "If you can access the website you should also be able to access the api. Error: {error}")
        })
        .json()
        .await
        .unwrap_or_else(|error| {
            fatal!("HttpGetter", "The latest url did not sent a valid timestamp. Error: {error}")
        });

    if let Some(last_update) = *last_update {
        if latest_update == last_update {
            return None;
        }
    }

    *last_update = Some(latest_update);

    Some(
        Request::get(&data_url)
            .send()
            .await
            .unwrap_or_else(|error| {
                fatal!("HttpGetter", "If you can access the website you should also be able to access the api. Error: {error}")
            })
            .json()
            .await
            .unwrap_or_else(|error| {
                fatal!("HttpGetter", "Could not convert the server response to the requested type. Error: {error}")
            }),
    )
}
