# \LoyaltyApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_loyalty_points**](LoyaltyApi.md#get_characters_character_id_loyalty_points) | **Get** /characters/{character_id}/loyalty/points/ | Get loyalty points
[**get_loyalty_stores_corporation_id_offers**](LoyaltyApi.md#get_loyalty_stores_corporation_id_offers) | **Get** /loyalty/stores/{corporation_id}/offers/ | List loyalty store offers


# **get_characters_character_id_loyalty_points**
> Vec<::models::GetCharactersCharacterIdLoyaltyPoints200Ok> get_characters_character_id_loyalty_points(ctx, character_id, optional)
Get loyalty points

Return a list of loyalty points for all corporations the character has worked for  --- Alternate route: `/dev/characters/{character_id}/loyalty/points/`  Alternate route: `/legacy/characters/{character_id}/loyalty/points/`  Alternate route: `/v1/characters/{character_id}/loyalty/points/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCharactersCharacterIdLoyaltyPoints200Ok>**](get_characters_character_id_loyalty_points_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_loyalty_stores_corporation_id_offers**
> Vec<::models::GetLoyaltyStoresCorporationIdOffers200Ok> get_loyalty_stores_corporation_id_offers(corporation_id, optional)
List loyalty store offers

Return a list of offers from a specific corporation's loyalty store  --- Alternate route: `/dev/loyalty/stores/{corporation_id}/offers/`  Alternate route: `/legacy/loyalty/stores/{corporation_id}/offers/`  Alternate route: `/v1/loyalty/stores/{corporation_id}/offers/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **corporation_id** | **i32**| An EVE corporation ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **corporation_id** | **i32**| An EVE corporation ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**Vec<::models::GetLoyaltyStoresCorporationIdOffers200Ok>**](get_loyalty_stores_corporation_id_offers_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

