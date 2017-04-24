extern crate hyper;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;

        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::{CredentialsError, ProvideAwsCredentials};
    
use rusoto_core::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;
#[doc="<p>Specifies the tags to add to a trail.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddTagsRequest {
                #[doc="<p>Specifies the ARN of the trail to which one or more tags will be added. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="ResourceId")]
pub resource_id: String,
#[doc="<p>Contains a list of CloudTrail tags, up to a limit of 50</p>"]
#[serde(rename="TagsList")]
pub tags_list: Option<TagsList>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AddTagsResponse;
            
pub type Boolean = bool;
pub type ByteBuffer = Vec<u8>;
#[doc="<p>Specifies the settings for each trail.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateTrailRequest {
                #[doc="<p>Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered. Not required unless you specify CloudWatchLogsRoleArn.</p>"]
#[serde(rename="CloudWatchLogsLogGroupArn")]
pub cloud_watch_logs_log_group_arn: Option<String>,
#[doc="<p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>"]
#[serde(rename="CloudWatchLogsRoleArn")]
pub cloud_watch_logs_role_arn: Option<String>,
#[doc="<p>Specifies whether log file integrity validation is enabled. The default is false.</p> <note> <p>When you disable log file integrity validation, the chain of digest files is broken after one hour. CloudTrail will not create digest files for log files that were delivered during a period in which log file integrity validation was disabled. For example, if you enable log file integrity validation at noon on January 1, disable it at noon on January 2, and re-enable it at noon on January 10, digest files will not be created for the log files delivered from noon on January 2 to noon on January 10. The same applies whenever you stop CloudTrail logging or delete a trail.</p> </note>"]
#[serde(rename="EnableLogFileValidation")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enable_log_file_validation: Option<Boolean>,
#[doc="<p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>"]
#[serde(rename="IncludeGlobalServiceEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_global_service_events: Option<Boolean>,
#[doc="<p>Specifies whether the trail is created in the current region or in all regions. The default is false.</p>"]
#[serde(rename="IsMultiRegionTrail")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_multi_region_trail: Option<Boolean>,
#[doc="<p>Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be a an alias name prefixed by \"alias/\", a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <p>Examples:</p> <ul> <li> <p>alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
#[serde(rename="KmsKeyId")]
pub kms_key_id: Option<String>,
#[doc="<p>Specifies the name of the trail. The name must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>"]
#[serde(rename="Name")]
pub name: String,
#[doc="<p>Specifies the name of the Amazon S3 bucket designated for publishing log files. See <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html\">Amazon S3 Bucket Naming Requirements</a>.</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: String,
#[doc="<p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html\">Finding Your CloudTrail Log Files</a>. The maximum length is 200 characters.</p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<String>,
#[doc="<p>Specifies the name of the Amazon SNS topic defined for notification of log file delivery. The maximum length is 256 characters.</p>"]
#[serde(rename="SnsTopicName")]
pub sns_topic_name: Option<String>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateTrailResponse {
                #[doc="<p>Specifies the Amazon Resource Name (ARN) of the log group to which CloudTrail logs will be delivered.</p>"]
#[serde(rename="CloudWatchLogsLogGroupArn")]
pub cloud_watch_logs_log_group_arn: Option<String>,
#[doc="<p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>"]
#[serde(rename="CloudWatchLogsRoleArn")]
pub cloud_watch_logs_role_arn: Option<String>,
#[doc="<p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>"]
#[serde(rename="IncludeGlobalServiceEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_global_service_events: Option<Boolean>,
#[doc="<p>Specifies whether the trail exists in one region or in all regions.</p>"]
#[serde(rename="IsMultiRegionTrail")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_multi_region_trail: Option<Boolean>,
#[doc="<p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>"]
#[serde(rename="KmsKeyId")]
pub kms_key_id: Option<String>,
#[doc="<p>Specifies whether log file integrity validation is enabled.</p>"]
#[serde(rename="LogFileValidationEnabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub log_file_validation_enabled: Option<Boolean>,
#[doc="<p>Specifies the name of the trail.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>Specifies the name of the Amazon S3 bucket designated for publishing log files.</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: Option<String>,
#[doc="<p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html\">Finding Your CloudTrail Log Files</a>.</p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<String>,
#[doc="<p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-1:123456789012:MyTopic</code> </p>"]
#[serde(rename="SnsTopicARN")]
pub sns_topic_arn: Option<String>,
#[doc="<p>Specifies the ARN of the trail that was created. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailARN")]
pub trail_arn: Option<String>,
            }
            
#[doc="<p>The Amazon S3 objects that you specify in your event selectors for your trail to log data events. Data events are object level API operations that access S3 objects, such as <code>GetObject</code>, <code>DeleteObject</code>, and <code>PutObject</code>. You can specify up to 50 S3 buckets and object prefixes for an event selector. </p> <p>Example</p> <ol> <li> <p>You create an event selector for a trail and specify an S3 bucket and an empty prefix, such as <code>arn:aws:s3:::bucket-1/</code>.</p> </li> <li> <p>You upload an image file to <code>bucket-1</code>.</p> </li> <li> <p>The <code>PutObject</code> API operation occurs on an object in the S3 bucket that you specified in the event selector. The trail processes and logs the event.</p> </li> <li> <p>You upload another image file to a different S3 bucket named <code>arn:aws:s3:::bucket-2</code>.</p> </li> <li> <p>The event occurs on an object in an S3 bucket that you didn't specify in the event selector. The trail doesn’t log the event.</p> </li> </ol>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct DataResource {
                #[doc="<p>The resource type in which you want to log data events. You can specify only the following value: <code>AWS::S3::Object</code>.</p>"]
#[serde(rename="Type")]
pub type_: Option<String>,
#[doc="<p>A list of ARN-like strings for the specified S3 objects.</p> <p>To log data events for all objects in an S3 bucket, specify the bucket and an empty object prefix such as <code>arn:aws:s3:::bucket-1/</code>. The trail logs data events for all objects in this S3 bucket.</p> <p>To log data events for specific objects, specify the S3 bucket and object prefix such as <code>arn:aws:s3:::bucket-1/example-images</code>. The trail logs data events for objects in this S3 bucket that match the prefix.</p>"]
#[serde(rename="Values")]
pub values: Option<DataResourceValues>,
            }
            
pub type DataResourceValues = Vec<String>;
pub type DataResources = Vec<DataResource>;
pub type Date = f64;
#[doc="<p>The request that specifies the name of a trail to delete.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteTrailRequest {
                #[doc="<p>Specifies the name or the CloudTrail ARN of the trail to be deleted. The format of a trail ARN is: <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="Name")]
pub name: String,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteTrailResponse;
            
#[doc="<p>Returns information about the trail.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeTrailsRequest {
                #[doc="<p>Specifies whether to include shadow trails in the response. A shadow trail is the replication in a region of a trail that was created in a different region. The default is true.</p>"]
#[serde(rename="includeShadowTrails")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_shadow_trails: Option<Boolean>,
#[doc="<p>Specifies a list of trail names, trail ARNs, or both, of the trails to describe. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p> <p>If an empty list is specified, information for the trail in the current region is returned.</p> <ul> <li> <p>If an empty list is specified and <code>IncludeShadowTrails</code> is false, then information for all trails in the current region is returned.</p> </li> <li> <p>If an empty list is specified and IncludeShadowTrails is null or true, then information for all trails in the current region and any associated shadow trails in other regions is returned.</p> </li> </ul> <note> <p>If one or more trail names are specified, information is returned only if the names match the names of trails belonging only to the current region. To return information about a trail in another region, you must specify its trail ARN.</p> </note>"]
#[serde(rename="trailNameList")]
pub trail_name_list: Option<TrailNameList>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeTrailsResponse {
                #[doc="<p>The list of trail objects.</p>"]
#[serde(rename="trailList")]
pub trail_list: Option<TrailList>,
            }
            
#[doc="<p>Contains information about an event that was returned by a lookup request. The result includes a representation of a CloudTrail event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Event {
                #[doc="<p>A JSON string that contains a representation of the event returned.</p>"]
#[serde(rename="CloudTrailEvent")]
pub cloud_trail_event: Option<String>,
#[doc="<p>The CloudTrail ID of the event returned.</p>"]
#[serde(rename="EventId")]
pub event_id: Option<String>,
#[doc="<p>The name of the event returned.</p>"]
#[serde(rename="EventName")]
pub event_name: Option<String>,
#[doc="<p>The AWS service that the request was made to.</p>"]
#[serde(rename="EventSource")]
pub event_source: Option<String>,
#[doc="<p>The date and time of the event returned.</p>"]
#[serde(rename="EventTime")]
pub event_time: Option<Date>,
#[doc="<p>A list of resources referenced by the event returned.</p>"]
#[serde(rename="Resources")]
pub resources: Option<ResourceList>,
#[doc="<p>A user name or role name of the requester that called the API in the event returned.</p>"]
#[serde(rename="Username")]
pub username: Option<String>,
            }
            
#[doc="<p>Use event selectors to specify the types of events that you want your trail to log. When an event occurs in your account, CloudTrail evaluates the event selector for all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event.</p> <p>You can configure up to five event selectors for a trail.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct EventSelector {
                #[doc="<p>CloudTrail supports logging only data events for S3 objects. You can specify up to 50 S3 buckets and object prefixes for an event selector.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-event-selectors-for-a-trail.html#data-events-resources\">Data Events</a> in the <i>AWS CloudTrail User Guide</i>.</p>"]
