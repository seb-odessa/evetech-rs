# GetCorporationsCorporationIdCustomsOffices200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alliance_tax_rate** | **f32** | Only present if alliance access is allowed | [optional] [default to null]
**allow_access_with_standings** | **bool** | standing_level and any standing related tax rate only present when this is true | [default to null]
**allow_alliance_access** | **bool** | allow_alliance_access boolean | [default to null]
**bad_standing_tax_rate** | **f32** | bad_standing_tax_rate number | [optional] [default to null]
**corporation_tax_rate** | **f32** | corporation_tax_rate number | [optional] [default to null]
**excellent_standing_tax_rate** | **f32** | Tax rate for entities with excellent level of standing, only present if this level is allowed, same for all other standing related tax rates | [optional] [default to null]
**good_standing_tax_rate** | **f32** | good_standing_tax_rate number | [optional] [default to null]
**neutral_standing_tax_rate** | **f32** | neutral_standing_tax_rate number | [optional] [default to null]
**office_id** | **i64** | unique ID of this customs office | [default to null]
**reinforce_exit_end** | **i32** | reinforce_exit_end integer | [default to null]
**reinforce_exit_start** | **i32** | Together with reinforce_exit_end, marks a 2-hour period where this customs office could exit reinforcement mode during the day after initial attack | [default to null]
**standing_level** | **String** | Access is allowed only for entities with this level of standing or better | [optional] [default to null]
**system_id** | **i32** | ID of the solar system this customs office is located in | [default to null]
**terrible_standing_tax_rate** | **f32** | terrible_standing_tax_rate number | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


