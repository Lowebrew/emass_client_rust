# \TestResultsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_test_results_by_system_id**](TestResultsApi.md#add_test_results_by_system_id) | **POST** /api/systems/{systemId}/test-results | Add one or many test results in a system
[**get_system_test_results**](TestResultsApi.md#get_system_test_results) | **GET** /api/systems/{systemId}/test-results | Get one or many test results in a system



## add_test_results_by_system_id

> models::TestResultsResponsePost add_test_results_by_system_id(system_id, serde_json_colon_colon_value)
Add one or many test results in a system

Adds test results for given `systemId`  **Request Body Required Fields** - `testedBy` - `testDate` - `description` - `complianceStatus` - `assessmentProcedure`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body to add test results to a system (systemId) | [required] |

### Return type

[**models::TestResultsResponsePost**](TestResultsResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_test_results

> models::TestResultsResponseGet get_system_test_results(system_id, control_acronyms, assessment_procedures, ccis, latest_only)
Get one or many test results in a system

Returns system test results information for matching parameters.<br>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**control_acronyms** | Option<**String**> | **Control Acronym**: Filter query by given system acronym (single value or comma separated). |  |
**assessment_procedures** | Option<**String**> | **Assessment Procedure**: Filter query by given Security Control Assessment Procedure (single value or comma separated). |  |
**ccis** | Option<**String**> | **CCI System**: Filter query by Control Correlation Identifiers (CCIs) (single value or comma separated). |  |
**latest_only** | Option<**bool**> | **Latest Results Only**: Indicates that only the latest test resultes are retrieved. |  |[default to true]

### Return type

[**models::TestResultsResponseGet**](TestResultsResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

