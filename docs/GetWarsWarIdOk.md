# GetWarsWarIdOk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggressor** | [***::models::GetWarsWarIdAggressor**](get_wars_war_id_aggressor.md) |  | [default to null]
**allies** | [**Vec<::models::GetWarsWarIdAlly>**](get_wars_war_id_ally.md) | allied corporations or alliances, each object contains either corporation_id or alliance_id | [optional] [default to null]
**declared** | **String** | Time that the war was declared | [default to null]
**defender** | [***::models::GetWarsWarIdDefender**](get_wars_war_id_defender.md) |  | [default to null]
**finished** | **String** | Time the war ended and shooting was no longer allowed | [optional] [default to null]
**id** | **i32** | ID of the specified war | [default to null]
**mutual** | **bool** | Was the war declared mutual by both parties | [default to null]
**open_for_allies** | **bool** | Is the war currently open for allies or not | [default to null]
**retracted** | **String** | Time the war was retracted but both sides could still shoot each other | [optional] [default to null]
**started** | **String** | Time when the war started and both sides could shoot each other | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


