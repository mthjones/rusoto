#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
pub type Arn = String;
pub type Boolean = bool;
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    pub name: RuleName,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    pub name: RuleName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRuleResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the rule.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<RuleArn>,
    #[doc="<p>The description of the rule.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<RuleDescription>,
    #[doc="<p>The event pattern. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html\">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>"]
    #[serde(rename="EventPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_pattern: Option<EventPattern>,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<RuleName>,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<RoleArn>,
    #[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule_expression: Option<ScheduleExpression>,
    #[doc="<p>Specifies whether the rule is enabled or disabled.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<RuleState>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DisableRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    pub name: RuleName,
}

#[doc="<p>The custom parameters to be used when the target is an Amazon ECS cluster.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EcsParameters {
    #[doc="<p>The number of tasks to create based on the <code>TaskDefinition</code>. The default is one.</p>"]
    #[serde(rename="TaskCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task_count: Option<LimitMin1>,
    #[doc="<p>The ARN of the task definition to use if the event target is an Amazon ECS cluster. </p>"]
    #[serde(rename="TaskDefinitionArn")]
    pub task_definition_arn: Arn,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct EnableRuleRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    pub name: RuleName,
}

pub type ErrorCode = String;
pub type ErrorMessage = String;
pub type EventId = String;
pub type EventPattern = String;
pub type EventResource = String;
pub type EventResourceList = Vec<EventResource>;
pub type EventTime = f64;
#[doc="<p>Contains the parameters needed for you to provide custom input to a target based on one or more pieces of data extracted from the event.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct InputTransformer {
    #[doc="<p>Map of JSON paths to be extracted from the event. These are key-value pairs, where each value is a JSON path. You must use JSON dot notation, not bracket notation.</p>"]
    #[serde(rename="InputPathsMap")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_paths_map: Option<TransformerPaths>,
    #[doc="<p>Input template where you can use the values of the keys from <code>InputPathsMap</code> to customize the data sent to the target.</p>"]
    #[serde(rename="InputTemplate")]
    pub input_template: TransformerInput,
}

pub type InputTransformerPathKey = String;
pub type Integer = i64;
#[doc="<p>This object enables you to specify a JSON path to extract from the event and use as the partition key for the Amazon Kinesis stream, so that you can control the shard to which the event goes. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct KinesisParameters {
    #[doc="<p>The JSON path to be extracted from the event and used as the partition key. For more information, see <a href=\"http://docs.aws.amazon.com/streams/latest/dev/key-concepts.html#partition-key\">Amazon Kinesis Streams Key Concepts</a> in the <i>Amazon Kinesis Streams Developer Guide</i>.</p>"]
    #[serde(rename="PartitionKeyPath")]
    pub partition_key_path: TargetPartitionKeyPath,
}

pub type LimitMax100 = i64;
pub type LimitMin1 = i64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRuleNamesByTargetRequest {
    #[doc="<p>The maximum number of results to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<LimitMax100>,
    #[doc="<p>The token returned by a previous call to retrieve the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The Amazon Resource Name (ARN) of the target resource.</p>"]
    #[serde(rename="TargetArn")]
    pub target_arn: TargetArn,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRuleNamesByTargetResponse {
    #[doc="<p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The names of the rules that can invoke the given target.</p>"]
    #[serde(rename="RuleNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_names: Option<RuleNameList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRulesRequest {
    #[doc="<p>The maximum number of results to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<LimitMax100>,
    #[doc="<p>The prefix matching the rule name.</p>"]
    #[serde(rename="NamePrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name_prefix: Option<RuleName>,
    #[doc="<p>The token returned by a previous call to retrieve the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRulesResponse {
    #[doc="<p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The rules that match the specified criteria.</p>"]
    #[serde(rename="Rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules: Option<RuleResponseList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTargetsByRuleRequest {
    #[doc="<p>The maximum number of results to return.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<LimitMax100>,
    #[doc="<p>The token returned by a previous call to retrieve the next set of results.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Rule")]
    pub rule: RuleName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTargetsByRuleResponse {
    #[doc="<p>Indicates whether there are additional results to retrieve. If there are no more results, the value is null.</p>"]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<NextToken>,
    #[doc="<p>The targets assigned to the rule.</p>"]
    #[serde(rename="Targets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub targets: Option<TargetList>,
}

pub type NextToken = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutEventsRequest {
    #[doc="<p>The entry that defines an event in your system. You can specify several parameters for the entry such as the source and type of the event, resources associated with the event, and so on.</p>"]
    #[serde(rename="Entries")]
    pub entries: PutEventsRequestEntryList,
}

#[doc="<p>Represents an event to be submitted.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutEventsRequestEntry {
    #[doc="<p>In the JSON sense, an object containing fields, which may also contain nested subobjects. No constraints are imposed on its contents.</p>"]
    #[serde(rename="Detail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,
    #[doc="<p>Free-form string used to decide what fields to expect in the event detail.</p>"]
    #[serde(rename="DetailType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail_type: Option<String>,
    #[doc="<p>AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.</p>"]
    #[serde(rename="Resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<EventResourceList>,
    #[doc="<p>The source of the event.</p>"]
    #[serde(rename="Source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<String>,
    #[doc="<p>The timestamp of the event, per <a href=\"https://www.rfc-editor.org/rfc/rfc3339.txt\">RFC3339</a>. If no timestamp is provided, the timestamp of the <a>PutEvents</a> call is used.</p>"]
    #[serde(rename="Time")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub time: Option<EventTime>,
}

pub type PutEventsRequestEntryList = Vec<PutEventsRequestEntry>;
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutEventsResponse {
    #[doc="<p>The successfully and unsuccessfully ingested events results. If the ingestion was successful, the entry has the event ID in it. Otherwise, you can use the error code and error message to identify the problem with the entry.</p>"]
    #[serde(rename="Entries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entries: Option<PutEventsResultEntryList>,
    #[doc="<p>The number of failed entries.</p>"]
    #[serde(rename="FailedEntryCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_entry_count: Option<Integer>,
}

#[doc="<p>Represents an event that failed to be submitted.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutEventsResultEntry {
    #[doc="<p>The error code that indicates why the event submission failed.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<ErrorCode>,
    #[doc="<p>The error message that explains why the event submission failed.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<ErrorMessage>,
    #[doc="<p>The ID of the event.</p>"]
    #[serde(rename="EventId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_id: Option<EventId>,
}

