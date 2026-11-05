# \HardwareBaselineApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_hw_baseline_assets**](HardwareBaselineApi.md#add_hw_baseline_assets) | **POST** /api/systems/{systemId}/hw-baseline | Add one or many hardware assets in a system
[**delete_hw_baseline_assets**](HardwareBaselineApi.md#delete_hw_baseline_assets) | **DELETE** /api/systems/{systemId}/hw-baseline | Delete one or many hardware assets in a system
[**get_system_hw_baseline**](HardwareBaselineApi.md#get_system_hw_baseline) | **GET** /api/systems/{systemId}/hw-baseline | Get hardware baseline for a system
[**update_hw_baseline_assets**](HardwareBaselineApi.md#update_hw_baseline_assets) | **PUT** /api/systems/{systemId}/hw-baseline | Update one or many hardware assets in a system



## add_hw_baseline_assets

> models::HwBaselineResponsePostPut add_hw_baseline_assets(system_id, hw_baseline_required_fields)
Add one or many hardware assets in a system

Adds assets to the Hardware Baseline for given `systemId`  **Request Body Required Fields** - `assetName`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**hw_baseline_required_fields** | [**Vec<models::HwBaselineRequiredFields>**](HwBaselineRequiredFields.md) | Example request body for adding hardware baseline assets to an existing System (systemId) | [required] |

### Return type

[**models::HwBaselineResponsePostPut**](HwBaselineResponsePostPut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_hw_baseline_assets

> models::HwBaselineResponseDelete delete_hw_baseline_assets(system_id, hw_baseline_request_delete_body_inner)
Delete one or many hardware assets in a system

Remove (delete) one or multiple assets from a system Hardware Baseline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**hw_baseline_request_delete_body_inner** | [**Vec<models::HwBaselineRequestDeleteBodyInner>**](HwBaselineRequestDeleteBodyInner.md) | Example request body for deleting one or many Hardware Baseline assets | [required] |

### Return type

[**models::HwBaselineResponseDelete**](HwBaselineResponseDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_hw_baseline

> models::HwBaselineResponseGet get_system_hw_baseline(system_id, page_index, page_size)
Get hardware baseline for a system

Returns the hardware baseline for a system matching the `systemId` path parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::HwBaselineResponseGet**](HwBaselineResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_hw_baseline_assets

> models::HwBaselineResponsePostPut update_hw_baseline_assets(system_id, hw_baseline_read_only_fields)
Update one or many hardware assets in a system

Updates assets in the Hardware Baseline for given `systemId`  **Request Body Required Fields** - `assetName` - `hardwareId`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**hw_baseline_read_only_fields** | [**Vec<models::HwBaselineReadOnlyFields>**](HwBaselineReadOnlyFields.md) | Example request body for updating hardware baseline assets to an existing System (systemId) | [required] |

### Return type

[**models::HwBaselineResponsePostPut**](HwBaselineResponsePostPut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

