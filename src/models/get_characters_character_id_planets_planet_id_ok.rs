/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdPlanetsPlanetIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdOk {
  /// links array
  #[serde(rename = "links")]
  links: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdLink>,
  /// pins array
  #[serde(rename = "pins")]
  pins: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdPin>,
  /// routes array
  #[serde(rename = "routes")]
  routes: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdRoute>
}

impl GetCharactersCharacterIdPlanetsPlanetIdOk {
  /// 200 ok object
  pub fn new(links: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdLink>, pins: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdPin>, routes: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdRoute>) -> GetCharactersCharacterIdPlanetsPlanetIdOk {
    GetCharactersCharacterIdPlanetsPlanetIdOk {
      links: links,
      pins: pins,
      routes: routes
    }
  }

  pub fn set_links(&mut self, links: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdLink>) {
    self.links = links;
  }

  pub fn with_links(mut self, links: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdLink>) -> GetCharactersCharacterIdPlanetsPlanetIdOk {
    self.links = links;
    self
  }

  pub fn links(&self) -> &Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdLink> {
    &self.links
  }


  pub fn set_pins(&mut self, pins: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdPin>) {
    self.pins = pins;
  }

  pub fn with_pins(mut self, pins: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdPin>) -> GetCharactersCharacterIdPlanetsPlanetIdOk {
    self.pins = pins;
    self
  }

  pub fn pins(&self) -> &Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdPin> {
    &self.pins
  }


  pub fn set_routes(&mut self, routes: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdRoute>) {
    self.routes = routes;
  }

  pub fn with_routes(mut self, routes: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdRoute>) -> GetCharactersCharacterIdPlanetsPlanetIdOk {
    self.routes = routes;
    self
  }

  pub fn routes(&self) -> &Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdRoute> {
    &self.routes
  }


}



