# MilestonesGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**milestone_id** | Option<**i64**> | [Required] Unique item identifier. | [optional]
**poam_id** | Option<**i64**> | [Required] Unique item identifier. | [optional]
**description** | Option<**String**> | [Required] Include milestone description. | [optional]
**scheduled_completion_date** | Option<**i64**> | [Required] Required for ongoing and completed POA&M items. Unix time format. | [optional]
**review_status** | Option<**ReviewStatus**> | [Read-Only] Values include the following options: (Not Approved,Under Review,Approved). (enum: Not Approved, Under Review, Approved) | [optional]
**created_by** | Option<**String**> | [Read-Only] Last, first name of the user that created the milestone. | [optional]
**created_date** | Option<**i64**> | [Read-Only] Timestamp representing when the milestone was entered into the database. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


