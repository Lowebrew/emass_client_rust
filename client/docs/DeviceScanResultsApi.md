# \DeviceScanResultsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_scan_results_by_system_id**](DeviceScanResultsApi.md#add_scan_results_by_system_id) | **POST** /api/systems/{systemId}/device-scan-results | Add device scans in a system



## add_scan_results_by_system_id

> models::DeviceScanResultsResponsePost add_scan_results_by_system_id(system_id, scan_type, filename, is_baseline)
Add device scans in a system

 **Request Body Required Field** - `scanType`  Scan Type Allow Values:  <ul>   <li>acasAsrArf (.zip)</li>   <li>acasNessus</li>   <li>disaStigViewerCklCklb (.ckl or .cklb)</li>   <li>disaStigViewerCmrs</li>   <li>policyAuditor (.zip)</li>          <li>scapComplianceChecker</li> </ul>   <strong>Business Rules</strong><br>  The body of a request through the Device Scan Results POST endpoint accepts a single binary file. Specific file extensions are expected depending upon the scanType parameter. For example, .ckl or .cklb files are accepted when using scanType is set to disaStigViewerCklCklb.  When set to acasAsrArf or policyAuditor, a .zip file is expected which should contain a single scan result (for example, a single pair of .asr and .arf files). Single files are expected for all other scan types as this endpoint requires files to be uploaded consecutively as opposed to in bulk.  Current scan types that are supported: <ul>   <li>ACAS: ASR/ARF</li>   <li>ACAS: NESSUS</li>   <li>DISA STIG Viewer: CKL/CKLB</li>   <li>DISA STIG Viewer: CMRS</li>   <li>Policy Auditor</li>          <li>SCAP Compliance Checker</li> </ul>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**scan_type** | **String** | **Scan Type**: The file scan type to upload | [required] |[default to disaStigViewerCklCklb]
**filename** | **std::path::PathBuf** | The file to upload. Can be a single file or a .zip file. | [required] |
**is_baseline** | Option<**bool**> | **Is Baseline**: Indicates that the imported file represents a baseline scan that includes all findings and results. Importing as a baseline scan, which assumes a common set of scan policies are used when conducting a scan, will replace a device's findings for a specific Benchmark. Applicable to ASR/ARF scans only.  |  |[default to false]

### Return type

[**models::DeviceScanResultsResponsePost**](DeviceScanResultsResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

