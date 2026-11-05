# \ArtifactsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_artifacts_by_system_id**](ArtifactsApi.md#add_artifacts_by_system_id) | **POST** /api/systems/{systemId}/artifacts | Add one or many artifacts in a system
[**delete_artifact**](ArtifactsApi.md#delete_artifact) | **DELETE** /api/systems/{systemId}/artifacts | Remove one or many artifacts in a system
[**get_system_artifacts**](ArtifactsApi.md#get_system_artifacts) | **GET** /api/systems/{systemId}/artifacts | Get one or many artifacts in a system
[**update_artifact_by_system_id**](ArtifactsApi.md#update_artifact_by_system_id) | **PUT** /api/systems/{systemId}/artifacts | Update one or many artifacts in a system



## add_artifacts_by_system_id

> models::ArtifactsResponsePutPost add_artifacts_by_system_id(system_id, filename, is_bulk, is_template, r#type, category)
Add one or many artifacts in a system

<strong>Information About Adding Artifacts</strong><br>  The body of a request through the Artifacts POST endpoint accepts a single binary file. Two Artifact POST methods are currently accepted: individual and bulk. Filename uniqueness within an eMASS system will be enforced by the API for both methods. <br><br> For POST requests that should result in a single artifact, the request should include the file. <br><br> For POST requests that should result in the creation of many artifacts, the request should include a single file with the extension \".zip\" only and the parameter isBulk should be set to true. This .zip file should contain one or more files corresponding to existing artifacts or new artifacts that will be created upon successful receipt. <br><br> Upon successful receipt of one or many artifacts, if a file is matched via filename to an artifact existing within the application, the file associated with the artifact will be updated. If no artifact is matched via filename to the application, a new artifact will be created with the following default values. Any values not specified below will be null <ul>   <li>isTemplate: false</li>   <li>type: Other</li>   <li>category: Evidence</li> </ul>  To update values other than the file itself, please submit a PUT request.<br>  <strong>Business Rules</strong><br>  Artifact cannot be saved if the fields below exceed the following character limits: <ul>   <li>Filename - 1,000 characters</li>   <li>Name - 100 characters</li>   <li>Description - 10,000 characters</li>   <li>Reference Page Number - 50 characters</li> </ul>  Artifact version cannot be saved if an Artifact with the same file name (filename) already exist in the system.  Artifact cannot be saved if the file size exceeds 30MB.  Artifact cannot be saved if the following fields are missing data: <ul>   <li>Filename (filename)</li>   <li>Type (type)</li>   <li>Category (category)</li> </ul> Artifact cannot be saved if the Last Review Date (`lastReviewedDate`) is set in the future.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**filename** | **std::path::PathBuf** | The file to upload. Must be a .zip file if isBulk is set to true, otherwise any acceptable artifact file. Max 30MB per artifact. | [required] |
**is_bulk** | Option<**bool**> | **Is Bulk**: If no value is specified, the default is false, and an individual artifact file is expected. When set to true, a .zip file is expected which can contain multiple artifact files.  |  |[default to false]
**is_template** | Option<**bool**> | Is the artifact a template?  Indicates whether an artifact is a template |  |
**r#type** | Option<**String**> | The type of artifact. Possible values are:  Procedure, Diagram, Policy, Labor, Document, Image, Other, Scan Result, Auditor Report. May also accept custom artifact type values set by system administrators. |  |
**category** | Option<**String**> | The category of artifact. Possible values are:  Implementation Guidance, Evidence.  May also accept custom artifact category values set by system administrators. |  |

### Return type

[**models::ArtifactsResponsePutPost**](ArtifactsResponsePutPost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artifact

> models::ArtifactsResponseDel delete_artifact(system_id, artifacts_request_delete_body_inner)
Remove one or many artifacts in a system

Remove the Artifact(s) matching `systemId` path parameter and request body artifact(s) file name<br><br>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**artifacts_request_delete_body_inner** | [**Vec<models::ArtifactsRequestDeleteBodyInner>**](ArtifactsRequestDeleteBodyInner.md) | Delete artifact files for the given System Id | [required] |

### Return type

[**models::ArtifactsResponseDel**](ArtifactsResponseDel.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_artifacts

> models::ArtifactsResponseGet get_system_artifacts(system_id, filename, control_acronyms, assessment_procedures, ccis, system_only)
Get one or many artifacts in a system

Returns selected artifacts matching parameters to include the file name containing the artifacts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**filename** | Option<**String**> | **File Name**: The file name (to include file-extension). |  |
**control_acronyms** | Option<**String**> | **Control Acronym**: Filter query by given system acronym (single value or comma separated). |  |
**assessment_procedures** | Option<**String**> | **Assessment Procedure**: Filter query by given Security Control Assessment Procedure (single value or comma separated). |  |
**ccis** | Option<**String**> | **CCI System**: Filter query by Control Correlation Identifiers (CCIs) (single value or comma separated). |  |
**system_only** | Option<**bool**> | **Systems Only**: Indicates that only system(s) information is retrieved. |  |[default to true]

### Return type

[**models::ArtifactsResponseGet**](ArtifactsResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artifact_by_system_id

> models::ArtifactsResponsePutPost update_artifact_by_system_id(system_id, artifacts_required_fields)
Update one or many artifacts in a system

Updates an artifact for given `systemId` path parameter<br><br> **Request Body Required Fields** - `filename` - `isTemplate` - `type` - `category`  The example request provides all optional fields  <strong>Information About Updating Artifacts</strong><br>  The PUT request will replace all existing data with the field/value combinations included in the request body.  If any fields are not included, the absent fields will become null.  The fields `name` and `isTemplate` are non-nullable fields. If not specified in the PUT command they will default to the following:   - `name=filename`   - `isTemplate=false`  Also, note that one-to-many fields (`controls` and `ccis`) will also be replaced with the values specified in the PUT.  If existing `control or cci` mappings exist in eMASS, the values in the PUT will not append, but rather replace all existing control and cci mappings with the values in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**artifacts_required_fields** | [**Vec<models::ArtifactsRequiredFields>**](ArtifactsRequiredFields.md) | See `information` above for additional instructions | [required] |

### Return type

[**models::ArtifactsResponsePutPost**](ArtifactsResponsePutPost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

