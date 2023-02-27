# GetCorporationsCorporationIdIndustryJobs200Ok

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_id** | **i32** | Job activity ID | [default to null]
**blueprint_id** | **i64** | blueprint_id integer | [default to null]
**blueprint_location_id** | **i64** | Location ID of the location from which the blueprint was installed. Normally a station ID, but can also be an asset (e.g. container) or corporation facility | [default to null]
**blueprint_type_id** | **i32** | blueprint_type_id integer | [default to null]
**completed_character_id** | **i32** | ID of the character which completed this job | [optional] [default to null]
**completed_date** | **String** | Date and time when this job was completed | [optional] [default to null]
**cost** | **f64** | The sume of job installation fee and industry facility tax | [optional] [default to null]
**duration** | **i32** | Job duration in seconds | [default to null]
**end_date** | **String** | Date and time when this job finished | [default to null]
**facility_id** | **i64** | ID of the facility where this job is running | [default to null]
**installer_id** | **i32** | ID of the character which installed this job | [default to null]
**job_id** | **i32** | Unique job ID | [default to null]
**licensed_runs** | **i32** | Number of runs blueprint is licensed for | [optional] [default to null]
**location_id** | **i64** | ID of the location for the industry facility | [default to null]
**output_location_id** | **i64** | Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility | [default to null]
**pause_date** | **String** | Date and time when this job was paused (i.e. time when the facility where this job was installed went offline) | [optional] [default to null]
**probability** | **f32** | Chance of success for invention | [optional] [default to null]
**product_type_id** | **i32** | Type ID of product (manufactured, copied or invented) | [optional] [default to null]
**runs** | **i32** | Number of runs for a manufacturing job, or number of copies to make for a blueprint copy | [default to null]
**start_date** | **String** | Date and time when this job started | [default to null]
**status** | **String** | status string | [default to null]
**successful_runs** | **i32** | Number of successful runs for this job. Equal to runs unless this is an invention job | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


