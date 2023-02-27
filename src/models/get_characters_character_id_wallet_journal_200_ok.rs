/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdWalletJournal200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdWalletJournal200Ok {
  /// The amount of ISK given or taken from the wallet as a result of the given transaction. Positive when ISK is deposited into the wallet and negative when ISK is withdrawn
  #[serde(rename = "amount")]
  amount: Option<f64>,
  /// Wallet balance after transaction occurred
  #[serde(rename = "balance")]
  balance: Option<f64>,
  /// An ID that gives extra context to the particular transaction. Because of legacy reasons the context is completely different per ref_type and means different things. It is also possible to not have a context_id
  #[serde(rename = "context_id")]
  context_id: Option<i64>,
  /// The type of the given context_id if present
  #[serde(rename = "context_id_type")]
  context_id_type: Option<String>,
  /// Date and time of transaction
  #[serde(rename = "date")]
  date: String,
  /// The reason for the transaction, mirrors what is seen in the client
  #[serde(rename = "description")]
  description: String,
  /// The id of the first party involved in the transaction. This attribute has no consistency and is different or non existant for particular ref_types. The description attribute will help make sense of what this attribute means. For more info about the given ID it can be dropped into the /universe/names/ ESI route to determine its type and name
  #[serde(rename = "first_party_id")]
  first_party_id: Option<i32>,
  /// Unique journal reference ID
  #[serde(rename = "id")]
  id: i64,
  /// The user stated reason for the transaction. Only applies to some ref_types
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// \"The transaction type for the given. transaction. Different transaction types will populate different attributes.\"
  #[serde(rename = "ref_type")]
  ref_type: String,
  /// The id of the second party involved in the transaction. This attribute has no consistency and is different or non existant for particular ref_types. The description attribute will help make sense of what this attribute means. For more info about the given ID it can be dropped into the /universe/names/ ESI route to determine its type and name
  #[serde(rename = "second_party_id")]
  second_party_id: Option<i32>,
  /// Tax amount received. Only applies to tax related transactions
  #[serde(rename = "tax")]
  tax: Option<f64>,
  /// The corporation ID receiving any tax paid. Only applies to tax related transactions
  #[serde(rename = "tax_receiver_id")]
  tax_receiver_id: Option<i32>
}

impl GetCharactersCharacterIdWalletJournal200Ok {
  /// 200 ok object
  pub fn new(date: String, description: String, id: i64, ref_type: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    GetCharactersCharacterIdWalletJournal200Ok {
      amount: None,
      balance: None,
      context_id: None,
      context_id_type: None,
      date: date,
      description: description,
      first_party_id: None,
      id: id,
      reason: None,
      ref_type: ref_type,
      second_party_id: None,
      tax: None,
      tax_receiver_id: None
    }
  }

  pub fn set_amount(&mut self, amount: f64) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f64) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f64> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_balance(&mut self, balance: f64) {
    self.balance = Some(balance);
  }

  pub fn with_balance(mut self, balance: f64) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.balance = Some(balance);
    self
  }

  pub fn balance(&self) -> Option<&f64> {
    self.balance.as_ref()
  }

  pub fn reset_balance(&mut self) {
    self.balance = None;
  }

  pub fn set_context_id(&mut self, context_id: i64) {
    self.context_id = Some(context_id);
  }

  pub fn with_context_id(mut self, context_id: i64) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.context_id = Some(context_id);
    self
  }

  pub fn context_id(&self) -> Option<&i64> {
    self.context_id.as_ref()
  }

  pub fn reset_context_id(&mut self) {
    self.context_id = None;
  }

  pub fn set_context_id_type(&mut self, context_id_type: String) {
    self.context_id_type = Some(context_id_type);
  }

  pub fn with_context_id_type(mut self, context_id_type: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.context_id_type = Some(context_id_type);
    self
  }

  pub fn context_id_type(&self) -> Option<&String> {
    self.context_id_type.as_ref()
  }

  pub fn reset_context_id_type(&mut self) {
    self.context_id_type = None;
  }

  pub fn set_date(&mut self, date: String) {
    self.date = date;
  }

  pub fn with_date(mut self, date: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.date = date;
    self
  }

  pub fn date(&self) -> &String {
    &self.date
  }


  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_first_party_id(&mut self, first_party_id: i32) {
    self.first_party_id = Some(first_party_id);
  }

  pub fn with_first_party_id(mut self, first_party_id: i32) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.first_party_id = Some(first_party_id);
    self
  }

  pub fn first_party_id(&self) -> Option<&i32> {
    self.first_party_id.as_ref()
  }

  pub fn reset_first_party_id(&mut self) {
    self.first_party_id = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i64) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.id = id;
    self
  }

  pub fn id(&self) -> &i64 {
    &self.id
  }


  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_ref_type(&mut self, ref_type: String) {
    self.ref_type = ref_type;
  }

  pub fn with_ref_type(mut self, ref_type: String) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.ref_type = ref_type;
    self
  }

  pub fn ref_type(&self) -> &String {
    &self.ref_type
  }


  pub fn set_second_party_id(&mut self, second_party_id: i32) {
    self.second_party_id = Some(second_party_id);
  }

  pub fn with_second_party_id(mut self, second_party_id: i32) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.second_party_id = Some(second_party_id);
    self
  }

  pub fn second_party_id(&self) -> Option<&i32> {
    self.second_party_id.as_ref()
  }

  pub fn reset_second_party_id(&mut self) {
    self.second_party_id = None;
  }

  pub fn set_tax(&mut self, tax: f64) {
    self.tax = Some(tax);
  }

  pub fn with_tax(mut self, tax: f64) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.tax = Some(tax);
    self
  }

  pub fn tax(&self) -> Option<&f64> {
    self.tax.as_ref()
  }

  pub fn reset_tax(&mut self) {
    self.tax = None;
  }

  pub fn set_tax_receiver_id(&mut self, tax_receiver_id: i32) {
    self.tax_receiver_id = Some(tax_receiver_id);
  }

  pub fn with_tax_receiver_id(mut self, tax_receiver_id: i32) -> GetCharactersCharacterIdWalletJournal200Ok {
    self.tax_receiver_id = Some(tax_receiver_id);
    self
  }

  pub fn tax_receiver_id(&self) -> Option<&i32> {
    self.tax_receiver_id.as_ref()
  }

  pub fn reset_tax_receiver_id(&mut self) {
    self.tax_receiver_id = None;
  }

}



