# HwBaselineResponseGetDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**hardware_id** | Option<**String**> | [Read-Only] GUID identifying the specific hardware asset. Required for a PUT call. | [optional]
**asset_name** | Option<**String**> | [Required] Name of the hardware asset. | [optional]
**component_type** | Option<**String**> | [Optional] Type of the hardware asset. | [optional]
**nickname** | Option<**String**> | [Optional] Nickname of the hardware asset. | [optional]
**asset_ip_address** | Option<**String**> | [Optional] IP address of the hardware asset. | [optional]
**public_facing** | Option<**bool**> | [Optional] Public facing is defined as any asset that is accessible from a commercial connection. | [optional]
**virtual_asset** | Option<**bool**> | [Optional] Determine if this is a virtual hardware asset. | [optional]
**manufacturer** | Option<**String**> | [Optional] Manufacturer of the hardware asset. Populated with \"Virtual\" by default if Virtual Asset is true, however this can be overridden. | [optional]
**model_number** | Option<**String**> | [Optional] Model number of the hardware asset. Populated with \"Virtual\" by default if Virtual Asset is true, however this can be overridden | [optional]
**serial_number** | Option<**String**> | [Optional] Serial number of the hardware asset. Populated with \"Virtual\" by default if Virtual Asset is true, however this can be overridden. | [optional]
**os_ios_fw_version** | Option<**String**> | [Optional] Operating System, IOS, or Firmware version of the hardware asset. | [optional]
**memory_size_type** | Option<**String**> | [Optional] Memory size / type of the hardware asset. | [optional]
**location** | Option<**String**> | [Optional] Location of the hardware asset. | [optional]
**approval_status** | Option<**String**> | [Optional] Approval status of the hardware asset. | [optional]
**critical_asset** | Option<**bool**> | [Optional] Indicates whether the asset is a critical information system asset. | [optional]
**public_facing_fqdn** | Option<**String**> | [Conditional] Public facing FQDN. Only applicable if Public Facing is set to true. | [optional]
**public_facing_ip_address** | Option<**String**> | [Conditional] Public facing IP address. Only applicable if Public Facing is set to true. | [optional]
**public_facing_urls** | Option<**String**> | [Conditional] Public facing URLs. Only applicable if Public Facing is set to true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


