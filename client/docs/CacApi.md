# \CacApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_system_cac**](CacApi.md#add_system_cac) | **POST** /api/systems/{systemId}/approval/cac | Submit control to second role of CAC
[**get_system_cac**](CacApi.md#get_system_cac) | **GET** /api/systems/{systemId}/approval/cac | Get location of one or many controls in CAC



## add_system_cac

> models::CacResponsePost add_system_cac(system_id, serde_json_colon_colon_value)
Submit control to second role of CAC

**Request Body Required Fields** - `controlAcronym` - `comments`  **NOTES:** - Comments `comments` are not required at the first role of the CAC but are required at the second role of the CAC. Comments cannot exceed 10,000 characters. - POST requests will only yield successful results if the control is currently sitting at the first role of the CAC. If the control is not currently sitting at the first role, then an error will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for adding control(s) to second role of CAC | [required] |

### Return type

[**models::CacResponsePost**](CacResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_cac

> models::CacResponseGet get_system_cac(system_id, control_acronyms)
Get location of one or many controls in CAC

Returns the location of a system's package in the Control Approval Chain (CAC) for matching `systemId` path parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**control_acronyms** | Option<**String**> | **Control Acronym**: Filter query by given system acronym (single value or comma separated). |  |

### Return type

[**models::CacResponseGet**](CacResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

