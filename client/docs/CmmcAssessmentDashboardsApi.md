# \CmmcAssessmentDashboardsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cmmc_assessment_requirement_objectives_details**](CmmcAssessmentDashboardsApi.md#get_cmmc_assessment_requirement_objectives_details) | **GET** /api/dashboards/cmmc-assessment-requirement-objectives-details | CMMC Assessment Requirement Objectives Details
[**get_cmmc_assessment_requirements_compliance_summary**](CmmcAssessmentDashboardsApi.md#get_cmmc_assessment_requirements_compliance_summary) | **GET** /api/dashboards/cmmc-assessment-requirements-compliance-summary | CMMC Assessment Requirements Compliance Summary
[**get_cmmc_assessment_security_requirements_details**](CmmcAssessmentDashboardsApi.md#get_cmmc_assessment_security_requirements_details) | **GET** /api/dashboards/cmmc-assessment-security-requirements-details | CMMC Assessment Security Requirements Details
[**get_cmmc_assessment_status_summary**](CmmcAssessmentDashboardsApi.md#get_cmmc_assessment_status_summary) | **GET** /api/dashboards/cmmc-assessment-status-summary | CMMC Assessment Status Summary



## get_cmmc_assessment_requirement_objectives_details

> models::GetSystemStatusDetails200Response get_cmmc_assessment_requirement_objectives_details(org_id, excludeinherited, page_index, page_size)
CMMC Assessment Requirement Objectives Details

Get CMMC assessment requirement objectives details dashboard information.

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


## get_cmmc_assessment_requirements_compliance_summary

> models::GetSystemStatusDetails200Response get_cmmc_assessment_requirements_compliance_summary(org_id, excludeinherited, page_index, page_size)
CMMC Assessment Requirements Compliance Summary

Get CMMC assessment requirements compliance summary dashboard information.

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


## get_cmmc_assessment_security_requirements_details

> models::GetSystemStatusDetails200Response get_cmmc_assessment_security_requirements_details(org_id, excludeinherited, page_index, page_size)
CMMC Assessment Security Requirements Details

Get CMMC assessment security requirements details dashboard information.

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


## get_cmmc_assessment_status_summary

> models::GetSystemStatusDetails200Response get_cmmc_assessment_status_summary(org_id, excludeinherited, page_index, page_size)
CMMC Assessment Status Summary

Get CMMC assessment status summary dashboard information.

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