#[serde(rename="DataResources")]
pub data_resources: Option<DataResources>,
#[doc="<p>Specify if you want your event selector to include management events for your trail.</p> <p> For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-event-selectors-for-a-trail.html#event-selector-for-management-events\">Management Events</a> in the <i>AWS CloudTrail User Guide</i>.</p> <p>By default, the value is <code>true</code>.</p>"]
#[serde(rename="IncludeManagementEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_management_events: Option<Boolean>,
#[doc="<p>Specify if you want your trail to log read-only events, write-only events, or all. For example, the EC2 <code>GetConsoleOutput</code> is a read-only API operation and <code>RunInstances</code> is a write-only API operation.</p> <p> By default, the value is <code>All</code>.</p>"]
#[serde(rename="ReadWriteType")]
pub read_write_type: Option<ReadWriteType>,
            }
            
pub type EventSelectors = Vec<EventSelector>;
pub type EventsList = Vec<Event>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetEventSelectorsRequest {
                #[doc="<p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If you specify a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailName")]
pub trail_name: Option<String>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetEventSelectorsResponse {
                #[doc="<p>The event selectors that are configured for the trail.</p>"]
#[serde(rename="EventSelectors")]
pub event_selectors: Option<EventSelectors>,
#[doc="<p>The specified trail ARN that has the event selectors.</p>"]
#[serde(rename="TrailARN")]
pub trail_arn: Option<String>,
            }
            
#[doc="<p>The name of a trail about which you want the current status.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetTrailStatusRequest {
                #[doc="<p>Specifies the name or the CloudTrail ARN of the trail for which you are requesting status. To get the status of a shadow trail (a replication of the trail in another region), you must specify its ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="Name")]
pub name: String,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetTrailStatusResponse {
                #[doc="<p>Whether the CloudTrail is currently logging AWS API calls.</p>"]
#[serde(rename="IsLogging")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_logging: Option<Boolean>,
#[doc="<p>Displays any CloudWatch Logs error that CloudTrail encountered when attempting to deliver logs to CloudWatch Logs.</p>"]
#[serde(rename="LatestCloudWatchLogsDeliveryError")]
pub latest_cloud_watch_logs_delivery_error: Option<String>,
#[doc="<p>Displays the most recent date and time when CloudTrail delivered logs to CloudWatch Logs.</p>"]
#[serde(rename="LatestCloudWatchLogsDeliveryTime")]
pub latest_cloud_watch_logs_delivery_time: Option<Date>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="LatestDeliveryAttemptSucceeded")]
pub latest_delivery_attempt_succeeded: Option<String>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="LatestDeliveryAttemptTime")]
pub latest_delivery_attempt_time: Option<String>,
#[doc="<p>Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver log files to the designated bucket. For more information see the topic <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html\">Error Responses</a> in the Amazon S3 API Reference. </p> <note> <p>This error occurs only when there is a problem with the destination S3 bucket and will not occur for timeouts. To resolve the issue, create a new bucket and call <code>UpdateTrail</code> to specify the new bucket, or fix the existing objects so that CloudTrail can again write to the bucket.</p> </note>"]
#[serde(rename="LatestDeliveryError")]
pub latest_delivery_error: Option<String>,
#[doc="<p>Specifies the date and time that CloudTrail last delivered log files to an account's Amazon S3 bucket.</p>"]
#[serde(rename="LatestDeliveryTime")]
pub latest_delivery_time: Option<Date>,
#[doc="<p>Displays any Amazon S3 error that CloudTrail encountered when attempting to deliver a digest file to the designated bucket. For more information see the topic <a href=\"http://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html\">Error Responses</a> in the Amazon S3 API Reference. </p> <note> <p>This error occurs only when there is a problem with the destination S3 bucket and will not occur for timeouts. To resolve the issue, create a new bucket and call <code>UpdateTrail</code> to specify the new bucket, or fix the existing objects so that CloudTrail can again write to the bucket.</p> </note>"]
#[serde(rename="LatestDigestDeliveryError")]
pub latest_digest_delivery_error: Option<String>,
#[doc="<p>Specifies the date and time that CloudTrail last delivered a digest file to an account's Amazon S3 bucket.</p>"]
#[serde(rename="LatestDigestDeliveryTime")]
pub latest_digest_delivery_time: Option<Date>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="LatestNotificationAttemptSucceeded")]
pub latest_notification_attempt_succeeded: Option<String>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="LatestNotificationAttemptTime")]
pub latest_notification_attempt_time: Option<String>,
#[doc="<p>Displays any Amazon SNS error that CloudTrail encountered when attempting to send a notification. For more information about Amazon SNS errors, see the <a href=\"http://docs.aws.amazon.com/sns/latest/dg/welcome.html\">Amazon SNS Developer Guide</a>. </p>"]
#[serde(rename="LatestNotificationError")]
pub latest_notification_error: Option<String>,
#[doc="<p>Specifies the date and time of the most recent Amazon SNS notification that CloudTrail has written a new log file to an account's Amazon S3 bucket.</p>"]
#[serde(rename="LatestNotificationTime")]
pub latest_notification_time: Option<Date>,
#[doc="<p>Specifies the most recent date and time when CloudTrail started recording API calls for an AWS account.</p>"]
#[serde(rename="StartLoggingTime")]
pub start_logging_time: Option<Date>,
#[doc="<p>Specifies the most recent date and time when CloudTrail stopped recording API calls for an AWS account.</p>"]
#[serde(rename="StopLoggingTime")]
pub stop_logging_time: Option<Date>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="TimeLoggingStarted")]
pub time_logging_started: Option<String>,
#[doc="<p>This field is deprecated.</p>"]
#[serde(rename="TimeLoggingStopped")]
pub time_logging_stopped: Option<String>,
            }
            
#[doc="<p>Requests the public keys for a specified time range.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListPublicKeysRequest {
                #[doc="<p>Optionally specifies, in UTC, the end of the time range to look up public keys for CloudTrail digest files. If not specified, the current time is used.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<Date>,
#[doc="<p>Reserved for future use.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<String>,
#[doc="<p>Optionally specifies, in UTC, the start of the time range to look up public keys for CloudTrail digest files. If not specified, the current time is used, and the current public key is returned.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<Date>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListPublicKeysResponse {
                #[doc="<p>Reserved for future use.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<String>,
#[doc="<p>Contains an array of PublicKey objects.</p> <note> <p>The returned public keys may have validity time ranges that overlap.</p> </note>"]
#[serde(rename="PublicKeyList")]
pub public_key_list: Option<PublicKeyList>,
            }
            
#[doc="<p>Specifies a list of trail tags to return.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListTagsRequest {
                #[doc="<p>Reserved for future use.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<String>,
#[doc="<p>Specifies a list of trail ARNs whose tags will be listed. The list has a limit of 20 ARNs. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="ResourceIdList")]
pub resource_id_list: ResourceIdList,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListTagsResponse {
                #[doc="<p>Reserved for future use.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<String>,
#[serde(rename="ResourceTagList")]
pub resource_tag_list: Option<ResourceTagList>,
            }
            
#[doc="<p>Specifies an attribute and value that filter the events returned.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct LookupAttribute {
                #[doc="<p>Specifies an attribute on which to filter the events returned.</p>"]
#[serde(rename="AttributeKey")]
pub attribute_key: LookupAttributeKey,
#[doc="<p>Specifies a value for the specified AttributeKey.</p>"]
#[serde(rename="AttributeValue")]
pub attribute_value: String,
            }
            
pub type LookupAttributeKey = String;
pub type LookupAttributesList = Vec<LookupAttribute>;
#[doc="<p>Contains a request for LookupEvents.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct LookupEventsRequest {
                #[doc="<p>Specifies that only events that occur before or at the specified time are returned. If the specified end time is before the specified start time, an error is returned.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<Date>,
#[doc="<p>Contains a list of lookup attributes. Currently the list can contain only one item.</p>"]
#[serde(rename="LookupAttributes")]
pub lookup_attributes: Option<LookupAttributesList>,
#[doc="<p>The number of events to return. Possible values are 1 through 50. The default is 10.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token to use to get the next page of results after a previous API call. This token must be passed in with the same parameters that were specified in the the original call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Specifies that only events that occur after or at the specified time are returned. If the specified start time is after the specified end time, an error is returned.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<Date>,
            }
            
#[doc="<p>Contains a response to a LookupEvents action.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct LookupEventsResponse {
                #[doc="<p>A list of events returned based on the lookup attributes specified and the CloudTrail event. The events list is sorted by time. The most recent event is listed first.</p>"]
#[serde(rename="Events")]
pub events: Option<EventsList>,
#[doc="<p>The token to use to get the next page of results after a previous API call. If the token does not appear, there are no more results to return. The token must be passed in with the same parameters as the previous call. For example, if the original call specified an AttributeKey of 'Username' with a value of 'root', the call with NextToken should include those same parameters.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
pub type MaxResults = i64;
pub type NextToken = String;
#[doc="<p>Contains information about a returned public key.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PublicKey {
                #[doc="<p>The fingerprint of the public key.</p>"]
