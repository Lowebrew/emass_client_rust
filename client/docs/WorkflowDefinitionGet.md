# WorkflowDefinitionGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_uid** | Option<**String**> | [Read-Only] Unique workflow definition identifier. | [optional]
**workflow** | Option<**String**> | [Read-Only] The workflow type. | [optional]
**version** | Option<**String**> | [Read-Only] Version of the workflow definition. | [optional]
**description** | Option<**String**> | [Read-Only] Description of the workflow or the stage transition. | [optional]
**is_active** | Option<**bool**> | [Read-Only] Returns true if the workflow is available to the site. | [optional]
**stages** | Option<[**Vec<models::Stage>**](Stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


