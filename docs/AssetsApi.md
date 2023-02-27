# \AssetsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_assets**](AssetsApi.md#get_characters_character_id_assets) | **Get** /characters/{character_id}/assets/ | Get character assets
[**get_corporations_corporation_id_assets**](AssetsApi.md#get_corporations_corporation_id_assets) | **Get** /corporations/{corporation_id}/assets/ | Get corporation assets
[**post_characters_character_id_assets_locations**](AssetsApi.md#post_characters_character_id_assets_locations) | **Post** /characters/{character_id}/assets/locations/ | Get character asset locations
[**post_characters_character_id_assets_names**](AssetsApi.md#post_characters_character_id_assets_names) | **Post** /characters/{character_id}/assets/names/ | Get character asset names
[**post_corporations_corporation_id_assets_locations**](AssetsApi.md#post_corporations_corporation_id_assets_locations) | **Post** /corporations/{corporation_id}/assets/locations/ | Get corporation asset locations
[**post_corporations_corporation_id_assets_names**](AssetsApi.md#post_corporations_corporation_id_assets_names) | **Post** /corporations/{corporation_id}/assets/names/ | Get corporation asset names


# **get_characters_character_id_assets**
> Vec<::models::GetCharactersCharacterIdAssets200Ok> get_characters_character_id_assets(ctx, character_id, optional)
Get character assets

Return a list of the characters assets  --- Alternate route: `/dev/characters/{character_id}/assets/`  Alternate route: `/v5/characters/{character_id}/assets/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCharactersCharacterIdAssets200Ok>**](get_characters_character_id_assets_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_assets**
> Vec<::models::GetCorporationsCorporationIdAssets200Ok> get_corporations_corporation_id_assets(ctx, corporation_id, optional)
Get corporation assets

Return a list of the corporation assets  --- Alternate route: `/dev/corporations/{corporation_id}/assets/`  Alternate route: `/v5/corporations/{corporation_id}/assets/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdAssets200Ok>**](get_corporations_corporation_id_assets_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_characters_character_id_assets_locations**
> Vec<::models::PostCharactersCharacterIdAssetsLocations200Ok> post_characters_character_id_assets_locations(ctx, character_id, item_ids, optional)
Get character asset locations

Return locations for a set of item ids, which you can get from character assets endpoint. Coordinates for items in hangars or stations are set to (0,0,0)  --- Alternate route: `/dev/characters/{character_id}/assets/locations/`  Alternate route: `/v2/characters/{character_id}/assets/locations/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **character_id** | **i32**| An EVE character ID | 
  **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i32**| An EVE character ID | 
 **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::PostCharactersCharacterIdAssetsLocations200Ok>**](post_characters_character_id_assets_locations_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_characters_character_id_assets_names**
> Vec<::models::PostCharactersCharacterIdAssetsNames200Ok> post_characters_character_id_assets_names(ctx, character_id, item_ids, optional)
Get character asset names

Return names for a set of item ids, which you can get from character assets endpoint. Typically used for items that can customize names, like containers or ships.  --- Alternate route: `/dev/characters/{character_id}/assets/names/`  Alternate route: `/legacy/characters/{character_id}/assets/names/`  Alternate route: `/v1/characters/{character_id}/assets/names/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **character_id** | **i32**| An EVE character ID | 
  **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **character_id** | **i32**| An EVE character ID | 
 **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::PostCharactersCharacterIdAssetsNames200Ok>**](post_characters_character_id_assets_names_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_corporations_corporation_id_assets_locations**
> Vec<::models::PostCorporationsCorporationIdAssetsLocations200Ok> post_corporations_corporation_id_assets_locations(ctx, corporation_id, item_ids, optional)
Get corporation asset locations

Return locations for a set of item ids, which you can get from corporation assets endpoint. Coordinates for items in hangars or stations are set to (0,0,0)  --- Alternate route: `/dev/corporations/{corporation_id}/assets/locations/`  Alternate route: `/v2/corporations/{corporation_id}/assets/locations/`   --- Requires one of the following EVE corporation role(s): Director 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **corporation_id** | **i32**| An EVE corporation ID | 
  **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **corporation_id** | **i32**| An EVE corporation ID | 
 **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::PostCorporationsCorporationIdAssetsLocations200Ok>**](post_corporations_corporation_id_assets_locations_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_corporations_corporation_id_assets_names**
> Vec<::models::PostCorporationsCorporationIdAssetsNames200Ok> post_corporations_corporation_id_assets_names(ctx, corporation_id, item_ids, optional)
Get corporation asset names

Return names for a set of item ids, which you can get from corporation assets endpoint. Only valid for items that can customize names, like containers or ships  --- Alternate route: `/dev/corporations/{corporation_id}/assets/names/`  Alternate route: `/legacy/corporations/{corporation_id}/assets/names/`  Alternate route: `/v1/corporations/{corporation_id}/assets/names/`   --- Requires one of the following EVE corporation role(s): Director 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **corporation_id** | **i32**| An EVE corporation ID | 
  **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **corporation_id** | **i32**| An EVE corporation ID | 
 **item_ids** | **Vec&lt;i64&gt;**| A list of item ids | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::PostCorporationsCorporationIdAssetsNames200Ok>**](post_corporations_corporation_id_assets_names_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

