# GetContractsPublicItemsContractId200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_blueprint_copy** | **bool** | is_blueprint_copy boolean | [optional] [default to null]
**is_included** | **bool** | true if the contract issuer has submitted this item with the contract, false if the isser is asking for this item in the contract | [default to null]
**item_id** | **i64** | Unique ID for the item being sold. Not present if item is being requested by contract rather than sold with contract | [optional] [default to null]
**material_efficiency** | **i32** | Material Efficiency Level of the blueprint | [optional] [default to null]
**quantity** | **i32** | Number of items in the stack | [default to null]
**record_id** | **i64** | Unique ID for the item, used by the contract system | [default to null]
**runs** | **i32** | Number of runs remaining if the blueprint is a copy, -1 if it is an original | [optional] [default to null]
**time_efficiency** | **i32** | Time Efficiency Level of the blueprint | [optional] [default to null]
**type_id** | **i32** | Type ID for item | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