#[serde(rename="Fingerprint")]
pub fingerprint: Option<String>,
#[doc="<p>The ending time of validity of the public key.</p>"]
#[serde(rename="ValidityEndTime")]
pub validity_end_time: Option<Date>,
#[doc="<p>The starting time of validity of the public key.</p>"]
#[serde(rename="ValidityStartTime")]
pub validity_start_time: Option<Date>,
#[doc="<p>The DER encoded public key value in PKCS#1 format.</p>"]
#[serde(rename="Value")]
#[serde(
                            deserialize_with="rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
pub value: Option<ByteBuffer>,
            }
            
pub type PublicKeyList = Vec<PublicKey>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutEventSelectorsRequest {
                #[doc="<p>Specifies the settings for your event selectors. You can configure up to five event selectors for a trail.</p>"]
#[serde(rename="EventSelectors")]
pub event_selectors: Option<EventSelectors>,
#[doc="<p>Specifies the name of the trail or trail ARN. If you specify a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If you specify a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailName")]
pub trail_name: Option<String>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutEventSelectorsResponse {
                #[doc="<p>Specifies the event selectors configured for your trail.</p>"]
#[serde(rename="EventSelectors")]
pub event_selectors: Option<EventSelectors>,
#[doc="<p>Specifies the ARN of the trail that was updated with event selectors. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailARN")]
pub trail_arn: Option<String>,
            }
            
pub type ReadWriteType = String;
#[doc="<p>Specifies the tags to remove from a trail.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTagsRequest {
                #[doc="<p>Specifies the ARN of the trail from which tags should be removed. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="ResourceId")]
pub resource_id: String,
#[doc="<p>Specifies a list of tags to be removed.</p>"]
#[serde(rename="TagsList")]
pub tags_list: Option<TagsList>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveTagsResponse;
            
#[doc="<p>Specifies the type and name of a resource referenced by an event.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Resource {
                #[doc="<p>The name of the resource referenced by the event returned. These are user-created names whose values will depend on the environment. For example, the resource name might be \"auto-scaling-test-group\" for an Auto Scaling Group or \"i-1234567\" for an EC2 Instance.</p>"]
#[serde(rename="ResourceName")]
pub resource_name: Option<String>,
#[doc="<p>The type of a resource referenced by the event returned. When the resource type cannot be determined, null is returned. Some examples of resource types are: <b>Instance</b> for EC2, <b>Trail</b> for CloudTrail, <b>DBInstance</b> for RDS, and <b>AccessKey</b> for IAM. For a list of resource types supported for event lookup, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/lookup_supported_resourcetypes.html\">Resource Types Supported for Event Lookup</a>.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: Option<String>,
            }
            
pub type ResourceIdList = Vec<String>;
#[doc="<p>A list of resources referenced by the event returned.</p>"]
pub type ResourceList = Vec<Resource>;
#[doc="<p>A resource tag.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ResourceTag {
                #[doc="<p>Specifies the ARN of the resource.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: Option<String>,
#[serde(rename="TagsList")]
pub tags_list: Option<TagsList>,
            }
            
#[doc="<p>A list of resource tags.</p>"]
pub type ResourceTagList = Vec<ResourceTag>;
#[doc="<p>The request to CloudTrail to start logging AWS API calls for an account.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct StartLoggingRequest {
                #[doc="<p>Specifies the name or the CloudTrail ARN of the trail for which CloudTrail logs AWS API calls. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="Name")]
pub name: String,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StartLoggingResponse;
            
#[doc="<p>Passes the request to CloudTrail to stop logging AWS API calls for the specified account.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct StopLoggingRequest {
                #[doc="<p>Specifies the name or the CloudTrail ARN of the trail for which CloudTrail will stop logging AWS API calls. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="Name")]
pub name: String,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StopLoggingResponse;
            
#[doc="<p>A custom key-value pair associated with a resource such as a CloudTrail trail.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>The key in a key-value pair. The key must be must be no longer than 128 Unicode characters. The key must be unique for the resource to which it applies.</p>"]
#[serde(rename="Key")]
pub key: String,
#[doc="<p>The value in a key-value pair of a tag. The value must be no longer than 256 Unicode characters.</p>"]
#[serde(rename="Value")]
pub value: Option<String>,
            }
            
#[doc="<p>A list of tags.</p>"]
pub type TagsList = Vec<Tag>;
#[doc="<p>The settings for a trail.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Trail {
                #[doc="<p>Specifies an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered.</p>"]
#[serde(rename="CloudWatchLogsLogGroupArn")]
pub cloud_watch_logs_log_group_arn: Option<String>,
#[doc="<p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>"]
#[serde(rename="CloudWatchLogsRoleArn")]
pub cloud_watch_logs_role_arn: Option<String>,
#[doc="<p>Specifies if the trail has custom event selectors.</p>"]
#[serde(rename="HasCustomEventSelectors")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub has_custom_event_selectors: Option<Boolean>,
#[doc="<p>The region in which the trail was created.</p>"]
#[serde(rename="HomeRegion")]
pub home_region: Option<String>,
#[doc="<p>Set to <b>True</b> to include AWS API calls from AWS global services such as IAM. Otherwise, <b>False</b>.</p>"]
#[serde(rename="IncludeGlobalServiceEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_global_service_events: Option<Boolean>,
#[doc="<p>Specifies whether the trail belongs only to one region or exists in all regions.</p>"]
#[serde(rename="IsMultiRegionTrail")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_multi_region_trail: Option<Boolean>,
#[doc="<p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>"]
#[serde(rename="KmsKeyId")]
pub kms_key_id: Option<String>,
#[doc="<p>Specifies whether log file validation is enabled.</p>"]
#[serde(rename="LogFileValidationEnabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub log_file_validation_enabled: Option<Boolean>,
#[doc="<p>Name of the trail set by calling <a>CreateTrail</a>. The maximum length is 128 characters.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>Name of the Amazon S3 bucket into which CloudTrail delivers your trail files. See <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html\">Amazon S3 Bucket Naming Requirements</a>.</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: Option<String>,
#[doc="<p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html\">Finding Your CloudTrail Log Files</a>.The maximum length is 200 characters.</p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<String>,
#[doc="<p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-1:123456789012:MyTopic</code> </p>"]
#[serde(rename="SnsTopicARN")]
pub sns_topic_arn: Option<String>,
#[doc="<p>Specifies the ARN of the trail. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailARN")]
pub trail_arn: Option<String>,
            }
            
pub type TrailList = Vec<Trail>;
pub type TrailNameList = Vec<String>;
#[doc="<p>Specifies settings to update for the trail.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateTrailRequest {
                #[doc="<p>Specifies a log group name using an Amazon Resource Name (ARN), a unique identifier that represents the log group to which CloudTrail logs will be delivered. Not required unless you specify CloudWatchLogsRoleArn.</p>"]
#[serde(rename="CloudWatchLogsLogGroupArn")]
pub cloud_watch_logs_log_group_arn: Option<String>,
#[doc="<p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>"]
#[serde(rename="CloudWatchLogsRoleArn")]
pub cloud_watch_logs_role_arn: Option<String>,
#[doc="<p>Specifies whether log file validation is enabled. The default is false.</p> <note> <p>When you disable log file integrity validation, the chain of digest files is broken after one hour. CloudTrail will not create digest files for log files that were delivered during a period in which log file integrity validation was disabled. For example, if you enable log file integrity validation at noon on January 1, disable it at noon on January 2, and re-enable it at noon on January 10, digest files will not be created for the log files delivered from noon on January 2 to noon on January 10. The same applies whenever you stop CloudTrail logging or delete a trail.</p> </note>"]
#[serde(rename="EnableLogFileValidation")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enable_log_file_validation: Option<Boolean>,
#[doc="<p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>"]
#[serde(rename="IncludeGlobalServiceEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_global_service_events: Option<Boolean>,
#[doc="<p>Specifies whether the trail applies only to the current region or to all regions. The default is false. If the trail exists only in the current region and this value is set to true, shadow trails (replications of the trail) will be created in the other regions. If the trail exists in all regions and this value is set to false, the trail will remain in the region where it was created, and its shadow trails in other regions will be deleted.</p>"]
#[serde(rename="IsMultiRegionTrail")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_multi_region_trail: Option<Boolean>,
#[doc="<p>Specifies the KMS key ID to use to encrypt the logs delivered by CloudTrail. The value can be a an alias name prefixed by \"alias/\", a fully specified ARN to an alias, a fully specified ARN to a key, or a globally unique identifier.</p> <p>Examples:</p> <ul> <li> <p>alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</p> </li> <li> <p>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</p> </li> <li> <p>12345678-1234-1234-1234-123456789012</p> </li> </ul>"]
#[serde(rename="KmsKeyId")]
pub kms_key_id: Option<String>,
#[doc="<p>Specifies the name of the trail or trail ARN. If <code>Name</code> is a trail name, the string must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul> <p>If <code>Name</code> is a trail ARN, it must be in the format:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="Name")]
pub name: String,
#[doc="<p>Specifies the name of the Amazon S3 bucket designated for publishing log files. See <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create_trail_naming_policy.html\">Amazon S3 Bucket Naming Requirements</a>.</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: Option<String>,
#[doc="<p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html\">Finding Your CloudTrail Log Files</a>. The maximum length is 200 characters.</p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<String>,
#[doc="<p>Specifies the name of the Amazon SNS topic defined for notification of log file delivery. The maximum length is 256 characters.</p>"]
#[serde(rename="SnsTopicName")]
pub sns_topic_name: Option<String>,
            }
            
