# GetCorporationsCorporationIdStructures200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**corporation_id** | **i32** | ID of the corporation that owns the structure | [default to null]
**fuel_expires** | **String** | Date on which the structure will run out of fuel | [optional] [default to null]
**name** | **String** | The structure name | [optional] [default to null]
**next_reinforce_apply** | **String** | The date and time when the structure&#39;s newly requested reinforcement times (e.g. next_reinforce_hour and next_reinforce_day) will take effect | [optional] [default to null]
**next_reinforce_hour** | **i32** | The requested change to reinforce_hour that will take effect at the time shown by next_reinforce_apply | [optional] [default to null]
**profile_id** | **i32** | The id of the ACL profile for this citadel | [default to null]
**reinforce_hour** | **i32** | The hour of day that determines the four hour window when the structure will randomly exit its reinforcement periods and become vulnerable to attack against its armor and/or hull. The structure will become vulnerable at a random time that is +/- 2 hours centered on the value of this property | [optional] [default to null]
**services** | [**Vec<::models::GetCorporationsCorporationIdStructuresService>**](get_corporations_corporation_id_structures_service.md) | Contains a list of service upgrades, and their state | [optional] [default to null]
**state** | **String** | state string | [default to null]
**state_timer_end** | **String** | Date at which the structure will move to it&#39;s next state | [optional] [default to null]
**state_timer_start** | **String** | Date at which the structure entered it&#39;s current state | [optional] [default to null]
**structure_id** | **i64** | The Item ID of the structure | [default to null]
**system_id** | **i32** | The solar system the structure is in | [default to null]
**type_id** | **i32** | The type id of the structure | [default to null]
**unanchors_at** | **String** | Date at which the structure will unanchor | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


