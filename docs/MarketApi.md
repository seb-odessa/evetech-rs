# \MarketApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_orders**](MarketApi.md#get_characters_character_id_orders) | **Get** /characters/{character_id}/orders/ | List open orders from a character
[**get_characters_character_id_orders_history**](MarketApi.md#get_characters_character_id_orders_history) | **Get** /characters/{character_id}/orders/history/ | List historical orders by a character
[**get_corporations_corporation_id_orders**](MarketApi.md#get_corporations_corporation_id_orders) | **Get** /corporations/{corporation_id}/orders/ | List open orders from a corporation
[**get_corporations_corporation_id_orders_history**](MarketApi.md#get_corporations_corporation_id_orders_history) | **Get** /corporations/{corporation_id}/orders/history/ | List historical orders from a corporation
[**get_markets_groups**](MarketApi.md#get_markets_groups) | **Get** /markets/groups/ | Get item groups
[**get_markets_groups_market_group_id**](MarketApi.md#get_markets_groups_market_group_id) | **Get** /markets/groups/{market_group_id}/ | Get item group information
[**get_markets_prices**](MarketApi.md#get_markets_prices) | **Get** /markets/prices/ | List market prices
[**get_markets_region_id_history**](MarketApi.md#get_markets_region_id_history) | **Get** /markets/{region_id}/history/ | List historical market statistics in a region
[**get_markets_region_id_orders**](MarketApi.md#get_markets_region_id_orders) | **Get** /markets/{region_id}/orders/ | List orders in a region
[**get_markets_region_id_types**](MarketApi.md#get_markets_region_id_types) | **Get** /markets/{region_id}/types/ | List type IDs relevant to a market
[**get_markets_structures_structure_id**](MarketApi.md#get_markets_structures_structure_id) | **Get** /markets/structures/{structure_id}/ | List orders in a structure


# **get_characters_character_id_orders**
> Vec<::models::GetCharactersCharacterIdOrders200Ok> get_characters_character_id_orders(ctx, character_id, optional)
List open orders from a character

List open market orders placed by a character  --- Alternate route: `/dev/characters/{character_id}/orders/`  Alternate route: `/v2/characters/{character_id}/orders/`  --- This route is cached for up to 1200 seconds

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

[**Vec<::models::GetCharactersCharacterIdOrders200Ok>**](get_characters_character_id_orders_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_characters_character_id_orders_history**
> Vec<::models::GetCharactersCharacterIdOrdersHistory200Ok> get_characters_character_id_orders_history(ctx, character_id, optional)
List historical orders by a character

List cancelled and expired market orders placed by a character up to 90 days in the past.  --- Alternate route: `/dev/characters/{character_id}/orders/history/`  Alternate route: `/legacy/characters/{character_id}/orders/history/`  Alternate route: `/v1/characters/{character_id}/orders/history/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCharactersCharacterIdOrdersHistory200Ok>**](get_characters_character_id_orders_history_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_orders**
> Vec<::models::GetCorporationsCorporationIdOrders200Ok> get_corporations_corporation_id_orders(ctx, corporation_id, optional)
List open orders from a corporation

List open market orders placed on behalf of a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/orders/`  Alternate route: `/v3/corporations/{corporation_id}/orders/`  --- This route is cached for up to 1200 seconds  --- Requires one of the following EVE corporation role(s): Accountant, Trader 

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

[**Vec<::models::GetCorporationsCorporationIdOrders200Ok>**](get_corporations_corporation_id_orders_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_orders_history**
> Vec<::models::GetCorporationsCorporationIdOrdersHistory200Ok> get_corporations_corporation_id_orders_history(ctx, corporation_id, optional)
List historical orders from a corporation

List cancelled and expired market orders placed on behalf of a corporation up to 90 days in the past.  --- Alternate route: `/dev/corporations/{corporation_id}/orders/history/`  Alternate route: `/v2/corporations/{corporation_id}/orders/history/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Accountant, Trader 

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

[**Vec<::models::GetCorporationsCorporationIdOrdersHistory200Ok>**](get_corporations_corporation_id_orders_history_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_groups**
> Vec<i32> get_markets_groups(optional)
Get item groups

Get a list of item groups  --- Alternate route: `/dev/markets/groups/`  Alternate route: `/legacy/markets/groups/`  Alternate route: `/v1/markets/groups/`  --- This route expires daily at 11:05

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

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_groups_market_group_id**
> ::models::GetMarketsGroupsMarketGroupIdOk get_markets_groups_market_group_id(market_group_id, optional)
Get item group information

Get information on an item group  --- Alternate route: `/dev/markets/groups/{market_group_id}/`  Alternate route: `/legacy/markets/groups/{market_group_id}/`  Alternate route: `/v1/markets/groups/{market_group_id}/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **market_group_id** | **i32**| An Eve item group ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **market_group_id** | **i32**| An Eve item group ID | 
 **accept_language** | **String**| Language to use in the response | [default to en]
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **language** | **String**| Language to use in the response, takes precedence over Accept-Language | [default to en]

### Return type

[**::models::GetMarketsGroupsMarketGroupIdOk**](get_markets_groups_market_group_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_prices**
> Vec<::models::GetMarketsPrices200Ok> get_markets_prices(optional)
List market prices

Return a list of prices  --- Alternate route: `/dev/markets/prices/`  Alternate route: `/legacy/markets/prices/`  Alternate route: `/v1/markets/prices/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetMarketsPrices200Ok>**](get_markets_prices_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_region_id_history**
> Vec<::models::GetMarketsRegionIdHistory200Ok> get_markets_region_id_history(region_id, type_id, optional)
List historical market statistics in a region

Return a list of historical market statistics for the specified type in a region  --- Alternate route: `/dev/markets/{region_id}/history/`  Alternate route: `/legacy/markets/{region_id}/history/`  Alternate route: `/v1/markets/{region_id}/history/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **region_id** | **i32**| Return statistics in this region | 
  **type_id** | **i32**| Return statistics for this type | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **region_id** | **i32**| Return statistics in this region | 
 **type_id** | **i32**| Return statistics for this type | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**Vec<::models::GetMarketsRegionIdHistory200Ok>**](get_markets_region_id_history_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_region_id_orders**
> Vec<::models::GetMarketsRegionIdOrders200Ok> get_markets_region_id_orders(order_type, region_id, optional)
List orders in a region

Return a list of orders in a region  --- Alternate route: `/dev/markets/{region_id}/orders/`  Alternate route: `/legacy/markets/{region_id}/orders/`  Alternate route: `/v1/markets/{region_id}/orders/`  --- This route is cached for up to 300 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **order_type** | **String**| Filter buy/sell orders, return all orders by default. If you query without type_id, we always return both buy and sell orders | [default to all]
  **region_id** | **i32**| Return orders in this region | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **order_type** | **String**| Filter buy/sell orders, return all orders by default. If you query without type_id, we always return both buy and sell orders | [default to all]
 **region_id** | **i32**| Return orders in this region | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **page** | **i32**| Which page of results to return | [default to 1]
 **type_id** | **i32**| Return orders only for this type | 

### Return type

[**Vec<::models::GetMarketsRegionIdOrders200Ok>**](get_markets_region_id_orders_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_region_id_types**
> Vec<i32> get_markets_region_id_types(region_id, optional)
List type IDs relevant to a market

Return a list of type IDs that have active orders in the region, for efficient market indexing.  --- Alternate route: `/dev/markets/{region_id}/types/`  Alternate route: `/legacy/markets/{region_id}/types/`  Alternate route: `/v1/markets/{region_id}/types/`  --- This route is cached for up to 600 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **region_id** | **i32**| Return statistics in this region | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **region_id** | **i32**| Return statistics in this region | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **page** | **i32**| Which page of results to return | [default to 1]

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_markets_structures_structure_id**
> Vec<::models::GetMarketsStructuresStructureId200Ok> get_markets_structures_structure_id(ctx, structure_id, optional)
List orders in a structure

Return all orders in a structure  --- Alternate route: `/dev/markets/structures/{structure_id}/`  Alternate route: `/legacy/markets/structures/{structure_id}/`  Alternate route: `/v1/markets/structures/{structure_id}/`  --- This route is cached for up to 300 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **structure_id** | **i64**| Return orders in this structure | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **structure_id** | **i64**| Return orders in this structure | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **page** | **i32**| Which page of results to return | [default to 1]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::GetMarketsStructuresStructureId200Ok>**](get_markets_structures_structure_id_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

