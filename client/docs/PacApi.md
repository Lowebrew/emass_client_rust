# \PacApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_system_pac**](PacApi.md#add_system_pac) | **POST** /api/systems/{systemId}/approval/pac | Initiate system workflow for review
[**get_system_pac**](PacApi.md#get_system_pac) | **GET** /api/systems/{systemId}/approval/pac | Get status of active workflows in a system



## add_system_pac

> models::PacResponsePost add_system_pac(system_id, serde_json_colon_colon_value)
Initiate system workflow for review

Adds a Package Approval Chain (PAC) for given `systemId` path parameter  **Request Body Required Fields** - `workflow` - `name` - `comments`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for adding system package to PAC for review | [required] |

### Return type

[**models::PacResponsePost**](PacResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_pac

> models::PacResponseGet get_system_pac(system_id)
Get status of active workflows in a system

Returns the location of a system's package in the Package Approval Chain (PAC) for matching `systemId` path parameter  **NOTES:** - If the indicated system has any active workflows, the response will include information   such as the workflow type and the current stage of each workflow. - If there are no active workflows, then a null data member will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |

### Return type

[**models::PacResponseGet**](PacResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