pub type PutEventsResultEntryList = Vec<PutEventsResultEntry>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRuleRequest {
    #[doc="<p>A description of the rule.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<RuleDescription>,
    #[doc="<p>The event pattern. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html\">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>"]
    #[serde(rename="EventPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_pattern: Option<EventPattern>,
    #[doc="<p>The name of the rule that you are creating or updating.</p>"]
    #[serde(rename="Name")]
    pub name: RuleName,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role associated with the rule.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<RoleArn>,
    #[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule_expression: Option<ScheduleExpression>,
    #[doc="<p>Indicates whether the rule is enabled or disabled.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<RuleState>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRuleResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the rule.</p>"]
    #[serde(rename="RuleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule_arn: Option<RuleArn>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutTargetsRequest {
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Rule")]
    pub rule: RuleName,
    #[doc="<p>The targets to update or add to the rule.</p>"]
    #[serde(rename="Targets")]
    pub targets: TargetList,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutTargetsResponse {
    #[doc="<p>The failed target entries.</p>"]
    #[serde(rename="FailedEntries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_entries: Option<PutTargetsResultEntryList>,
    #[doc="<p>The number of failed entries.</p>"]
    #[serde(rename="FailedEntryCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_entry_count: Option<Integer>,
}

#[doc="<p>Represents a target that failed to be added to a rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutTargetsResultEntry {
    #[doc="<p>The error code that indicates why the target addition failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<ErrorCode>,
    #[doc="<p>The error message that explains why the target addition failed.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<ErrorMessage>,
    #[doc="<p>The ID of the target.</p>"]
    #[serde(rename="TargetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<TargetId>,
}

pub type PutTargetsResultEntryList = Vec<PutTargetsResultEntry>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveTargetsRequest {
    #[doc="<p>The IDs of the targets to remove from the rule.</p>"]
    #[serde(rename="Ids")]
    pub ids: TargetIdList,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Rule")]
    pub rule: RuleName,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveTargetsResponse {
    #[doc="<p>The failed target entries.</p>"]
    #[serde(rename="FailedEntries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_entries: Option<RemoveTargetsResultEntryList>,
    #[doc="<p>The number of failed entries.</p>"]
    #[serde(rename="FailedEntryCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_entry_count: Option<Integer>,
}

#[doc="<p>Represents a target that failed to be removed from a rule.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveTargetsResultEntry {
    #[doc="<p>The error code that indicates why the target removal failed. If the value is <code>ConcurrentModificationException</code>, too many requests were made at the same time.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<ErrorCode>,
    #[doc="<p>The error message that explains why the target removal failed.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<ErrorMessage>,
    #[doc="<p>The ID of the target.</p>"]
    #[serde(rename="TargetId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<TargetId>,
}

