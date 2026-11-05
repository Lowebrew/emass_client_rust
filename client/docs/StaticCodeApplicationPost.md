# StaticCodeApplicationPost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**raw_severity** | Option<**RawSeverity**> | [Optional] Scan vulnerability ratting (enum: Low, Medium, Moderate, High, Critical) | [optional]
**code_check_name** | Option<**String**> | [Required] Name of the software vulnerability or weakness. | [optional]
**count** | Option<**i64**> | [Required] Number of instances observed for a specified finding. | [optional]
**scan_date** | Option<**i64**> | [Required] The date of the scan. Unix date format. | [optional]
**cwe_id** | Option<**String**> | [Required] The Common Weakness Enumerator (CWE) identifier. | [optional]
**clear_findings** | Option<**bool**> | [Optional] When used by itself, can clear out all application findings for a single application/version pairing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


