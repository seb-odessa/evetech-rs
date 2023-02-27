use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  alliance_api: Box<::apis::AllianceApi>,
  assets_api: Box<::apis::AssetsApi>,
  bookmarks_api: Box<::apis::BookmarksApi>,
  calendar_api: Box<::apis::CalendarApi>,
  character_api: Box<::apis::CharacterApi>,
  clones_api: Box<::apis::ClonesApi>,
  contacts_api: Box<::apis::ContactsApi>,
  contracts_api: Box<::apis::ContractsApi>,
  corporation_api: Box<::apis::CorporationApi>,
  dogma_api: Box<::apis::DogmaApi>,
  faction_warfare_api: Box<::apis::FactionWarfareApi>,
  fittings_api: Box<::apis::FittingsApi>,
  fleets_api: Box<::apis::FleetsApi>,
  incursions_api: Box<::apis::IncursionsApi>,
  industry_api: Box<::apis::IndustryApi>,
  insurance_api: Box<::apis::InsuranceApi>,
  killmails_api: Box<::apis::KillmailsApi>,
  location_api: Box<::apis::LocationApi>,
  loyalty_api: Box<::apis::LoyaltyApi>,
  mail_api: Box<::apis::MailApi>,
  market_api: Box<::apis::MarketApi>,
  opportunities_api: Box<::apis::OpportunitiesApi>,
  planetary_interaction_api: Box<::apis::PlanetaryInteractionApi>,
  routes_api: Box<::apis::RoutesApi>,
  search_api: Box<::apis::SearchApi>,
  skills_api: Box<::apis::SkillsApi>,
  sovereignty_api: Box<::apis::SovereigntyApi>,
  status_api: Box<::apis::StatusApi>,
  universe_api: Box<::apis::UniverseApi>,
  user_interface_api: Box<::apis::UserInterfaceApi>,
  wallet_api: Box<::apis::WalletApi>,
  wars_api: Box<::apis::WarsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      alliance_api: Box::new(::apis::AllianceApiClient::new(rc.clone())),
      assets_api: Box::new(::apis::AssetsApiClient::new(rc.clone())),
      bookmarks_api: Box::new(::apis::BookmarksApiClient::new(rc.clone())),
      calendar_api: Box::new(::apis::CalendarApiClient::new(rc.clone())),
      character_api: Box::new(::apis::CharacterApiClient::new(rc.clone())),
      clones_api: Box::new(::apis::ClonesApiClient::new(rc.clone())),
      contacts_api: Box::new(::apis::ContactsApiClient::new(rc.clone())),
      contracts_api: Box::new(::apis::ContractsApiClient::new(rc.clone())),
      corporation_api: Box::new(::apis::CorporationApiClient::new(rc.clone())),
      dogma_api: Box::new(::apis::DogmaApiClient::new(rc.clone())),
      faction_warfare_api: Box::new(::apis::FactionWarfareApiClient::new(rc.clone())),
      fittings_api: Box::new(::apis::FittingsApiClient::new(rc.clone())),
      fleets_api: Box::new(::apis::FleetsApiClient::new(rc.clone())),
      incursions_api: Box::new(::apis::IncursionsApiClient::new(rc.clone())),
      industry_api: Box::new(::apis::IndustryApiClient::new(rc.clone())),
      insurance_api: Box::new(::apis::InsuranceApiClient::new(rc.clone())),
      killmails_api: Box::new(::apis::KillmailsApiClient::new(rc.clone())),
      location_api: Box::new(::apis::LocationApiClient::new(rc.clone())),
      loyalty_api: Box::new(::apis::LoyaltyApiClient::new(rc.clone())),
      mail_api: Box::new(::apis::MailApiClient::new(rc.clone())),
      market_api: Box::new(::apis::MarketApiClient::new(rc.clone())),
      opportunities_api: Box::new(::apis::OpportunitiesApiClient::new(rc.clone())),
      planetary_interaction_api: Box::new(::apis::PlanetaryInteractionApiClient::new(rc.clone())),
      routes_api: Box::new(::apis::RoutesApiClient::new(rc.clone())),
      search_api: Box::new(::apis::SearchApiClient::new(rc.clone())),
      skills_api: Box::new(::apis::SkillsApiClient::new(rc.clone())),
      sovereignty_api: Box::new(::apis::SovereigntyApiClient::new(rc.clone())),
      status_api: Box::new(::apis::StatusApiClient::new(rc.clone())),
      universe_api: Box::new(::apis::UniverseApiClient::new(rc.clone())),
      user_interface_api: Box::new(::apis::UserInterfaceApiClient::new(rc.clone())),
      wallet_api: Box::new(::apis::WalletApiClient::new(rc.clone())),
      wars_api: Box::new(::apis::WarsApiClient::new(rc.clone())),
    }
  }

  pub fn alliance_api(&self) -> &::apis::AllianceApi{
    self.alliance_api.as_ref()
  }

  pub fn assets_api(&self) -> &::apis::AssetsApi{
    self.assets_api.as_ref()
  }

  pub fn bookmarks_api(&self) -> &::apis::BookmarksApi{
    self.bookmarks_api.as_ref()
  }

  pub fn calendar_api(&self) -> &::apis::CalendarApi{
    self.calendar_api.as_ref()
  }

  pub fn character_api(&self) -> &::apis::CharacterApi{
    self.character_api.as_ref()
  }

  pub fn clones_api(&self) -> &::apis::ClonesApi{
    self.clones_api.as_ref()
  }

  pub fn contacts_api(&self) -> &::apis::ContactsApi{
    self.contacts_api.as_ref()
  }

  pub fn contracts_api(&self) -> &::apis::ContractsApi{
    self.contracts_api.as_ref()
  }

  pub fn corporation_api(&self) -> &::apis::CorporationApi{
    self.corporation_api.as_ref()
  }

  pub fn dogma_api(&self) -> &::apis::DogmaApi{
    self.dogma_api.as_ref()
  }

  pub fn faction_warfare_api(&self) -> &::apis::FactionWarfareApi{
    self.faction_warfare_api.as_ref()
  }

  pub fn fittings_api(&self) -> &::apis::FittingsApi{
    self.fittings_api.as_ref()
  }

  pub fn fleets_api(&self) -> &::apis::FleetsApi{
    self.fleets_api.as_ref()
  }

  pub fn incursions_api(&self) -> &::apis::IncursionsApi{
    self.incursions_api.as_ref()
  }

  pub fn industry_api(&self) -> &::apis::IndustryApi{
    self.industry_api.as_ref()
  }

  pub fn insurance_api(&self) -> &::apis::InsuranceApi{
    self.insurance_api.as_ref()
  }

  pub fn killmails_api(&self) -> &::apis::KillmailsApi{
    self.killmails_api.as_ref()
  }

  pub fn location_api(&self) -> &::apis::LocationApi{
    self.location_api.as_ref()
  }

  pub fn loyalty_api(&self) -> &::apis::LoyaltyApi{
    self.loyalty_api.as_ref()
  }

  pub fn mail_api(&self) -> &::apis::MailApi{
    self.mail_api.as_ref()
  }

  pub fn market_api(&self) -> &::apis::MarketApi{
    self.market_api.as_ref()
  }

  pub fn opportunities_api(&self) -> &::apis::OpportunitiesApi{
    self.opportunities_api.as_ref()
  }

  pub fn planetary_interaction_api(&self) -> &::apis::PlanetaryInteractionApi{
    self.planetary_interaction_api.as_ref()
  }

  pub fn routes_api(&self) -> &::apis::RoutesApi{
    self.routes_api.as_ref()
  }

  pub fn search_api(&self) -> &::apis::SearchApi{
    self.search_api.as_ref()
  }

  pub fn skills_api(&self) -> &::apis::SkillsApi{
    self.skills_api.as_ref()
  }

  pub fn sovereignty_api(&self) -> &::apis::SovereigntyApi{
    self.sovereignty_api.as_ref()
  }

  pub fn status_api(&self) -> &::apis::StatusApi{
    self.status_api.as_ref()
  }

  pub fn universe_api(&self) -> &::apis::UniverseApi{
    self.universe_api.as_ref()
  }

  pub fn user_interface_api(&self) -> &::apis::UserInterfaceApi{
    self.user_interface_api.as_ref()
  }

  pub fn wallet_api(&self) -> &::apis::WalletApi{
    self.wallet_api.as_ref()
  }

  pub fn wars_api(&self) -> &::apis::WarsApi{
    self.wars_api.as_ref()
  }


}
