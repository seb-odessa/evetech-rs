# GetSovereigntyCampaigns200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attackers_score** | **f32** | Score for all attacking parties, only present in Defense Events.  | [optional] [default to null]
**campaign_id** | **i32** | Unique ID for this campaign. | [default to null]
**constellation_id** | **i32** | The constellation in which the campaign will take place.  | [default to null]
**defender_id** | **i32** | Defending alliance, only present in Defense Events  | [optional] [default to null]
**defender_score** | **f32** | Score for the defending alliance, only present in Defense Events.  | [optional] [default to null]
**event_type** | **String** | Type of event this campaign is for. tcu_defense, ihub_defense and station_defense are referred to as \&quot;Defense Events\&quot;, station_freeport as \&quot;Freeport Events\&quot;.  | [default to null]
**participants** | [**Vec<::models::GetSovereigntyCampaignsParticipant>**](get_sovereignty_campaigns_participant.md) | Alliance participating and their respective scores, only present in Freeport Events.  | [optional] [default to null]
**solar_system_id** | **i32** | The solar system the structure is located in.  | [default to null]
**start_time** | **String** | Time the event is scheduled to start.  | [default to null]
**structure_id** | **i64** | The structure item ID that is related to this campaign.  | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


