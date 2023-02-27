# \SearchApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_search**](SearchApi.md#get_characters_character_id_search) | **Get** /characters/{character_id}/search/ | Search on a string


# **get_characters_character_id_search**
> ::models::GetCharactersCharacterIdSearchOk get_characters_character_id_search(ctx, categories, character_id, search, optional)
Search on a string

Search for entities that match a given sub-string.  --- Alternate route: `/dev/characters/{character_id}/search/`  Alternate route: `/legacy/characters/{character_id}/search/`  Alternate route: `/v3/characters/{character_id}/search/`  --- This route is cached for up to 3600 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **categories** | [**Vec&lt;String&gt;**](String.md)| Type of entities to search for | 
  **character_id** | **i32**| An EVE character ID | 
  **search** | **String**| The string to search on | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **categories** | [**Vec&lt;String&gt;**](String.md)| Type of entities to search for | 
 **character_id** | **i32**| An EVE character ID | 
 **search** | **String**| The string to search on | 
 **accept_language** | **String**| Language to use in the response | [default to en]
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **language** | **String**| Language to use in the response, takes precedence over Accept-Language | [default to en]
 **strict** | **bool**| Whether the search should be a strict match | [default to false]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**::models::GetCharactersCharacterIdSearchOk**](get_characters_character_id_search_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

