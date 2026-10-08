use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    address: NominatimAddress,
}

#[derive(Debug, Deserialize)]
struct NominatimAddress {
    suburb: Option<String>,
    neighbourhood: Option<String>,
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    state: Option<String>,
}

pub async fn reverse_geocode(lat: f64, lon: f64) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://nominatim.openstreetmap.org/reverse")
        .query(&[
            ("lat", lat.to_string()),
            ("lon", lon.to_string()),
            ("format", "json".to_string()),
        ])
        .header("Accept-Language", "en")
        .header("User-Agent", "PillDetect/1.0")
        .send()
        .await?
        .json::<NominatimResponse>()
        .await?;

    let address = response.address;

    let locality = address
        .suburb
        .or(address.neighbourhood)
        .or(address.city.clone())
        .or(address.town.clone())
        .or(address.village.clone());

    let city = address.city.or(address.town).or(address.village);

    let state = address.state;

    Ok([locality, city, state]
        .into_iter()
        .flatten()
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>()
        .join(", "))
}
