# GetCharactersCharacterIdOrdersHistory200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration** | **i32** | Number of days the order was valid for (starting from the issued date). An order expires at time issued + duration | [default to null]
**escrow** | **f64** | For buy orders, the amount of ISK in escrow | [optional] [default to null]
**is_buy_order** | **bool** | True if the order is a bid (buy) order | [optional] [default to null]
**is_corporation** | **bool** | Signifies whether the buy/sell order was placed on behalf of a corporation. | [default to null]
**issued** | **String** | Date and time when this order was issued | [default to null]
**location_id** | **i64** | ID of the location where order was placed | [default to null]
**min_volume** | **i32** | For buy orders, the minimum quantity that will be accepted in a matching sell order | [optional] [default to null]
**order_id** | **i64** | Unique order ID | [default to null]
**price** | **f64** | Cost per unit for this order | [default to null]
**range** | **String** | Valid order range, numbers are ranges in jumps | [default to null]
**region_id** | **i32** | ID of the region where order was placed | [default to null]
**state** | **String** | Current order state | [default to null]
**type_id** | **i32** | The type ID of the item transacted in this order | [default to null]
**volume_remain** | **i32** | Quantity of items still required or offered | [default to null]
**volume_total** | **i32** | Quantity of items required or offered at time order was placed | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


