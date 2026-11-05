# \ArtifactsExportApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_artifacts_export**](ArtifactsExportApi.md#get_system_artifacts_export) | **GET** /api/systems/{systemId}/artifacts-export | Get the file of an artifact in a system



## get_system_artifacts_export

> std::path::PathBuf get_system_artifacts_export(system_id, filename, compress)
Get the file of an artifact in a system

<strong>Sample Responce</strong><br>  Binary file associated with given filename.<br>  If `compress` parameter is specified, zip archive of binary file associated with given filename.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**filename** | **String** | **File Name**: The file name (to include file-extension). | [required] |
**compress** | Option<**bool**> | **Compress File**: Determines if returned file is compressed. |  |[default to true]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

