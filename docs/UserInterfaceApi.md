# \UserInterfaceApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_ui_autopilot_waypoint**](UserInterfaceApi.md#post_ui_autopilot_waypoint) | **Post** /ui/autopilot/waypoint/ | Set Autopilot Waypoint
[**post_ui_openwindow_contract**](UserInterfaceApi.md#post_ui_openwindow_contract) | **Post** /ui/openwindow/contract/ | Open Contract Window
[**post_ui_openwindow_information**](UserInterfaceApi.md#post_ui_openwindow_information) | **Post** /ui/openwindow/information/ | Open Information Window
[**post_ui_openwindow_marketdetails**](UserInterfaceApi.md#post_ui_openwindow_marketdetails) | **Post** /ui/openwindow/marketdetails/ | Open Market Details
[**post_ui_openwindow_newmail**](UserInterfaceApi.md#post_ui_openwindow_newmail) | **Post** /ui/openwindow/newmail/ | Open New Mail Window


# **post_ui_autopilot_waypoint**
> post_ui_autopilot_waypoint(ctx, add_to_beginning, clear_other_waypoints, destination_id, optional)
Set Autopilot Waypoint

Set a solar system as autopilot waypoint  --- Alternate route: `/dev/ui/autopilot/waypoint/`  Alternate route: `/legacy/ui/autopilot/waypoint/`  Alternate route: `/v2/ui/autopilot/waypoint/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **add_to_beginning** | **bool**| Whether this solar system should be added to the beginning of all waypoints | [default to false]
  **clear_other_waypoints** | **bool**| Whether clean other waypoints beforing adding this one | [default to false]
  **destination_id** | **i64**| The destination to travel to, can be solar system, station or structure&#39;s id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **add_to_beginning** | **bool**| Whether this solar system should be added to the beginning of all waypoints | [default to false]
 **clear_other_waypoints** | **bool**| Whether clean other waypoints beforing adding this one | [default to false]
 **destination_id** | **i64**| The destination to travel to, can be solar system, station or structure&#39;s id | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_ui_openwindow_contract**
> post_ui_openwindow_contract(ctx, contract_id, optional)
Open Contract Window

Open the contract window inside the client  --- Alternate route: `/dev/ui/openwindow/contract/`  Alternate route: `/legacy/ui/openwindow/contract/`  Alternate route: `/v1/ui/openwindow/contract/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **contract_id** | **i32**| The contract to open | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **contract_id** | **i32**| The contract to open | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_ui_openwindow_information**
> post_ui_openwindow_information(ctx, target_id, optional)
Open Information Window

Open the information window for a character, corporation or alliance inside the client  --- Alternate route: `/dev/ui/openwindow/information/`  Alternate route: `/legacy/ui/openwindow/information/`  Alternate route: `/v1/ui/openwindow/information/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **target_id** | **i32**| The target to open | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **target_id** | **i32**| The target to open | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_ui_openwindow_marketdetails**
> post_ui_openwindow_marketdetails(ctx, type_id, optional)
Open Market Details

Open the market details window for a specific typeID inside the client  --- Alternate route: `/dev/ui/openwindow/marketdetails/`  Alternate route: `/legacy/ui/openwindow/marketdetails/`  Alternate route: `/v1/ui/openwindow/marketdetails/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **type_id** | **i32**| The item type to open in market window | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **type_id** | **i32**| The item type to open in market window | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_ui_openwindow_newmail**
> post_ui_openwindow_newmail(ctx, new_mail, optional)
Open New Mail Window

Open the New Mail window, according to settings from the request if applicable  --- Alternate route: `/dev/ui/openwindow/newmail/`  Alternate route: `/legacy/ui/openwindow/newmail/`  Alternate route: `/v1/ui/openwindow/newmail/` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **new_mail** | [**PostUiOpenwindowNewmailNewMail**](PostUiOpenwindowNewmailNewMail.md)| The details of mail to create | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **new_mail** | [**PostUiOpenwindowNewmailNewMail**](PostUiOpenwindowNewmailNewMail.md)| The details of mail to create | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

