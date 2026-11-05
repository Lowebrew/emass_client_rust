# \WorkflowDefinitionsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_workflow_definitions**](WorkflowDefinitionsApi.md#get_workflow_definitions) | **GET** /api/workflows/definitions | Get workflow definitions in a site



## get_workflow_definitions

> models::WorkflowDefinitionResponseGet get_workflow_definitions(include_inactive, registration_type)
Get workflow definitions in a site

View all workflow schemas available on the eMASS instance filtered by status `includeInactive` and registration type `registrationType`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_inactive** | Option<**bool**> | **Include Inactive**: If no value is specified, the default returns false to not include outdated workflow definitions. |  |[default to true]
**registration_type** | Option<**String**> | **Registration Type**: Filter record by selected registration type (single value or comma delimited values).  **Valid Options Are:** assessAndAuthorize, assessOnly, guest, regular, functional, cloudServiceProvider, commonControlProvider, authorizationToUse, reciprocityAcceptanc  |  |[default to regular]

### Return type

[**models::WorkflowDefinitionResponseGet**](WorkflowDefinitionResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

