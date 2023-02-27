# \FactionWarfareApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_fw_stats**](FactionWarfareApi.md#get_characters_character_id_fw_stats) | **Get** /characters/{character_id}/fw/stats/ | Overview of a character involved in faction warfare
[**get_corporations_corporation_id_fw_stats**](FactionWarfareApi.md#get_corporations_corporation_id_fw_stats) | **Get** /corporations/{corporation_id}/fw/stats/ | Overview of a corporation involved in faction warfare
[**get_fw_leaderboards**](FactionWarfareApi.md#get_fw_leaderboards) | **Get** /fw/leaderboards/ | List of the top factions in faction warfare
[**get_fw_leaderboards_characters**](FactionWarfareApi.md#get_fw_leaderboards_characters) | **Get** /fw/leaderboards/characters/ | List of the top pilots in faction warfare
[**get_fw_leaderboards_corporations**](FactionWarfareApi.md#get_fw_leaderboards_corporations) | **Get** /fw/leaderboards/corporations/ | List of the top corporations in faction warfare
[**get_fw_stats**](FactionWarfareApi.md#get_fw_stats) | **Get** /fw/stats/ | An overview of statistics about factions involved in faction warfare
[**get_fw_systems**](FactionWarfareApi.md#get_fw_systems) | **Get** /fw/systems/ | Ownership of faction warfare systems
[**get_fw_wars**](FactionWarfareApi.md#get_fw_wars) | **Get** /fw/wars/ | Data about which NPC factions are at war


# **get_characters_character_id_fw_stats**
> ::models::GetCharactersCharacterIdFwStatsOk get_characters_character_id_fw_stats(ctx, character_id, optional)
Overview of a character involved in faction warfare

Statistical overview of a character involved in faction warfare  --- Alternate route: `/dev/characters/{character_id}/fw/stats/`  Alternate route: `/legacy/characters/{character_id}/fw/stats/`  Alternate route: `/v1/characters/{character_id}/fw/stats/`  Alternate route: `/v2/characters/{character_id}/fw/stats/`  --- This route expires daily at 11:05

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
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**::models::GetCharactersCharacterIdFwStatsOk**](get_characters_character_id_fw_stats_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_fw_stats**
> ::models::GetCorporationsCorporationIdFwStatsOk get_corporations_corporation_id_fw_stats(ctx, corporation_id, optional)
Overview of a corporation involved in faction warfare

Statistics about a corporation involved in faction warfare  --- Alternate route: `/dev/corporations/{corporation_id}/fw/stats/`  Alternate route: `/legacy/corporations/{corporation_id}/fw/stats/`  Alternate route: `/v1/corporations/{corporation_id}/fw/stats/`  Alternate route: `/v2/corporations/{corporation_id}/fw/stats/`  --- This route expires daily at 11:05

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
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**::models::GetCorporationsCorporationIdFwStatsOk**](get_corporations_corporation_id_fw_stats_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_leaderboards**
> ::models::GetFwLeaderboardsOk get_fw_leaderboards(optional)
List of the top factions in faction warfare

Top 4 leaderboard of factions for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/`  Alternate route: `/legacy/fw/leaderboards/`  Alternate route: `/v1/fw/leaderboards/`  Alternate route: `/v2/fw/leaderboards/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetFwLeaderboardsOk**](get_fw_leaderboards_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_leaderboards_characters**
> ::models::GetFwLeaderboardsCharactersOk get_fw_leaderboards_characters(optional)
List of the top pilots in faction warfare

Top 100 leaderboard of pilots for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/characters/`  Alternate route: `/legacy/fw/leaderboards/characters/`  Alternate route: `/v1/fw/leaderboards/characters/`  Alternate route: `/v2/fw/leaderboards/characters/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetFwLeaderboardsCharactersOk**](get_fw_leaderboards_characters_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_leaderboards_corporations**
> ::models::GetFwLeaderboardsCorporationsOk get_fw_leaderboards_corporations(optional)
List of the top corporations in faction warfare

Top 10 leaderboard of corporations for kills and victory points separated by total, last week and yesterday  --- Alternate route: `/dev/fw/leaderboards/corporations/`  Alternate route: `/legacy/fw/leaderboards/corporations/`  Alternate route: `/v1/fw/leaderboards/corporations/`  Alternate route: `/v2/fw/leaderboards/corporations/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetFwLeaderboardsCorporationsOk**](get_fw_leaderboards_corporations_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_stats**
> Vec<::models::GetFwStats200Ok> get_fw_stats(optional)
An overview of statistics about factions involved in faction warfare

Statistical overviews of factions involved in faction warfare  --- Alternate route: `/dev/fw/stats/`  Alternate route: `/legacy/fw/stats/`  Alternate route: `/v1/fw/stats/`  Alternate route: `/v2/fw/stats/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**Vec<::models::GetFwStats200Ok>**](get_fw_stats_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_systems**
> Vec<::models::GetFwSystems200Ok> get_fw_systems(optional)
Ownership of faction warfare systems

An overview of the current ownership of faction warfare solar systems  --- Alternate route: `/dev/fw/systems/`  Alternate route: `/legacy/fw/systems/`  Alternate route: `/v2/fw/systems/`  Alternate route: `/v3/fw/systems/`  --- This route is cached for up to 1800 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**Vec<::models::GetFwSystems200Ok>**](get_fw_systems_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_fw_wars**
> Vec<::models::GetFwWars200Ok> get_fw_wars(optional)
Data about which NPC factions are at war

Data about which NPC factions are at war  --- Alternate route: `/dev/fw/wars/`  Alternate route: `/legacy/fw/wars/`  Alternate route: `/v1/fw/wars/`  Alternate route: `/v2/fw/wars/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**Vec<::models::GetFwWars200Ok>**](get_fw_wars_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

