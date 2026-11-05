# \ControlsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_controls**](ControlsApi.md#get_system_controls) | **GET** /api/systems/{systemId}/controls | Get control information in a system for one or many controls
[**update_control_by_system_id**](ControlsApi.md#update_control_by_system_id) | **PUT** /api/systems/{systemId}/controls | Update control information in a system for one or many controls



## get_system_controls

> models::ControlsResponseGet get_system_controls(system_id, acronyms)
Get control information in a system for one or many controls

Returns system control information for matching `systemId` path parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**acronyms** | Option<**String**> | **Acronym**: The system acronym(s) being queried (single value or comma delimited values). |  |[default to PM-6]

### Return type

[**models::ControlsResponseGet**](ControlsResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_control_by_system_id

> models::ControlsResponsePut update_control_by_system_id(system_id, controls_required_fields)
Update control information in a system for one or many controls

Update a Control for given `systemId`<br>  **Request Body Required Fields** - `acronym` - `responsibleEntities` - `controlDesignation` - `estimatedCompletionDate` - `implementationNarrative`  <br> **Business Rules**  The following **optional fields** (plus the **Request Body Required Fields**) are required based on the Implementation Status (`implementationStatus`) field value:<br>  <table>   <thead>     <tr><th><b>Status</b></th><th><b>Required Fields</b></th></tr>   </thead>   <tbody>     <tr><td><b>Planned</b> or <b>Implemented</b></td><td><code>slcmCriticality, slcmFrequency, slcmMethod, slcmReporting, slcmTracking, slcmComments</code></td></tr>     <tr><td><b>Not Applicable</b></td><td><code>naJustification</code></td></tr>     <tr><td><b>Manually Inherited</b></td><td><code>commonControlProvider, slcmCriticality, slcmFrequency, slcmMethod, slcmReporting, slcmTracking, slcmComments</code></td></tr>   </tbody> </table>   **NOTES:** - Risk Assessment information cannot be updated if a Security Control is `Inherited`. - Risk Assessment information cannot be updated for a DIACAP system record. - Implementation Plan information cannot be saved if the these fields exceed 2,000 character limits:   - `naJustification`,`responsibleEntities`,`implementationNarrative`,`slcmCriticality`   - `slcmFrequency`,`slcmMethod`,`slcmReporting`,`slcmTracking`,`slcmComments` - Implementation Plan or Risk Assessment information cannot be updated if Security Control does not exist in the system record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**controls_required_fields** | [**Vec<models::ControlsRequiredFields>**](ControlsRequiredFields.md) | Example request body for updating an existing control for a given system. | [required] |

### Return type

[**models::ControlsResponsePut**](ControlsResponsePut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

