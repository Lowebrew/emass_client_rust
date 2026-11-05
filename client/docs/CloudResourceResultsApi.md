# \CloudResourceResultsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_cloud_resources_by_system_id**](CloudResourceResultsApi.md#add_cloud_resources_by_system_id) | **POST** /api/systems/{systemId}/cloud-resource-results | Add one or many cloud resources and their scan results
[**delete_cloud_resources**](CloudResourceResultsApi.md#delete_cloud_resources) | **DELETE** /api/systems/{systemId}/cloud-resource-results | Remove one or many cloud resources in a system



## add_cloud_resources_by_system_id

> models::CloudResourcesResponsePost add_cloud_resources_by_system_id(system_id, serde_json_colon_colon_value)
Add one or many cloud resources and their scan results

Add cloud resources and their scan results in the assets module for a system `systemId`  **Request Body Required Fields** - `provider` - `resourceId` - `resourceName` - `resourceType` - `complianceResults` (Object Array)   - `cspPolicyDefinitionId`   - `isCompliant`   - `policyDefinitionTitle`  **Example Request Body Required Fields** ``` [   {     \"provider\": \"provide name\",     \"resourceId\": \"resource identification\",     \"resourceName\": \"resource name\",     \"resourceType\": \"resource type\",     \"complianceResults\": [       {         \"cspPolicyDefinitionId\": \"CSP policy definition identification\",         \"policyDefinitionTitle\": \"policy definition title\",         \"isCompliant\": [true or false]       }     ]   } ] ```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for adding cloud resources and their scan results | [required] |

### Return type

[**models::CloudResourcesResponsePost**](CloudResourcesResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cloud_resources

> models::CloudResourcesDelete delete_cloud_resources(system_id, cloud_resources_delete_body_inner)
Remove one or many cloud resources in a system

Removes cloud resources and their scan results in the assets module for a system `systemId`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**cloud_resources_delete_body_inner** | [**Vec<models::CloudResourcesDeleteBodyInner>**](CloudResourcesDeleteBodyInner.md) | Delete the given Cloud Resource Id | [required] |

### Return type

[**models::CloudResourcesDelete**](CloudResourcesDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

