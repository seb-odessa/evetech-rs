# \KillmailsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_killmails_recent**](KillmailsApi.md#get_characters_character_id_killmails_recent) | **Get** /characters/{character_id}/killmails/recent/ | Get a character&#39;s recent kills and losses
[**get_corporations_corporation_id_killmails_recent**](KillmailsApi.md#get_corporations_corporation_id_killmails_recent) | **Get** /corporations/{corporation_id}/killmails/recent/ | Get a corporation&#39;s recent kills and losses
[**get_killmails_killmail_id_killmail_hash**](KillmailsApi.md#get_killmails_killmail_id_killmail_hash) | **Get** /killmails/{killmail_id}/{killmail_hash}/ | Get a single killmail


# **get_characters_character_id_killmails_recent**
> Vec<::models::GetCharactersCharacterIdKillmailsRecent200Ok> get_characters_character_id_killmails_recent(ctx, character_id, optional)
Get a character's recent kills and losses

Return a list of a character's kills and losses going back 90 days  --- Alternate route: `/dev/characters/{character_id}/killmails/recent/`  Alternate route: `/legacy/characters/{character_id}/killmails/recent/`  Alternate route: `/v1/characters/{character_id}/killmails/recent/`  --- This route is cached for up to 300 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **character_id** | **i32**| An EVE character ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i32**| An EVE character ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **page** | **i32**| Which page of results to return | [default to 1]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::GetCharactersCharacterIdKillmailsRecent200Ok>**](get_characters_character_id_killmails_recent_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_killmails_recent**
> Vec<::models::GetCorporationsCorporationIdKillmailsRecent200Ok> get_corporations_corporation_id_killmails_recent(ctx, corporation_id, optional)
Get a corporation's recent kills and losses

Get a list of a corporation's kills and losses going back 90 days  --- Alternate route: `/dev/corporations/{corporation_id}/killmails/recent/`  Alternate route: `/legacy/corporations/{corporation_id}/killmails/recent/`  Alternate route: `/v1/corporations/{corporation_id}/killmails/recent/`  --- This route is cached for up to 300 seconds  --- Requires one of the following EVE corporation role(s): Director 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **corporation_id** | **i32**| An EVE corporation ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **corporation_id** | **i32**| An EVE corporation ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **page** | **i32**| Which page of results to return | [default to 1]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::GetCorporationsCorporationIdKillmailsRecent200Ok>**](get_corporations_corporation_id_killmails_recent_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_killmails_killmail_id_killmail_hash**
> ::models::GetKillmailsKillmailIdKillmailHashOk get_killmails_killmail_id_killmail_hash(killmail_hash, killmail_id, optional)
Get a single killmail

Return a single killmail from its ID and hash  --- Alternate route: `/dev/killmails/{killmail_id}/{killmail_hash}/`  Alternate route: `/legacy/killmails/{killmail_id}/{killmail_hash}/`  Alternate route: `/v1/killmails/{killmail_id}/{killmail_hash}/`  --- This route is cached for up to 30758400 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **killmail_hash** | **String**| The killmail hash for verification | 
  **killmail_id** | **i32**| The killmail ID to be queried | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **killmail_hash** | **String**| The killmail hash for verification | 
 **killmail_id** | **i32**| The killmail ID to be queried | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetKillmailsKillmailIdKillmailHashOk**](get_killmails_killmail_id_killmail_hash_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

