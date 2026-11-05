# ArtifactsResponseGetDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**filename** | Option<**String**> | [Required] File name should match exactly one file within the provided zip file. 1000 Characters. or Application/zip file. Max 30MB per artifact.  | [optional]
**is_template** | Option<**bool**> | [Required] Indicates whether an artifact template. | [optional]
**r#type** | Option<**String**> | [Required] Artifact type options | [optional]
**category** | Option<**String**> | [Required] Artifact category options | [optional]
**is_inherited** | Option<**bool**> | [Read-only] Indicates whether an artifact is inherited. | [optional]
**ccis** | Option<**String**> | [Read-Only] CCI mapping for Assessment Procedures associated with the artifact. | [optional]
**mime_content_type** | Option<**String**> | [Read-Only] Standard MIME content type derived from file extension. | [optional]
**file_size** | Option<**String**> | [Read-Only] File size of attached artifact. | [optional]
**name** | Option<**String**> | [Optional] Artifact name. Character Limit = 100. | [optional]
**description** | Option<**String**> | [Optional] Artifact description. 10,000 Characters. | [optional]
**reference_page_number** | Option<**String**> | [Optional] Artifact reference page number. 50 Characters. | [optional]
**assessment_procedures** | Option<**String**> | [Optional] The Security Control Assessment Procedure being associated with the artifact. | [optional]
**controls** | Option<**String**> | [Optional] Control acronym associated with the artifact. NIST SP 800-53 Revision 4 defined. | [optional]
**expiration_date** | Option<**i64**> | [Optional] Date Artifact expires and requires review. In Unix Date format. | [optional]
**last_reviewed_date** | Option<**i64**> | [Optional] Date Artifact was last reviewed. Unix time format. | [optional]
**signed_date** | Option<**i64**> | [Optional] Date artifact was signed. Unix time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


