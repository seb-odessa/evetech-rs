# \CorporationApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_corporations_corporation_id**](CorporationApi.md#get_corporations_corporation_id) | **Get** /corporations/{corporation_id}/ | Get corporation information
[**get_corporations_corporation_id_alliancehistory**](CorporationApi.md#get_corporations_corporation_id_alliancehistory) | **Get** /corporations/{corporation_id}/alliancehistory/ | Get alliance history
[**get_corporations_corporation_id_blueprints**](CorporationApi.md#get_corporations_corporation_id_blueprints) | **Get** /corporations/{corporation_id}/blueprints/ | Get corporation blueprints
[**get_corporations_corporation_id_containers_logs**](CorporationApi.md#get_corporations_corporation_id_containers_logs) | **Get** /corporations/{corporation_id}/containers/logs/ | Get all corporation ALSC logs
[**get_corporations_corporation_id_divisions**](CorporationApi.md#get_corporations_corporation_id_divisions) | **Get** /corporations/{corporation_id}/divisions/ | Get corporation divisions
[**get_corporations_corporation_id_facilities**](CorporationApi.md#get_corporations_corporation_id_facilities) | **Get** /corporations/{corporation_id}/facilities/ | Get corporation facilities
[**get_corporations_corporation_id_icons**](CorporationApi.md#get_corporations_corporation_id_icons) | **Get** /corporations/{corporation_id}/icons/ | Get corporation icon
[**get_corporations_corporation_id_medals**](CorporationApi.md#get_corporations_corporation_id_medals) | **Get** /corporations/{corporation_id}/medals/ | Get corporation medals
[**get_corporations_corporation_id_medals_issued**](CorporationApi.md#get_corporations_corporation_id_medals_issued) | **Get** /corporations/{corporation_id}/medals/issued/ | Get corporation issued medals
[**get_corporations_corporation_id_members**](CorporationApi.md#get_corporations_corporation_id_members) | **Get** /corporations/{corporation_id}/members/ | Get corporation members
[**get_corporations_corporation_id_members_limit**](CorporationApi.md#get_corporations_corporation_id_members_limit) | **Get** /corporations/{corporation_id}/members/limit/ | Get corporation member limit
[**get_corporations_corporation_id_members_titles**](CorporationApi.md#get_corporations_corporation_id_members_titles) | **Get** /corporations/{corporation_id}/members/titles/ | Get corporation&#39;s members&#39; titles
[**get_corporations_corporation_id_membertracking**](CorporationApi.md#get_corporations_corporation_id_membertracking) | **Get** /corporations/{corporation_id}/membertracking/ | Track corporation members
[**get_corporations_corporation_id_roles**](CorporationApi.md#get_corporations_corporation_id_roles) | **Get** /corporations/{corporation_id}/roles/ | Get corporation member roles
[**get_corporations_corporation_id_roles_history**](CorporationApi.md#get_corporations_corporation_id_roles_history) | **Get** /corporations/{corporation_id}/roles/history/ | Get corporation member roles history
[**get_corporations_corporation_id_shareholders**](CorporationApi.md#get_corporations_corporation_id_shareholders) | **Get** /corporations/{corporation_id}/shareholders/ | Get corporation shareholders
[**get_corporations_corporation_id_standings**](CorporationApi.md#get_corporations_corporation_id_standings) | **Get** /corporations/{corporation_id}/standings/ | Get corporation standings
[**get_corporations_corporation_id_starbases**](CorporationApi.md#get_corporations_corporation_id_starbases) | **Get** /corporations/{corporation_id}/starbases/ | Get corporation starbases (POSes)
[**get_corporations_corporation_id_starbases_starbase_id**](CorporationApi.md#get_corporations_corporation_id_starbases_starbase_id) | **Get** /corporations/{corporation_id}/starbases/{starbase_id}/ | Get starbase (POS) detail
[**get_corporations_corporation_id_structures**](CorporationApi.md#get_corporations_corporation_id_structures) | **Get** /corporations/{corporation_id}/structures/ | Get corporation structures
[**get_corporations_corporation_id_titles**](CorporationApi.md#get_corporations_corporation_id_titles) | **Get** /corporations/{corporation_id}/titles/ | Get corporation titles
[**get_corporations_npccorps**](CorporationApi.md#get_corporations_npccorps) | **Get** /corporations/npccorps/ | Get npc corporations


# **get_corporations_corporation_id**
> ::models::GetCorporationsCorporationIdOk get_corporations_corporation_id(corporation_id, optional)
Get corporation information

Public information about a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/`  Alternate route: `/v5/corporations/{corporation_id}/`  --- This route is cached for up to 3600 seconds

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

[**::models::GetCorporationsCorporationIdOk**](get_corporations_corporation_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_alliancehistory**
> Vec<::models::GetCorporationsCorporationIdAlliancehistory200Ok> get_corporations_corporation_id_alliancehistory(corporation_id, optional)
Get alliance history

Get a list of all the alliances a corporation has been a member of  --- Alternate route: `/dev/corporations/{corporation_id}/alliancehistory/`  Alternate route: `/v3/corporations/{corporation_id}/alliancehistory/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCorporationsCorporationIdAlliancehistory200Ok>**](get_corporations_corporation_id_alliancehistory_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_blueprints**
> Vec<::models::GetCorporationsCorporationIdBlueprints200Ok> get_corporations_corporation_id_blueprints(ctx, corporation_id, optional)
Get corporation blueprints

Returns a list of blueprints the corporation owns  --- Alternate route: `/dev/corporations/{corporation_id}/blueprints/`  Alternate route: `/v3/corporations/{corporation_id}/blueprints/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdBlueprints200Ok>**](get_corporations_corporation_id_blueprints_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_containers_logs**
> Vec<::models::GetCorporationsCorporationIdContainersLogs200Ok> get_corporations_corporation_id_containers_logs(ctx, corporation_id, optional)
Get all corporation ALSC logs

Returns logs recorded in the past seven days from all audit log secure containers (ALSC) owned by a given corporation  --- Alternate route: `/dev/corporations/{corporation_id}/containers/logs/`  Alternate route: `/v3/corporations/{corporation_id}/containers/logs/`  --- This route is cached for up to 600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdContainersLogs200Ok>**](get_corporations_corporation_id_containers_logs_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_divisions**
> ::models::GetCorporationsCorporationIdDivisionsOk get_corporations_corporation_id_divisions(ctx, corporation_id, optional)
Get corporation divisions

Return corporation hangar and wallet division names, only show if a division is not using the default name  --- Alternate route: `/dev/corporations/{corporation_id}/divisions/`  Alternate route: `/v2/corporations/{corporation_id}/divisions/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**::models::GetCorporationsCorporationIdDivisionsOk**](get_corporations_corporation_id_divisions_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_facilities**
> Vec<::models::GetCorporationsCorporationIdFacilities200Ok> get_corporations_corporation_id_facilities(ctx, corporation_id, optional)
Get corporation facilities

Return a corporation's facilities  --- Alternate route: `/dev/corporations/{corporation_id}/facilities/`  Alternate route: `/v2/corporations/{corporation_id}/facilities/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Factory_Manager 

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

[**Vec<::models::GetCorporationsCorporationIdFacilities200Ok>**](get_corporations_corporation_id_facilities_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_icons**
> ::models::GetCorporationsCorporationIdIconsOk get_corporations_corporation_id_icons(corporation_id, optional)
Get corporation icon

Get the icon urls for a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/icons/`  Alternate route: `/v2/corporations/{corporation_id}/icons/`  --- This route is cached for up to 3600 seconds

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

[**::models::GetCorporationsCorporationIdIconsOk**](get_corporations_corporation_id_icons_ok.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_medals**
> Vec<::models::GetCorporationsCorporationIdMedals200Ok> get_corporations_corporation_id_medals(ctx, corporation_id, optional)
Get corporation medals

Returns a corporation's medals  --- Alternate route: `/dev/corporations/{corporation_id}/medals/`  Alternate route: `/v2/corporations/{corporation_id}/medals/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCorporationsCorporationIdMedals200Ok>**](get_corporations_corporation_id_medals_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_medals_issued**
> Vec<::models::GetCorporationsCorporationIdMedalsIssued200Ok> get_corporations_corporation_id_medals_issued(ctx, corporation_id, optional)
Get corporation issued medals

Returns medals issued by a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/medals/issued/`  Alternate route: `/v2/corporations/{corporation_id}/medals/issued/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdMedalsIssued200Ok>**](get_corporations_corporation_id_medals_issued_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_members**
> Vec<i32> get_corporations_corporation_id_members(ctx, corporation_id, optional)
Get corporation members

Return the current member list of a corporation, the token's character need to be a member of the corporation.  --- Alternate route: `/dev/corporations/{corporation_id}/members/`  Alternate route: `/v4/corporations/{corporation_id}/members/`  --- This route is cached for up to 3600 seconds

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

**Vec<i32>**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_members_limit**
> i32 get_corporations_corporation_id_members_limit(ctx, corporation_id, optional)
Get corporation member limit

Return a corporation's member limit, not including CEO himself  --- Alternate route: `/dev/corporations/{corporation_id}/members/limit/`  Alternate route: `/v2/corporations/{corporation_id}/members/limit/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

**i32**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_members_titles**
> Vec<::models::GetCorporationsCorporationIdMembersTitles200Ok> get_corporations_corporation_id_members_titles(ctx, corporation_id, optional)
Get corporation's members' titles

Returns a corporation's members' titles  --- Alternate route: `/dev/corporations/{corporation_id}/members/titles/`  Alternate route: `/v2/corporations/{corporation_id}/members/titles/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdMembersTitles200Ok>**](get_corporations_corporation_id_members_titles_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_membertracking**
> Vec<::models::GetCorporationsCorporationIdMembertracking200Ok> get_corporations_corporation_id_membertracking(ctx, corporation_id, optional)
Track corporation members

Returns additional information about a corporation's members which helps tracking their activities  --- Alternate route: `/dev/corporations/{corporation_id}/membertracking/`  Alternate route: `/v2/corporations/{corporation_id}/membertracking/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdMembertracking200Ok>**](get_corporations_corporation_id_membertracking_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_roles**
> Vec<::models::GetCorporationsCorporationIdRoles200Ok> get_corporations_corporation_id_roles(ctx, corporation_id, optional)
Get corporation member roles

Return the roles of all members if the character has the personnel manager role or any grantable role.  --- Alternate route: `/dev/corporations/{corporation_id}/roles/`  Alternate route: `/v2/corporations/{corporation_id}/roles/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCorporationsCorporationIdRoles200Ok>**](get_corporations_corporation_id_roles_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_roles_history**
> Vec<::models::GetCorporationsCorporationIdRolesHistory200Ok> get_corporations_corporation_id_roles_history(ctx, corporation_id, optional)
Get corporation member roles history

Return how roles have changed for a coporation's members, up to a month  --- Alternate route: `/dev/corporations/{corporation_id}/roles/history/`  Alternate route: `/v2/corporations/{corporation_id}/roles/history/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdRolesHistory200Ok>**](get_corporations_corporation_id_roles_history_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_shareholders**
> Vec<::models::GetCorporationsCorporationIdShareholders200Ok> get_corporations_corporation_id_shareholders(ctx, corporation_id, optional)
Get corporation shareholders

Return the current shareholders of a corporation.  --- Alternate route: `/dev/corporations/{corporation_id}/shareholders/`  Alternate route: `/legacy/corporations/{corporation_id}/shareholders/`  Alternate route: `/v1/corporations/{corporation_id}/shareholders/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdShareholders200Ok>**](get_corporations_corporation_id_shareholders_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_standings**
> Vec<::models::GetCorporationsCorporationIdStandings200Ok> get_corporations_corporation_id_standings(ctx, corporation_id, optional)
Get corporation standings

Return corporation standings from agents, NPC corporations, and factions  --- Alternate route: `/dev/corporations/{corporation_id}/standings/`  Alternate route: `/v2/corporations/{corporation_id}/standings/`  --- This route is cached for up to 3600 seconds

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

[**Vec<::models::GetCorporationsCorporationIdStandings200Ok>**](get_corporations_corporation_id_standings_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_starbases**
> Vec<::models::GetCorporationsCorporationIdStarbases200Ok> get_corporations_corporation_id_starbases(ctx, corporation_id, optional)
Get corporation starbases (POSes)

Returns list of corporation starbases (POSes)  --- Alternate route: `/dev/corporations/{corporation_id}/starbases/`  Alternate route: `/v2/corporations/{corporation_id}/starbases/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdStarbases200Ok>**](get_corporations_corporation_id_starbases_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_starbases_starbase_id**
> ::models::GetCorporationsCorporationIdStarbasesStarbaseIdOk get_corporations_corporation_id_starbases_starbase_id(ctx, corporation_id, starbase_id, system_id, optional)
Get starbase (POS) detail

Returns various settings and fuels of a starbase (POS)  --- Alternate route: `/dev/corporations/{corporation_id}/starbases/{starbase_id}/`  Alternate route: `/v2/corporations/{corporation_id}/starbases/{starbase_id}/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **corporation_id** | **i32**| An EVE corporation ID | 
  **starbase_id** | **i64**| An EVE starbase (POS) ID | 
  **system_id** | **i32**| The solar system this starbase (POS) is located in, | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **corporation_id** | **i32**| An EVE corporation ID | 
 **starbase_id** | **i64**| An EVE starbase (POS) ID | 
 **system_id** | **i32**| The solar system this starbase (POS) is located in, | 
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**::models::GetCorporationsCorporationIdStarbasesStarbaseIdOk**](get_corporations_corporation_id_starbases_starbase_id_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_structures**
> Vec<::models::GetCorporationsCorporationIdStructures200Ok> get_corporations_corporation_id_structures(ctx, corporation_id, optional)
Get corporation structures

Get a list of corporation structures. This route's version includes the changes to structures detailed in this blog: https://www.eveonline.com/article/upwell-2.0-structures-changes-coming-on-february-13th  --- Alternate route: `/dev/corporations/{corporation_id}/structures/`  Alternate route: `/v4/corporations/{corporation_id}/structures/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Station_Manager 

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
 **accept_language** | **String**| Language to use in the response | [default to en]
 **datasource** | **String**| The server name you would like data from | [default to tranquility]
 **if_none_match** | **String**| ETag from a previous request. A 304 will be returned if this matches the current ETag | 
 **language** | **String**| Language to use in the response, takes precedence over Accept-Language | [default to en]
 **page** | **i32**| Which page of results to return | [default to 1]
 **token** | **String**| Access token to use if unable to set a header | 

### Return type

[**Vec<::models::GetCorporationsCorporationIdStructures200Ok>**](get_corporations_corporation_id_structures_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_corporation_id_titles**
> Vec<::models::GetCorporationsCorporationIdTitles200Ok> get_corporations_corporation_id_titles(ctx, corporation_id, optional)
Get corporation titles

Returns a corporation's titles  --- Alternate route: `/dev/corporations/{corporation_id}/titles/`  Alternate route: `/v2/corporations/{corporation_id}/titles/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

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

[**Vec<::models::GetCorporationsCorporationIdTitles200Ok>**](get_corporations_corporation_id_titles_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_corporations_npccorps**
> Vec<i32> get_corporations_npccorps(optional)
Get npc corporations

Get a list of npc corporations  --- Alternate route: `/dev/corporations/npccorps/`  Alternate route: `/v2/corporations/npccorps/`  --- This route expires daily at 11:05

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