#[doc="<p>Returns the objects or data listed below if successful. Otherwise, returns an error.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateTrailResponse {
                #[doc="<p>Specifies the Amazon Resource Name (ARN) of the log group to which CloudTrail logs will be delivered.</p>"]
#[serde(rename="CloudWatchLogsLogGroupArn")]
pub cloud_watch_logs_log_group_arn: Option<String>,
#[doc="<p>Specifies the role for the CloudWatch Logs endpoint to assume to write to a user's log group.</p>"]
#[serde(rename="CloudWatchLogsRoleArn")]
pub cloud_watch_logs_role_arn: Option<String>,
#[doc="<p>Specifies whether the trail is publishing events from global services such as IAM to the log files.</p>"]
#[serde(rename="IncludeGlobalServiceEvents")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub include_global_service_events: Option<Boolean>,
#[doc="<p>Specifies whether the trail exists in one region or in all regions.</p>"]
#[serde(rename="IsMultiRegionTrail")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_multi_region_trail: Option<Boolean>,
#[doc="<p>Specifies the KMS key ID that encrypts the logs delivered by CloudTrail. The value is a fully specified ARN to a KMS key in the format:</p> <p> <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p>"]
#[serde(rename="KmsKeyId")]
pub kms_key_id: Option<String>,
#[doc="<p>Specifies whether log file integrity validation is enabled.</p>"]
#[serde(rename="LogFileValidationEnabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub log_file_validation_enabled: Option<Boolean>,
#[doc="<p>Specifies the name of the trail.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>Specifies the name of the Amazon S3 bucket designated for publishing log files.</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: Option<String>,
#[doc="<p>Specifies the Amazon S3 key prefix that comes after the name of the bucket you have designated for log file delivery. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-find-log-files.html\">Finding Your CloudTrail Log Files</a>.</p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<String>,
#[doc="<p>Specifies the ARN of the Amazon SNS topic that CloudTrail uses to send notifications when log files are delivered. The format of a topic ARN is:</p> <p> <code>arn:aws:sns:us-east-1:123456789012:MyTopic</code> </p>"]
#[serde(rename="SnsTopicARN")]
pub sns_topic_arn: Option<String>,
#[doc="<p>Specifies the ARN of the trail that was updated. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>"]
#[serde(rename="TrailARN")]
pub trail_arn: Option<String>,
            }
            
