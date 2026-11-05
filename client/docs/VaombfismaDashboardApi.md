# \VaombfismaDashboardApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_va_omb_fsma_saop_summary**](VaombfismaDashboardApi.md#get_va_omb_fsma_saop_summary) | **GET** /api/dashboards/va-omb-fisma-saop-summary | VA OMB FISMA SAOP Summary



## get_va_omb_fsma_saop_summary

> models::GetSystemStatusDetails200Response get_va_omb_fsma_saop_summary(org_id, excludeinherited, page_index, page_size)
VA OMB FISMA SAOP Summary

Get VA OMB-FISMA SAOP summary dashboard information.

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

