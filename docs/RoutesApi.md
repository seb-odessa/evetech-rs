# \RoutesApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_route_origin_destination**](RoutesApi.md#get_route_origin_destination) | **Get** /route/{origin}/{destination}/ | Get route


# **get_route_origin_destination**
> Vec<i32> get_route_origin_destination(destination, origin, optional)
Get route

Get the systems between origin and destination  --- Alternate route: `/dev/route/{origin}/{destination}/`  Alternate route: `/legacy/route/{origin}/{destination}/`  Alternate route: `/v1/route/{origin}/{destination}/`  --- This route is cached for up to 86400 seconds

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **destination** | **i32**| destination solar system ID | 
  **origin** | **i32**| origin solar system ID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **destination** | **i32**| destination solar system ID | 
 **origin** | **i32**| origin solar system ID | 
 **avoid** | [**Vec&lt;i32&gt;**](i32.md)| avoid solar system ID(s) | 
 **connections** | [**Vec&lt;Vec&lt;i32&gt;&gt;**](Vec&lt;i32&gt;.md)| connected solar system pairs | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **flag** | **String**| route security preference | [default to shortest]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

