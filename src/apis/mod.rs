use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod alliance_api;
pub use self::alliance_api::{ AllianceApi, AllianceApiClient };
mod assets_api;
pub use self::assets_api::{ AssetsApi, AssetsApiClient };
mod bookmarks_api;
pub use self::bookmarks_api::{ BookmarksApi, BookmarksApiClient };
mod calendar_api;
pub use self::calendar_api::{ CalendarApi, CalendarApiClient };
mod character_api;
pub use self::character_api::{ CharacterApi, CharacterApiClient };
mod clones_api;
pub use self::clones_api::{ ClonesApi, ClonesApiClient };
mod contacts_api;
pub use self::contacts_api::{ ContactsApi, ContactsApiClient };
mod contracts_api;
pub use self::contracts_api::{ ContractsApi, ContractsApiClient };
mod corporation_api;
pub use self::corporation_api::{ CorporationApi, CorporationApiClient };
mod dogma_api;
pub use self::dogma_api::{ DogmaApi, DogmaApiClient };
mod faction_warfare_api;
pub use self::faction_warfare_api::{ FactionWarfareApi, FactionWarfareApiClient };
mod fittings_api;
pub use self::fittings_api::{ FittingsApi, FittingsApiClient };
mod fleets_api;
pub use self::fleets_api::{ FleetsApi, FleetsApiClient };
mod incursions_api;
pub use self::incursions_api::{ IncursionsApi, IncursionsApiClient };
mod industry_api;
pub use self::industry_api::{ IndustryApi, IndustryApiClient };
mod insurance_api;
pub use self::insurance_api::{ InsuranceApi, InsuranceApiClient };
mod killmails_api;
pub use self::killmails_api::{ KillmailsApi, KillmailsApiClient };
mod location_api;
pub use self::location_api::{ LocationApi, LocationApiClient };
mod loyalty_api;
pub use self::loyalty_api::{ LoyaltyApi, LoyaltyApiClient };
mod mail_api;
pub use self::mail_api::{ MailApi, MailApiClient };
mod market_api;
pub use self::market_api::{ MarketApi, MarketApiClient };
mod opportunities_api;
pub use self::opportunities_api::{ OpportunitiesApi, OpportunitiesApiClient };
mod planetary_interaction_api;
pub use self::planetary_interaction_api::{ PlanetaryInteractionApi, PlanetaryInteractionApiClient };
mod routes_api;
pub use self::routes_api::{ RoutesApi, RoutesApiClient };
mod search_api;
pub use self::search_api::{ SearchApi, SearchApiClient };
mod skills_api;
pub use self::skills_api::{ SkillsApi, SkillsApiClient };
mod sovereignty_api;
pub use self::sovereignty_api::{ SovereigntyApi, SovereigntyApiClient };
mod status_api;
pub use self::status_api::{ StatusApi, StatusApiClient };
mod universe_api;
pub use self::universe_api::{ UniverseApi, UniverseApiClient };
mod user_interface_api;
pub use self::user_interface_api::{ UserInterfaceApi, UserInterfaceApiClient };
mod wallet_api;
pub use self::wallet_api::{ WalletApi, WalletApiClient };
mod wars_api;
pub use self::wars_api::{ WarsApi, WarsApiClient };

pub mod configuration;
pub mod client;
