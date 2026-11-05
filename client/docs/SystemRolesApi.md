# \SystemRolesApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_roles**](SystemRolesApi.md#get_system_roles) | **GET** /api/system-roles | Get available roles
[**get_system_roles_by_category_id**](SystemRolesApi.md#get_system_roles_by_category_id) | **GET** /api/system-roles/{roleCategory} | Get system roles



## get_system_roles

> models::SystemRolesResponse get_system_roles()
Get available roles

Returns all available roles

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemRolesResponse**](SystemRolesResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_roles_by_category_id

> models::SystemRolesCategoryResponse get_system_roles_by_category_id(role_category, role, policy)
Get system roles

Returns the role(s) data matching parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_category** | **String** | **Role Category**: The system role category been queried | [required] |[default to PAC]
**role** | **String** | **Role**: Accepts single value from options available at base system-roles endpoint e.g., SCA. | [required] |[default to IAO]
**policy** | Option<**String**> | **System Policy**: Filter query by system policy. If no value is specified, the default returns RMF policy information for dual-policy systems. |  |[default to rmf]

### Return type

[**models::SystemRolesCategoryResponse**](SystemRolesCategoryResponse.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

