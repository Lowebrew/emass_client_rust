# \CmmcAssessmentsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cmmc_assessments**](CmmcAssessmentsApi.md#get_cmmc_assessments) | **GET** /api/cmmc-assessments | Get CMMC assessment information



## get_cmmc_assessments

> models::CmmcResponseGet get_cmmc_assessments(since_date)
Get CMMC assessment information

Get all CMMC assessment after the given date `sinceDate` parameter. It is available to CMMC eMASS only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since_date** | **String** | **Date** CMMC date (Unix date format) | [required] |

### Return type

[**models::CmmcResponseGet**](CmmcResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

