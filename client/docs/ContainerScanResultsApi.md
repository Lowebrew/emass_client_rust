# \ContainerScanResultsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_container_sans_by_system_id**](ContainerScanResultsApi.md#add_container_sans_by_system_id) | **POST** /api/systems/{systemId}/container-scan-results | Add one or many containers and their scan results
[**delete_container_sans**](ContainerScanResultsApi.md#delete_container_sans) | **DELETE** /api/systems/{systemId}/container-scan-results | Remove one or many containers in a system



## add_container_sans_by_system_id

> models::ContainersResponsePost add_container_sans_by_system_id(system_id, serde_json_colon_colon_value)
Add one or many containers and their scan results

Add containers and their scan results in the assets module for a system `systemId`.  **Request Body Required Fields** - `containerId` - `containerName` - `time` - `benchmarks` (Object Array)   - `benchmark`   - `results` (Object Array)     - `ruleId`     - `status`     - `lastSeen`  **Example Request Body Required Fields** ``` [   {     \"containerId\": \"container identification\",     \"containerName\": \"container name\",     \"time\": Datetime of scan/result (1648217219),     \"benchmarks\": [       {         \"benchmark\": \"RHEL_8_STIG\",         \"results\": [           {             \"ruleId\": \"rule identification\",             \"status\": [Pass,Fail,Other,Not Reviewed,Not Checked,Not Applicable],             \"lastSeen\": Unix date format (1648217219)           }, {             \"ruleId\": \"rule identification\",             \"status\": [Pass,Fail,Other,Not Reviewed,Not Checked,Not Applicable],             \"lastSeen\": Unix date format (1648217219)           }         ]       }     ]   } ] ````

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for adding containers and their scan results | [required] |

### Return type

[**models::ContainersResponsePost**](ContainersResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_container_sans

> models::ContainersResponseDelete delete_container_sans(system_id, container_resources_delete_body_inner)
Remove one or many containers in a system

Removes container scan resources and their scan results in the assets module for a system `systemId`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**container_resources_delete_body_inner** | [**Vec<models::ContainerResourcesDeleteBodyInner>**](ContainerResourcesDeleteBodyInner.md) | Delete the given Container Scan Id | [required] |

### Return type

[**models::ContainersResponseDelete**](ContainersResponseDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