/// Errors returned by AddTags
                #[derive(Debug, PartialEq)]
                pub enum AddTagsError {
                    
///<p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>
CloudTrailARNInvalid(String),
///<p>This exception is thrown when the key or value specified for the tag does not match the regular expression <code>^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*)$</code>.</p>
InvalidTagParameter(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the specified resource is not found.</p>
ResourceNotFound(String),
///<p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
ResourceTypeNotSupported(String),
///<p>The number of tags per trail has exceeded the permitted amount. Currently, the limit is 50.</p>
TagsLimitExceeded(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddTagsError {
                    pub fn from_body(body: &str) -> AddTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "CloudTrailARNInvalidException" => AddTagsError::CloudTrailARNInvalid(String::from(error_message)),"InvalidTagParameterException" => AddTagsError::InvalidTagParameter(String::from(error_message)),"InvalidTrailNameException" => AddTagsError::InvalidTrailName(String::from(error_message)),"OperationNotPermittedException" => AddTagsError::OperationNotPermitted(String::from(error_message)),"ResourceNotFoundException" => AddTagsError::ResourceNotFound(String::from(error_message)),"ResourceTypeNotSupportedException" => AddTagsError::ResourceTypeNotSupported(String::from(error_message)),"TagsLimitExceededException" => AddTagsError::TagsLimitExceeded(String::from(error_message)),"UnsupportedOperationException" => AddTagsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => AddTagsError::Validation(error_message.to_string()),_ => AddTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddTagsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddTagsError {
                    fn from(err: serde_json::error::Error) -> AddTagsError {
                        AddTagsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddTagsError {
                    fn from(err: CredentialsError) -> AddTagsError {
                        AddTagsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddTagsError {
                    fn from(err: HttpDispatchError) -> AddTagsError {
                        AddTagsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddTagsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddTagsError {
                    fn description(&self) -> &str {
                        match *self {
                            AddTagsError::CloudTrailARNInvalid(ref cause) => cause,AddTagsError::InvalidTagParameter(ref cause) => cause,AddTagsError::InvalidTrailName(ref cause) => cause,AddTagsError::OperationNotPermitted(ref cause) => cause,AddTagsError::ResourceNotFound(ref cause) => cause,AddTagsError::ResourceTypeNotSupported(ref cause) => cause,AddTagsError::TagsLimitExceeded(ref cause) => cause,AddTagsError::UnsupportedOperation(ref cause) => cause,AddTagsError::Validation(ref cause) => cause,AddTagsError::Credentials(ref err) => err.description(),AddTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),AddTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateTrail
                #[derive(Debug, PartialEq)]
                pub enum CreateTrailError {
                    
///<p>Cannot set a CloudWatch Logs delivery for this region.</p>
CloudWatchLogsDeliveryUnavailable(String),
///<p>This exception is thrown when the policy on the S3 bucket or KMS key is not sufficient.</p>
InsufficientEncryptionPolicy(String),
///<p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
InsufficientS3BucketPolicy(String),
///<p>This exception is thrown when the policy on the SNS topic is not sufficient.</p>
InsufficientSnsTopicPolicy(String),
///<p>This exception is thrown when the provided CloudWatch log group is not valid.</p>
InvalidCloudWatchLogsLogGroupArn(String),
///<p>This exception is thrown when the provided role is not valid.</p>
InvalidCloudWatchLogsRoleArn(String),
///<p>This exception is thrown when the KMS key ARN is invalid.</p>
InvalidKmsKeyId(String),
///<p>This exception is thrown when the combination of parameters provided is not valid.</p>
InvalidParameterCombination(String),
///<p>This exception is thrown when the provided S3 bucket name is not valid.</p>
InvalidS3BucketName(String),
///<p>This exception is thrown when the provided S3 prefix is not valid.</p>
InvalidS3Prefix(String),
///<p>This exception is thrown when the provided SNS topic name is not valid.</p>
InvalidSnsTopicName(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when there is an issue with the specified KMS key and the trail can’t be updated.</p>
Kms(String),
///<p>This exception is deprecated.</p>
KmsKeyDisabled(String),
///<p>This exception is thrown when the KMS key does not exist, or when the S3 bucket and the KMS key are not in the same region.</p>
KmsKeyNotFound(String),
///<p>This exception is thrown when the maximum number of trails is reached.</p>
MaximumNumberOfTrailsExceeded(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the specified S3 bucket does not exist.</p>
S3BucketDoesNotExist(String),
///<p>This exception is thrown when the specified trail already exists.</p>
TrailAlreadyExists(String),
///<p>This exception is deprecated.</p>
TrailNotProvided(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateTrailError {
                    pub fn from_body(body: &str) -> CreateTrailError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "CloudWatchLogsDeliveryUnavailableException" => CreateTrailError::CloudWatchLogsDeliveryUnavailable(String::from(error_message)),"InsufficientEncryptionPolicyException" => CreateTrailError::InsufficientEncryptionPolicy(String::from(error_message)),"InsufficientS3BucketPolicyException" => CreateTrailError::InsufficientS3BucketPolicy(String::from(error_message)),"InsufficientSnsTopicPolicyException" => CreateTrailError::InsufficientSnsTopicPolicy(String::from(error_message)),"InvalidCloudWatchLogsLogGroupArnException" => CreateTrailError::InvalidCloudWatchLogsLogGroupArn(String::from(error_message)),"InvalidCloudWatchLogsRoleArnException" => CreateTrailError::InvalidCloudWatchLogsRoleArn(String::from(error_message)),"InvalidKmsKeyIdException" => CreateTrailError::InvalidKmsKeyId(String::from(error_message)),"InvalidParameterCombinationException" => CreateTrailError::InvalidParameterCombination(String::from(error_message)),"InvalidS3BucketNameException" => CreateTrailError::InvalidS3BucketName(String::from(error_message)),"InvalidS3PrefixException" => CreateTrailError::InvalidS3Prefix(String::from(error_message)),"InvalidSnsTopicNameException" => CreateTrailError::InvalidSnsTopicName(String::from(error_message)),"InvalidTrailNameException" => CreateTrailError::InvalidTrailName(String::from(error_message)),"KmsException" => CreateTrailError::Kms(String::from(error_message)),"KmsKeyDisabledException" => CreateTrailError::KmsKeyDisabled(String::from(error_message)),"KmsKeyNotFoundException" => CreateTrailError::KmsKeyNotFound(String::from(error_message)),"MaximumNumberOfTrailsExceededException" => CreateTrailError::MaximumNumberOfTrailsExceeded(String::from(error_message)),"OperationNotPermittedException" => CreateTrailError::OperationNotPermitted(String::from(error_message)),"S3BucketDoesNotExistException" => CreateTrailError::S3BucketDoesNotExist(String::from(error_message)),"TrailAlreadyExistsException" => CreateTrailError::TrailAlreadyExists(String::from(error_message)),"TrailNotProvidedException" => CreateTrailError::TrailNotProvided(String::from(error_message)),"UnsupportedOperationException" => CreateTrailError::UnsupportedOperation(String::from(error_message)),"ValidationException" => CreateTrailError::Validation(error_message.to_string()),_ => CreateTrailError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateTrailError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateTrailError {
                    fn from(err: serde_json::error::Error) -> CreateTrailError {
                        CreateTrailError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateTrailError {
                    fn from(err: CredentialsError) -> CreateTrailError {
                        CreateTrailError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateTrailError {
                    fn from(err: HttpDispatchError) -> CreateTrailError {
                        CreateTrailError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateTrailError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateTrailError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateTrailError::CloudWatchLogsDeliveryUnavailable(ref cause) => cause,CreateTrailError::InsufficientEncryptionPolicy(ref cause) => cause,CreateTrailError::InsufficientS3BucketPolicy(ref cause) => cause,CreateTrailError::InsufficientSnsTopicPolicy(ref cause) => cause,CreateTrailError::InvalidCloudWatchLogsLogGroupArn(ref cause) => cause,CreateTrailError::InvalidCloudWatchLogsRoleArn(ref cause) => cause,CreateTrailError::InvalidKmsKeyId(ref cause) => cause,CreateTrailError::InvalidParameterCombination(ref cause) => cause,CreateTrailError::InvalidS3BucketName(ref cause) => cause,CreateTrailError::InvalidS3Prefix(ref cause) => cause,CreateTrailError::InvalidSnsTopicName(ref cause) => cause,CreateTrailError::InvalidTrailName(ref cause) => cause,CreateTrailError::Kms(ref cause) => cause,CreateTrailError::KmsKeyDisabled(ref cause) => cause,CreateTrailError::KmsKeyNotFound(ref cause) => cause,CreateTrailError::MaximumNumberOfTrailsExceeded(ref cause) => cause,CreateTrailError::OperationNotPermitted(ref cause) => cause,CreateTrailError::S3BucketDoesNotExist(ref cause) => cause,CreateTrailError::TrailAlreadyExists(ref cause) => cause,CreateTrailError::TrailNotProvided(ref cause) => cause,CreateTrailError::UnsupportedOperation(ref cause) => cause,CreateTrailError::Validation(ref cause) => cause,CreateTrailError::Credentials(ref err) => err.description(),CreateTrailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateTrailError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteTrail
                #[derive(Debug, PartialEq)]
                pub enum DeleteTrailError {
                    
///<p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
InvalidHomeRegion(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteTrailError {
                    pub fn from_body(body: &str) -> DeleteTrailError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidHomeRegionException" => DeleteTrailError::InvalidHomeRegion(String::from(error_message)),"InvalidTrailNameException" => DeleteTrailError::InvalidTrailName(String::from(error_message)),"TrailNotFoundException" => DeleteTrailError::TrailNotFound(String::from(error_message)),"ValidationException" => DeleteTrailError::Validation(error_message.to_string()),_ => DeleteTrailError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteTrailError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteTrailError {
                    fn from(err: serde_json::error::Error) -> DeleteTrailError {
                        DeleteTrailError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteTrailError {
                    fn from(err: CredentialsError) -> DeleteTrailError {
                        DeleteTrailError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteTrailError {
                    fn from(err: HttpDispatchError) -> DeleteTrailError {
                        DeleteTrailError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteTrailError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteTrailError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteTrailError::InvalidHomeRegion(ref cause) => cause,DeleteTrailError::InvalidTrailName(ref cause) => cause,DeleteTrailError::TrailNotFound(ref cause) => cause,DeleteTrailError::Validation(ref cause) => cause,DeleteTrailError::Credentials(ref err) => err.description(),DeleteTrailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteTrailError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeTrails
                #[derive(Debug, PartialEq)]
                pub enum DescribeTrailsError {
                    
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeTrailsError {
                    pub fn from_body(body: &str) -> DescribeTrailsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "OperationNotPermittedException" => DescribeTrailsError::OperationNotPermitted(String::from(error_message)),"UnsupportedOperationException" => DescribeTrailsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => DescribeTrailsError::Validation(error_message.to_string()),_ => DescribeTrailsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeTrailsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeTrailsError {
                    fn from(err: serde_json::error::Error) -> DescribeTrailsError {
                        DescribeTrailsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeTrailsError {
                    fn from(err: CredentialsError) -> DescribeTrailsError {
                        DescribeTrailsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeTrailsError {
                    fn from(err: HttpDispatchError) -> DescribeTrailsError {
                        DescribeTrailsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeTrailsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeTrailsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeTrailsError::OperationNotPermitted(ref cause) => cause,DescribeTrailsError::UnsupportedOperation(ref cause) => cause,DescribeTrailsError::Validation(ref cause) => cause,DescribeTrailsError::Credentials(ref err) => err.description(),DescribeTrailsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeTrailsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetEventSelectors
                #[derive(Debug, PartialEq)]
                pub enum GetEventSelectorsError {
                    
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetEventSelectorsError {
                    pub fn from_body(body: &str) -> GetEventSelectorsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidTrailNameException" => GetEventSelectorsError::InvalidTrailName(String::from(error_message)),"OperationNotPermittedException" => GetEventSelectorsError::OperationNotPermitted(String::from(error_message)),"TrailNotFoundException" => GetEventSelectorsError::TrailNotFound(String::from(error_message)),"UnsupportedOperationException" => GetEventSelectorsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => GetEventSelectorsError::Validation(error_message.to_string()),_ => GetEventSelectorsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetEventSelectorsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetEventSelectorsError {
                    fn from(err: serde_json::error::Error) -> GetEventSelectorsError {
                        GetEventSelectorsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetEventSelectorsError {
                    fn from(err: CredentialsError) -> GetEventSelectorsError {
                        GetEventSelectorsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetEventSelectorsError {
                    fn from(err: HttpDispatchError) -> GetEventSelectorsError {
                        GetEventSelectorsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetEventSelectorsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetEventSelectorsError {
                    fn description(&self) -> &str {
                        match *self {
                            GetEventSelectorsError::InvalidTrailName(ref cause) => cause,GetEventSelectorsError::OperationNotPermitted(ref cause) => cause,GetEventSelectorsError::TrailNotFound(ref cause) => cause,GetEventSelectorsError::UnsupportedOperation(ref cause) => cause,GetEventSelectorsError::Validation(ref cause) => cause,GetEventSelectorsError::Credentials(ref err) => err.description(),GetEventSelectorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetEventSelectorsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetTrailStatus
                #[derive(Debug, PartialEq)]
                pub enum GetTrailStatusError {
                    
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetTrailStatusError {
                    pub fn from_body(body: &str) -> GetTrailStatusError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidTrailNameException" => GetTrailStatusError::InvalidTrailName(String::from(error_message)),"TrailNotFoundException" => GetTrailStatusError::TrailNotFound(String::from(error_message)),"ValidationException" => GetTrailStatusError::Validation(error_message.to_string()),_ => GetTrailStatusError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetTrailStatusError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetTrailStatusError {
                    fn from(err: serde_json::error::Error) -> GetTrailStatusError {
                        GetTrailStatusError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetTrailStatusError {
                    fn from(err: CredentialsError) -> GetTrailStatusError {
                        GetTrailStatusError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetTrailStatusError {
                    fn from(err: HttpDispatchError) -> GetTrailStatusError {
                        GetTrailStatusError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetTrailStatusError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetTrailStatusError {
                    fn description(&self) -> &str {
                        match *self {
                            GetTrailStatusError::InvalidTrailName(ref cause) => cause,GetTrailStatusError::TrailNotFound(ref cause) => cause,GetTrailStatusError::Validation(ref cause) => cause,GetTrailStatusError::Credentials(ref err) => err.description(),GetTrailStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetTrailStatusError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListPublicKeys
                #[derive(Debug, PartialEq)]
                pub enum ListPublicKeysError {
                    
///<p>Occurs if the timestamp values are invalid. Either the start time occurs after the end time or the time range is outside the range of possible values.</p>
InvalidTimeRange(String),
///<p>Reserved for future use.</p>
InvalidToken(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListPublicKeysError {
                    pub fn from_body(body: &str) -> ListPublicKeysError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidTimeRangeException" => ListPublicKeysError::InvalidTimeRange(String::from(error_message)),"InvalidTokenException" => ListPublicKeysError::InvalidToken(String::from(error_message)),"OperationNotPermittedException" => ListPublicKeysError::OperationNotPermitted(String::from(error_message)),"UnsupportedOperationException" => ListPublicKeysError::UnsupportedOperation(String::from(error_message)),"ValidationException" => ListPublicKeysError::Validation(error_message.to_string()),_ => ListPublicKeysError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListPublicKeysError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListPublicKeysError {
                    fn from(err: serde_json::error::Error) -> ListPublicKeysError {
                        ListPublicKeysError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListPublicKeysError {
                    fn from(err: CredentialsError) -> ListPublicKeysError {
                        ListPublicKeysError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListPublicKeysError {
                    fn from(err: HttpDispatchError) -> ListPublicKeysError {
                        ListPublicKeysError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListPublicKeysError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListPublicKeysError {
                    fn description(&self) -> &str {
                        match *self {
                            ListPublicKeysError::InvalidTimeRange(ref cause) => cause,ListPublicKeysError::InvalidToken(ref cause) => cause,ListPublicKeysError::OperationNotPermitted(ref cause) => cause,ListPublicKeysError::UnsupportedOperation(ref cause) => cause,ListPublicKeysError::Validation(ref cause) => cause,ListPublicKeysError::Credentials(ref err) => err.description(),ListPublicKeysError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListPublicKeysError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTags
                #[derive(Debug, PartialEq)]
                pub enum ListTagsError {
                    
///<p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>
CloudTrailARNInvalid(String),
///<p>Reserved for future use.</p>
InvalidToken(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the specified resource is not found.</p>
ResourceNotFound(String),
///<p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
ResourceTypeNotSupported(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTagsError {
                    pub fn from_body(body: &str) -> ListTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "CloudTrailARNInvalidException" => ListTagsError::CloudTrailARNInvalid(String::from(error_message)),"InvalidTokenException" => ListTagsError::InvalidToken(String::from(error_message)),"InvalidTrailNameException" => ListTagsError::InvalidTrailName(String::from(error_message)),"OperationNotPermittedException" => ListTagsError::OperationNotPermitted(String::from(error_message)),"ResourceNotFoundException" => ListTagsError::ResourceNotFound(String::from(error_message)),"ResourceTypeNotSupportedException" => ListTagsError::ResourceTypeNotSupported(String::from(error_message)),"UnsupportedOperationException" => ListTagsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => ListTagsError::Validation(error_message.to_string()),_ => ListTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListTagsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListTagsError {
                    fn from(err: serde_json::error::Error) -> ListTagsError {
                        ListTagsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListTagsError {
                    fn from(err: CredentialsError) -> ListTagsError {
                        ListTagsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTagsError {
                    fn from(err: HttpDispatchError) -> ListTagsError {
                        ListTagsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTagsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTagsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTagsError::CloudTrailARNInvalid(ref cause) => cause,ListTagsError::InvalidToken(ref cause) => cause,ListTagsError::InvalidTrailName(ref cause) => cause,ListTagsError::OperationNotPermitted(ref cause) => cause,ListTagsError::ResourceNotFound(ref cause) => cause,ListTagsError::ResourceTypeNotSupported(ref cause) => cause,ListTagsError::UnsupportedOperation(ref cause) => cause,ListTagsError::Validation(ref cause) => cause,ListTagsError::Credentials(ref err) => err.description(),ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by LookupEvents
                #[derive(Debug, PartialEq)]
                pub enum LookupEventsError {
                    
///<p>Occurs when an invalid lookup attribute is specified.</p>
InvalidLookupAttributes(String),
///<p>This exception is thrown if the limit specified is invalid.</p>
InvalidMaxResults(String),
///<p>Invalid token or token that was previously used in a request with different parameters. This exception is thrown if the token is invalid.</p>
InvalidNextToken(String),
///<p>Occurs if the timestamp values are invalid. Either the start time occurs after the end time or the time range is outside the range of possible values.</p>
InvalidTimeRange(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl LookupEventsError {
                    pub fn from_body(body: &str) -> LookupEventsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidLookupAttributesException" => LookupEventsError::InvalidLookupAttributes(String::from(error_message)),"InvalidMaxResultsException" => LookupEventsError::InvalidMaxResults(String::from(error_message)),"InvalidNextTokenException" => LookupEventsError::InvalidNextToken(String::from(error_message)),"InvalidTimeRangeException" => LookupEventsError::InvalidTimeRange(String::from(error_message)),"ValidationException" => LookupEventsError::Validation(error_message.to_string()),_ => LookupEventsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => LookupEventsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for LookupEventsError {
                    fn from(err: serde_json::error::Error) -> LookupEventsError {
                        LookupEventsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for LookupEventsError {
                    fn from(err: CredentialsError) -> LookupEventsError {
                        LookupEventsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for LookupEventsError {
                    fn from(err: HttpDispatchError) -> LookupEventsError {
                        LookupEventsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for LookupEventsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for LookupEventsError {
                    fn description(&self) -> &str {
                        match *self {
                            LookupEventsError::InvalidLookupAttributes(ref cause) => cause,LookupEventsError::InvalidMaxResults(ref cause) => cause,LookupEventsError::InvalidNextToken(ref cause) => cause,LookupEventsError::InvalidTimeRange(ref cause) => cause,LookupEventsError::Validation(ref cause) => cause,LookupEventsError::Credentials(ref err) => err.description(),LookupEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),LookupEventsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutEventSelectors
                #[derive(Debug, PartialEq)]
                pub enum PutEventSelectorsError {
                    
///<p>This exception is thrown when the <code>PutEventSelectors</code> operation is called with an invalid number of event selectors, data resources, or an invalid value for a parameter:</p> <ul> <li> <p>Specify a valid number of event selectors (1 to 5) for a trail.</p> </li> <li> <p>Specify a valid number of data resources (1 to 50) for an event selector.</p> </li> <li> <p>Specify a valid value for a parameter. For example, specifying the <code>ReadWriteType</code> parameter with a value of <code>read-only</code> is invalid.</p> </li> </ul>
InvalidEventSelectors(String),
///<p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
InvalidHomeRegion(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutEventSelectorsError {
                    pub fn from_body(body: &str) -> PutEventSelectorsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidEventSelectorsException" => PutEventSelectorsError::InvalidEventSelectors(String::from(error_message)),"InvalidHomeRegionException" => PutEventSelectorsError::InvalidHomeRegion(String::from(error_message)),"InvalidTrailNameException" => PutEventSelectorsError::InvalidTrailName(String::from(error_message)),"OperationNotPermittedException" => PutEventSelectorsError::OperationNotPermitted(String::from(error_message)),"TrailNotFoundException" => PutEventSelectorsError::TrailNotFound(String::from(error_message)),"UnsupportedOperationException" => PutEventSelectorsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => PutEventSelectorsError::Validation(error_message.to_string()),_ => PutEventSelectorsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutEventSelectorsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutEventSelectorsError {
                    fn from(err: serde_json::error::Error) -> PutEventSelectorsError {
                        PutEventSelectorsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutEventSelectorsError {
                    fn from(err: CredentialsError) -> PutEventSelectorsError {
                        PutEventSelectorsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutEventSelectorsError {
                    fn from(err: HttpDispatchError) -> PutEventSelectorsError {
                        PutEventSelectorsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutEventSelectorsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutEventSelectorsError {
                    fn description(&self) -> &str {
                        match *self {
                            PutEventSelectorsError::InvalidEventSelectors(ref cause) => cause,PutEventSelectorsError::InvalidHomeRegion(ref cause) => cause,PutEventSelectorsError::InvalidTrailName(ref cause) => cause,PutEventSelectorsError::OperationNotPermitted(ref cause) => cause,PutEventSelectorsError::TrailNotFound(ref cause) => cause,PutEventSelectorsError::UnsupportedOperation(ref cause) => cause,PutEventSelectorsError::Validation(ref cause) => cause,PutEventSelectorsError::Credentials(ref err) => err.description(),PutEventSelectorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutEventSelectorsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTags
                #[derive(Debug, PartialEq)]
                pub enum RemoveTagsError {
                    
///<p>This exception is thrown when an operation is called with an invalid trail ARN. The format of a trail ARN is:</p> <p> <code>arn:aws:cloudtrail:us-east-1:123456789012:trail/MyTrail</code> </p>
CloudTrailARNInvalid(String),
///<p>This exception is thrown when the key or value specified for the tag does not match the regular expression <code>^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*)$</code>.</p>
InvalidTagParameter(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the specified resource is not found.</p>
ResourceNotFound(String),
///<p>This exception is thrown when the specified resource type is not supported by CloudTrail.</p>
ResourceTypeNotSupported(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTagsError {
                    pub fn from_body(body: &str) -> RemoveTagsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "CloudTrailARNInvalidException" => RemoveTagsError::CloudTrailARNInvalid(String::from(error_message)),"InvalidTagParameterException" => RemoveTagsError::InvalidTagParameter(String::from(error_message)),"InvalidTrailNameException" => RemoveTagsError::InvalidTrailName(String::from(error_message)),"OperationNotPermittedException" => RemoveTagsError::OperationNotPermitted(String::from(error_message)),"ResourceNotFoundException" => RemoveTagsError::ResourceNotFound(String::from(error_message)),"ResourceTypeNotSupportedException" => RemoveTagsError::ResourceTypeNotSupported(String::from(error_message)),"UnsupportedOperationException" => RemoveTagsError::UnsupportedOperation(String::from(error_message)),"ValidationException" => RemoveTagsError::Validation(error_message.to_string()),_ => RemoveTagsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTagsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTagsError {
                    fn from(err: serde_json::error::Error) -> RemoveTagsError {
                        RemoveTagsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveTagsError {
                    fn from(err: CredentialsError) -> RemoveTagsError {
                        RemoveTagsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveTagsError {
                    fn from(err: HttpDispatchError) -> RemoveTagsError {
                        RemoveTagsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveTagsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTagsError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTagsError::CloudTrailARNInvalid(ref cause) => cause,RemoveTagsError::InvalidTagParameter(ref cause) => cause,RemoveTagsError::InvalidTrailName(ref cause) => cause,RemoveTagsError::OperationNotPermitted(ref cause) => cause,RemoveTagsError::ResourceNotFound(ref cause) => cause,RemoveTagsError::ResourceTypeNotSupported(ref cause) => cause,RemoveTagsError::UnsupportedOperation(ref cause) => cause,RemoveTagsError::Validation(ref cause) => cause,RemoveTagsError::Credentials(ref err) => err.description(),RemoveTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RemoveTagsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by StartLogging
                #[derive(Debug, PartialEq)]
                pub enum StartLoggingError {
                    
///<p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
InvalidHomeRegion(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl StartLoggingError {
                    pub fn from_body(body: &str) -> StartLoggingError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidHomeRegionException" => StartLoggingError::InvalidHomeRegion(String::from(error_message)),"InvalidTrailNameException" => StartLoggingError::InvalidTrailName(String::from(error_message)),"TrailNotFoundException" => StartLoggingError::TrailNotFound(String::from(error_message)),"ValidationException" => StartLoggingError::Validation(error_message.to_string()),_ => StartLoggingError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => StartLoggingError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for StartLoggingError {
                    fn from(err: serde_json::error::Error) -> StartLoggingError {
                        StartLoggingError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for StartLoggingError {
                    fn from(err: CredentialsError) -> StartLoggingError {
                        StartLoggingError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for StartLoggingError {
                    fn from(err: HttpDispatchError) -> StartLoggingError {
                        StartLoggingError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for StartLoggingError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for StartLoggingError {
                    fn description(&self) -> &str {
                        match *self {
                            StartLoggingError::InvalidHomeRegion(ref cause) => cause,StartLoggingError::InvalidTrailName(ref cause) => cause,StartLoggingError::TrailNotFound(ref cause) => cause,StartLoggingError::Validation(ref cause) => cause,StartLoggingError::Credentials(ref err) => err.description(),StartLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),StartLoggingError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by StopLogging
                #[derive(Debug, PartialEq)]
                pub enum StopLoggingError {
                    
///<p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
InvalidHomeRegion(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl StopLoggingError {
                    pub fn from_body(body: &str) -> StopLoggingError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidHomeRegionException" => StopLoggingError::InvalidHomeRegion(String::from(error_message)),"InvalidTrailNameException" => StopLoggingError::InvalidTrailName(String::from(error_message)),"TrailNotFoundException" => StopLoggingError::TrailNotFound(String::from(error_message)),"ValidationException" => StopLoggingError::Validation(error_message.to_string()),_ => StopLoggingError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => StopLoggingError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for StopLoggingError {
                    fn from(err: serde_json::error::Error) -> StopLoggingError {
                        StopLoggingError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for StopLoggingError {
                    fn from(err: CredentialsError) -> StopLoggingError {
                        StopLoggingError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for StopLoggingError {
                    fn from(err: HttpDispatchError) -> StopLoggingError {
                        StopLoggingError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for StopLoggingError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for StopLoggingError {
                    fn description(&self) -> &str {
                        match *self {
                            StopLoggingError::InvalidHomeRegion(ref cause) => cause,StopLoggingError::InvalidTrailName(ref cause) => cause,StopLoggingError::TrailNotFound(ref cause) => cause,StopLoggingError::Validation(ref cause) => cause,StopLoggingError::Credentials(ref err) => err.description(),StopLoggingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),StopLoggingError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateTrail
                #[derive(Debug, PartialEq)]
                pub enum UpdateTrailError {
                    
///<p>Cannot set a CloudWatch Logs delivery for this region.</p>
CloudWatchLogsDeliveryUnavailable(String),
///<p>This exception is thrown when the policy on the S3 bucket or KMS key is not sufficient.</p>
InsufficientEncryptionPolicy(String),
///<p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
InsufficientS3BucketPolicy(String),
///<p>This exception is thrown when the policy on the SNS topic is not sufficient.</p>
InsufficientSnsTopicPolicy(String),
///<p>This exception is thrown when the provided CloudWatch log group is not valid.</p>
InvalidCloudWatchLogsLogGroupArn(String),
///<p>This exception is thrown when the provided role is not valid.</p>
InvalidCloudWatchLogsRoleArn(String),
///<p>This exception is thrown when an operation is called on a trail from a region other than the region in which the trail was created.</p>
InvalidHomeRegion(String),
///<p>This exception is thrown when the KMS key ARN is invalid.</p>
InvalidKmsKeyId(String),
///<p>This exception is thrown when the combination of parameters provided is not valid.</p>
InvalidParameterCombination(String),
///<p>This exception is thrown when the provided S3 bucket name is not valid.</p>
InvalidS3BucketName(String),
///<p>This exception is thrown when the provided S3 prefix is not valid.</p>
InvalidS3Prefix(String),
///<p>This exception is thrown when the provided SNS topic name is not valid.</p>
InvalidSnsTopicName(String),
///<p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p> <ul> <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li> <li> <p>Start with a letter or number, and end with a letter or number</p> </li> <li> <p>Be between 3 and 128 characters</p> </li> <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are invalid.</p> </li> <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li> </ul>
InvalidTrailName(String),
///<p>This exception is thrown when there is an issue with the specified KMS key and the trail can’t be updated.</p>
Kms(String),
///<p>This exception is deprecated.</p>
KmsKeyDisabled(String),
///<p>This exception is thrown when the KMS key does not exist, or when the S3 bucket and the KMS key are not in the same region.</p>
KmsKeyNotFound(String),
///<p>This exception is thrown when the requested operation is not permitted.</p>
OperationNotPermitted(String),
///<p>This exception is thrown when the specified S3 bucket does not exist.</p>
S3BucketDoesNotExist(String),
///<p>This exception is thrown when the trail with the given name is not found.</p>
TrailNotFound(String),
///<p>This exception is deprecated.</p>
TrailNotProvided(String),
///<p>This exception is thrown when the requested operation is not supported.</p>
UnsupportedOperation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateTrailError {
                    pub fn from_body(body: &str) -> UpdateTrailError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "CloudWatchLogsDeliveryUnavailableException" => UpdateTrailError::CloudWatchLogsDeliveryUnavailable(String::from(error_message)),"InsufficientEncryptionPolicyException" => UpdateTrailError::InsufficientEncryptionPolicy(String::from(error_message)),"InsufficientS3BucketPolicyException" => UpdateTrailError::InsufficientS3BucketPolicy(String::from(error_message)),"InsufficientSnsTopicPolicyException" => UpdateTrailError::InsufficientSnsTopicPolicy(String::from(error_message)),"InvalidCloudWatchLogsLogGroupArnException" => UpdateTrailError::InvalidCloudWatchLogsLogGroupArn(String::from(error_message)),"InvalidCloudWatchLogsRoleArnException" => UpdateTrailError::InvalidCloudWatchLogsRoleArn(String::from(error_message)),"InvalidHomeRegionException" => UpdateTrailError::InvalidHomeRegion(String::from(error_message)),"InvalidKmsKeyIdException" => UpdateTrailError::InvalidKmsKeyId(String::from(error_message)),"InvalidParameterCombinationException" => UpdateTrailError::InvalidParameterCombination(String::from(error_message)),"InvalidS3BucketNameException" => UpdateTrailError::InvalidS3BucketName(String::from(error_message)),"InvalidS3PrefixException" => UpdateTrailError::InvalidS3Prefix(String::from(error_message)),"InvalidSnsTopicNameException" => UpdateTrailError::InvalidSnsTopicName(String::from(error_message)),"InvalidTrailNameException" => UpdateTrailError::InvalidTrailName(String::from(error_message)),"KmsException" => UpdateTrailError::Kms(String::from(error_message)),"KmsKeyDisabledException" => UpdateTrailError::KmsKeyDisabled(String::from(error_message)),"KmsKeyNotFoundException" => UpdateTrailError::KmsKeyNotFound(String::from(error_message)),"OperationNotPermittedException" => UpdateTrailError::OperationNotPermitted(String::from(error_message)),"S3BucketDoesNotExistException" => UpdateTrailError::S3BucketDoesNotExist(String::from(error_message)),"TrailNotFoundException" => UpdateTrailError::TrailNotFound(String::from(error_message)),"TrailNotProvidedException" => UpdateTrailError::TrailNotProvided(String::from(error_message)),"UnsupportedOperationException" => UpdateTrailError::UnsupportedOperation(String::from(error_message)),"ValidationException" => UpdateTrailError::Validation(error_message.to_string()),_ => UpdateTrailError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateTrailError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateTrailError {
                    fn from(err: serde_json::error::Error) -> UpdateTrailError {
                        UpdateTrailError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateTrailError {
                    fn from(err: CredentialsError) -> UpdateTrailError {
                        UpdateTrailError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateTrailError {
                    fn from(err: HttpDispatchError) -> UpdateTrailError {
                        UpdateTrailError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateTrailError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateTrailError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateTrailError::CloudWatchLogsDeliveryUnavailable(ref cause) => cause,UpdateTrailError::InsufficientEncryptionPolicy(ref cause) => cause,UpdateTrailError::InsufficientS3BucketPolicy(ref cause) => cause,UpdateTrailError::InsufficientSnsTopicPolicy(ref cause) => cause,UpdateTrailError::InvalidCloudWatchLogsLogGroupArn(ref cause) => cause,UpdateTrailError::InvalidCloudWatchLogsRoleArn(ref cause) => cause,UpdateTrailError::InvalidHomeRegion(ref cause) => cause,UpdateTrailError::InvalidKmsKeyId(ref cause) => cause,UpdateTrailError::InvalidParameterCombination(ref cause) => cause,UpdateTrailError::InvalidS3BucketName(ref cause) => cause,UpdateTrailError::InvalidS3Prefix(ref cause) => cause,UpdateTrailError::InvalidSnsTopicName(ref cause) => cause,UpdateTrailError::InvalidTrailName(ref cause) => cause,UpdateTrailError::Kms(ref cause) => cause,UpdateTrailError::KmsKeyDisabled(ref cause) => cause,UpdateTrailError::KmsKeyNotFound(ref cause) => cause,UpdateTrailError::OperationNotPermitted(ref cause) => cause,UpdateTrailError::S3BucketDoesNotExist(ref cause) => cause,UpdateTrailError::TrailNotFound(ref cause) => cause,UpdateTrailError::TrailNotProvided(ref cause) => cause,UpdateTrailError::UnsupportedOperation(ref cause) => cause,UpdateTrailError::Validation(ref cause) => cause,UpdateTrailError::Credentials(ref err) => err.description(),UpdateTrailError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateTrailError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// A client for the CloudTrail API.
        pub struct CloudTrailClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> CloudTrailClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  CloudTrailClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }

        

                #[doc="<p>Adds one or more tags to a trail, up to a limit of 50. Tags must be unique per trail. Overwrites an existing tag's value when a new value is specified for an existing tag key. If you specify a key without a value, the tag will be created with the specified key and a value of null. You can tag a trail that applies to all regions only from the region in which the trail was created (that is, from its home region).</p>"]
                pub fn add_tags(&self, input: &AddTagsRequest)  -> Result<AddTagsResponse, AddTagsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.AddTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AddTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a trail that specifies the settings for delivery of log data to an Amazon S3 bucket. A maximum of five trails can exist in a region, irrespective of the region in which they were created.</p>"]
                pub fn create_trail(&self, input: &CreateTrailRequest)  -> Result<CreateTrailResponse, CreateTrailError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.CreateTrail");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateTrailResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateTrailError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a trail. This operation must be called from the region in which the trail was created. <code>DeleteTrail</code> cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>"]
                pub fn delete_trail(&self, input: &DeleteTrailRequest)  -> Result<DeleteTrailResponse, DeleteTrailError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DeleteTrail");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteTrailResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteTrailError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves settings for the trail associated with the current region for your account.</p>"]
                pub fn describe_trails(&self, input: &DescribeTrailsRequest)  -> Result<DescribeTrailsResponse, DescribeTrailsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.DescribeTrails");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTrailsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeTrailsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the settings for the event selectors that you configured for your trail. The information returned for your event selectors includes the following:</p> <ul> <li> <p>The S3 objects that you are logging for data events.</p> </li> <li> <p>If your event selector includes management events.</p> </li> <li> <p>If your event selector includes read-only events, write-only events, or all. </p> </li> </ul> <p>For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-event-selectors-for-a-trail.html\">Configuring Event Selectors for Trails</a> in the <i>AWS CloudTrail User Guide</i>.</p>"]
                pub fn get_event_selectors(&self, input: &GetEventSelectorsRequest)  -> Result<GetEventSelectorsResponse, GetEventSelectorsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetEventSelectors");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetEventSelectorsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetEventSelectorsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns a JSON-formatted list of information about the specified trail. Fields include information on delivery errors, Amazon SNS and Amazon S3 errors, and start and stop logging times for each trail. This operation returns trail status from a single region. To return trail status from all regions, you must call the operation on each region.</p>"]
                pub fn get_trail_status(&self, input: &GetTrailStatusRequest)  -> Result<GetTrailStatusResponse, GetTrailStatusError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.GetTrailStatus");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetTrailStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetTrailStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns all public keys whose private keys were used to sign the digest files within the specified time range. The public key is needed to validate digest files that were signed with its corresponding private key.</p> <note> <p>CloudTrail uses different private/public key pairs per region. Each digest file is signed with a private key unique to its region. Therefore, when you validate a digest file from a particular region, you must look in the same region for its corresponding public key.</p> </note>"]
                pub fn list_public_keys(&self, input: &ListPublicKeysRequest)  -> Result<ListPublicKeysResponse, ListPublicKeysError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.ListPublicKeys");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListPublicKeysResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListPublicKeysError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the tags for the trail in the current region.</p>"]
                pub fn list_tags(&self, input: &ListTagsRequest)  -> Result<ListTagsResponse, ListTagsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.ListTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Looks up API activity events captured by CloudTrail that create, update, or delete resources in your account. Events for a region can be looked up for the times in which you had CloudTrail turned on in that region during the last seven days. Lookup supports the following attributes:</p> <ul> <li> <p>Event ID</p> </li> <li> <p>Event name</p> </li> <li> <p>Resource name</p> </li> <li> <p>Resource type</p> </li> <li> <p>User name</p> </li> </ul> <p>All attributes are optional. The default number of results returned is 10, with a maximum of 50 possible. The response includes a token that you can use to get the next page of results.</p> <important> <p>The rate of lookup requests is limited to one per second per account. If this limit is exceeded, a throttling error occurs.</p> </important> <important> <p>Events that occurred during the selected time range will not be available for lookup if CloudTrail logging was not enabled when the events occurred.</p> </important>"]
                pub fn lookup_events(&self, input: &LookupEventsRequest)  -> Result<LookupEventsResponse, LookupEventsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.LookupEvents");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<LookupEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(LookupEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Configures an event selector for your trail. Use event selectors to specify the type of events that you want your trail to log. When an event occurs in your account, CloudTrail evaluates the event selectors in all trails. For each trail, if the event matches any event selector, the trail processes and logs the event. If the event doesn't match any event selector, the trail doesn't log the event. </p> <p>Example</p> <ol> <li> <p>You create an event selector for a trail and specify that you want write-only events.</p> </li> <li> <p>The EC2 <code>GetConsoleOutput</code> and <code>RunInstances</code> API operations occur in your account.</p> </li> <li> <p>CloudTrail evaluates whether the events match your event selectors.</p> </li> <li> <p>The <code>RunInstances</code> is a write-only event and it matches your event selector. The trail logs the event.</p> </li> <li> <p>The <code>GetConsoleOutput</code> is a read-only event but it doesn't match your event selector. The trail doesn't log the event. </p> </li> </ol> <p>The <code>PutEventSelectors</code> operation must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p> <p>You can configure up to five event selectors for each trail. For more information, see <a href=\"http://docs.aws.amazon.com/awscloudtrail/latest/userguide/create-event-selectors-for-a-trail.html\">Configuring Event Selectors for Trails</a> in the <i>AWS CloudTrail User Guide</i>.</p>"]
                pub fn put_event_selectors(&self, input: &PutEventSelectorsRequest)  -> Result<PutEventSelectorsResponse, PutEventSelectorsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.PutEventSelectors");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutEventSelectorsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutEventSelectorsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes the specified tags from a trail.</p>"]
                pub fn remove_tags(&self, input: &RemoveTagsRequest)  -> Result<RemoveTagsResponse, RemoveTagsError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.RemoveTags");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTagsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RemoveTagsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Starts the recording of AWS API calls and log file delivery for a trail. For a trail that is enabled in all regions, this operation must be called from the region in which the trail was created. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail that is enabled in all regions.</p>"]
                pub fn start_logging(&self, input: &StartLoggingRequest)  -> Result<StartLoggingResponse, StartLoggingError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.StartLogging");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartLoggingResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(StartLoggingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Suspends the recording of AWS API calls and log file delivery for the specified trail. Under most circumstances, there is no need to use this action. You can update a trail without stopping it first. This action is the only way to stop recording. For a trail enabled in all regions, this operation must be called from the region in which the trail was created, or an <code>InvalidHomeRegionException</code> will occur. This operation cannot be called on the shadow trails (replicated trails in other regions) of a trail enabled in all regions.</p>"]
                pub fn stop_logging(&self, input: &StopLoggingRequest)  -> Result<StopLoggingResponse, StopLoggingError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.StopLogging");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<StopLoggingResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(StopLoggingError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Updates the settings that specify delivery of log files. Changes to a trail do not require stopping the CloudTrail service. Use this action to designate an existing bucket for log delivery. If the existing bucket has previously been a target for CloudTrail log files, an IAM policy exists for the bucket. <code>UpdateTrail</code> must be called from the region in which the trail was created; otherwise, an <code>InvalidHomeRegionException</code> is thrown.</p>"]
                pub fn update_trail(&self, input: &UpdateTrailRequest)  -> Result<UpdateTrailResponse, UpdateTrailError> {
                    let mut request = SignedRequest::new("POST", "cloudtrail", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "com.amazonaws.cloudtrail.v20131101.CloudTrail_20131101.UpdateTrail");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateTrailResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateTrailError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}
