# PacGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow** | Option<**Workflow**> | [Required] Values include the following:(Assess and Authorize, Assess Only, Security Plan Approval) (enum: Assess and Authorize, Assess Only, Security Plan Approval) | [optional]
**name** | Option<**String**> | [Required] Package name. 100 Characters. | [optional]
**current_stage_name** | Option<**String**> | [Read-Only] Name of the current stage in the active workflow. | [optional]
**current_stage** | Option<**i32**> | [Read-Only] Number of the current stage in the active workflow. | [optional]
**total_stages** | Option<**i32**> | [Read-Only] Total number of stages in the active workflow. | [optional]
**days_at_current_stage** | Option<**i32**> | [Read-Only] Indicates the number of days at current workflow stage. | [optional]
**comments** | Option<**String**> | [Required]] Comments submitted upon initiation of the indicated workflow. Character Limit = 4,000. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