pub type RemoveTargetsResultEntryList = Vec<RemoveTargetsResultEntry>;
pub type RoleArn = String;
#[doc="<p>Contains information about a rule in Amazon CloudWatch Events.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Rule {
    #[doc="<p>The Amazon Resource Name (ARN) of the rule.</p>"]
    #[serde(rename="Arn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub arn: Option<RuleArn>,
    #[doc="<p>The description of the rule.</p>"]
    #[serde(rename="Description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<RuleDescription>,
    #[doc="<p>The event pattern of the rule. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html\">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>"]
    #[serde(rename="EventPattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_pattern: Option<EventPattern>,
    #[doc="<p>The name of the rule.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<RuleName>,
    #[doc="<p>The Amazon Resource Name (ARN) of the role that is used for target invocation.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<RoleArn>,
    #[doc="<p>The scheduling expression. For example, \"cron(0 20 * * ? *)\", \"rate(5 minutes)\".</p>"]
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schedule_expression: Option<ScheduleExpression>,
    #[doc="<p>The state of the rule.</p>"]
    #[serde(rename="State")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<RuleState>,
}

pub type RuleArn = String;
pub type RuleDescription = String;
pub type RuleName = String;
pub type RuleNameList = Vec<RuleName>;
pub type RuleResponseList = Vec<Rule>;
pub type RuleState = String;
#[doc="<p>This parameter contains the criteria (either InstanceIds or a tag) used to specify which EC2 instances are to be sent the command. </p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RunCommandParameters {
    #[doc="<p>Currently, we support including only one RunCommandTarget block, which specifies either an array of InstanceIds or a tag.</p>"]
    #[serde(rename="RunCommandTargets")]
    pub run_command_targets: RunCommandTargets,
}

#[doc="<p>Information about the EC2 instances that are to be sent the command, specified as key-value pairs. Each <code>RunCommandTarget</code> block can include only one key, but this key may specify multiple values.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RunCommandTarget {
    #[doc="<p>Can be either <code>tag:</code> <i>tag-key</i> or <code>InstanceIds</code>.</p>"]
    #[serde(rename="Key")]
    pub key: RunCommandTargetKey,
    #[doc="<p>If <code>Key</code> is <code>tag:</code> <i>tag-key</i>, <code>Values</code> is a list of tag values. If <code>Key</code> is <code>InstanceIds</code>, <code>Values</code> is a list of Amazon EC2 instance IDs.</p>"]
    #[serde(rename="Values")]
    pub values: RunCommandTargetValues,
}

pub type RunCommandTargetKey = String;
pub type RunCommandTargetValue = String;
pub type RunCommandTargetValues = Vec<RunCommandTargetValue>;
pub type RunCommandTargets = Vec<RunCommandTarget>;
pub type ScheduleExpression = String;
#[doc="<p>Targets are the resources to be invoked when a rule is triggered. Target types include EC2 instances, AWS Lambda functions, Amazon Kinesis streams, Amazon ECS tasks, AWS Step Functions state machines, Run Command, and built-in targets.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Target {
    #[doc="<p>The Amazon Resource Name (ARN) of the target.</p>"]
    #[serde(rename="Arn")]
    pub arn: TargetArn,
    #[doc="<p>Contains the Amazon ECS task definition and task count to be used, if the event target is an Amazon ECS task. For more information about Amazon ECS tasks, see <a href=\"http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html\">Task Definitions </a> in the <i>Amazon EC2 Container Service Developer Guide</i>.</p>"]
    #[serde(rename="EcsParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    #[doc="<p>The ID of the target.</p>"]
    #[serde(rename="Id")]
    pub id: TargetId,
    #[doc="<p>Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. You must use JSON dot notation, not bracket notation. For more information, see <a href=\"http://www.rfc-editor.org/rfc/rfc7159.txt\">The JavaScript Object Notation (JSON) Data Interchange Format</a>.</p>"]
    #[serde(rename="Input")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input: Option<TargetInput>,
    #[doc="<p>The value of the JSONPath that is used for extracting part of the matched event when passing it to the target. You must use JSON dot notation, not bracket notation. For more information about JSON paths, see <a href=\"http://goessner.net/articles/JsonPath/\">JSONPath</a>.</p>"]
    #[serde(rename="InputPath")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_path: Option<TargetInputPath>,
    #[doc="<p>Settings to enable you to provide custom input to a target based on certain event data. You can extract one or more key-value pairs from the event and then use that data to send customized input to the target.</p>"]
    #[serde(rename="InputTransformer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub input_transformer: Option<InputTransformer>,
    #[doc="<p>The custom parameter you can use to control shard assignment, when the target is an Amazon Kinesis stream. If you do not include this parameter, the default is to use the <code>eventId</code> as the partition key.</p>"]
    #[serde(rename="KinesisParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,
    #[doc="<p>The Amazon Resource Name (ARN) of the IAM role to be used for this target when the rule is triggered. If one rule triggers multiple targets, you can use a different IAM role for each target.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<RoleArn>,
    #[doc="<p>Parameters used when you are using the rule to invoke Amazon EC2 Run Command.</p>"]
    #[serde(rename="RunCommandParameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_command_parameters: Option<RunCommandParameters>,
}

