# \StaticCodeScansApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_static_code_scans_by_system_id**](StaticCodeScansApi.md#add_static_code_scans_by_system_id) | **POST** /api/systems/{systemId}/static-code-scans | Upload static code scans or Clear static code scans



## add_static_code_scans_by_system_id

> models::StaticCodeResponsePost add_static_code_scans_by_system_id(system_id, static_code_request_post_body)
Upload static code scans or Clear static code scans

Upload or clear application scan findings into a system's `systemId` assets module.  **Request Body Required Fields** - `application` (Object)   - `applicationName`   - `version` - `applicationFindings` (Object Array)   - `codeCheckName`   - `count`   - `scanDate`   - `cweId`  **NOTE:** To clear an application's findings, use only the field `clearFindings` as the Request body and set it to true. Example: ``` [   {     \"application\": {       \"applicationName\": \"application name\",       \"version\": \"application version\"     },     \"applicationFindings\": [       { \"clearFindings\": true }     ]   } ] ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**static_code_request_post_body** | [**StaticCodeRequestPostBody**](StaticCodeRequestPostBody.md) | Example request body for adding static code scans or Clear static code scans | [required] |

### Return type

[**models::StaticCodeResponsePost**](StaticCodeResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

