# \SystemWorkflowsDashboardsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_workflows_history_details**](SystemWorkflowsDashboardsApi.md#get_system_workflows_history_details) | **GET** /api/dashboards/system-workflows-history-details | System Workflows History Details
[**get_system_workflows_history_stage_details**](SystemWorkflowsDashboardsApi.md#get_system_workflows_history_stage_details) | **GET** /api/dashboards/system-workflows-history-stage-details | System Workflows History Stage Details
[**get_system_workflows_history_summary**](SystemWorkflowsDashboardsApi.md#get_system_workflows_history_summary) | **GET** /api/dashboards/system-workflows-history-summary | System Workflows History Summary



## get_system_workflows_history_details

> models::GetSystemStatusDetails200Response get_system_workflows_history_details(org_id, excludeinherited, page_index, page_size)
System Workflows History Details

Get systems workflow history details dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_workflows_history_stage_details

> models::GetSystemStatusDetails200Response get_system_workflows_history_stage_details(org_id, excludeinherited, page_index, page_size)
System Workflows History Stage Details

Get systems workflow history stage details dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_workflows_history_summary

> models::GetSystemStatusDetails200Response get_system_workflows_history_summary(org_id, excludeinherited, page_index, page_size)
System Workflows History Summary

Get systems workflow history summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

