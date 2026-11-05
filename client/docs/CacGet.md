# CacGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**control_acronym** | Option<**String**> | [Required] System acronym name. | [optional]
**compliance_status** | Option<**String**> | [Read-only] Compliance status of the control. | [optional]
**current_stage_name** | Option<**String**> | [Read-Only] Role in current stage. | [optional]
**current_stage** | Option<**i32**> | [Read-Only] Current step in the Control Approval Chain. | [optional]
**total_stages** | Option<**i32**> | [Read-Only] Total number of steps in Control Approval Chain. | [optional]
**comments** | Option<**String**> | [Conditional] Control Approval Chain comments - 2000 Characters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


