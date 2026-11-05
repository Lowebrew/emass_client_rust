# \SystemsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system**](SystemsApi.md#get_system) | **GET** /api/systems/{systemId} | Get system information for a specific system
[**get_systems**](SystemsApi.md#get_systems) | **GET** /api/systems | Get system information



## get_system

> models::SystemResponse get_system(system_id, policy)
Get system information for a specific system

Returns the system matching provided parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**policy** | Option<**String**> | **System Policy**: Filter query by system policy. If no value is specified, the default returns RMF policy information for dual-policy systems. |  |[default to rmf]

### Return type

[**models::SystemResponse**](SystemResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_systems

> models::SystemsResponse get_systems(coams_id, ditpr_id, include_decommissioned, include_ditpr_metrics, policy, registration_type, reports_for_scorecard)
Get system information

Returns all system(s) that match the query parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coams_id** | Option<**String**> | **COAMS ID**: Filter query by Cyber Operational Attributes Management System (COAMS). |  |
**ditpr_id** | Option<**String**> | **DITPR ID**: Filter query by DoD Information Technology (IT) Portfolio Repository (DITPR). |  |
**include_decommissioned** | Option<**bool**> | **Include Decommissioned Systems**: Indicates if decommissioned systems are retrieved. If no value is specified, the default returns true to include decommissioned systems. |  |[default to true]
**include_ditpr_metrics** | Option<**bool**> | **Include DITPR**: Indicates if DITPR metrics are retrieved. This query string parameter cannot be used in conjunction with the following parameters:   - ditprId   - coamsId  If no value is specified, the default returns false to not include DITPR Metrics. |  |[default to false]
**policy** | Option<**String**> | **System Policy**: Filter query by system policy. If no value is specified, the default returns RMF policy information for dual-policy systems. |  |[default to rmf]
**registration_type** | Option<**String**> | **Registration Type**: Filter record by selected registration type (single value or comma delimited values).  **Valid Options Are:** assessAndAuthorize, assessOnly, guest, regular, functional, cloudServiceProvider, commonControlProvider, authorizationToUse, reciprocityAcceptanc  |  |[default to regular]
**reports_for_scorecard** | Option<**bool**> | **DoD Cyber Hygiene Scorecard**: Used to filter results to only return systems that report to the DoD Cyber Hygiene Scorecard. |  |[default to true]

### Return type

[**models::SystemsResponse**](SystemsResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

