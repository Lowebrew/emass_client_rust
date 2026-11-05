# PoamReadOnlyFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_id** | Option<**String**> | [Read-Only] Unique identifier of the authorization term/condition linked to the POA&M Item. | [optional]
**is_inherited** | Option<**bool**> | [Read-only] Indicates whether a test result is inherited. | [optional]
**cci** | Option<**String**> | [Read-Only] CCI associated with POA&M Item. | [optional]
**review_status** | Option<**ReviewStatus**> | [Read-Only] Values include the following options: (Not Approved,Under Review,Approved) (enum: Not Approved, Under Review, Approved) | [optional]
**created_date** | Option<**i64**> | [Read-Only] Timestamp representing when the POA&M Item was entered into the database. | [optional]
**extension_date** | Option<**i64**> | [Read-Only] Value returned for a POA&M Item with review status \"Approved\" and has a milestone with a scheduled completion date that extends beyond the POA&M Item's scheduled completion date.  | [optional]
**pending_extension_date** | Option<**i64**> | [Read-Only] Value returned for a POA&M Item with a review status of \"Approved\" and an unapproved milestone with a scheduled completion date that extends beyond the POA&M Item's scheduled completion date.  | [optional]
**artifacts** | Option<**String**> | [Read-Only] Lists the filenames of any artifact files attached to the POA&M Item. Multiple values are separated by “; ”. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


