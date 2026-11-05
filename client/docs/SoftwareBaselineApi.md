# \SoftwareBaselineApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_sw_baseline_assets**](SoftwareBaselineApi.md#add_sw_baseline_assets) | **POST** /api/systems/{systemId}/sw-baseline | Add one or many software assets in a system
[**delete_sw_baseline_assets**](SoftwareBaselineApi.md#delete_sw_baseline_assets) | **DELETE** /api/systems/{systemId}/sw-baseline | Delete one or many software assets in a system
[**get_system_sw_baseline**](SoftwareBaselineApi.md#get_system_sw_baseline) | **GET** /api/systems/{systemId}/sw-baseline | Get software baseline for a system
[**update_sw_baseline_assets**](SoftwareBaselineApi.md#update_sw_baseline_assets) | **PUT** /api/systems/{systemId}/sw-baseline | Update one or many software assets in a system



## add_sw_baseline_assets

> models::SwBaselineResponsePostPut add_sw_baseline_assets(system_id, sw_baseline_required_fields)
Add one or many software assets in a system

Adds assets to the Software Baseline for given `systemId`  **Request Body Required Fields** - `softwareVendor` - `softwareName` - `version`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**sw_baseline_required_fields** | [**Vec<models::SwBaselineRequiredFields>**](SwBaselineRequiredFields.md) | Example request body for adding software baseline assets to an existing System (systemId) | [required] |

### Return type

[**models::SwBaselineResponsePostPut**](SwBaselineResponsePostPut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sw_baseline_assets

> models::SwBaselineResponseDelete delete_sw_baseline_assets(system_id, sw_baseline_request_delete_body_inner)
Delete one or many software assets in a system

Remove (delete) one or multiple assets from a system Software Baseline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**sw_baseline_request_delete_body_inner** | [**Vec<models::SwBaselineRequestDeleteBodyInner>**](SwBaselineRequestDeleteBodyInner.md) | Example request body for deleting one or many Software Baseline assets | [required] |

### Return type

[**models::SwBaselineResponseDelete**](SwBaselineResponseDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_sw_baseline

> models::SwBaselineResponseGet get_system_sw_baseline(system_id, page_index, page_size)
Get software baseline for a system

Returns the software baseline for a system matching the `systemId` path parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::SwBaselineResponseGet**](SwBaselineResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sw_baseline_assets

> models::SwBaselineResponsePostPut update_sw_baseline_assets(system_id, sw_baseline_read_only_fields)
Update one or many software assets in a system

Updates assets in the Software Baseline for given `systemId`  **Request Body Required Fields** - `softwareId` - `softwareVendor` - `softwareName` - `version`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**sw_baseline_read_only_fields** | [**Vec<models::SwBaselineReadOnlyFields>**](SwBaselineReadOnlyFields.md) | Example request body for updating software baseline assets to an existing System (systemId) | [required] |

### Return type

[**models::SwBaselineResponsePostPut**](SwBaselineResponsePostPut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

