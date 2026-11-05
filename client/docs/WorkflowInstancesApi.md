# \WorkflowInstancesApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_system_workflow_instances**](WorkflowInstancesApi.md#get_system_workflow_instances) | **GET** /api/workflows/instances | Get workflow instances in a site
[**get_system_workflow_instances_by_workflow_instance_id**](WorkflowInstancesApi.md#get_system_workflow_instances_by_workflow_instance_id) | **GET** /api/workflows/instances/{workflowInstanceId} | Get workflow instance by ID



## get_system_workflow_instances

> models::WorkflowInstancesResponseGet get_system_workflow_instances(include_comments, include_decommission_systems, page_index, since_date, status)
Get workflow instances in a site

View detailed information on all active and historical workflows filtered by provided parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_comments** | Option<**bool**> | **Include Comments**: If no value is specified, the default returns true to not include transition comments. Note: Corresponds to the Comments textbox that is required at most workflow transitions. Does not include other text input fields such as Terms / Conditions for Authorization.  |  |[default to true]
**include_decommission_systems** | Option<**bool**> | **Include Decommission Systems**: If no value is specified, the default returns false to exclude decommissioned systems.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0. **Note:** Pages contain 1000 workflow instances.  |  |[default to 0]
**since_date** | Option<**String**> | **Date**: Filter on authorization/assessment date (Unix date format). Note: Filters off the lastEditedDate field. Note: The authorization/assessment decisions on completed workflows can be edited for up to 30 days after the initial decision is made.  |  |
**status** | Option<**String**> | **Status**: Filter by status. If no value is specified, the default returns all to include both active and inactive workflows. Note: Any workflows at a current stage of Complete or Cancelled are inactive. Ongoing workflows currently at other stages are active.  |  |[default to all]

### Return type

[**models::WorkflowInstancesResponseGet**](WorkflowInstancesResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_workflow_instances_by_workflow_instance_id

> models::WorkflowInstanceResponseGet get_system_workflow_instances_by_workflow_instance_id(workflow_instance_id)
Get workflow instance by ID

View detailed historical workflow information for `workflowInstanceId`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_instance_id** | **i32** | **Workflow Instance Id**: The unique workflow definition identifier. | [required] |

### Return type

[**models::WorkflowInstanceResponseGet**](WorkflowInstanceResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