pub type TargetArn = String;
pub type TargetId = String;
pub type TargetIdList = Vec<TargetId>;
pub type TargetInput = String;
pub type TargetInputPath = String;
pub type TargetList = Vec<Target>;
pub type TargetPartitionKeyPath = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct TestEventPatternRequest {
    #[doc="<p>The event, in JSON format, to test against the event pattern.</p>"]
    #[serde(rename="Event")]
    pub event: String,
    #[doc="<p>The event pattern. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/CloudWatchEventsandEventPatterns.html\">Events and Event Patterns</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p>"]
    #[serde(rename="EventPattern")]
    pub event_pattern: EventPattern,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct TestEventPatternResponse {
    #[doc="<p>Indicates whether the event matches the event pattern.</p>"]
    #[serde(rename="Result")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub result: Option<Boolean>,
}

pub type TransformerInput = String;
pub type TransformerPaths = ::std::collections::HashMap<InputTransformerPathKey, TargetInputPath>;
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteRuleError {
    pub fn from_body(body: &str) -> DeleteRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        DeleteRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => DeleteRuleError::Internal(String::from(error_message)),
                    "ValidationException" => DeleteRuleError::Validation(error_message.to_string()),
                    _ => DeleteRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRuleError {
    fn from(err: serde_json::error::Error) -> DeleteRuleError {
        DeleteRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRuleError {
    fn from(err: CredentialsError) -> DeleteRuleError {
        DeleteRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRuleError {
    fn from(err: HttpDispatchError) -> DeleteRuleError {
        DeleteRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteRuleError::ConcurrentModification(ref cause) => cause,
            DeleteRuleError::Internal(ref cause) => cause,
            DeleteRuleError::Validation(ref cause) => cause,
            DeleteRuleError::Credentials(ref err) => err.description(),
            DeleteRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRule
#[derive(Debug, PartialEq)]
pub enum DescribeRuleError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeRuleError {
    pub fn from_body(body: &str) -> DescribeRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => DescribeRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DescribeRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRuleError::Validation(error_message.to_string())
                    }
                    _ => DescribeRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRuleError {
    fn from(err: serde_json::error::Error) -> DescribeRuleError {
        DescribeRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRuleError {
    fn from(err: CredentialsError) -> DescribeRuleError {
        DescribeRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRuleError {
    fn from(err: HttpDispatchError) -> DescribeRuleError {
        DescribeRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRuleError {
    fn description(&self) -> &str {
        match *self {
            DescribeRuleError::Internal(ref cause) => cause,
            DescribeRuleError::ResourceNotFound(ref cause) => cause,
            DescribeRuleError::Validation(ref cause) => cause,
            DescribeRuleError::Credentials(ref err) => err.description(),
            DescribeRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableRule
#[derive(Debug, PartialEq)]
pub enum DisableRuleError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DisableRuleError {
    pub fn from_body(body: &str) -> DisableRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        DisableRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => DisableRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        DisableRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableRuleError::Validation(error_message.to_string())
                    }
                    _ => DisableRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableRuleError {
    fn from(err: serde_json::error::Error) -> DisableRuleError {
        DisableRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableRuleError {
    fn from(err: CredentialsError) -> DisableRuleError {
        DisableRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableRuleError {
    fn from(err: HttpDispatchError) -> DisableRuleError {
        DisableRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for DisableRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableRuleError {
    fn description(&self) -> &str {
        match *self {
            DisableRuleError::ConcurrentModification(ref cause) => cause,
            DisableRuleError::Internal(ref cause) => cause,
            DisableRuleError::ResourceNotFound(ref cause) => cause,
            DisableRuleError::Validation(ref cause) => cause,
            DisableRuleError::Credentials(ref err) => err.description(),
            DisableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableRule
#[derive(Debug, PartialEq)]
pub enum EnableRuleError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl EnableRuleError {
    pub fn from_body(body: &str) -> EnableRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        EnableRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => EnableRuleError::Internal(String::from(error_message)),
                    "ResourceNotFoundException" => {
                        EnableRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => EnableRuleError::Validation(error_message.to_string()),
                    _ => EnableRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableRuleError {
    fn from(err: serde_json::error::Error) -> EnableRuleError {
        EnableRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableRuleError {
    fn from(err: CredentialsError) -> EnableRuleError {
        EnableRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableRuleError {
    fn from(err: HttpDispatchError) -> EnableRuleError {
        EnableRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for EnableRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableRuleError {
    fn description(&self) -> &str {
        match *self {
            EnableRuleError::ConcurrentModification(ref cause) => cause,
            EnableRuleError::Internal(ref cause) => cause,
            EnableRuleError::ResourceNotFound(ref cause) => cause,
            EnableRuleError::Validation(ref cause) => cause,
            EnableRuleError::Credentials(ref err) => err.description(),
            EnableRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRuleNamesByTarget
#[derive(Debug, PartialEq)]
pub enum ListRuleNamesByTargetError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRuleNamesByTargetError {
    pub fn from_body(body: &str) -> ListRuleNamesByTargetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ListRuleNamesByTargetError::Internal(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRuleNamesByTargetError::Validation(error_message.to_string())
                    }
                    _ => ListRuleNamesByTargetError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRuleNamesByTargetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRuleNamesByTargetError {
    fn from(err: serde_json::error::Error) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRuleNamesByTargetError {
    fn from(err: CredentialsError) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRuleNamesByTargetError {
    fn from(err: HttpDispatchError) -> ListRuleNamesByTargetError {
        ListRuleNamesByTargetError::HttpDispatch(err)
    }
}
impl fmt::Display for ListRuleNamesByTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRuleNamesByTargetError {
    fn description(&self) -> &str {
        match *self {
            ListRuleNamesByTargetError::Internal(ref cause) => cause,
            ListRuleNamesByTargetError::Validation(ref cause) => cause,
            ListRuleNamesByTargetError::Credentials(ref err) => err.description(),
            ListRuleNamesByTargetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListRuleNamesByTargetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRulesError {
    pub fn from_body(body: &str) -> ListRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => ListRulesError::Internal(String::from(error_message)),
                    "ValidationException" => ListRulesError::Validation(error_message.to_string()),
                    _ => ListRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRulesError {
    fn from(err: serde_json::error::Error) -> ListRulesError {
        ListRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRulesError {
    fn from(err: CredentialsError) -> ListRulesError {
        ListRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRulesError {
    fn from(err: HttpDispatchError) -> ListRulesError {
        ListRulesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRulesError {
    fn description(&self) -> &str {
        match *self {
            ListRulesError::Internal(ref cause) => cause,
            ListRulesError::Validation(ref cause) => cause,
            ListRulesError::Credentials(ref err) => err.description(),
            ListRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTargetsByRule
#[derive(Debug, PartialEq)]
pub enum ListTargetsByRuleError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTargetsByRuleError {
    pub fn from_body(body: &str) -> ListTargetsByRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ListTargetsByRuleError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTargetsByRuleError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTargetsByRuleError::Validation(error_message.to_string())
                    }
                    _ => ListTargetsByRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTargetsByRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTargetsByRuleError {
    fn from(err: serde_json::error::Error) -> ListTargetsByRuleError {
        ListTargetsByRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTargetsByRuleError {
    fn from(err: CredentialsError) -> ListTargetsByRuleError {
        ListTargetsByRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTargetsByRuleError {
    fn from(err: HttpDispatchError) -> ListTargetsByRuleError {
        ListTargetsByRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for ListTargetsByRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTargetsByRuleError {
    fn description(&self) -> &str {
        match *self {
            ListTargetsByRuleError::Internal(ref cause) => cause,
            ListTargetsByRuleError::ResourceNotFound(ref cause) => cause,
            ListTargetsByRuleError::Validation(ref cause) => cause,
            ListTargetsByRuleError::Credentials(ref err) => err.description(),
            ListTargetsByRuleError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTargetsByRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEvents
#[derive(Debug, PartialEq)]
pub enum PutEventsError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutEventsError {
    pub fn from_body(body: &str) -> PutEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => PutEventsError::Internal(String::from(error_message)),
                    "ValidationException" => PutEventsError::Validation(error_message.to_string()),
                    _ => PutEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutEventsError {
    fn from(err: serde_json::error::Error) -> PutEventsError {
        PutEventsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutEventsError {
    fn from(err: CredentialsError) -> PutEventsError {
        PutEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutEventsError {
    fn from(err: HttpDispatchError) -> PutEventsError {
        PutEventsError::HttpDispatch(err)
    }
}
impl fmt::Display for PutEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEventsError {
    fn description(&self) -> &str {
        match *self {
            PutEventsError::Internal(ref cause) => cause,
            PutEventsError::Validation(ref cause) => cause,
            PutEventsError::Credentials(ref err) => err.description(),
            PutEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRule
#[derive(Debug, PartialEq)]
pub enum PutRuleError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    ///<p>You tried to create more rules or add more targets to a rule than is allowed.</p>
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutRuleError {
    pub fn from_body(body: &str) -> PutRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        PutRuleError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => PutRuleError::Internal(String::from(error_message)),
                    "InvalidEventPatternException" => {
                        PutRuleError::InvalidEventPattern(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        PutRuleError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => PutRuleError::Validation(error_message.to_string()),
                    _ => PutRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRuleError {
    fn from(err: serde_json::error::Error) -> PutRuleError {
        PutRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRuleError {
    fn from(err: CredentialsError) -> PutRuleError {
        PutRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRuleError {
    fn from(err: HttpDispatchError) -> PutRuleError {
        PutRuleError::HttpDispatch(err)
    }
}
impl fmt::Display for PutRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRuleError {
    fn description(&self) -> &str {
        match *self {
            PutRuleError::ConcurrentModification(ref cause) => cause,
            PutRuleError::Internal(ref cause) => cause,
            PutRuleError::InvalidEventPattern(ref cause) => cause,
            PutRuleError::LimitExceeded(ref cause) => cause,
            PutRuleError::Validation(ref cause) => cause,
            PutRuleError::Credentials(ref err) => err.description(),
            PutRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutTargets
#[derive(Debug, PartialEq)]
pub enum PutTargetsError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>You tried to create more rules or add more targets to a rule than is allowed.</p>
    LimitExceeded(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutTargetsError {
    pub fn from_body(body: &str) -> PutTargetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        PutTargetsError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => PutTargetsError::Internal(String::from(error_message)),
                    "LimitExceededException" => {
                        PutTargetsError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutTargetsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => PutTargetsError::Validation(error_message.to_string()),
                    _ => PutTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutTargetsError {
    fn from(err: serde_json::error::Error) -> PutTargetsError {
        PutTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutTargetsError {
    fn from(err: CredentialsError) -> PutTargetsError {
        PutTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutTargetsError {
    fn from(err: HttpDispatchError) -> PutTargetsError {
        PutTargetsError::HttpDispatch(err)
    }
}
impl fmt::Display for PutTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutTargetsError {
    fn description(&self) -> &str {
        match *self {
            PutTargetsError::ConcurrentModification(ref cause) => cause,
            PutTargetsError::Internal(ref cause) => cause,
            PutTargetsError::LimitExceeded(ref cause) => cause,
            PutTargetsError::ResourceNotFound(ref cause) => cause,
            PutTargetsError::Validation(ref cause) => cause,
            PutTargetsError::Credentials(ref err) => err.description(),
            PutTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTargets
#[derive(Debug, PartialEq)]
pub enum RemoveTargetsError {
    ///<p>There is concurrent modification on a rule or target.</p>
    ConcurrentModification(String),
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The rule does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveTargetsError {
    pub fn from_body(body: &str) -> RemoveTargetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        RemoveTargetsError::ConcurrentModification(String::from(error_message))
                    }
                    "InternalException" => {
                        RemoveTargetsError::Internal(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveTargetsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveTargetsError::Validation(error_message.to_string())
                    }
                    _ => RemoveTargetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTargetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTargetsError {
    fn from(err: serde_json::error::Error) -> RemoveTargetsError {
        RemoveTargetsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveTargetsError {
    fn from(err: CredentialsError) -> RemoveTargetsError {
        RemoveTargetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTargetsError {
    fn from(err: HttpDispatchError) -> RemoveTargetsError {
        RemoveTargetsError::HttpDispatch(err)
    }
}
impl fmt::Display for RemoveTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTargetsError {
    fn description(&self) -> &str {
        match *self {
            RemoveTargetsError::ConcurrentModification(ref cause) => cause,
            RemoveTargetsError::Internal(ref cause) => cause,
            RemoveTargetsError::ResourceNotFound(ref cause) => cause,
            RemoveTargetsError::Validation(ref cause) => cause,
            RemoveTargetsError::Credentials(ref err) => err.description(),
            RemoveTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemoveTargetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestEventPattern
#[derive(Debug, PartialEq)]
pub enum TestEventPatternError {
    ///<p>This exception occurs due to unexpected causes.</p>
    Internal(String),
    ///<p>The event pattern is not valid.</p>
    InvalidEventPattern(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TestEventPatternError {
    pub fn from_body(body: &str) -> TestEventPatternError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        TestEventPatternError::Internal(String::from(error_message))
                    }
                    "InvalidEventPatternException" => {
                        TestEventPatternError::InvalidEventPattern(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestEventPatternError::Validation(error_message.to_string())
                    }
                    _ => TestEventPatternError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestEventPatternError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestEventPatternError {
    fn from(err: serde_json::error::Error) -> TestEventPatternError {
        TestEventPatternError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestEventPatternError {
    fn from(err: CredentialsError) -> TestEventPatternError {
        TestEventPatternError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestEventPatternError {
    fn from(err: HttpDispatchError) -> TestEventPatternError {
        TestEventPatternError::HttpDispatch(err)
    }
}
impl fmt::Display for TestEventPatternError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestEventPatternError {
    fn description(&self) -> &str {
        match *self {
            TestEventPatternError::Internal(ref cause) => cause,
            TestEventPatternError::InvalidEventPattern(ref cause) => cause,
            TestEventPatternError::Validation(ref cause) => cause,
            TestEventPatternError::Credentials(ref err) => err.description(),
            TestEventPatternError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestEventPatternError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon CloudWatch Events API. Amazon CloudWatch Events clients implement this trait.
pub trait CloudWatchEvents {
    #[doc="<p>Deletes the specified rule.</p> <p>You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Please allow a short period of time for changes to take effect.</p>"]
    fn delete_rule(&self, input: &DeleteRuleRequest) -> Result<(), DeleteRuleError>;


    #[doc="<p>Describes the specified rule.</p>"]
    fn describe_rule(&self,
                     input: &DescribeRuleRequest)
                     -> Result<DescribeRuleResponse, DescribeRuleError>;


    #[doc="<p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Please allow a short period of time for changes to take effect.</p>"]
    fn disable_rule(&self, input: &DisableRuleRequest) -> Result<(), DisableRuleError>;


    #[doc="<p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect.</p>"]
    fn enable_rule(&self, input: &EnableRuleRequest) -> Result<(), EnableRuleError>;


    #[doc="<p>Lists the rules for the specified target. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account.</p>"]
    fn list_rule_names_by_target
        (&self,
         input: &ListRuleNamesByTargetRequest)
         -> Result<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError>;


    #[doc="<p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p>"]
    fn list_rules(&self, input: &ListRulesRequest) -> Result<ListRulesResponse, ListRulesError>;


    #[doc="<p>Lists the targets assigned to the specified rule.</p>"]
    fn list_targets_by_rule(&self,
                            input: &ListTargetsByRuleRequest)
                            -> Result<ListTargetsByRuleResponse, ListTargetsByRuleError>;


    #[doc="<p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>"]
    fn put_events(&self, input: &PutEventsRequest) -> Result<PutEventsResponse, PutEventsError>;


    #[doc="<p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>"]
    fn put_rule(&self, input: &PutRuleRequest) -> Result<PutRuleResponse, PutRuleError>;


    #[doc="<p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered. Example targets include EC2 instances, AWS Lambda functions, Amazon Kinesis streams, Amazon ECS tasks, AWS Step Functions state machines, and built-in targets. Note that creating rules with built-in targets is supported only in the AWS Management Console.</p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is an Amazon Kinesis stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Amazon Kinesis streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTarget</code>. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html\">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p> <b>Input</b>, <b>InputPath</b> and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON form (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>Input</code>, <code>InputPath</code>, or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>"]
    fn put_targets(&self,
                   input: &PutTargetsRequest)
                   -> Result<PutTargetsResponse, PutTargetsError>;


    #[doc="<p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>"]
    fn remove_targets(&self,
                      input: &RemoveTargetsRequest)
                      -> Result<RemoveTargetsResponse, RemoveTargetsError>;


    #[doc="<p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>"]
    fn test_event_pattern(&self,
                          input: &TestEventPatternRequest)
                          -> Result<TestEventPatternResponse, TestEventPatternError>;
}
/// A client for the Amazon CloudWatch Events API.
pub struct CloudWatchEventsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CloudWatchEventsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CloudWatchEventsClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CloudWatchEvents for CloudWatchEventsClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Deletes the specified rule.</p> <p>You must remove all targets from a rule using <a>RemoveTargets</a> before you can delete the rule.</p> <p>When you delete a rule, incoming events might continue to match to the deleted rule. Please allow a short period of time for changes to take effect.</p>"]
    fn delete_rule(&self, input: &DeleteRuleRequest) -> Result<(), DeleteRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DeleteRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DeleteRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Describes the specified rule.</p>"]
    fn describe_rule(&self,
                     input: &DescribeRuleRequest)
                     -> Result<DescribeRuleResponse, DescribeRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DescribeRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Disables the specified rule. A disabled rule won't match any events, and won't self-trigger if it has a schedule expression.</p> <p>When you disable a rule, incoming events might continue to match to the disabled rule. Please allow a short period of time for changes to take effect.</p>"]
    fn disable_rule(&self, input: &DisableRuleRequest) -> Result<(), DisableRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.DisableRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(DisableRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Enables the specified rule. If the rule does not exist, the operation fails.</p> <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Please allow a short period of time for changes to take effect.</p>"]
    fn enable_rule(&self, input: &EnableRuleRequest) -> Result<(), EnableRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.EnableRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => Ok(()),
            _ => Err(EnableRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the rules for the specified target. You can see which of the rules in Amazon CloudWatch Events can invoke a specific target in your account.</p>"]
    fn list_rule_names_by_target
        (&self,
         input: &ListRuleNamesByTargetRequest)
         -> Result<ListRuleNamesByTargetResponse, ListRuleNamesByTargetError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRuleNamesByTarget");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRuleNamesByTargetResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListRuleNamesByTargetError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Lists your Amazon CloudWatch Events rules. You can either list all the rules or you can provide a prefix to match to the rule names.</p>"]
    fn list_rules(&self, input: &ListRulesRequest) -> Result<ListRulesResponse, ListRulesError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListRules");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListRulesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListRulesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists the targets assigned to the specified rule.</p>"]
    fn list_targets_by_rule(&self,
                            input: &ListTargetsByRuleRequest)
                            -> Result<ListTargetsByRuleResponse, ListTargetsByRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.ListTargetsByRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTargetsByRuleResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListTargetsByRuleError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Sends custom events to Amazon CloudWatch Events so that they can be matched to rules.</p>"]
    fn put_events(&self, input: &PutEventsRequest) -> Result<PutEventsResponse, PutEventsError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(PutEventsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates or updates the specified rule. Rules are enabled by default, or based on value of the state. You can disable a rule using <a>DisableRule</a>.</p> <p>When you create or update a rule, incoming events might not immediately start matching to new or updated rules. Please allow a short period of time for changes to take effect.</p> <p>A rule must contain at least an EventPattern or ScheduleExpression. Rules with EventPatterns are triggered when a matching event is observed. Rules with ScheduleExpressions self-trigger based on the given schedule. A rule can have both an EventPattern and a ScheduleExpression, in which case the rule triggers on matching events as well as on a schedule.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>"]
    fn put_rule(&self, input: &PutRuleRequest) -> Result<PutRuleResponse, PutRuleError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutRule");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<PutRuleResponse>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(PutRuleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Adds the specified targets to the specified rule, or updates the targets if they are already associated with the rule.</p> <p>Targets are the resources that are invoked when a rule is triggered. Example targets include EC2 instances, AWS Lambda functions, Amazon Kinesis streams, Amazon ECS tasks, AWS Step Functions state machines, and built-in targets. Note that creating rules with built-in targets is supported only in the AWS Management Console.</p> <p>For some target types, <code>PutTargets</code> provides target-specific parameters. If the target is an Amazon Kinesis stream, you can optionally specify which shard the event goes to by using the <code>KinesisParameters</code> argument. To invoke a command on multiple EC2 instances with one rule, you can use the <code>RunCommandParameters</code> field.</p> <p>To be able to make API calls against the resources that you own, Amazon CloudWatch Events needs the appropriate permissions. For AWS Lambda and Amazon SNS resources, CloudWatch Events relies on resource-based policies. For EC2 instances, Amazon Kinesis streams, and AWS Step Functions state machines, CloudWatch Events relies on IAM roles that you specify in the <code>RoleARN</code> argument in <code>PutTarget</code>. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/events/auth-and-access-control-cwe.html\">Authentication and Access Control</a> in the <i>Amazon CloudWatch Events User Guide</i>.</p> <p> <b>Input</b>, <b>InputPath</b> and <b>InputTransformer</b> are mutually exclusive and optional parameters of a target. When a rule is triggered due to a matched event:</p> <ul> <li> <p>If none of the following arguments are specified for a target, then the entire event is passed to the target in JSON form (unless the target is Amazon EC2 Run Command or Amazon ECS task, in which case nothing from the event is passed to the target).</p> </li> <li> <p>If <b>Input</b> is specified in the form of valid JSON, then the matched event is overridden with this constant.</p> </li> <li> <p>If <b>InputPath</b> is specified in the form of JSONPath (for example, <code>$.detail</code>), then only the part of the event specified in the path is passed to the target (for example, only the detail part of the event is passed).</p> </li> <li> <p>If <b>InputTransformer</b> is specified, then one or more specified JSONPaths are extracted from the event and used as values in a template that you specify as the input to the target.</p> </li> </ul> <p>When you specify <code>Input</code>, <code>InputPath</code>, or <code>InputTransformer</code>, you must use JSON dot notation, not bracket notation.</p> <p>When you add targets to a rule and the associated rule triggers soon after, new or updated targets might not be immediately invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>"]
    fn put_targets(&self,
                   input: &PutTargetsRequest)
                   -> Result<PutTargetsResponse, PutTargetsError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.PutTargets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutTargetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(PutTargetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Removes the specified targets from the specified rule. When the rule is triggered, those targets are no longer be invoked.</p> <p>When you remove a target, when the associated rule triggers, removed targets might continue to be invoked. Please allow a short period of time for changes to take effect.</p> <p>This action can partially fail if too many requests are made at the same time. If that happens, <code>FailedEntryCount</code> is non-zero in the response and each entry in <code>FailedEntries</code> provides the ID of the failed target and the error code.</p>"]
    fn remove_targets(&self,
                      input: &RemoveTargetsRequest)
                      -> Result<RemoveTargetsResponse, RemoveTargetsError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.RemoveTargets");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTargetsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RemoveTargetsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Tests whether the specified event pattern matches the provided event.</p> <p>Most services in AWS treat : or / as the same character in Amazon Resource Names (ARNs). However, CloudWatch Events uses an exact match in event patterns and rules. Be sure to use the correct ARN characters when creating event patterns so that they match the ARN syntax in the event you want to match.</p>"]
    fn test_event_pattern(&self,
                          input: &TestEventPatternRequest)
                          -> Result<TestEventPatternResponse, TestEventPatternError> {
        let mut request = SignedRequest::new("POST", "events", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSEvents.TestEventPattern");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<TestEventPatternResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(TestEventPatternError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
