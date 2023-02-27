# \AllianceApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_alliances**](AllianceApi.md#get_alliances) | **Get** /alliances/ | List all alliances
[**get_alliances_alliance_id**](AllianceApi.md#get_alliances_alliance_id) | **Get** /alliances/{alliance_id}/ | Get alliance information
[**get_alliances_alliance_id_corporations**](AllianceApi.md#get_alliances_alliance_id_corporations) | **Get** /alliances/{alliance_id}/corporations/ | List alliance&#39;s corporations
[**get_alliances_alliance_id_icons**](AllianceApi.md#get_alliances_alliance_id_icons) | **Get** /alliances/{alliance_id}/icons/ | Get alliance icon


# **get_alliances**
> Vec<i32> get_alliances(optional)
List all alliances

List all active player alliances  --- Alternate route: `/dev/alliances/`  Alternate route: `/legacy/alliances/`  Alternate route: `/v1/alliances/`  Alternate route: `/v2/alliances/`  --- This route is cached for up to 3600 seconds

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

# **get_alliances_alliance_id**
> ::models::GetAlliancesAllianceIdOk get_alliances_alliance_id(alliance_id, optional)
Get alliance information

Public information about an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/`  Alternate route: `/legacy/alliances/{alliance_id}/`  Alternate route: `/v3/alliances/{alliance_id}/`  Alternate route: `/v4/alliances/{alliance_id}/`  --- This route is cached for up to 3600 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **alliance_id** | **i32**| An EVE alliance ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alliance_id** | **i32**| An EVE alliance ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetAlliancesAllianceIdOk**](get_alliances_alliance_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_alliances_alliance_id_corporations**
> Vec<i32> get_alliances_alliance_id_corporations(alliance_id, optional)
List alliance's corporations

List all current member corporations of an alliance  --- Alternate route: `/dev/alliances/{alliance_id}/corporations/`  Alternate route: `/legacy/alliances/{alliance_id}/corporations/`  Alternate route: `/v1/alliances/{alliance_id}/corporations/`  Alternate route: `/v2/alliances/{alliance_id}/corporations/`  --- This route is cached for up to 3600 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **alliance_id** | **i32**| An EVE alliance ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alliance_id** | **i32**| An EVE alliance ID | 
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

# **get_alliances_alliance_id_icons**
> ::models::GetAlliancesAllianceIdIconsOk get_alliances_alliance_id_icons(alliance_id, optional)
Get alliance icon

Get the icon urls for a alliance  --- Alternate route: `/legacy/alliances/{alliance_id}/icons/`  Alternate route: `/v1/alliances/{alliance_id}/icons/`  --- This route expires daily at 11:05  --- [Diff of the upcoming changes](https://esi.evetech.net/diff/latest/dev/#GET-/alliances/{alliance_id}/icons/)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **alliance_id** | **i32**| An EVE alliance ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alliance_id** | **i32**| An EVE alliance ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetAlliancesAllianceIdIconsOk**](get_alliances_alliance_id_icons_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

