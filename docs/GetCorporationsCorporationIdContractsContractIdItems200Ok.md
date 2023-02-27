# GetCorporationsCorporationIdContractsContractIdItems200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_included** | **bool** | true if the contract issuer has submitted this item with the contract, false if the isser is asking for this item in the contract | [default to null]
**is_singleton** | **bool** | is_singleton boolean | [default to null]
**quantity** | **i32** | Number of items in the stack | [default to null]
**raw_quantity** | **i32** | -1 indicates that the item is a singleton (non-stackable). If the item happens to be a Blueprint, -1 is an Original and -2 is a Blueprint Copy | [optional] [default to null]
**record_id** | **i64** | Unique ID for the item | [default to null]
**type_id** | **i32** | Type ID for item | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


