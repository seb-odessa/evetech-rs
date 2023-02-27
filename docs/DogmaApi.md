# \DogmaApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dogma_attributes**](DogmaApi.md#get_dogma_attributes) | **Get** /dogma/attributes/ | Get attributes
[**get_dogma_attributes_attribute_id**](DogmaApi.md#get_dogma_attributes_attribute_id) | **Get** /dogma/attributes/{attribute_id}/ | Get attribute information
[**get_dogma_dynamic_items_type_id_item_id**](DogmaApi.md#get_dogma_dynamic_items_type_id_item_id) | **Get** /dogma/dynamic/items/{type_id}/{item_id}/ | Get dynamic item information
[**get_dogma_effects**](DogmaApi.md#get_dogma_effects) | **Get** /dogma/effects/ | Get effects
[**get_dogma_effects_effect_id**](DogmaApi.md#get_dogma_effects_effect_id) | **Get** /dogma/effects/{effect_id}/ | Get effect information


# **get_dogma_attributes**
> Vec<i32> get_dogma_attributes(optional)
Get attributes

Get a list of dogma attribute ids  --- Alternate route: `/dev/dogma/attributes/`  Alternate route: `/legacy/dogma/attributes/`  Alternate route: `/v1/dogma/attributes/`  --- This route expires daily at 11:05

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

# **get_dogma_attributes_attribute_id**
> ::models::GetDogmaAttributesAttributeIdOk get_dogma_attributes_attribute_id(attribute_id, optional)
Get attribute information

Get information on a dogma attribute  --- Alternate route: `/dev/dogma/attributes/{attribute_id}/`  Alternate route: `/legacy/dogma/attributes/{attribute_id}/`  Alternate route: `/v1/dogma/attributes/{attribute_id}/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **attribute_id** | **i32**| A dogma attribute ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **attribute_id** | **i32**| A dogma attribute ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetDogmaAttributesAttributeIdOk**](get_dogma_attributes_attribute_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dogma_dynamic_items_type_id_item_id**
> ::models::GetDogmaDynamicItemsTypeIdItemIdOk get_dogma_dynamic_items_type_id_item_id(item_id, type_id, optional)
Get dynamic item information

Returns info about a dynamic item resulting from mutation with a mutaplasmid.  --- Alternate route: `/dev/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/legacy/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/v1/dogma/dynamic/items/{type_id}/{item_id}/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **item_id** | **i64**| item_id integer | 
  **type_id** | **i32**| type_id integer | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **item_id** | **i64**| item_id integer | 
 **type_id** | **i32**| type_id integer | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetDogmaDynamicItemsTypeIdItemIdOk**](get_dogma_dynamic_items_type_id_item_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_dogma_effects**
> Vec<i32> get_dogma_effects(optional)
Get effects

Get a list of dogma effect ids  --- Alternate route: `/dev/dogma/effects/`  Alternate route: `/legacy/dogma/effects/`  Alternate route: `/v1/dogma/effects/`  --- This route expires daily at 11:05

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

# **get_dogma_effects_effect_id**
> ::models::GetDogmaEffectsEffectIdOk get_dogma_effects_effect_id(effect_id, optional)
Get effect information

Get information on a dogma effect  --- Alternate route: `/dev/dogma/effects/{effect_id}/`  Alternate route: `/v2/dogma/effects/{effect_id}/`  --- This route expires daily at 11:05

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **effect_id** | **i32**| A dogma effect ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **effect_id** | **i32**| A dogma effect ID | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

[**::models::GetDogmaEffectsEffectIdOk**](get_dogma_effects_effect_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

