# WorkflowInstanceGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_uid** | Option<**String**> | [Read-Only] Unique workflow definition identifier. | [optional]
**system_id** | Option<**i64**> | [Read-only] Unique system record identifier. | [optional]
**system_name** | Option<**String**> | [Read-Only] The system name. | [optional]
**workflow_instance_id** | Option<**i64**> | [Read-Only] Unique workflow instance identifier. | [optional]
**package_name** | Option<**String**> | [Read-Only] The package name. | [optional]
**created_date** | Option<**i64**> | [Read-Only] Date the workflow instance or the workflow transition was created. | [optional]
**last_edited_date** | Option<**i64**> | [Read-Only] Date the workflow was last acted on. | [optional]
**last_edited_by** | Option<**String**> | [Read-Only] User that last acted on the workflow. | [optional]
**workflow** | Option<**String**> | [Read-Only] The workflow type. | [optional]
**version** | Option<**i64**> | [Read-Only] Version of the workflow definition. | [optional]
**current_stage** | Option<**String**> | [Read-Only] Name of the current stage. | [optional]
**transitions** | Option<[**Vec<models::InstanceTransitions>**](InstanceTransitions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


