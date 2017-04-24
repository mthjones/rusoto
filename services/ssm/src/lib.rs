extern crate hyper;
extern crate rusoto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto::request::DispatchSignedRequest;
        use rusoto::region;

        use std::fmt;
        use std::error::Error;
        use rusoto::request::HttpDispatchError;
        use rusoto::{CredentialsError, ProvideAwsCredentials};
    
use rusoto::signature::SignedRequest;
        use serde_json::Value as SerdeJsonValue;
        use serde_json::from_str;
pub type AccountId = String;
pub type AccountIdList = Vec<AccountId>;
#[doc="<p>An activation registers one or more on-premises servers or virtual machines (VMs) with AWS so that you can configure those servers or VMs using Run Command. A server or VM that has been registered with AWS is called a managed instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Activation {
                #[doc="<p>The ID created by Systems Manager when you submitted the activation.</p>"]
#[serde(rename="ActivationId")]
pub activation_id: Option<ActivationId>,
#[doc="<p>The date the activation was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<CreatedDate>,
#[doc="<p>A name for the managed instance when it is created.</p>"]
#[serde(rename="DefaultInstanceName")]
pub default_instance_name: Option<DefaultInstanceName>,
#[doc="<p>A user defined description of the activation.</p>"]
#[serde(rename="Description")]
pub description: Option<ActivationDescription>,
#[doc="<p>The date when this activation can no longer be used to register managed instances.</p>"]
#[serde(rename="ExpirationDate")]
pub expiration_date: Option<ExpirationDate>,
#[doc="<p>Whether or not the activation is expired.</p>"]
#[serde(rename="Expired")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub expired: Option<Boolean>,
#[doc="<p>The Amazon Identity and Access Management (IAM) role to assign to the managed instance.</p>"]
#[serde(rename="IamRole")]
pub iam_role: Option<IamRole>,
#[doc="<p>The maximum number of managed instances that can be registered using this activation.</p>"]
#[serde(rename="RegistrationLimit")]
pub registration_limit: Option<RegistrationLimit>,
#[doc="<p>The number of managed instances already registered with this activation.</p>"]
#[serde(rename="RegistrationsCount")]
pub registrations_count: Option<RegistrationsCount>,
            }
            
pub type ActivationCode = String;
pub type ActivationDescription = String;
pub type ActivationId = String;
pub type ActivationList = Vec<Activation>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AddTagsToResourceRequest {
                #[doc="<p>The resource ID you want to tag.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: ResourceId,
#[doc="<p>Specifies the type of resource you are tagging.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: ResourceTypeForTagging,
#[doc="<p> One or more tags. The value parameter is required, but if you don't want the tag to have a value, specify the parameter with no value, and we set the value to an empty string. </p>"]
#[serde(rename="Tags")]
pub tags: TagList,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AddTagsToResourceResult;
            
pub type AgentErrorCode = String;
pub type ApproveAfterDays = i64;
#[doc="<p>Describes an association of a Systems Manager document and an instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Association {
                #[doc="<p>The ID created by the system when you create an association. An association is a binding between a document and a set of targets with a schedule.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>The version of the document used in the association.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The ID of the instance.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The date on which the association was last run.</p>"]
#[serde(rename="LastExecutionDate")]
pub last_execution_date: Option<DateTime>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
#[doc="<p>Information about the association.</p>"]
#[serde(rename="Overview")]
pub overview: Option<AssociationOverview>,
#[doc="<p>A cron expression that specifies a schedule when the association runs.</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>The instances targeted by the request to create an association. </p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
            }
            
#[doc="<p>Describes the parameters for a document.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AssociationDescription {
                #[doc="<p>The association ID.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>The date when the association was made.</p>"]
#[serde(rename="Date")]
pub date: Option<DateTime>,
#[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The ID of the instance.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The date on which the association was last run.</p>"]
#[serde(rename="LastExecutionDate")]
pub last_execution_date: Option<DateTime>,
#[doc="<p>The last date on which the association was successfully run.</p>"]
#[serde(rename="LastSuccessfulExecutionDate")]
pub last_successful_execution_date: Option<DateTime>,
#[doc="<p>The date when the association was last updated.</p>"]
#[serde(rename="LastUpdateAssociationDate")]
pub last_update_association_date: Option<DateTime>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
#[doc="<p>An Amazon S3 bucket where you want to store the output details of the request.</p>"]
#[serde(rename="OutputLocation")]
pub output_location: Option<InstanceAssociationOutputLocation>,
#[doc="<p>Information about the association.</p>"]
#[serde(rename="Overview")]
pub overview: Option<AssociationOverview>,
#[doc="<p>A description of the parameters for a document. </p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>A cron expression that specifies a schedule when the association runs.</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>The association status.</p>"]
#[serde(rename="Status")]
pub status: Option<AssociationStatus>,
#[doc="<p>The instances targeted by the request. </p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
            }
            
pub type AssociationDescriptionList = Vec<AssociationDescription>;
#[doc="<p>Describes a filter.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AssociationFilter {
                #[doc="<p>The name of the filter.</p>"]
#[serde(rename="key")]
pub key: AssociationFilterKey,
#[doc="<p>The filter value.</p>"]
#[serde(rename="value")]
pub value: AssociationFilterValue,
            }
            
pub type AssociationFilterKey = String;
pub type AssociationFilterList = Vec<AssociationFilter>;
pub type AssociationFilterValue = String;
pub type AssociationId = String;
pub type AssociationList = Vec<Association>;
#[doc="<p>Information about the association.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AssociationOverview {
                #[doc="<p>Returns the number of targets for the association status. For example, if you created an association with two instances, and one of them was successful, this would return the count of instances by status.</p>"]
#[serde(rename="AssociationStatusAggregatedCount")]
pub association_status_aggregated_count: Option<AssociationStatusAggregatedCount>,
#[doc="<p>A detailed status of the association.</p>"]
#[serde(rename="DetailedStatus")]
pub detailed_status: Option<StatusName>,
#[doc="<p>The status of the association. Status can be: <code>Pending</code>, <code>Success</code>, or <code>Failed</code>.</p>"]
#[serde(rename="Status")]
pub status: Option<StatusName>,
            }
            
#[doc="<p>Describes an association status.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct AssociationStatus {
                #[doc="<p>A user-defined string.</p>"]
#[serde(rename="AdditionalInfo")]
pub additional_info: Option<StatusAdditionalInfo>,
#[doc="<p>The date when the status changed.</p>"]
#[serde(rename="Date")]
pub date: DateTime,
#[doc="<p>The reason for the status.</p>"]
#[serde(rename="Message")]
pub message: StatusMessage,
#[doc="<p>The status.</p>"]
#[serde(rename="Name")]
pub name: AssociationStatusName,
            }
            
pub type AssociationStatusAggregatedCount = ::std::collections::HashMap<StatusName, InstanceCount>;
pub type AssociationStatusName = String;
pub type AttributeName = String;
pub type AttributeValue = String;
pub type AutomationActionName = String;
#[doc="<p>Detailed information about the current state of an individual Automation execution.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutomationExecution {
                #[doc="<p>The execution ID.</p>"]
#[serde(rename="AutomationExecutionId")]
pub automation_execution_id: Option<AutomationExecutionId>,
#[doc="<p>The execution status of the Automation.</p>"]
#[serde(rename="AutomationExecutionStatus")]
pub automation_execution_status: Option<AutomationExecutionStatus>,
#[doc="<p>The name of the Automation document used during the execution.</p>"]
#[serde(rename="DocumentName")]
pub document_name: Option<DocumentName>,
#[doc="<p>The version of the document to use during execution.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The time the execution finished.</p>"]
#[serde(rename="ExecutionEndTime")]
pub execution_end_time: Option<DateTime>,
#[doc="<p>The time the execution started.</p>"]
#[serde(rename="ExecutionStartTime")]
pub execution_start_time: Option<DateTime>,
#[doc="<p>A message describing why an execution has failed, if the status is set to Failed.</p>"]
#[serde(rename="FailureMessage")]
pub failure_message: Option<String>,
#[doc="<p>The list of execution outputs as defined in the automation document.</p>"]
#[serde(rename="Outputs")]
pub outputs: Option<AutomationParameterMap>,
#[doc="<p>The key-value map of execution parameters, which were supplied when calling <code>StartAutomationExecution</code>.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<AutomationParameterMap>,
#[doc="<p>A list of details about the current state of all steps that comprise an execution. An Automation document contains a list of steps that are executed in order.</p>"]
#[serde(rename="StepExecutions")]
pub step_executions: Option<StepExecutionList>,
            }
            
#[doc="<p>A filter used to match specific automation executions. This is used to limit the scope of Automation execution information returned.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct AutomationExecutionFilter {
                #[doc="<p>The aspect of the Automation execution information that should be limited.</p>"]
#[serde(rename="Key")]
pub key: AutomationExecutionFilterKey,
#[doc="<p>The values used to limit the execution information associated with the filter's key.</p>"]
#[serde(rename="Values")]
pub values: AutomationExecutionFilterValueList,
            }
            
pub type AutomationExecutionFilterKey = String;
pub type AutomationExecutionFilterList = Vec<AutomationExecutionFilter>;
pub type AutomationExecutionFilterValue = String;
pub type AutomationExecutionFilterValueList = Vec<AutomationExecutionFilterValue>;
pub type AutomationExecutionId = String;
#[doc="<p>Details about a specific Automation execution.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct AutomationExecutionMetadata {
                #[doc="<p>The execution ID.</p>"]
#[serde(rename="AutomationExecutionId")]
pub automation_execution_id: Option<AutomationExecutionId>,
#[doc="<p>The status of the execution. Valid values include: Running, Succeeded, Failed, Timed out, or Cancelled.</p>"]
#[serde(rename="AutomationExecutionStatus")]
pub automation_execution_status: Option<AutomationExecutionStatus>,
#[doc="<p>The name of the Automation document used during execution.</p>"]
#[serde(rename="DocumentName")]
pub document_name: Option<DocumentName>,
#[doc="<p>The document version used during the execution.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The IAM role ARN of the user who executed the Automation.</p>"]
#[serde(rename="ExecutedBy")]
pub executed_by: Option<String>,
#[doc="<p>The time the execution finished. This is not populated if the execution is still in progress.</p>"]
#[serde(rename="ExecutionEndTime")]
pub execution_end_time: Option<DateTime>,
#[doc="<p>The time the execution started.&gt;</p>"]
#[serde(rename="ExecutionStartTime")]
pub execution_start_time: Option<DateTime>,
#[doc="<p>An Amazon S3 bucket where execution information is stored.</p>"]
#[serde(rename="LogFile")]
pub log_file: Option<String>,
#[doc="<p>The list of execution outputs as defined in the Automation document.</p>"]
#[serde(rename="Outputs")]
pub outputs: Option<AutomationParameterMap>,
            }
            
pub type AutomationExecutionMetadataList = Vec<AutomationExecutionMetadata>;
pub type AutomationExecutionStatus = String;
pub type AutomationParameterKey = String;
pub type AutomationParameterMap = ::std::collections::HashMap<AutomationParameterKey, AutomationParameterValueList>;
pub type AutomationParameterValue = String;
pub type AutomationParameterValueList = Vec<AutomationParameterValue>;
pub type BaselineDescription = String;
pub type BaselineId = String;
pub type BaselineName = String;
pub type BatchErrorMessage = String;
pub type Boolean = bool;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CancelCommandRequest {
                #[doc="<p>The ID of the command you want to cancel.</p>"]
#[serde(rename="CommandId")]
pub command_id: CommandId,
#[doc="<p>(Optional) A list of instance IDs on which you want to cancel the command. If not provided, the command is canceled on every instance on which it was requested.</p>"]
#[serde(rename="InstanceIds")]
pub instance_ids: Option<InstanceIdList>,
            }
            
#[doc="<p>Whether or not the command was successfully canceled. There is no guarantee that a request can be canceled.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CancelCommandResult;
            
pub type ClientToken = String;
#[doc="<p>Describes a command request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Command {
                #[doc="<p>A unique identifier for this command.</p>"]
#[serde(rename="CommandId")]
pub command_id: Option<CommandId>,
#[doc="<p>User-specified information about the command, such as a brief description of what the command should do.</p>"]
#[serde(rename="Comment")]
pub comment: Option<Comment>,
#[doc="<p>The number of targets for which the command invocation reached a terminal state. Terminal states include the following: <code>Success</code>, <code>Failed</code>, <code>Execution Timed Out</code>, <code>Delivery Timed Out</code>, <code>Canceled</code>, <code>Terminated</code>, or <code>Undeliverable</code>.</p>"]
#[serde(rename="CompletedCount")]
pub completed_count: Option<CompletedCount>,
#[doc="<p>The name of the document requested for execution.</p>"]
#[serde(rename="DocumentName")]
pub document_name: Option<DocumentName>,
#[doc="<p>The number of targets for which the status is <code>Failed</code> or <code>Execution Timed Out</code>.</p>"]
#[serde(rename="ErrorCount")]
pub error_count: Option<ErrorCount>,
#[doc="<p>If this time is reached and the command has not already started executing, it will not execute. Calculated based on the ExpiresAfter user input provided as part of the SendCommand API.</p>"]
#[serde(rename="ExpiresAfter")]
pub expires_after: Option<DateTime>,
#[doc="<p>The instance IDs against which this command was requested.</p>"]
#[serde(rename="InstanceIds")]
pub instance_ids: Option<InstanceIdList>,
#[doc="<p>The maximum number of instances that are allowed to execute the command at the same time. You can specify a number of instances, such as 10, or a percentage of instances, such as 10%. The default value is 50. For more information about how to use <code>MaxConcurrency</code>, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="MaxConcurrency")]
pub max_concurrency: Option<MaxConcurrency>,
#[doc="<p>The maximum number of errors allowed before the system stops sending the command to additional targets. You can specify a number of errors, such as 10, or a percentage or errors, such as 10%. The default value is 50. For more information about how to use <code>MaxErrors</code>, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="MaxErrors")]
pub max_errors: Option<MaxErrors>,
#[doc="<p>Configurations for sending notifications about command status changes. </p>"]
#[serde(rename="NotificationConfig")]
pub notification_config: Option<NotificationConfig>,
#[doc="<p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>"]
#[serde(rename="OutputS3BucketName")]
pub output_s3_bucket_name: Option<S3BucketName>,
#[doc="<p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command.</p>"]
#[serde(rename="OutputS3KeyPrefix")]
pub output_s3_key_prefix: Option<S3KeyPrefix>,
#[doc="<p>The region where the Amazon Simple Storage Service (Amazon S3) output bucket is located. The default value is the region where Run Command is being called.</p>"]
#[serde(rename="OutputS3Region")]
pub output_s3_region: Option<S3Region>,
#[doc="<p>The parameter values to be inserted in the document when executing the command.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>The date and time the command was requested.</p>"]
#[serde(rename="RequestedDateTime")]
pub requested_date_time: Option<DateTime>,
#[doc="<p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes. </p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<ServiceRole>,
#[doc="<p>The status of the command.</p>"]
#[serde(rename="Status")]
pub status: Option<CommandStatus>,
#[doc="<p>A detailed status of the command execution. <code>StatusDetails</code> includes more information than <code>Status</code> because it includes states resulting from error and concurrency control parameters. <code>StatusDetails</code> can show different results than <code>Status</code>. For more information about these statuses, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html\">Run Command Status</a>. <code>StatusDetails</code> can be one of the following values:</p> <ul> <li> <p>Pending – The command has not been sent to any instances.</p> </li> <li> <p>In Progress – The command has been sent to at least one instance but has not reached a final state on all instances.</p> </li> <li> <p>Success – The command successfully executed on all invocations. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out – The value of <code>MaxErrors</code> or more command invocations shows a status of <code>Delivery Timed Out</code>. This is a terminal state.</p> </li> <li> <p>Execution Timed Out – The value of <code>MaxErrors</code> or more command invocations shows a status of <code>Execution Timed Out</code>. This is a terminal state.</p> </li> <li> <p>Failed – The value of <code>MaxErrors</code> or more command invocations shows a status of <code>Failed</code>. This is a terminal state.</p> </li> <li> <p>Incomplete – The command was attempted on all instances and one or more invocations does not have a value of <code>Success</code> but not enough invocations failed for the status to be <code>Failed</code>. This is a terminal state.</p> </li> <li> <p>Canceled – The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Rate Exceeded – The number of instances targeted by the command exceeded the account limit for pending invocations. The system has canceled the command before executing it on any instance. This is a terminal state.</p> </li> </ul>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<StatusDetails>,
#[doc="<p>The number of targets for the command.</p>"]
#[serde(rename="TargetCount")]
pub target_count: Option<TargetCount>,
#[doc="<p>An array of search criteria that targets instances using a <code>Key</code>,<code>Value</code> combination that you specify. <code>Targets</code> is required if you don't provide one or more instance IDs in the call.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
            }
            
#[doc="<p>Describes a command filter.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CommandFilter {
                #[doc="<p>The name of the filter. For example, requested date and time.</p>"]
#[serde(rename="key")]
pub key: CommandFilterKey,
#[doc="<p>The filter value. For example: June 30, 2015.</p>"]
#[serde(rename="value")]
pub value: CommandFilterValue,
            }
            
pub type CommandFilterKey = String;
pub type CommandFilterList = Vec<CommandFilter>;
pub type CommandFilterValue = String;
pub type CommandId = String;
#[doc="<p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user executes SendCommand against three instances, then a command invocation is created for each requested instance ID. A command invocation returns status and detail information about a command you executed. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CommandInvocation {
                #[doc="<p>The command against which this invocation was requested.</p>"]
#[serde(rename="CommandId")]
pub command_id: Option<CommandId>,
#[serde(rename="CommandPlugins")]
pub command_plugins: Option<CommandPluginList>,
#[doc="<p>User-specified information about the command, such as a brief description of what the command should do.</p>"]
#[serde(rename="Comment")]
pub comment: Option<Comment>,
#[doc="<p>The document name that was requested for execution.</p>"]
#[serde(rename="DocumentName")]
pub document_name: Option<DocumentName>,
#[doc="<p>The instance ID in which this invocation was requested.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the invocation target. For Amazon EC2 instances this is the value for the <code>aws:Name</code> tag. For on-premises instances, this is the name of the instance.</p>"]
#[serde(rename="InstanceName")]
pub instance_name: Option<InstanceTagName>,
#[doc="<p>Configurations for sending notifications about command status changes on a per instance basis.</p>"]
#[serde(rename="NotificationConfig")]
pub notification_config: Option<NotificationConfig>,
#[doc="<p>The time and date the request was sent to this instance.</p>"]
#[serde(rename="RequestedDateTime")]
pub requested_date_time: Option<DateTime>,
#[doc="<p>The IAM service role that Run Command uses to act on your behalf when sending notifications about command status changes on a per instance basis.</p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<ServiceRole>,
#[doc="<p>The URL to the plugin’s StdErr file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, <code>StandardErrorUrl</code> is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>"]
#[serde(rename="StandardErrorUrl")]
pub standard_error_url: Option<Url>,
#[doc="<p>The URL to the plugin’s StdOut file in Amazon S3, if the Amazon S3 bucket was defined for the parent command. For an invocation, <code>StandardOutputUrl</code> is populated if there is just one plugin defined for the command, and the Amazon S3 bucket was defined for the command.</p>"]
#[serde(rename="StandardOutputUrl")]
pub standard_output_url: Option<Url>,
#[doc="<p>Whether or not the invocation succeeded, failed, or is pending.</p>"]
#[serde(rename="Status")]
pub status: Option<CommandInvocationStatus>,
#[doc="<p>A detailed status of the command execution for each invocation (each instance targeted by the command). <code>StatusDetails</code> includes more information than <code>Status</code> because it includes states resulting from error and concurrency control parameters. <code>StatusDetails</code> can show different results than <code>Status</code>. For more information about these statuses, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html\">Run Command Status</a>. <code>StatusDetails</code> can be one of the following values:</p> <ul> <li> <p>Pending – The command has not been sent to the instance.</p> </li> <li> <p>In Progress – The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success – The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out – The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command’s <code>MaxErrors</code> limit, but they do contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Execution Timed Out – Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed – The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled – The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable – The command can't be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don't count against the parent command’s <code>MaxErrors</code> limit and don't contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Terminated – The parent command exceeded its <code>MaxErrors</code> limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<StatusDetails>,
#[doc="<p> Gets the trace output sent by the agent. </p>"]
#[serde(rename="TraceOutput")]
pub trace_output: Option<InvocationTraceOutput>,
            }
            
pub type CommandInvocationList = Vec<CommandInvocation>;
pub type CommandInvocationStatus = String;
pub type CommandList = Vec<Command>;
pub type CommandMaxResults = i64;
#[doc="<p>Describes plugin details.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CommandPlugin {
                #[doc="<p>The name of the plugin. Must be one of the following: aws:updateAgent, aws:domainjoin, aws:applications, aws:runPowerShellScript, aws:psmodule, aws:cloudWatch, aws:runShellScript, or aws:updateSSMAgent. </p>"]
#[serde(rename="Name")]
pub name: Option<CommandPluginName>,
#[doc="<p>Output of the plugin execution.</p>"]
#[serde(rename="Output")]
pub output: Option<CommandPluginOutput>,
#[doc="<p>The S3 bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> <code>test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript</code> </p> <p> <code>test_folder</code> is the name of the Amazon S3 bucket;</p> <p> <code>ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix</code> is the name of the S3 prefix;</p> <p> <code>i-1234567876543</code> is the instance ID;</p> <p> <code>awsrunShellScript</code> is the name of the plugin.</p>"]
#[serde(rename="OutputS3BucketName")]
pub output_s3_bucket_name: Option<S3BucketName>,
#[doc="<p>The S3 directory path inside the bucket where the responses to the command executions should be stored. This was requested when issuing the command. For example, in the following response:</p> <p> <code>test_folder/ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix/i-1234567876543/awsrunShellScript</code> </p> <p> <code>test_folder</code> is the name of the Amazon S3 bucket;</p> <p> <code>ab19cb99-a030-46dd-9dfc-8eSAMPLEPre-Fix</code> is the name of the S3 prefix;</p> <p> <code>i-1234567876543</code> is the instance ID;</p> <p> <code>awsrunShellScript</code> is the name of the plugin.</p>"]
#[serde(rename="OutputS3KeyPrefix")]
pub output_s3_key_prefix: Option<S3KeyPrefix>,
#[doc="<p>The name of the region where the output is stored in Amazon S3.</p>"]
#[serde(rename="OutputS3Region")]
pub output_s3_region: Option<S3Region>,
#[doc="<p>A numeric response code generated after executing the plugin. </p>"]
#[serde(rename="ResponseCode")]
pub response_code: Option<ResponseCode>,
#[doc="<p>The time the plugin stopped executing. Could stop prematurely if, for example, a cancel command was sent. </p>"]
#[serde(rename="ResponseFinishDateTime")]
pub response_finish_date_time: Option<DateTime>,
#[doc="<p>The time the plugin started executing. </p>"]
#[serde(rename="ResponseStartDateTime")]
pub response_start_date_time: Option<DateTime>,
#[doc="<p>The URL for the complete text written by the plugin to stderr. If execution is not yet complete, then this string is empty.</p>"]
#[serde(rename="StandardErrorUrl")]
pub standard_error_url: Option<Url>,
#[doc="<p>The URL for the complete text written by the plugin to stdout in Amazon S3. If the Amazon S3 bucket for the command was not specified, then this string is empty.</p>"]
#[serde(rename="StandardOutputUrl")]
pub standard_output_url: Option<Url>,
#[doc="<p>The status of this plugin. You can execute a document with multiple plugins.</p>"]
#[serde(rename="Status")]
pub status: Option<CommandPluginStatus>,
#[doc="<p>A detailed status of the plugin execution. <code>StatusDetails</code> includes more information than <code>Status</code> because it includes states resulting from error and concurrency control parameters. <code>StatusDetails</code> can show different results than <code>Status</code>. For more information about these statuses, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html\">Run Command Status</a>. <code>StatusDetails</code> can be one of the following values:</p> <ul> <li> <p>Pending – The command has not been sent to the instance.</p> </li> <li> <p>In Progress – The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Success – The execution of the command or plugin was successfully completed. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out – The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command’s <code>MaxErrors</code> limit, but they do contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Execution Timed Out – Command execution started on the instance, but the execution was not complete before the execution timeout expired. Execution timeouts count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed – The command was not successful on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled – The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable – The command can't be delivered to the instance. The instance might not exist, or it might not be responding. Undeliverable invocations don't count against the parent command’s <code>MaxErrors</code> limit, and they don't contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Terminated – The parent command exceeded its <code>MaxErrors</code> limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<StatusDetails>,
            }
            
pub type CommandPluginList = Vec<CommandPlugin>;
pub type CommandPluginName = String;
pub type CommandPluginOutput = String;
pub type CommandPluginStatus = String;
pub type CommandStatus = String;
pub type Comment = String;
pub type CompletedCount = i64;
pub type ComputerName = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateActivationRequest {
                #[doc="<p>The name of the registered, managed instance as it will appear in the Amazon EC2 console or when you use the AWS command line tools to list EC2 resources.</p>"]
#[serde(rename="DefaultInstanceName")]
pub default_instance_name: Option<DefaultInstanceName>,
#[doc="<p>A user-defined description of the resource that you want to register with Amazon EC2. </p>"]
#[serde(rename="Description")]
pub description: Option<ActivationDescription>,
#[doc="<p>The date by which this activation request should expire. The default value is 24 hours.</p>"]
#[serde(rename="ExpirationDate")]
pub expiration_date: Option<ExpirationDate>,
#[doc="<p>The Amazon Identity and Access Management (IAM) role that you want to assign to the managed instance. </p>"]
#[serde(rename="IamRole")]
pub iam_role: IamRole,
#[doc="<p>Specify the maximum number of managed instances you want to register. The default value is 1 instance.</p>"]
#[serde(rename="RegistrationLimit")]
pub registration_limit: Option<RegistrationLimit>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateActivationResult {
                #[doc="<p>The code the system generates when it processes the activation. The activation code functions like a password to validate the activation ID. </p>"]
#[serde(rename="ActivationCode")]
pub activation_code: Option<ActivationCode>,
#[doc="<p>The ID number generated by the system when it processed the activation. The activation ID functions like a user name.</p>"]
#[serde(rename="ActivationId")]
pub activation_id: Option<ActivationId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateAssociationBatchRequest {
                #[doc="<p>One or more associations.</p>"]
#[serde(rename="Entries")]
pub entries: CreateAssociationBatchRequestEntries,
            }
            
pub type CreateAssociationBatchRequestEntries = Vec<CreateAssociationBatchRequestEntry>;
#[doc="<p> Describes the association of a Systems Manager document and an instance.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct CreateAssociationBatchRequestEntry {
                #[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p> The ID of the instance. </p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p> The name of the configuration document. </p>"]
#[serde(rename="Name")]
pub name: DocumentName,
#[doc="<p>An Amazon S3 bucket where you want to store the results of this request.</p>"]
#[serde(rename="OutputLocation")]
pub output_location: Option<InstanceAssociationOutputLocation>,
#[doc="<p>A description of the parameters for a document. </p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>A cron expression that specifies a schedule when the association runs.</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>The instances targeted by the request.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateAssociationBatchResult {
                #[doc="<p>Information about the associations that failed.</p>"]
#[serde(rename="Failed")]
pub failed: Option<FailedCreateAssociationList>,
#[doc="<p>Information about the associations that succeeded.</p>"]
#[serde(rename="Successful")]
pub successful: Option<AssociationDescriptionList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateAssociationRequest {
                #[doc="<p>The document version you want to associate with the target(s). Can be a specific version or the default version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The instance ID.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the Systems Manager document.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
#[doc="<p>An Amazon S3 bucket where you want to store the output details of the request. For example:</p> <p> <code>\"{ \\\"S3Location\\\": { \\\"OutputS3Region\\\": \\\"&lt;region&gt;\\\", \\\"OutputS3BucketName\\\": \\\"bucket name\\\", \\\"OutputS3KeyPrefix\\\": \\\"folder name\\\" } }\"</code> </p>"]
#[serde(rename="OutputLocation")]
pub output_location: Option<InstanceAssociationOutputLocation>,
#[doc="<p>The parameters for the documents runtime configuration. </p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>A cron expression when the association will be applied to the target(s). Supported expressions are every half, 1, 2, 4, 8 or 12 hour(s); every specified day and time of the week. For example: cron(0 0/30 * 1/1 * ? *) to run every thirty minutes; cron(0 0 0/4 1/1 * ? *) to run every four hours; and cron(0 0 10 ? * SUN *) to run every Sunday at 10 a.m.</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
#[doc="<p>The targets (either instances or tags) for the association. Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateAssociationResult {
                #[doc="<p>Information about the association.</p>"]
#[serde(rename="AssociationDescription")]
pub association_description: Option<AssociationDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateDocumentRequest {
                #[doc="<p>A valid JSON string.</p>"]
#[serde(rename="Content")]
pub content: DocumentContent,
#[doc="<p>The type of document to create. Valid document types include: Policy, Automation, and Command.</p>"]
#[serde(rename="DocumentType")]
pub document_type: Option<DocumentType>,
#[doc="<p>A name for the Systems Manager document.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateDocumentResult {
                #[doc="<p>Information about the Systems Manager document.</p>"]
#[serde(rename="DocumentDescription")]
pub document_description: Option<DocumentDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreateMaintenanceWindowRequest {
                #[doc="<p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>"]
#[serde(rename="AllowUnassociatedTargets")]
pub allow_unassociated_targets: MaintenanceWindowAllowUnassociatedTargets,
#[doc="<p>User-provided idempotency token.</p>"]
#[serde(rename="ClientToken")]
pub client_token: Option<ClientToken>,
#[doc="<p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>"]
#[serde(rename="Cutoff")]
pub cutoff: MaintenanceWindowCutoff,
#[doc="<p>The duration of the Maintenance Window in hours.</p>"]
#[serde(rename="Duration")]
pub duration: MaintenanceWindowDurationHours,
#[doc="<p>The name of the Maintenance Window.</p>"]
#[serde(rename="Name")]
pub name: MaintenanceWindowName,
#[doc="<p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>"]
#[serde(rename="Schedule")]
pub schedule: MaintenanceWindowSchedule,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreateMaintenanceWindowResult {
                #[doc="<p>The ID of the created Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct CreatePatchBaselineRequest {
                #[doc="<p>A set of rules used to include patches in the baseline.</p>"]
#[serde(rename="ApprovalRules")]
pub approval_rules: Option<PatchRuleGroup>,
#[doc="<p>A list of explicitly approved patches for the baseline.</p>"]
#[serde(rename="ApprovedPatches")]
pub approved_patches: Option<PatchIdList>,
#[doc="<p>Caller-provided idempotency token.</p>"]
#[serde(rename="ClientToken")]
pub client_token: Option<ClientToken>,
#[doc="<p>A description of the patch baseline.</p>"]
#[serde(rename="Description")]
pub description: Option<BaselineDescription>,
#[doc="<p>A set of global filters used to exclude patches from the baseline.</p>"]
#[serde(rename="GlobalFilters")]
pub global_filters: Option<PatchFilterGroup>,
#[doc="<p>The name of the patch baseline.</p>"]
#[serde(rename="Name")]
pub name: BaselineName,
#[doc="<p>A list of explicitly rejected patches for the baseline.</p>"]
#[serde(rename="RejectedPatches")]
pub rejected_patches: Option<PatchIdList>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct CreatePatchBaselineResult {
                #[doc="<p>The ID of the created patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
            }
            
pub type CreatedDate = f64;
pub type DateTime = f64;
pub type DefaultBaseline = bool;
pub type DefaultInstanceName = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteActivationRequest {
                #[doc="<p>The ID of the activation that you want to delete.</p>"]
#[serde(rename="ActivationId")]
pub activation_id: ActivationId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteActivationResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteAssociationRequest {
                #[doc="<p>The association ID that you want to delete.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>The ID of the instance.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the Systems Manager document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteAssociationResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteDocumentRequest {
                #[doc="<p>The name of the document.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteDocumentResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteMaintenanceWindowRequest {
                #[doc="<p>The ID of the Maintenance Window to delete.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteMaintenanceWindowResult {
                #[doc="<p>The ID of the deleted Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeleteParameterRequest {
                #[doc="<p>The name of the parameter to delete.</p>"]
#[serde(rename="Name")]
pub name: PSParameterName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeleteParameterResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeletePatchBaselineRequest {
                #[doc="<p>The ID of the patch baseline to delete.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeletePatchBaselineResult {
                #[doc="<p>The ID of the deleted patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeregisterManagedInstanceRequest {
                #[doc="<p>The ID assigned to the managed instance when you registered it using the activation process. </p>"]
#[serde(rename="InstanceId")]
pub instance_id: ManagedInstanceId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeregisterManagedInstanceResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeregisterPatchBaselineForPatchGroupRequest {
                #[doc="<p>The ID of the patch baseline to deregister the patch group from.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
#[doc="<p>The name of the patch group that should be deregistered from the patch baseline.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeregisterPatchBaselineForPatchGroupResult {
                #[doc="<p>The ID of the patch baseline the patch group was deregistered from.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The name of the patch group deregistered from the patch baseline.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: Option<PatchGroup>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeregisterTargetFromMaintenanceWindowRequest {
                #[doc="<p>The ID of the Maintenance Window the target should be removed from.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
#[doc="<p>The ID of the target definition to remove.</p>"]
#[serde(rename="WindowTargetId")]
pub window_target_id: MaintenanceWindowTargetId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeregisterTargetFromMaintenanceWindowResult {
                #[doc="<p>The ID of the Maintenance Window the target was removed from.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
#[doc="<p>The ID of the removed target definition.</p>"]
#[serde(rename="WindowTargetId")]
pub window_target_id: Option<MaintenanceWindowTargetId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DeregisterTaskFromMaintenanceWindowRequest {
                #[doc="<p>The ID of the Maintenance Window the task should be removed from.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
#[doc="<p>The ID of the task to remove from the Maintenance Window.</p>"]
#[serde(rename="WindowTaskId")]
pub window_task_id: MaintenanceWindowTaskId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DeregisterTaskFromMaintenanceWindowResult {
                #[doc="<p>The ID of the Maintenance Window the task was removed from.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
#[doc="<p>The ID of the task removed from the Maintenance Window.</p>"]
#[serde(rename="WindowTaskId")]
pub window_task_id: Option<MaintenanceWindowTaskId>,
            }
            
#[doc="<p>Filter for the DescribeActivation API.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeActivationsFilter {
                #[doc="<p>The name of the filter.</p>"]
#[serde(rename="FilterKey")]
pub filter_key: Option<DescribeActivationsFilterKeys>,
#[doc="<p>The filter values.</p>"]
#[serde(rename="FilterValues")]
pub filter_values: Option<StringList>,
            }
            
pub type DescribeActivationsFilterKeys = String;
pub type DescribeActivationsFilterList = Vec<DescribeActivationsFilter>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeActivationsRequest {
                #[doc="<p>A filter to view information about your activations.</p>"]
#[serde(rename="Filters")]
pub filters: Option<DescribeActivationsFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>A token to start the list. Use this token to get the next set of results. </p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeActivationsResult {
                #[doc="<p>A list of activations for your AWS account.</p>"]
#[serde(rename="ActivationList")]
pub activation_list: Option<ActivationList>,
#[doc="<p> The token for the next set of items to return. Use this token to get the next set of results. </p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeAssociationRequest {
                #[doc="<p>The association ID for which you want information.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>The instance ID.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeAssociationResult {
                #[doc="<p>Information about the association.</p>"]
#[serde(rename="AssociationDescription")]
pub association_description: Option<AssociationDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeAutomationExecutionsRequest {
                #[doc="<p>Filters used to limit the scope of executions that are requested.</p>"]
#[serde(rename="Filters")]
pub filters: Option<AutomationExecutionFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeAutomationExecutionsResult {
                #[doc="<p>The list of details about each automation execution which has occurred which matches the filter specification, if any.</p>"]
#[serde(rename="AutomationExecutionMetadataList")]
pub automation_execution_metadata_list: Option<AutomationExecutionMetadataList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeAvailablePatchesRequest {
                #[doc="<p>Filters used to scope down the returned patches.</p>"]
#[serde(rename="Filters")]
pub filters: Option<PatchOrchestratorFilterList>,
#[doc="<p>The maximum number of patches to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchBaselineMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeAvailablePatchesResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>An array of patches. Each entry in the array is a patch structure.</p>"]
#[serde(rename="Patches")]
pub patches: Option<PatchList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeDocumentPermissionRequest {
                #[doc="<p>The name of the document for which you are the owner.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
#[doc="<p>The permission type for the document. The permission type can be <i>Share</i>.</p>"]
#[serde(rename="PermissionType")]
pub permission_type: DocumentPermissionType,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeDocumentPermissionResponse {
                #[doc="<p>The account IDs that have permission to use this document. The ID can be either an AWS account or <i>All</i>.</p>"]
#[serde(rename="AccountIds")]
pub account_ids: Option<AccountIdList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeDocumentRequest {
                #[doc="<p>The document version for which you want information. Can be a specific version or the default version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: DocumentARN,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeDocumentResult {
                #[doc="<p>Information about the SSM document.</p>"]
#[serde(rename="Document")]
pub document: Option<DocumentDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeEffectiveInstanceAssociationsRequest {
                #[doc="<p>The instance ID for which you want to view all associations.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<EffectiveInstanceAssociationMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeEffectiveInstanceAssociationsResult {
                #[doc="<p>The associations for the requested instance.</p>"]
#[serde(rename="Associations")]
pub associations: Option<InstanceAssociationList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeEffectivePatchesForPatchBaselineRequest {
                #[doc="<p>The ID of the patch baseline to retrieve the effective patches for.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
#[doc="<p>The maximum number of patches to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchBaselineMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeEffectivePatchesForPatchBaselineResult {
                #[doc="<p>An array of patches and patch status.</p>"]
#[serde(rename="EffectivePatches")]
pub effective_patches: Option<EffectivePatchList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInstanceAssociationsStatusRequest {
                #[doc="<p>The instance IDs for which you want association status information.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInstanceAssociationsStatusResult {
                #[doc="<p>Status information about the association.</p>"]
#[serde(rename="InstanceAssociationStatusInfos")]
pub instance_association_status_infos: Option<InstanceAssociationStatusInfos>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInstanceInformationRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of instances.</p>"]
#[serde(rename="Filters")]
pub filters: Option<InstanceInformationStringFilterList>,
#[doc="<p>One or more filters. Use a filter to return a more specific list of instances.</p>"]
#[serde(rename="InstanceInformationFilterList")]
pub instance_information_filter_list: Option<InstanceInformationFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results. </p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResultsEC2Compatible>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInstanceInformationResult {
                #[doc="<p>The instance information list.</p>"]
#[serde(rename="InstanceInformationList")]
pub instance_information_list: Option<InstanceInformationList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty. </p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInstancePatchStatesForPatchGroupRequest {
                #[doc="<p>Each entry in the array is a structure containing:</p> <p>Key (string 1 ≤ length ≤ 200)</p> <p> Values (array containing a single string)</p> <p> Type (string “Equal”, “NotEqual”, “LessThan”, “GreaterThan”)</p>"]
#[serde(rename="Filters")]
pub filters: Option<InstancePatchStateFilterList>,
#[doc="<p>The maximum number of patches to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchComplianceMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The name of the patch group for which the patch state information should be retrieved.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInstancePatchStatesForPatchGroupResult {
                #[doc="<p>The high-level patch state for the requested instances. </p>"]
#[serde(rename="InstancePatchStates")]
pub instance_patch_states: Option<InstancePatchStatesList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInstancePatchStatesRequest {
                #[doc="<p>The ID of the instance whose patch state information should be retrieved.</p>"]
#[serde(rename="InstanceIds")]
pub instance_ids: InstanceIdList,
#[doc="<p>The maximum number of instances to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchComplianceMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInstancePatchStatesResult {
                #[doc="<p>The high-level patch state for the requested instances.</p>"]
#[serde(rename="InstancePatchStates")]
pub instance_patch_states: Option<InstancePatchStateList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeInstancePatchesRequest {
                #[doc="<p>Each entry in the array is a structure containing:</p> <p>Key (string, 1 ≤ length ≤ 128)</p> <p>Values (array of strings 1 ≤ length ≤ 256)</p>"]
#[serde(rename="Filters")]
pub filters: Option<PatchOrchestratorFilterList>,
#[doc="<p>The ID of the instance whose patch state information should be retrieved.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The maximum number of patches to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchComplianceMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeInstancePatchesResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Each entry in the array is a structure containing:</p> <p>Title (string)</p> <p>KBId (string)</p> <p>Classification (string)</p> <p>Severity (string)</p> <p>State (string – “INSTALLED”, “INSTALLED_OTHER”, “MISSING”, “NOT_APPLICABLE”, “FAILED”)</p> <p>InstalledTime (DateTime)</p> <p>InstalledBy (string)</p>"]
#[serde(rename="Patches")]
pub patches: Option<PatchComplianceDataList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowExecutionTaskInvocationsRequest {
                #[doc="<p>Optional filters used to scope down the returned task invocations. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED.</p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The ID of the specific task in the Maintenance Window task that should be retrieved.</p>"]
#[serde(rename="TaskId")]
pub task_id: MaintenanceWindowExecutionTaskId,
#[doc="<p>The ID of the Maintenance Window execution the task is part of.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: MaintenanceWindowExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowExecutionTaskInvocationsResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the task invocation results per invocation.</p>"]
#[serde(rename="WindowExecutionTaskInvocationIdentities")]
pub window_execution_task_invocation_identities: Option<MaintenanceWindowExecutionTaskInvocationIdentityList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowExecutionTasksRequest {
                #[doc="<p>Optional filters used to scope down the returned tasks. The supported filter key is STATUS with the corresponding values PENDING, IN_PROGRESS, SUCCESS, FAILED, TIMED_OUT, CANCELLING, and CANCELLED. </p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The ID of the Maintenance Window execution whose task executions should be retrieved.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: MaintenanceWindowExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowExecutionTasksResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the task executions.</p>"]
#[serde(rename="WindowExecutionTaskIdentities")]
pub window_execution_task_identities: Option<MaintenanceWindowExecutionTaskIdentityList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowExecutionsRequest {
                #[doc="<p>Each entry in the array is a structure containing:</p> <p>Key (string, 1 ≤ length ≤ 128)</p> <p>Values (array of strings 1 ≤ length ≤ 256)</p> <p>The supported Keys are <code>ExecutedBefore</code> and <code>ExecutedAfter</code> with the value being a date/time string such as 2016-11-04T05:00:00Z.</p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The ID of the Maintenance Window whose executions should be retrieved.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowExecutionsResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the Maintenance Windows execution.</p>"]
#[serde(rename="WindowExecutions")]
pub window_executions: Option<MaintenanceWindowExecutionList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowTargetsRequest {
                #[doc="<p>Optional filters that can be used to narrow down the scope of the returned window targets. The supported filter keys are <code>Type</code>, <code>WindowTargetId</code> and <code>OwnerInformation</code>.</p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The ID of the Maintenance Window whose targets should be retrieved.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowTargetsResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the targets in the Maintenance Window.</p>"]
#[serde(rename="Targets")]
pub targets: Option<MaintenanceWindowTargetList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowTasksRequest {
                #[doc="<p>Optional filters used to narrow down the scope of the returned tasks. The supported filter keys are <code>WindowTaskId</code>, <code>TaskArn</code>, <code>Priority</code>, and <code>TaskType</code>.</p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The ID of the Maintenance Window whose tasks should be retrieved.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowTasksResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the tasks in the Maintenance Window.</p>"]
#[serde(rename="Tasks")]
pub tasks: Option<MaintenanceWindowTaskList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeMaintenanceWindowsRequest {
                #[doc="<p>Optional filters used to narrow down the scope of the returned Maintenance Windows. Supported filter keys are <code>Name</code> and <code>Enabled</code>.</p>"]
#[serde(rename="Filters")]
pub filters: Option<MaintenanceWindowFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaintenanceWindowMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeMaintenanceWindowsResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Information about the Maintenance Windows.</p>"]
#[serde(rename="WindowIdentities")]
pub window_identities: Option<MaintenanceWindowIdentityList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribeParametersRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="Filters")]
pub filters: Option<ParametersFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribeParametersResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Parameters returned by the request.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<ParameterMetadataList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribePatchBaselinesRequest {
                #[doc="<p>Each element in the array is a structure containing: </p> <p>Key: (string, “NAME_PREFIX” or “OWNER”)</p> <p>Value: (array of strings, exactly 1 entry, 1 ≤ length ≤ 255)</p>"]
#[serde(rename="Filters")]
pub filters: Option<PatchOrchestratorFilterList>,
#[doc="<p>The maximum number of patch baselines to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchBaselineMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribePatchBaselinesResult {
                #[doc="<p>An array of PatchBaselineIdentity elements.</p>"]
#[serde(rename="BaselineIdentities")]
pub baseline_identities: Option<PatchBaselineIdentityList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribePatchGroupStateRequest {
                #[doc="<p>The name of the patch group whose patch snapshot should be retrieved.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribePatchGroupStateResult {
                #[doc="<p>The number of instances in the patch group.</p>"]
#[serde(rename="Instances")]
pub instances: Option<Integer>,
#[doc="<p>The number of instances with patches from the patch baseline that failed to install.</p>"]
#[serde(rename="InstancesWithFailedPatches")]
pub instances_with_failed_patches: Option<Integer>,
#[doc="<p>The number of instances with patches installed that aren’t defined in the patch baseline.</p>"]
#[serde(rename="InstancesWithInstalledOtherPatches")]
pub instances_with_installed_other_patches: Option<Integer>,
#[doc="<p>The number of instances with installed patches.</p>"]
#[serde(rename="InstancesWithInstalledPatches")]
pub instances_with_installed_patches: Option<Integer>,
#[doc="<p>The number of instances with missing patches from the patch baseline.</p>"]
#[serde(rename="InstancesWithMissingPatches")]
pub instances_with_missing_patches: Option<Integer>,
#[doc="<p>The number of instances with patches that aren’t applicable.</p>"]
#[serde(rename="InstancesWithNotApplicablePatches")]
pub instances_with_not_applicable_patches: Option<Integer>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DescribePatchGroupsRequest {
                #[doc="<p>The maximum number of patch groups to return (per page).</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<PatchBaselineMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DescribePatchGroupsResult {
                #[doc="<p>Each entry in the array contains:</p> <p>PatchGroup: string (1 ≤ length ≤ 256, Regex: ^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-@]*)$)</p> <p>PatchBaselineIdentity: A PatchBaselineIdentity element. </p>"]
#[serde(rename="Mappings")]
pub mappings: Option<PatchGroupPatchBaselineMappingList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
pub type DescriptionInDocument = String;
pub type DocumentARN = String;
pub type DocumentContent = String;
#[doc="<p>A default version of a document.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DocumentDefaultVersionDescription {
                #[doc="<p>The default version of the document.</p>"]
#[serde(rename="DefaultVersion")]
pub default_version: Option<DocumentVersion>,
#[doc="<p>The name of the document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
            }
            
#[doc="<p> Describes an SSM document. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DocumentDescription {
                #[doc="<p> The date when the document was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<DateTime>,
#[doc="<p>The default version.</p>"]
#[serde(rename="DefaultVersion")]
pub default_version: Option<DocumentVersion>,
#[doc="<p> A description of the document. </p>"]
#[serde(rename="Description")]
pub description: Option<DescriptionInDocument>,
#[doc="<p>The type of document. </p>"]
#[serde(rename="DocumentType")]
pub document_type: Option<DocumentType>,
#[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note>"]
#[serde(rename="Hash")]
pub hash: Option<DocumentHash>,
#[doc="<p>Sha256 or Sha1.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note>"]
#[serde(rename="HashType")]
pub hash_type: Option<DocumentHashType>,
#[doc="<p>The latest version of the document.</p>"]
#[serde(rename="LatestVersion")]
pub latest_version: Option<DocumentVersion>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentARN>,
#[doc="<p>The AWS user account of the person who created the document.</p>"]
#[serde(rename="Owner")]
pub owner: Option<DocumentOwner>,
#[doc="<p>A description of the parameters for a document.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<DocumentParameterList>,
#[doc="<p>The list of OS platforms compatible with this SSM document. </p>"]
#[serde(rename="PlatformTypes")]
pub platform_types: Option<PlatformTypeList>,
#[doc="<p>The schema version.</p>"]
#[serde(rename="SchemaVersion")]
pub schema_version: Option<DocumentSchemaVersion>,
#[doc="<p>The SHA1 hash of the document, which you can use for verification purposes.</p>"]
#[serde(rename="Sha1")]
pub sha_1: Option<DocumentSha1>,
#[doc="<p>The status of the SSM document.</p>"]
#[serde(rename="Status")]
pub status: Option<DocumentStatus>,
            }
            
#[doc="<p>Describes a filter.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct DocumentFilter {
                #[doc="<p>The name of the filter.</p>"]
#[serde(rename="key")]
pub key: DocumentFilterKey,
#[doc="<p>The value of the filter.</p>"]
#[serde(rename="value")]
pub value: DocumentFilterValue,
            }
            
pub type DocumentFilterKey = String;
pub type DocumentFilterList = Vec<DocumentFilter>;
pub type DocumentFilterValue = String;
pub type DocumentHash = String;
pub type DocumentHashType = String;
#[doc="<p>Describes the name of an SSM document.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DocumentIdentifier {
                #[doc="<p>The document type.</p>"]
#[serde(rename="DocumentType")]
pub document_type: Option<DocumentType>,
#[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentARN>,
#[doc="<p>The AWS user account of the person who created the document.</p>"]
#[serde(rename="Owner")]
pub owner: Option<DocumentOwner>,
#[doc="<p>The operating system platform. </p>"]
#[serde(rename="PlatformTypes")]
pub platform_types: Option<PlatformTypeList>,
#[doc="<p>The schema version.</p>"]
#[serde(rename="SchemaVersion")]
pub schema_version: Option<DocumentSchemaVersion>,
            }
            
pub type DocumentIdentifierList = Vec<DocumentIdentifier>;
pub type DocumentName = String;
pub type DocumentOwner = String;
#[doc="<p>Parameters specified in a System Manager document that execute on the server when the command is run. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DocumentParameter {
                #[doc="<p>If specified, the default values for the parameters. Parameters without a default value are required. Parameters with a default value are optional.</p>"]
#[serde(rename="DefaultValue")]
pub default_value: Option<DocumentParameterDefaultValue>,
#[doc="<p>A description of what the parameter does, how to use it, the default value, and whether or not the parameter is optional.</p>"]
#[serde(rename="Description")]
pub description: Option<DocumentParameterDescrption>,
#[doc="<p>The name of the parameter.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentParameterName>,
#[doc="<p>The type of parameter. The type can be either “String” or “StringList”.</p>"]
#[serde(rename="Type")]
pub type_: Option<DocumentParameterType>,
            }
            
pub type DocumentParameterDefaultValue = String;
pub type DocumentParameterDescrption = String;
pub type DocumentParameterList = Vec<DocumentParameter>;
pub type DocumentParameterName = String;
pub type DocumentParameterType = String;
pub type DocumentPermissionType = String;
pub type DocumentSchemaVersion = String;
pub type DocumentSha1 = String;
pub type DocumentStatus = String;
pub type DocumentType = String;
pub type DocumentVersion = String;
#[doc="<p>Version information about the document.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct DocumentVersionInfo {
                #[doc="<p>The date the document was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<DateTime>,
#[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>An identifier for the default version of the document.</p>"]
#[serde(rename="IsDefaultVersion")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_default_version: Option<Boolean>,
#[doc="<p>The document name.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
            }
            
pub type DocumentVersionList = Vec<DocumentVersionInfo>;
pub type DocumentVersionNumber = String;
pub type EffectiveInstanceAssociationMaxResults = i64;
#[doc="<p>The EffectivePatch structure defines metadata about a patch along with the approval state of the patch in a particular patch baseline. The approval state includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct EffectivePatch {
                #[doc="<p>Provides metadata for a patch, including information such as the KB ID, severity, classification and a URL for where more information can be obtained about the patch.</p>"]
#[serde(rename="Patch")]
pub patch: Option<Patch>,
#[doc="<p>The status of the patch in a patch baseline. This includes information about whether the patch is currently approved, due to be approved by a rule, explicitly approved, or explicitly rejected and the date the patch was or will be approved.</p>"]
#[serde(rename="PatchStatus")]
pub patch_status: Option<PatchStatus>,
            }
            
pub type EffectivePatchList = Vec<EffectivePatch>;
pub type ErrorCount = i64;
pub type ExpirationDate = f64;
#[doc="<p>Describes a failed association.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct FailedCreateAssociation {
                #[doc="<p>The association.</p>"]
#[serde(rename="Entry")]
pub entry: Option<CreateAssociationBatchRequestEntry>,
#[doc="<p>The source of the failure.</p>"]
#[serde(rename="Fault")]
pub fault: Option<Fault>,
#[doc="<p>A description of the failure.</p>"]
#[serde(rename="Message")]
pub message: Option<BatchErrorMessage>,
            }
            
pub type FailedCreateAssociationList = Vec<FailedCreateAssociation>;
pub type Fault = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetAutomationExecutionRequest {
                #[doc="<p>The unique identifier for an existing automation execution to examine. The execution ID is returned by <code>StartAutomationExecution</code> when the execution of an Automation document is initiated.</p>"]
#[serde(rename="AutomationExecutionId")]
pub automation_execution_id: AutomationExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetAutomationExecutionResult {
                #[doc="<p>Detailed information about the current state of an automation execution.</p>"]
#[serde(rename="AutomationExecution")]
pub automation_execution: Option<AutomationExecution>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetCommandInvocationRequest {
                #[doc="<p>(Required) The parent command ID of the invocation plugin.</p>"]
#[serde(rename="CommandId")]
pub command_id: CommandId,
#[doc="<p>(Required) The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>(Optional) The name of the plugin for which you want detailed results. If the document contains only one plugin, the name can be omitted and the details will be returned.</p>"]
#[serde(rename="PluginName")]
pub plugin_name: Option<CommandPluginName>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetCommandInvocationResult {
                #[doc="<p>The parent command ID of the invocation plugin.</p>"]
#[serde(rename="CommandId")]
pub command_id: Option<CommandId>,
#[doc="<p>The comment text for the command.</p>"]
#[serde(rename="Comment")]
pub comment: Option<Comment>,
#[doc="<p>The name of the document that was executed. For example, AWS-RunShellScript.</p>"]
#[serde(rename="DocumentName")]
pub document_name: Option<DocumentName>,
#[doc="<p>Duration since <code>ExecutionStartDateTime</code>.</p>"]
#[serde(rename="ExecutionElapsedTime")]
pub execution_elapsed_time: Option<StringDateTime>,
#[doc="<p>The date and time the plugin was finished executing. Date and time are written in ISO 8601 format. For example, August 28, 2016 is represented as 2016-08-28. If the plugin has not started to execute, the string is empty.</p>"]
#[serde(rename="ExecutionEndDateTime")]
pub execution_end_date_time: Option<StringDateTime>,
#[doc="<p>The date and time the plugin started executing. Date and time are written in ISO 8601 format. For example, August 28, 2016 is represented as 2016-08-28. If the plugin has not started to execute, the string is empty.</p>"]
#[serde(rename="ExecutionStartDateTime")]
pub execution_start_date_time: Option<StringDateTime>,
#[doc="<p>The ID of the managed instance targeted by the command. A managed instance can be an Amazon EC2 instance or an instance in your hybrid environment that is configured for Systems Manager.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the plugin for which you want detailed results. For example, aws:RunShellScript is a plugin.</p>"]
#[serde(rename="PluginName")]
pub plugin_name: Option<CommandPluginName>,
#[doc="<p>The error level response code for the plugin script. If the response code is <code>-1</code>, then the command has not started executing on the instance, or it was not received by the instance.</p>"]
#[serde(rename="ResponseCode")]
pub response_code: Option<ResponseCode>,
#[doc="<p>The first 8,000 characters written by the plugin to stderr. If the command has not finished executing, then this string is empty.</p>"]
#[serde(rename="StandardErrorContent")]
pub standard_error_content: Option<StandardErrorContent>,
#[doc="<p>The URL for the complete text written by the plugin to stderr. If the command has not finished executing, then this string is empty.</p>"]
#[serde(rename="StandardErrorUrl")]
pub standard_error_url: Option<Url>,
#[doc="<p>The first 24,000 characters written by the plugin to stdout. If the command has not finished executing, if <code>ExecutionStatus</code> is neither <code>Succeeded</code> nor <code>Failed</code>, then this string is empty.</p>"]
#[serde(rename="StandardOutputContent")]
pub standard_output_content: Option<StandardOutputContent>,
#[doc="<p>The URL for the complete text written by the plugin to stdout in Amazon S3. If an Amazon S3 bucket was not specified, then this string is empty.</p>"]
#[serde(rename="StandardOutputUrl")]
pub standard_output_url: Option<Url>,
#[doc="<p>The status of the parent command for this invocation. This status can be different than <code>StatusDetails</code>.</p>"]
#[serde(rename="Status")]
pub status: Option<CommandInvocationStatus>,
#[doc="<p>A detailed status of the command execution for an invocation. <code>StatusDetails</code> includes more information than <code>Status</code> because it includes states resulting from error and concurrency control parameters. <code>StatusDetails</code> can show different results than <code>Status</code>. For more information about these statuses, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-about-status.html\">Run Command Status</a>. <code>StatusDetails</code> can be one of the following values:</p> <ul> <li> <p>Pending – The command has not been sent to the instance.</p> </li> <li> <p>In Progress – The command has been sent to the instance but has not reached a terminal state.</p> </li> <li> <p>Delayed – The system attempted to send the command to the target, but the target was not available. The instance might not be available because of network issues, the instance was stopped, etc. The system will try to deliver the command again.</p> </li> <li> <p>Success – The command or plugin was executed successfully. This is a terminal state.</p> </li> <li> <p>Delivery Timed Out – The command was not delivered to the instance before the delivery timeout expired. Delivery timeouts do not count against the parent command’s <code>MaxErrors</code> limit, but they do contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Execution Timed Out – The command started to execute on the instance, but the execution was not complete before the timeout expired. Execution timeouts count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Failed – The command wasn't executed successfully on the instance. For a plugin, this indicates that the result code was not zero. For a command invocation, this indicates that the result code for one or more plugins was not zero. Invocation failures count against the <code>MaxErrors</code> limit of the parent command. This is a terminal state.</p> </li> <li> <p>Canceled – The command was terminated before it was completed. This is a terminal state.</p> </li> <li> <p>Undeliverable – The command can't be delivered to the instance. The instance might not exist or might not be responding. Undeliverable invocations don't count against the parent command’s <code>MaxErrors</code> limit and don't contribute to whether the parent command status is <code>Success</code> or <code>Incomplete</code>. This is a terminal state.</p> </li> <li> <p>Terminated – The parent command exceeded its <code>MaxErrors</code> limit and subsequent command invocations were canceled by the system. This is a terminal state.</p> </li> </ul>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<StatusDetails>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDefaultPatchBaselineRequest;
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDefaultPatchBaselineResult {
                #[doc="<p>The ID of the default patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDeployablePatchSnapshotForInstanceRequest {
                #[doc="<p>The ID of the instance for which the appropriate patch snapshot should be retrieved.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The user-defined snapshot ID.</p>"]
#[serde(rename="SnapshotId")]
pub snapshot_id: SnapshotId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDeployablePatchSnapshotForInstanceResult {
                #[doc="<p>The ID of the instance.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>A pre-signed Amazon S3 URL that can be used to download the patch snapshot.</p>"]
#[serde(rename="SnapshotDownloadUrl")]
pub snapshot_download_url: Option<SnapshotDownloadUrl>,
#[doc="<p>The user-defined snapshot ID.</p>"]
#[serde(rename="SnapshotId")]
pub snapshot_id: Option<SnapshotId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetDocumentRequest {
                #[doc="<p>The document version for which you want information.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: DocumentARN,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetDocumentResult {
                #[doc="<p>The contents of the SSM document.</p>"]
#[serde(rename="Content")]
pub content: Option<DocumentContent>,
#[doc="<p>The document type.</p>"]
#[serde(rename="DocumentType")]
pub document_type: Option<DocumentType>,
#[doc="<p>The document version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentARN>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetInventoryRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="Filters")]
pub filters: Option<InventoryFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The list of inventory item types to return.</p>"]
#[serde(rename="ResultAttributes")]
pub result_attributes: Option<ResultAttributeList>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetInventoryResult {
                #[doc="<p>Collection of inventory entities such as a collection of instance inventory. </p>"]
#[serde(rename="Entities")]
pub entities: Option<InventoryResultEntityList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
pub type GetInventorySchemaMaxResults = i64;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetInventorySchemaRequest {
                #[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<GetInventorySchemaMaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The type of inventory item to return.</p>"]
#[serde(rename="TypeName")]
pub type_name: Option<InventoryItemTypeNameFilter>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetInventorySchemaResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Inventory schemas returned by the request.</p>"]
#[serde(rename="Schemas")]
pub schemas: Option<InventoryItemSchemaResultList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetMaintenanceWindowExecutionRequest {
                #[doc="<p>The ID of the Maintenance Window execution that includes the task.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: MaintenanceWindowExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetMaintenanceWindowExecutionResult {
                #[doc="<p>The time the Maintenance Window finished executing.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<DateTime>,
#[doc="<p>The time the Maintenance Window started executing.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<DateTime>,
#[doc="<p>The status of the Maintenance Window execution.</p>"]
#[serde(rename="Status")]
pub status: Option<MaintenanceWindowExecutionStatus>,
#[doc="<p>The details explaining the Status. Only available for certain status values.</p>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<MaintenanceWindowExecutionStatusDetails>,
#[doc="<p>The ID of the task executions from the Maintenance Window execution.</p>"]
#[serde(rename="TaskIds")]
pub task_ids: Option<MaintenanceWindowExecutionTaskIdList>,
#[doc="<p>The ID of the Maintenance Window execution.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: Option<MaintenanceWindowExecutionId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetMaintenanceWindowExecutionTaskRequest {
                #[doc="<p>The ID of the specific task execution in the Maintenance Window task that should be retrieved.</p>"]
#[serde(rename="TaskId")]
pub task_id: MaintenanceWindowExecutionTaskId,
#[doc="<p>The ID of the Maintenance Window execution that includes the task.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: MaintenanceWindowExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetMaintenanceWindowExecutionTaskResult {
                #[doc="<p>The time the task execution completed.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<DateTime>,
#[doc="<p>The defined maximum number of task executions that could be run in parallel.</p>"]
#[serde(rename="MaxConcurrency")]
pub max_concurrency: Option<MaxConcurrency>,
#[doc="<p>The defined maximum number of task execution errors allowed before scheduling of the task execution would have been stopped.</p>"]
#[serde(rename="MaxErrors")]
pub max_errors: Option<MaxErrors>,
#[doc="<p>The priority of the task.</p>"]
#[serde(rename="Priority")]
pub priority: Option<MaintenanceWindowTaskPriority>,
#[doc="<p>The role that was assumed when executing the task.</p>"]
#[serde(rename="ServiceRole")]
pub service_role: Option<ServiceRole>,
#[doc="<p>The time the task execution started.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<DateTime>,
#[doc="<p>The status of the task.</p>"]
#[serde(rename="Status")]
pub status: Option<MaintenanceWindowExecutionStatus>,
#[doc="<p>The details explaining the Status. Only available for certain status values.</p>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<MaintenanceWindowExecutionStatusDetails>,
#[doc="<p>The ARN of the executed task.</p>"]
#[serde(rename="TaskArn")]
pub task_arn: Option<MaintenanceWindowTaskArn>,
#[doc="<p>The ID of the specific task execution in the Maintenance Window task that was retrieved.</p>"]
#[serde(rename="TaskExecutionId")]
pub task_execution_id: Option<MaintenanceWindowExecutionTaskId>,
#[doc="<p>The parameters passed to the task when it was executed. The map has the following format:</p> <p>Key: string, 1 ≤ length ≤ 255</p> <p>Value: an array of strings where each string 1 ≤ length ≤ 255</p>"]
#[serde(rename="TaskParameters")]
pub task_parameters: Option<MaintenanceWindowTaskParametersList>,
#[doc="<p>The type of task executed.</p>"]
#[serde(rename="Type")]
pub type_: Option<MaintenanceWindowTaskType>,
#[doc="<p>The ID of the Maintenance Window execution that includes the task.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: Option<MaintenanceWindowExecutionId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetMaintenanceWindowRequest {
                #[doc="<p>The ID of the desired Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetMaintenanceWindowResult {
                #[doc="<p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>"]
#[serde(rename="AllowUnassociatedTargets")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub allow_unassociated_targets: Option<MaintenanceWindowAllowUnassociatedTargets>,
#[doc="<p>The date the Maintenance Window was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<DateTime>,
#[doc="<p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>"]
#[serde(rename="Cutoff")]
pub cutoff: Option<MaintenanceWindowCutoff>,
#[doc="<p>The duration of the Maintenance Window in hours.</p>"]
#[serde(rename="Duration")]
pub duration: Option<MaintenanceWindowDurationHours>,
#[doc="<p>Whether the Maintenance Windows is enabled.</p>"]
#[serde(rename="Enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<MaintenanceWindowEnabled>,
#[doc="<p>The date the Maintenance Window was last modified.</p>"]
#[serde(rename="ModifiedDate")]
pub modified_date: Option<DateTime>,
#[doc="<p>The name of the Maintenance Window.</p>"]
#[serde(rename="Name")]
pub name: Option<MaintenanceWindowName>,
#[doc="<p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>"]
#[serde(rename="Schedule")]
pub schedule: Option<MaintenanceWindowSchedule>,
#[doc="<p>The ID of the created Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetParameterHistoryRequest {
                #[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The name of a parameter you want to query.</p>"]
#[serde(rename="Name")]
pub name: PSParameterName,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>"]
#[serde(rename="WithDecryption")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub with_decryption: Option<Boolean>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetParameterHistoryResult {
                #[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>A list of parameters returned by the request.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<ParameterHistoryList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetParametersRequest {
                #[doc="<p>Names of the parameters for which you want to query information.</p>"]
#[serde(rename="Names")]
pub names: ParameterNameList,
#[doc="<p>Return decrypted secure string value. Return decrypted values for secure string parameters. This flag is ignored for String and StringList parameter types.</p>"]
#[serde(rename="WithDecryption")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub with_decryption: Option<Boolean>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetParametersResult {
                #[doc="<p>A list of parameters that are not formatted correctly or do not run when executed.</p>"]
#[serde(rename="InvalidParameters")]
pub invalid_parameters: Option<ParameterNameList>,
#[doc="<p>A list of details for a parameter.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<ParameterList>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetPatchBaselineForPatchGroupRequest {
                #[doc="<p>The name of the patch group whose patch baseline should be retrieved.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetPatchBaselineForPatchGroupResult {
                #[doc="<p>The ID of the patch baseline that should be used for the patch group.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The name of the patch group.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: Option<PatchGroup>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct GetPatchBaselineRequest {
                #[doc="<p>The ID of the patch baseline to retrieve.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct GetPatchBaselineResult {
                #[doc="<p>A set of rules used to include patches in the baseline.</p>"]
#[serde(rename="ApprovalRules")]
pub approval_rules: Option<PatchRuleGroup>,
#[doc="<p>A list of explicitly approved patches for the baseline.</p>"]
#[serde(rename="ApprovedPatches")]
pub approved_patches: Option<PatchIdList>,
#[doc="<p>The ID of the retrieved patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The date the patch baseline was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<DateTime>,
#[doc="<p>A description of the patch baseline.</p>"]
#[serde(rename="Description")]
pub description: Option<BaselineDescription>,
#[doc="<p>A set of global filters used to exclude patches from the baseline.</p>"]
#[serde(rename="GlobalFilters")]
pub global_filters: Option<PatchFilterGroup>,
#[doc="<p>The date the patch baseline was last modified.</p>"]
#[serde(rename="ModifiedDate")]
pub modified_date: Option<DateTime>,
#[doc="<p>The name of the patch baseline.</p>"]
#[serde(rename="Name")]
pub name: Option<BaselineName>,
#[doc="<p>Patch groups included in the patch baseline.</p>"]
#[serde(rename="PatchGroups")]
pub patch_groups: Option<PatchGroupList>,
#[doc="<p>A list of explicitly rejected patches for the baseline.</p>"]
#[serde(rename="RejectedPatches")]
pub rejected_patches: Option<PatchIdList>,
            }
            
pub type IPAddress = String;
pub type IamRole = String;
#[doc="<p>Status information about the aggregated associations.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceAggregatedAssociationOverview {
                #[doc="<p>Detailed status information about the aggregated associations.</p>"]
#[serde(rename="DetailedStatus")]
pub detailed_status: Option<StatusName>,
#[doc="<p>The number of associations for the instance(s).</p>"]
#[serde(rename="InstanceAssociationStatusAggregatedCount")]
pub instance_association_status_aggregated_count: Option<InstanceAssociationStatusAggregatedCount>,
            }
            
#[doc="<p>One or more association documents on the instance. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceAssociation {
                #[doc="<p>The association ID.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>The content of the association document for the instance(s).</p>"]
#[serde(rename="Content")]
pub content: Option<DocumentContent>,
#[doc="<p>The instance ID.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
            }
            
pub type InstanceAssociationExecutionSummary = String;
pub type InstanceAssociationList = Vec<InstanceAssociation>;
#[doc="<p>An Amazon S3 bucket where you want to store the results of this request.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct InstanceAssociationOutputLocation {
                #[doc="<p>An Amazon S3 bucket where you want to store the results of this request.</p>"]
#[serde(rename="S3Location")]
pub s3_location: Option<S3OutputLocation>,
            }
            
#[doc="<p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceAssociationOutputUrl {
                #[doc="<p>The URL of Amazon S3 bucket where you want to store the results of this request.</p>"]
#[serde(rename="S3OutputUrl")]
pub s3_output_url: Option<S3OutputUrl>,
            }
            
pub type InstanceAssociationStatusAggregatedCount = ::std::collections::HashMap<StatusName, InstanceCount>;
#[doc="<p>Status information about the instance association.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceAssociationStatusInfo {
                #[doc="<p>The association ID.</p>"]
#[serde(rename="AssociationId")]
pub association_id: Option<AssociationId>,
#[doc="<p>Detailed status information about the instance association.</p>"]
#[serde(rename="DetailedStatus")]
pub detailed_status: Option<StatusName>,
#[doc="<p>The association document verions.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>An error code returned by the request to create the association.</p>"]
#[serde(rename="ErrorCode")]
pub error_code: Option<AgentErrorCode>,
#[doc="<p>The date the instance association executed. </p>"]
#[serde(rename="ExecutionDate")]
pub execution_date: Option<DateTime>,
#[doc="<p>Summary information about association execution.</p>"]
#[serde(rename="ExecutionSummary")]
pub execution_summary: Option<InstanceAssociationExecutionSummary>,
#[doc="<p>The instance ID where the association was created.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The name of the association.</p>"]
#[serde(rename="Name")]
pub name: Option<DocumentName>,
#[doc="<p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>"]
#[serde(rename="OutputUrl")]
pub output_url: Option<InstanceAssociationOutputUrl>,
#[doc="<p>Status information about the instance association.</p>"]
#[serde(rename="Status")]
pub status: Option<StatusName>,
            }
            
pub type InstanceAssociationStatusInfos = Vec<InstanceAssociationStatusInfo>;
pub type InstanceCount = i64;
pub type InstanceId = String;
pub type InstanceIdList = Vec<InstanceId>;
#[doc="<p>Describes a filter for a specific list of instances. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstanceInformation {
                #[doc="<p>The activation ID created by Systems Manager when the server or VM was registered.</p>"]
#[serde(rename="ActivationId")]
pub activation_id: Option<ActivationId>,
#[doc="<p>The version of the SSM Agent running on your Linux instance. </p>"]
#[serde(rename="AgentVersion")]
pub agent_version: Option<Version>,
#[doc="<p>Information about the association.</p>"]
#[serde(rename="AssociationOverview")]
pub association_overview: Option<InstanceAggregatedAssociationOverview>,
#[doc="<p>The status of the association.</p>"]
#[serde(rename="AssociationStatus")]
pub association_status: Option<StatusName>,
#[doc="<p>The fully qualified host name of the managed instance.</p>"]
#[serde(rename="ComputerName")]
pub computer_name: Option<ComputerName>,
#[doc="<p>The IP address of the managed instance.</p>"]
#[serde(rename="IPAddress")]
pub ip_address: Option<IPAddress>,
#[doc="<p>The Amazon Identity and Access Management (IAM) role assigned to EC2 instances or managed instances. </p>"]
#[serde(rename="IamRole")]
pub iam_role: Option<IamRole>,
#[doc="<p>The instance ID. </p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>Indicates whether latest version of the SSM Agent is running on your instance. </p>"]
#[serde(rename="IsLatestVersion")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub is_latest_version: Option<Boolean>,
#[doc="<p>The date the association was last executed.</p>"]
#[serde(rename="LastAssociationExecutionDate")]
pub last_association_execution_date: Option<DateTime>,
#[doc="<p>The date and time when agent last pinged Systems Manager service. </p>"]
#[serde(rename="LastPingDateTime")]
pub last_ping_date_time: Option<DateTime>,
#[doc="<p>The last date the association was successfully run.</p>"]
#[serde(rename="LastSuccessfulAssociationExecutionDate")]
pub last_successful_association_execution_date: Option<DateTime>,
#[doc="<p>The name of the managed instance.</p>"]
#[serde(rename="Name")]
pub name: Option<String>,
#[doc="<p>Connection status of the SSM Agent. </p>"]
#[serde(rename="PingStatus")]
pub ping_status: Option<PingStatus>,
#[doc="<p>The name of the operating system platform running on your instance. </p>"]
#[serde(rename="PlatformName")]
pub platform_name: Option<String>,
#[doc="<p>The operating system platform type. </p>"]
#[serde(rename="PlatformType")]
pub platform_type: Option<PlatformType>,
#[doc="<p>The version of the OS platform running on your instance. </p>"]
#[serde(rename="PlatformVersion")]
pub platform_version: Option<String>,
#[doc="<p>The date the server or VM was registered with AWS as a managed instance.</p>"]
#[serde(rename="RegistrationDate")]
pub registration_date: Option<DateTime>,
#[doc="<p>The type of instance. Instances are either EC2 instances or managed instances. </p>"]
#[serde(rename="ResourceType")]
pub resource_type: Option<ResourceType>,
            }
            
#[doc="<p>Describes a filter for a specific list of instances. </p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InstanceInformationFilter {
                #[doc="<p>The name of the filter. </p>"]
#[serde(rename="key")]
pub key: InstanceInformationFilterKey,
#[doc="<p>The filter values.</p>"]
#[serde(rename="valueSet")]
pub value_set: InstanceInformationFilterValueSet,
            }
            
pub type InstanceInformationFilterKey = String;
pub type InstanceInformationFilterList = Vec<InstanceInformationFilter>;
pub type InstanceInformationFilterValue = String;
pub type InstanceInformationFilterValueSet = Vec<InstanceInformationFilterValue>;
pub type InstanceInformationList = Vec<InstanceInformation>;
#[doc="<p>The filters to describe or get information about your managed instances.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InstanceInformationStringFilter {
                #[doc="<p>The filter key name to describe your instances. For example:</p> <p>\"InstanceIds\"|\"AgentVersion\"|\"PingStatus\"|\"PlatformTypes\"|\"ActivationIds\"|\"IamRole\"|\"ResourceType\"|”AssociationStatus”|”Tag Key”</p>"]
#[serde(rename="Key")]
pub key: InstanceInformationStringFilterKey,
#[doc="<p>The filter values.</p>"]
#[serde(rename="Values")]
pub values: InstanceInformationFilterValueSet,
            }
            
pub type InstanceInformationStringFilterKey = String;
pub type InstanceInformationStringFilterList = Vec<InstanceInformationStringFilter>;
#[doc="<p>Defines the high-level patch compliance state for a managed instance, providing information about the number of installed, missing, not applicable, and failed patches along with metadata about the operation when this information was gathered for the instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InstancePatchState {
                #[doc="<p>The ID of the patch baseline used to patch the instance.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
#[doc="<p>The number of patches from the patch baseline that were attempted to be installed during the last patching operation, but failed to install.</p>"]
#[serde(rename="FailedCount")]
pub failed_count: Option<PatchFailedCount>,
#[doc="<p>The number of patches from the patch baseline that are installed on the instance.</p>"]
#[serde(rename="InstalledCount")]
pub installed_count: Option<PatchInstalledCount>,
#[doc="<p>The number of patches not specified in the patch baseline that are installed on the instance.</p>"]
#[serde(rename="InstalledOtherCount")]
pub installed_other_count: Option<PatchInstalledOtherCount>,
#[doc="<p>The ID of the managed instance the high-level patch compliance information was collected for.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The number of patches from the patch baseline that are applicable for the instance but aren’t currently installed.</p>"]
#[serde(rename="MissingCount")]
pub missing_count: Option<PatchMissingCount>,
#[doc="<p>The number of patches from the patch baseline that aren’t applicable for the instance and hence aren’t installed on the instance.</p>"]
#[serde(rename="NotApplicableCount")]
pub not_applicable_count: Option<PatchNotApplicableCount>,
#[doc="<p>The type of patching operation that was performed: SCAN (assess patch compliance state) or INSTALL (install missing patches).</p>"]
#[serde(rename="Operation")]
pub operation: PatchOperationType,
#[doc="<p>The time the most recent patching operation completed on the instance.</p>"]
#[serde(rename="OperationEndTime")]
pub operation_end_time: PatchOperationEndTime,
#[doc="<p>The time the most recent patching operation was started on the instance.</p>"]
#[serde(rename="OperationStartTime")]
pub operation_start_time: PatchOperationStartTime,
#[doc="<p>Placeholder information, this field will always be empty in the current release of the service.</p>"]
#[serde(rename="OwnerInformation")]
pub owner_information: Option<OwnerInformation>,
#[doc="<p>The name of the patch group the managed instance belongs to.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
#[doc="<p>The ID of the patch baseline snapshot used during the patching operation when this compliance data was collected.</p>"]
#[serde(rename="SnapshotId")]
pub snapshot_id: Option<SnapshotId>,
            }
            
#[doc="<p>Defines a filter used in DescribeInstancePatchStatesForPatchGroup used to scope down the information returned by the API.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InstancePatchStateFilter {
                #[doc="<p>The key for the filter. Supported values are FailedCount, InstalledCount, InstalledOtherCount, MissingCount and NotApplicableCount.</p>"]
#[serde(rename="Key")]
pub key: InstancePatchStateFilterKey,
#[doc="<p>The type of comparison that should be performed for the value: Equal, NotEqual, LessThan or GreaterThan.</p>"]
#[serde(rename="Type")]
pub type_: InstancePatchStateOperatorType,
#[doc="<p>The value for the filter, must be an integer greater than or equal to 0.</p>"]
#[serde(rename="Values")]
pub values: InstancePatchStateFilterValues,
            }
            
pub type InstancePatchStateFilterKey = String;
pub type InstancePatchStateFilterList = Vec<InstancePatchStateFilter>;
pub type InstancePatchStateFilterValue = String;
pub type InstancePatchStateFilterValues = Vec<InstancePatchStateFilterValue>;
pub type InstancePatchStateList = Vec<InstancePatchState>;
pub type InstancePatchStateOperatorType = String;
pub type InstancePatchStatesList = Vec<InstancePatchState>;
pub type InstanceTagName = String;
pub type Integer = i64;
pub type InventoryAttributeDataType = String;
#[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InventoryFilter {
                #[doc="<p>The name of the filter key.</p>"]
#[serde(rename="Key")]
pub key: InventoryFilterKey,
#[doc="<p>The type of filter. Valid values include the following: \"Equal\"|\"NotEqual\"|\"BeginWith\"|\"LessThan\"|\"GreaterThan\"</p>"]
#[serde(rename="Type")]
pub type_: Option<InventoryQueryOperatorType>,
#[doc="<p>Inventory filter values. Example: inventory filter where instance IDs are specified as values Key=AWS:InstanceInformation.InstanceId,Values= i-a12b3c4d5e6g, i-1a2b3c4d5e6,Type=Equal </p>"]
#[serde(rename="Values")]
pub values: InventoryFilterValueList,
            }
            
pub type InventoryFilterKey = String;
pub type InventoryFilterList = Vec<InventoryFilter>;
pub type InventoryFilterValue = String;
pub type InventoryFilterValueList = Vec<InventoryFilterValue>;
#[doc="<p>Information collected from managed instances based on your inventory policy document</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct InventoryItem {
                #[doc="<p>The time the inventory information was collected.</p>"]
#[serde(rename="CaptureTime")]
pub capture_time: InventoryItemCaptureTime,
#[doc="<p>The inventory data of the inventory type.</p>"]
#[serde(rename="Content")]
pub content: Option<InventoryItemEntryList>,
#[doc="<p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The <code>PutInventory</code> API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>"]
#[serde(rename="ContentHash")]
pub content_hash: Option<InventoryItemContentHash>,
#[doc="<p>The schema version for the inventory item.</p>"]
#[serde(rename="SchemaVersion")]
pub schema_version: InventoryItemSchemaVersion,
#[doc="<p>The name of the inventory type. Default inventory item type names start with <code>AWS</code>. Custom inventory type names will start with <code>Custom</code>. Default inventory item types include the following: <code>AWS:AWSComponent</code>, <code>AWS:Application</code>, <code>AWS:InstanceInformation</code>, <code>AWS:Network</code>, and <code>AWS:WindowsUpdate</code>.</p>"]
#[serde(rename="TypeName")]
pub type_name: InventoryItemTypeName,
            }
            
#[doc="<p>Attributes are the entries within the inventory item content. It contains name and value.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InventoryItemAttribute {
                #[doc="<p>The data type of the inventory item attribute. </p>"]
#[serde(rename="DataType")]
pub data_type: InventoryAttributeDataType,
#[doc="<p>Name of the inventory item attribute.</p>"]
#[serde(rename="Name")]
pub name: InventoryItemAttributeName,
            }
            
pub type InventoryItemAttributeList = Vec<InventoryItemAttribute>;
pub type InventoryItemAttributeName = String;
pub type InventoryItemCaptureTime = String;
pub type InventoryItemContentHash = String;
pub type InventoryItemEntry = ::std::collections::HashMap<AttributeName, AttributeValue>;
pub type InventoryItemEntryList = Vec<InventoryItemEntry>;
pub type InventoryItemList = Vec<InventoryItem>;
#[doc="<p>The inventory item schema definition. Users can use this to compose inventory query filters.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InventoryItemSchema {
                #[doc="<p>The schema attributes for inventory. This contains data type and attribute name.</p>"]
#[serde(rename="Attributes")]
pub attributes: InventoryItemAttributeList,
#[doc="<p>The name of the inventory type. Default inventory item type names start with <code>AWS</code>. Custom inventory type names will start with <code>Custom</code>. Default inventory item types include the following: <code>AWS:AWSComponent</code>, <code>AWS:Application</code>, <code>AWS:InstanceInformation</code>, <code>AWS:Network</code>, and <code>AWS:WindowsUpdate</code>.</p>"]
#[serde(rename="TypeName")]
pub type_name: InventoryItemTypeName,
#[doc="<p>The schema version for the inventory item.</p>"]
#[serde(rename="Version")]
pub version: Option<InventoryItemSchemaVersion>,
            }
            
pub type InventoryItemSchemaResultList = Vec<InventoryItemSchema>;
pub type InventoryItemSchemaVersion = String;
pub type InventoryItemTypeName = String;
pub type InventoryItemTypeNameFilter = String;
pub type InventoryQueryOperatorType = String;
#[doc="<p>Inventory query results.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InventoryResultEntity {
                #[doc="<p>The data section in the inventory result entity json.</p>"]
#[serde(rename="Data")]
pub data: Option<InventoryResultItemMap>,
#[doc="<p>ID of the inventory result entity. For example, for managed instance inventory the result will be the managed instance ID. For EC2 instance inventory, the result will be the instance ID. </p>"]
#[serde(rename="Id")]
pub id: Option<InventoryResultEntityId>,
            }
            
pub type InventoryResultEntityId = String;
pub type InventoryResultEntityList = Vec<InventoryResultEntity>;
#[doc="<p>The inventory result item.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct InventoryResultItem {
                #[doc="<p>The time inventory item data was captured.</p>"]
#[serde(rename="CaptureTime")]
pub capture_time: Option<InventoryItemCaptureTime>,
#[doc="<p>Contains all the inventory data of the item type. Results include attribute names and values. </p>"]
#[serde(rename="Content")]
pub content: InventoryItemEntryList,
#[doc="<p>MD5 hash of the inventory item type contents. The content hash is used to determine whether to update inventory information. The <code>PutInventory</code> API does not update the inventory item type contents if the MD5 hash has not changed since last update. </p>"]
#[serde(rename="ContentHash")]
pub content_hash: Option<InventoryItemContentHash>,
#[doc="<p>The schema version for the inventory result item/</p>"]
#[serde(rename="SchemaVersion")]
pub schema_version: InventoryItemSchemaVersion,
#[doc="<p>The name of the inventory result item type.</p>"]
#[serde(rename="TypeName")]
pub type_name: InventoryItemTypeName,
            }
            
pub type InventoryResultItemKey = String;
pub type InventoryResultItemMap = ::std::collections::HashMap<InventoryResultItemKey, InventoryResultItem>;
pub type InvocationTraceOutput = String;
pub type KeyList = Vec<TagKey>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListAssociationsRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="AssociationFilterList")]
pub association_filter_list: Option<AssociationFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListAssociationsResult {
                #[doc="<p>The associations.</p>"]
#[serde(rename="Associations")]
pub associations: Option<AssociationList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListCommandInvocationsRequest {
                #[doc="<p>(Optional) The invocations for a specific command ID.</p>"]
#[serde(rename="CommandId")]
pub command_id: Option<CommandId>,
#[doc="<p>(Optional) If set this returns the response of the command executions and any command output. By default this is set to False. </p>"]
#[serde(rename="Details")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub details: Option<Boolean>,
#[doc="<p>(Optional) One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="Filters")]
pub filters: Option<CommandFilterList>,
#[doc="<p>(Optional) The command execution details for a specific instance ID.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<CommandMaxResults>,
#[doc="<p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListCommandInvocationsResult {
                #[doc="<p>(Optional) A list of all invocations. </p>"]
#[serde(rename="CommandInvocations")]
pub command_invocations: Option<CommandInvocationList>,
#[doc="<p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListCommandsRequest {
                #[doc="<p>(Optional) If provided, lists only the specified command.</p>"]
#[serde(rename="CommandId")]
pub command_id: Option<CommandId>,
#[doc="<p>(Optional) One or more filters. Use a filter to return a more specific list of results. </p>"]
#[serde(rename="Filters")]
pub filters: Option<CommandFilterList>,
#[doc="<p>(Optional) Lists commands issued against this instance ID.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>(Optional) The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<CommandMaxResults>,
#[doc="<p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListCommandsResult {
                #[doc="<p>(Optional) The list of commands requested by the user. </p>"]
#[serde(rename="Commands")]
pub commands: Option<CommandList>,
#[doc="<p>(Optional) The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDocumentVersionsRequest {
                #[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The name of the document about which you want version information.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDocumentVersionsResult {
                #[doc="<p>The document versions.</p>"]
#[serde(rename="DocumentVersions")]
pub document_versions: Option<DocumentVersionList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListDocumentsRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="DocumentFilterList")]
pub document_filter_list: Option<DocumentFilterList>,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListDocumentsResult {
                #[doc="<p>The names of the SSM documents.</p>"]
#[serde(rename="DocumentIdentifiers")]
pub document_identifiers: Option<DocumentIdentifierList>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListInventoryEntriesRequest {
                #[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[serde(rename="Filters")]
pub filters: Option<InventoryFilterList>,
#[doc="<p>The instance ID for which you want inventory information.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>"]
#[serde(rename="MaxResults")]
pub max_results: Option<MaxResults>,
#[doc="<p>The token for the next set of items to return. (You received this token from a previous call.)</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The type of inventory item for which you want information.</p>"]
#[serde(rename="TypeName")]
pub type_name: InventoryItemTypeName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListInventoryEntriesResult {
                #[doc="<p>The time that inventory information was collected for the instance(s).</p>"]
#[serde(rename="CaptureTime")]
pub capture_time: Option<InventoryItemCaptureTime>,
#[doc="<p>A list of inventory items on the instance(s).</p>"]
#[serde(rename="Entries")]
pub entries: Option<InventoryItemEntryList>,
#[doc="<p>The instance ID targeted by the request to query inventory information.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: Option<InstanceId>,
#[doc="<p>The token to use when requesting the next set of items. If there are no additional items to return, the string is empty.</p>"]
#[serde(rename="NextToken")]
pub next_token: Option<NextToken>,
#[doc="<p>The inventory schema version used by the instance(s).</p>"]
#[serde(rename="SchemaVersion")]
pub schema_version: Option<InventoryItemSchemaVersion>,
#[doc="<p>The type of inventory item returned by the request.</p>"]
#[serde(rename="TypeName")]
pub type_name: Option<InventoryItemTypeName>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ListTagsForResourceRequest {
                #[doc="<p>The resource ID for which you want to see a list of tags.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: ResourceId,
#[doc="<p>Returns a list of tags for a specific resource type.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: ResourceTypeForTagging,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ListTagsForResourceResult {
                #[doc="<p>A list of tags.</p>"]
#[serde(rename="TagList")]
pub tag_list: Option<TagList>,
            }
            
#[doc="<p>Information about an Amazon S3 bucket to write instance-level logs to.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct LoggingInfo {
                #[doc="<p>The name of an Amazon S3 bucket where execution logs are stored .</p>"]
#[serde(rename="S3BucketName")]
pub s3_bucket_name: S3BucketName,
#[doc="<p>(Optional) The Amazon S3 bucket subfolder. </p>"]
#[serde(rename="S3KeyPrefix")]
pub s3_key_prefix: Option<S3KeyPrefix>,
#[doc="<p>The region where the Amazon S3 bucket is located.</p>"]
#[serde(rename="S3Region")]
pub s3_region: S3Region,
            }
            
pub type MaintenanceWindowAllowUnassociatedTargets = bool;
pub type MaintenanceWindowCutoff = i64;
pub type MaintenanceWindowDurationHours = i64;
pub type MaintenanceWindowEnabled = bool;
#[doc="<p>Describes the information about an execution of a Maintenance Window. </p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowExecution {
                #[doc="<p>The time the execution finished.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<DateTime>,
#[doc="<p>The time the execution started.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<DateTime>,
#[doc="<p>The status of the execution.</p>"]
#[serde(rename="Status")]
pub status: Option<MaintenanceWindowExecutionStatus>,
#[doc="<p>The details explaining the Status. Only available for certain status values.</p>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<MaintenanceWindowExecutionStatusDetails>,
#[doc="<p>The ID of the Maintenance Window execution.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: Option<MaintenanceWindowExecutionId>,
#[doc="<p>The ID of the Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
pub type MaintenanceWindowExecutionId = String;
pub type MaintenanceWindowExecutionList = Vec<MaintenanceWindowExecution>;
pub type MaintenanceWindowExecutionStatus = String;
pub type MaintenanceWindowExecutionStatusDetails = String;
pub type MaintenanceWindowExecutionTaskExecutionId = String;
pub type MaintenanceWindowExecutionTaskId = String;
pub type MaintenanceWindowExecutionTaskIdList = Vec<MaintenanceWindowExecutionTaskId>;
#[doc="<p>Information about a task execution performed as part of a Maintenance Window execution.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowExecutionTaskIdentity {
                #[doc="<p>The time the task execution finished.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<DateTime>,
#[doc="<p>The time the task execution started.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<DateTime>,
#[doc="<p>The status of the task execution.</p>"]
#[serde(rename="Status")]
pub status: Option<MaintenanceWindowExecutionStatus>,
#[doc="<p>The details explaining the status of the task execution. Only available for certain status values.</p>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<MaintenanceWindowExecutionStatusDetails>,
#[doc="<p>The ARN of the executed task.</p>"]
#[serde(rename="TaskArn")]
pub task_arn: Option<MaintenanceWindowTaskArn>,
#[doc="<p>The ID of the specific task execution in the Maintenance Window execution.</p>"]
#[serde(rename="TaskExecutionId")]
pub task_execution_id: Option<MaintenanceWindowExecutionTaskId>,
#[doc="<p>The type of executed task.</p>"]
#[serde(rename="TaskType")]
pub task_type: Option<MaintenanceWindowTaskType>,
#[doc="<p>The ID of the Maintenance Window execution that ran the task.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: Option<MaintenanceWindowExecutionId>,
            }
            
pub type MaintenanceWindowExecutionTaskIdentityList = Vec<MaintenanceWindowExecutionTaskIdentity>;
pub type MaintenanceWindowExecutionTaskInvocationId = String;
#[doc="<p>Describes the information about a task invocation for a particular target as part of a task execution performed as part of a Maintenance Window execution.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowExecutionTaskInvocationIdentity {
                #[doc="<p>The time the invocation finished.</p>"]
#[serde(rename="EndTime")]
pub end_time: Option<DateTime>,
#[doc="<p>The ID of the action performed in the service that actually handled the task invocation. If the task type is RUN_COMMAND, this value is the command ID.</p>"]
#[serde(rename="ExecutionId")]
pub execution_id: Option<MaintenanceWindowExecutionTaskExecutionId>,
#[doc="<p>The ID of the task invocation.</p>"]
#[serde(rename="InvocationId")]
pub invocation_id: Option<MaintenanceWindowExecutionTaskInvocationId>,
#[doc="<p>User-provided value that was specified when the target was registered with the Maintenance Window. This was also included in any CloudWatch events raised during the task invocation.</p>"]
#[serde(rename="OwnerInformation")]
pub owner_information: Option<OwnerInformation>,
#[doc="<p>The parameters that were provided for the invocation when it was executed.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<MaintenanceWindowExecutionTaskInvocationParameters>,
#[doc="<p>The time the invocation started.</p>"]
#[serde(rename="StartTime")]
pub start_time: Option<DateTime>,
#[doc="<p>The status of the task invocation.</p>"]
#[serde(rename="Status")]
pub status: Option<MaintenanceWindowExecutionStatus>,
#[doc="<p>The details explaining the status of the task invocation. Only available for certain Status values. </p>"]
#[serde(rename="StatusDetails")]
pub status_details: Option<MaintenanceWindowExecutionStatusDetails>,
#[doc="<p>The ID of the specific task execution in the Maintenance Window execution.</p>"]
#[serde(rename="TaskExecutionId")]
pub task_execution_id: Option<MaintenanceWindowExecutionTaskId>,
#[doc="<p>The ID of the Maintenance Window execution that ran the task.</p>"]
#[serde(rename="WindowExecutionId")]
pub window_execution_id: Option<MaintenanceWindowExecutionId>,
#[doc="<p>The ID of the target definition in this Maintenance Window the invocation was performed for.</p>"]
#[serde(rename="WindowTargetId")]
pub window_target_id: Option<MaintenanceWindowTaskTargetId>,
            }
            
pub type MaintenanceWindowExecutionTaskInvocationIdentityList = Vec<MaintenanceWindowExecutionTaskInvocationIdentity>;
pub type MaintenanceWindowExecutionTaskInvocationParameters = String;
#[doc="<p>Filter used in the request.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct MaintenanceWindowFilter {
                #[doc="<p>The name of the filter.</p>"]
#[serde(rename="Key")]
pub key: Option<MaintenanceWindowFilterKey>,
#[doc="<p>The filter values.</p>"]
#[serde(rename="Values")]
pub values: Option<MaintenanceWindowFilterValues>,
            }
            
pub type MaintenanceWindowFilterKey = String;
pub type MaintenanceWindowFilterList = Vec<MaintenanceWindowFilter>;
pub type MaintenanceWindowFilterValue = String;
pub type MaintenanceWindowFilterValues = Vec<MaintenanceWindowFilterValue>;
pub type MaintenanceWindowId = String;
#[doc="<p>Information about the Maintenance Window.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowIdentity {
                #[doc="<p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>"]
#[serde(rename="Cutoff")]
pub cutoff: Option<MaintenanceWindowCutoff>,
#[doc="<p>The duration of the Maintenance Window in hours.</p>"]
#[serde(rename="Duration")]
pub duration: Option<MaintenanceWindowDurationHours>,
#[doc="<p>Whether the Maintenance Window is enabled.</p>"]
#[serde(rename="Enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<MaintenanceWindowEnabled>,
#[doc="<p>The name of the Maintenance Window.</p>"]
#[serde(rename="Name")]
pub name: Option<MaintenanceWindowName>,
#[doc="<p>The ID of the Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
pub type MaintenanceWindowIdentityList = Vec<MaintenanceWindowIdentity>;
pub type MaintenanceWindowMaxResults = i64;
pub type MaintenanceWindowName = String;
pub type MaintenanceWindowResourceType = String;
pub type MaintenanceWindowSchedule = String;
#[doc="<p>The target registered with the Maintenance Window.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowTarget {
                #[doc="<p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.</p>"]
#[serde(rename="OwnerInformation")]
pub owner_information: Option<OwnerInformation>,
#[doc="<p>The type of target.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: Option<MaintenanceWindowResourceType>,
#[doc="<p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
#[doc="<p>The Maintenance Window ID where the target is registered.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
#[doc="<p>The ID of the target.</p>"]
#[serde(rename="WindowTargetId")]
pub window_target_id: Option<MaintenanceWindowTargetId>,
            }
            
pub type MaintenanceWindowTargetId = String;
pub type MaintenanceWindowTargetList = Vec<MaintenanceWindowTarget>;
#[doc="<p>Information about a task defined for a Maintenance Window.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct MaintenanceWindowTask {
                #[doc="<p>Information about an Amazon S3 bucket to write task-level logs to.</p>"]
#[serde(rename="LoggingInfo")]
pub logging_info: Option<LoggingInfo>,
#[doc="<p>The maximum number of targets this task can be run for in parallel.</p>"]
#[serde(rename="MaxConcurrency")]
pub max_concurrency: Option<MaxConcurrency>,
#[doc="<p>The maximum number of errors allowed before this task stops being scheduled.</p>"]
#[serde(rename="MaxErrors")]
pub max_errors: Option<MaxErrors>,
#[doc="<p>The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>"]
#[serde(rename="Priority")]
pub priority: Option<MaintenanceWindowTaskPriority>,
#[doc="<p>The role that should be assumed when executing the task</p>"]
#[serde(rename="ServiceRoleArn")]
pub service_role_arn: Option<ServiceRole>,
#[doc="<p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
#[doc="<p>The ARN of the task to execute.</p>"]
#[serde(rename="TaskArn")]
pub task_arn: Option<MaintenanceWindowTaskArn>,
#[doc="<p>The parameters that should be passed to the task when it is executed.</p>"]
#[serde(rename="TaskParameters")]
pub task_parameters: Option<MaintenanceWindowTaskParameters>,
#[doc="<p>The type of task.</p>"]
#[serde(rename="Type")]
pub type_: Option<MaintenanceWindowTaskType>,
#[doc="<p>The Maintenance Window ID where the task is registered.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
#[doc="<p>The task ID.</p>"]
#[serde(rename="WindowTaskId")]
pub window_task_id: Option<MaintenanceWindowTaskId>,
            }
            
pub type MaintenanceWindowTaskArn = String;
pub type MaintenanceWindowTaskId = String;
pub type MaintenanceWindowTaskList = Vec<MaintenanceWindowTask>;
pub type MaintenanceWindowTaskParameterName = String;
pub type MaintenanceWindowTaskParameterValue = String;
#[doc="<p>Defines the values for a task parameter.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct MaintenanceWindowTaskParameterValueExpression {
                #[doc="<p>This field contains an array of 0 or more strings, each 1 to 255 characters in length.</p>"]
#[serde(rename="Values")]
pub values: Option<MaintenanceWindowTaskParameterValueList>,
            }
            
pub type MaintenanceWindowTaskParameterValueList = Vec<MaintenanceWindowTaskParameterValue>;
pub type MaintenanceWindowTaskParameters = ::std::collections::HashMap<MaintenanceWindowTaskParameterName, MaintenanceWindowTaskParameterValueExpression>;
pub type MaintenanceWindowTaskParametersList = Vec<MaintenanceWindowTaskParameters>;
pub type MaintenanceWindowTaskPriority = i64;
pub type MaintenanceWindowTaskTargetId = String;
pub type MaintenanceWindowTaskType = String;
pub type ManagedInstanceId = String;
pub type MaxConcurrency = String;
pub type MaxErrors = String;
pub type MaxResults = i64;
pub type MaxResultsEC2Compatible = i64;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ModifyDocumentPermissionRequest {
                #[doc="<p>The AWS user accounts that should have access to the document. The account IDs can either be a group of account IDs or <i>All</i>.</p>"]
#[serde(rename="AccountIdsToAdd")]
pub account_ids_to_add: Option<AccountIdList>,
#[doc="<p>The AWS user accounts that should no longer have access to the document. The AWS user account can either be a group of account IDs or <i>All</i>. This action has a higher priority than <i>AccountIdsToAdd</i>. If you specify an account ID to add and the same ID to remove, the system removes access to the document.</p>"]
#[serde(rename="AccountIdsToRemove")]
pub account_ids_to_remove: Option<AccountIdList>,
#[doc="<p>The name of the document that you want to share.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
#[doc="<p>The permission type for the document. The permission type can be <i>Share</i>.</p>"]
#[serde(rename="PermissionType")]
pub permission_type: DocumentPermissionType,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ModifyDocumentPermissionResponse;
            
pub type NextToken = String;
pub type NormalStringMap = ::std::collections::HashMap<String, String>;
pub type NotificationArn = String;
#[doc="<p>Configurations for sending notifications.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct NotificationConfig {
                #[doc="<p>An Amazon Resource Name (ARN) for a Simple Notification Service (SNS) topic. Run Command pushes notifications about command status changes to this topic.</p>"]
#[serde(rename="NotificationArn")]
pub notification_arn: Option<NotificationArn>,
#[doc="<p>The different events for which you can receive notifications. These events include the following: All (events), InProgress, Success, TimedOut, Cancelled, Failed. To learn more about these events, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/monitor-commands.html\">Setting Up Events and Notifications</a> in the <i>Amazon EC2 Systems Manager User Guide</i>.</p>"]
#[serde(rename="NotificationEvents")]
pub notification_events: Option<NotificationEventList>,
#[doc="<p>Command: Receive notification when the status of a command changes. Invocation: For commands sent to multiple instances, receive notification on a per-instance basis when the status of a command changes. </p>"]
#[serde(rename="NotificationType")]
pub notification_type: Option<NotificationType>,
            }
            
pub type NotificationEvent = String;
pub type NotificationEventList = Vec<NotificationEvent>;
pub type NotificationType = String;
pub type OwnerInformation = String;
pub type PSParameterName = String;
pub type PSParameterValue = String;
#[doc="<p>An Amazon EC2 Systems Manager parameter in Parameter Store.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Parameter {
                #[doc="<p>The name of the parameter.</p>"]
#[serde(rename="Name")]
pub name: Option<PSParameterName>,
#[doc="<p>The type of parameter. Valid values include the following: String, String list, Secure string.</p>"]
#[serde(rename="Type")]
pub type_: Option<ParameterType>,
#[doc="<p>The parameter value.</p>"]
#[serde(rename="Value")]
pub value: Option<PSParameterValue>,
            }
            
pub type ParameterDescription = String;
#[doc="<p>Information about parameter usage.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ParameterHistory {
                #[doc="<p>Information about the parameter.</p>"]
#[serde(rename="Description")]
pub description: Option<ParameterDescription>,
#[doc="<p>The ID of the query key used for this parameter.</p>"]
#[serde(rename="KeyId")]
pub key_id: Option<ParameterKeyId>,
#[doc="<p>Date the parameter was last changed or updated.</p>"]
#[serde(rename="LastModifiedDate")]
pub last_modified_date: Option<DateTime>,
#[doc="<p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>"]
#[serde(rename="LastModifiedUser")]
pub last_modified_user: Option<String>,
#[doc="<p>The name of the parameter.</p>"]
#[serde(rename="Name")]
pub name: Option<PSParameterName>,
#[doc="<p>The type of parameter used.</p>"]
#[serde(rename="Type")]
pub type_: Option<ParameterType>,
#[doc="<p>The parameter value.</p>"]
#[serde(rename="Value")]
pub value: Option<PSParameterValue>,
            }
            
pub type ParameterHistoryList = Vec<ParameterHistory>;
pub type ParameterKeyId = String;
pub type ParameterList = Vec<Parameter>;
#[doc="<p>Metada includes information like the ARN of the last user and the date/time the parameter was last used.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct ParameterMetadata {
                #[doc="<p>Description of the parameter actions.</p>"]
#[serde(rename="Description")]
pub description: Option<ParameterDescription>,
#[doc="<p>The ID of the query key used for this parameter.</p>"]
#[serde(rename="KeyId")]
pub key_id: Option<ParameterKeyId>,
#[doc="<p>Date the parameter was last changed or updated.</p>"]
#[serde(rename="LastModifiedDate")]
pub last_modified_date: Option<DateTime>,
#[doc="<p>Amazon Resource Name (ARN) of the AWS user who last changed the parameter.</p>"]
#[serde(rename="LastModifiedUser")]
pub last_modified_user: Option<String>,
#[doc="<p>The parameter name.</p>"]
#[serde(rename="Name")]
pub name: Option<PSParameterName>,
#[doc="<p>The type of parameter. Valid parameter types include the following: String, String list, Secure string.</p>"]
#[serde(rename="Type")]
pub type_: Option<ParameterType>,
            }
            
pub type ParameterMetadataList = Vec<ParameterMetadata>;
pub type ParameterName = String;
pub type ParameterNameList = Vec<PSParameterName>;
pub type ParameterType = String;
pub type ParameterValue = String;
pub type ParameterValueList = Vec<ParameterValue>;
pub type Parameters = ::std::collections::HashMap<ParameterName, ParameterValueList>;
#[doc="<p>One or more filters. Use a filter to return a more specific list of results.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ParametersFilter {
                #[doc="<p>The name of the filter.</p>"]
#[serde(rename="Key")]
pub key: Option<ParametersFilterKey>,
#[doc="<p>The filter values.</p>"]
#[serde(rename="Values")]
pub values: ParametersFilterValueList,
            }
            
pub type ParametersFilterKey = String;
pub type ParametersFilterList = Vec<ParametersFilter>;
pub type ParametersFilterValue = String;
pub type ParametersFilterValueList = Vec<ParametersFilterValue>;
#[doc="<p>Represents metadata about a patch.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct Patch {
                #[doc="<p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>"]
#[serde(rename="Classification")]
pub classification: Option<PatchClassification>,
#[doc="<p>The URL where more information can be obtained about the patch.</p>"]
#[serde(rename="ContentUrl")]
pub content_url: Option<PatchContentUrl>,
#[doc="<p>The description of the patch.</p>"]
#[serde(rename="Description")]
pub description: Option<PatchDescription>,
#[doc="<p>The ID of the patch (this is different than the Microsoft Knowledge Base ID).</p>"]
#[serde(rename="Id")]
pub id: Option<PatchId>,
#[doc="<p>The Microsoft Knowledge Base ID of the patch.</p>"]
#[serde(rename="KbNumber")]
pub kb_number: Option<PatchKbNumber>,
#[doc="<p>The language of the patch if it’s language-specific.</p>"]
#[serde(rename="Language")]
pub language: Option<PatchLanguage>,
#[doc="<p>The ID of the MSRC bulletin the patch is related to.</p>"]
#[serde(rename="MsrcNumber")]
pub msrc_number: Option<PatchMsrcNumber>,
#[doc="<p>The severity of the patch (for example Critical, Important, Moderate).</p>"]
#[serde(rename="MsrcSeverity")]
pub msrc_severity: Option<PatchMsrcSeverity>,
#[doc="<p>The specific product the patch is applicable for (for example, WindowsServer2016).</p>"]
#[serde(rename="Product")]
pub product: Option<PatchProduct>,
#[doc="<p>The product family the patch is applicable for (for example, Windows).</p>"]
#[serde(rename="ProductFamily")]
pub product_family: Option<PatchProductFamily>,
#[doc="<p>The date the patch was released.</p>"]
#[serde(rename="ReleaseDate")]
pub release_date: Option<DateTime>,
#[doc="<p>The title of the patch.</p>"]
#[serde(rename="Title")]
pub title: Option<PatchTitle>,
#[doc="<p>The name of the vendor providing the patch.</p>"]
#[serde(rename="Vendor")]
pub vendor: Option<PatchVendor>,
            }
            
#[doc="<p>Defines the basic information about a patch baseline.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PatchBaselineIdentity {
                #[doc="<p>The description of the patch baseline.</p>"]
#[serde(rename="BaselineDescription")]
pub baseline_description: Option<BaselineDescription>,
#[doc="<p>The ID of the patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The name of the patch baseline.</p>"]
#[serde(rename="BaselineName")]
pub baseline_name: Option<BaselineName>,
#[doc="<p>Whether this is the default baseline.</p>"]
#[serde(rename="DefaultBaseline")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub default_baseline: Option<DefaultBaseline>,
            }
            
pub type PatchBaselineIdentityList = Vec<PatchBaselineIdentity>;
pub type PatchBaselineMaxResults = i64;
pub type PatchClassification = String;
#[doc="<p>Information about the state of a patch on a particular instance as it relates to the patch baseline used to patch the instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PatchComplianceData {
                #[doc="<p>The classification of the patch (for example, SecurityUpdates, Updates, CriticalUpdates).</p>"]
#[serde(rename="Classification")]
pub classification: PatchClassification,
#[doc="<p>The date/time the patch was installed on the instance.</p>"]
#[serde(rename="InstalledTime")]
pub installed_time: PatchInstalledTime,
#[doc="<p>The Microsoft Knowledge Base ID of the patch.</p>"]
#[serde(rename="KBId")]
pub kb_id: PatchKbNumber,
#[doc="<p>The severity of the patch (for example, Critical, Important, Moderate).</p>"]
#[serde(rename="Severity")]
pub severity: PatchSeverity,
#[doc="<p>The state of the patch on the instance (INSTALLED, INSTALLED_OTHER, MISSING, NOT_APPLICABLE or FAILED).</p>"]
#[serde(rename="State")]
pub state: PatchComplianceDataState,
#[doc="<p>The title of the patch.</p>"]
#[serde(rename="Title")]
pub title: PatchTitle,
            }
            
pub type PatchComplianceDataList = Vec<PatchComplianceData>;
pub type PatchComplianceDataState = String;
pub type PatchComplianceMaxResults = i64;
pub type PatchContentUrl = String;
pub type PatchDeploymentStatus = String;
pub type PatchDescription = String;
pub type PatchFailedCount = i64;
#[doc="<p>Defines a patch filter.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PatchFilter {
                #[doc="<p>The key for the filter (PRODUCT, CLASSIFICATION, MSRC_SEVERITY, PATCH_ID)</p>"]
#[serde(rename="Key")]
pub key: PatchFilterKey,
#[doc="<p>The value for the filter key.</p>"]
#[serde(rename="Values")]
pub values: PatchFilterValueList,
            }
            
#[doc="<p>A set of patch filters, typically used for approval rules.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PatchFilterGroup {
                #[doc="<p>The set of patch filters that make up the group.</p>"]
#[serde(rename="PatchFilters")]
pub patch_filters: PatchFilterList,
            }
            
pub type PatchFilterKey = String;
pub type PatchFilterList = Vec<PatchFilter>;
pub type PatchFilterValue = String;
pub type PatchFilterValueList = Vec<PatchFilterValue>;
pub type PatchGroup = String;
pub type PatchGroupList = Vec<PatchGroup>;
#[doc="<p>The mapping between a patch group and the patch baseline the patch group is registered with.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PatchGroupPatchBaselineMapping {
                #[doc="<p>The patch baseline the patch group is registered with.</p>"]
#[serde(rename="BaselineIdentity")]
pub baseline_identity: Option<PatchBaselineIdentity>,
#[doc="<p>The name of the patch group registered with the patch baseline.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: Option<PatchGroup>,
            }
            
pub type PatchGroupPatchBaselineMappingList = Vec<PatchGroupPatchBaselineMapping>;
pub type PatchId = String;
pub type PatchIdList = Vec<PatchId>;
pub type PatchInstalledCount = i64;
pub type PatchInstalledOtherCount = i64;
pub type PatchInstalledTime = f64;
pub type PatchKbNumber = String;
pub type PatchLanguage = String;
pub type PatchList = Vec<Patch>;
pub type PatchMissingCount = i64;
pub type PatchMsrcNumber = String;
pub type PatchMsrcSeverity = String;
pub type PatchNotApplicableCount = i64;
pub type PatchOperationEndTime = f64;
pub type PatchOperationStartTime = f64;
pub type PatchOperationType = String;
#[doc="<p>Defines a filter used in Patch Manager APIs.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PatchOrchestratorFilter {
                #[doc="<p>The key for the filter.</p>"]
#[serde(rename="Key")]
pub key: Option<PatchOrchestratorFilterKey>,
#[doc="<p>The value for the filter.</p>"]
#[serde(rename="Values")]
pub values: Option<PatchOrchestratorFilterValues>,
            }
            
pub type PatchOrchestratorFilterKey = String;
pub type PatchOrchestratorFilterList = Vec<PatchOrchestratorFilter>;
pub type PatchOrchestratorFilterValue = String;
pub type PatchOrchestratorFilterValues = Vec<PatchOrchestratorFilterValue>;
pub type PatchProduct = String;
pub type PatchProductFamily = String;
#[doc="<p>Defines an approval rule for a patch baseline.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PatchRule {
                #[doc="<p>The number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.</p>"]
#[serde(rename="ApproveAfterDays")]
pub approve_after_days: ApproveAfterDays,
#[doc="<p>The patch filter group that defines the criteria for the rule.</p>"]
#[serde(rename="PatchFilterGroup")]
pub patch_filter_group: PatchFilterGroup,
            }
            
#[doc="<p>A set of rules defining the approval rules for a patch baseline.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct PatchRuleGroup {
                #[doc="<p>The rules that make up the rule group.</p>"]
#[serde(rename="PatchRules")]
pub patch_rules: PatchRuleList,
            }
            
pub type PatchRuleList = Vec<PatchRule>;
pub type PatchSeverity = String;
#[doc="<p>Information about the approval status of a patch.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PatchStatus {
                #[doc="<p>The date the patch was approved (or will be approved if the status is PENDING_APPROVAL).</p>"]
#[serde(rename="ApprovalDate")]
pub approval_date: Option<DateTime>,
#[doc="<p>The approval status of a patch (APPROVED, PENDING_APPROVAL, EXPLICIT_APPROVED, EXPLICIT_REJECTED).</p>"]
#[serde(rename="DeploymentStatus")]
pub deployment_status: Option<PatchDeploymentStatus>,
            }
            
pub type PatchTitle = String;
pub type PatchVendor = String;
pub type PingStatus = String;
pub type PlatformType = String;
pub type PlatformTypeList = Vec<PlatformType>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutInventoryRequest {
                #[doc="<p>One or more instance IDs where you want to add or update inventory items.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The inventory items that you want to add or update on instances.</p>"]
#[serde(rename="Items")]
pub items: InventoryItemList,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutInventoryResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct PutParameterRequest {
                #[doc="<p>Information about the parameter that you want to add to the system</p>"]
#[serde(rename="Description")]
pub description: Option<ParameterDescription>,
#[doc="<p>The parameter key ID that you want to add to the system.</p>"]
#[serde(rename="KeyId")]
pub key_id: Option<ParameterKeyId>,
#[doc="<p>The name of the parameter that you want to add to the system.</p>"]
#[serde(rename="Name")]
pub name: PSParameterName,
#[doc="<p>Overwrite an existing parameter. If not specified, will default to \"false\".</p>"]
#[serde(rename="Overwrite")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub overwrite: Option<Boolean>,
#[doc="<p>The type of parameter that you want to add to the system.</p>"]
#[serde(rename="Type")]
pub type_: ParameterType,
#[doc="<p>The parameter value that you want to add to the system.</p>"]
#[serde(rename="Value")]
pub value: PSParameterValue,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct PutParameterResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterDefaultPatchBaselineRequest {
                #[doc="<p>The ID of the patch baseline that should be the default patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RegisterDefaultPatchBaselineResult {
                #[doc="<p>The ID of the default patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterPatchBaselineForPatchGroupRequest {
                #[doc="<p>The ID of the patch baseline to register the patch group with.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
#[doc="<p>The name of the patch group that should be registered with the patch baseline.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: PatchGroup,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RegisterPatchBaselineForPatchGroupResult {
                #[doc="<p>The ID of the patch baseline the patch group was registered with.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The name of the patch group registered with the patch baseline.</p>"]
#[serde(rename="PatchGroup")]
pub patch_group: Option<PatchGroup>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterTargetWithMaintenanceWindowRequest {
                #[doc="<p>User-provided idempotency token.</p>"]
#[serde(rename="ClientToken")]
pub client_token: Option<ClientToken>,
#[doc="<p>User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.</p>"]
#[serde(rename="OwnerInformation")]
pub owner_information: Option<OwnerInformation>,
#[doc="<p>The type of target being registered with the Maintenance Window.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: MaintenanceWindowResourceType,
#[doc="<p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>"]
#[serde(rename="Targets")]
pub targets: Targets,
#[doc="<p>The ID of the Maintenance Window the target should be registered with.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RegisterTargetWithMaintenanceWindowResult {
                #[doc="<p>The ID of the target definition in this Maintenance Window.</p>"]
#[serde(rename="WindowTargetId")]
pub window_target_id: Option<MaintenanceWindowTargetId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RegisterTaskWithMaintenanceWindowRequest {
                #[doc="<p>User-provided idempotency token.</p>"]
#[serde(rename="ClientToken")]
pub client_token: Option<ClientToken>,
#[doc="<p>A structure containing information about an Amazon S3 bucket to write instance-level logs to. </p>"]
#[serde(rename="LoggingInfo")]
pub logging_info: Option<LoggingInfo>,
#[doc="<p>The maximum number of targets this task can be run for in parallel.</p>"]
#[serde(rename="MaxConcurrency")]
pub max_concurrency: MaxConcurrency,
#[doc="<p>The maximum number of errors allowed before this task stops being scheduled.</p>"]
#[serde(rename="MaxErrors")]
pub max_errors: MaxErrors,
#[doc="<p>The priority of the task in the Maintenance Window, the lower the number the higher the priority. Tasks in a Maintenance Window are scheduled in priority order with tasks that have the same priority scheduled in parallel.</p>"]
#[serde(rename="Priority")]
pub priority: Option<MaintenanceWindowTaskPriority>,
#[doc="<p>The role that should be assumed when executing the task.</p>"]
#[serde(rename="ServiceRoleArn")]
pub service_role_arn: ServiceRole,
#[doc="<p>The targets (either instances or tags). Instances are specified using Key=instanceids,Values=&lt;instanceid1&gt;,&lt;instanceid2&gt;. Tags are specified using Key=&lt;tag name&gt;,Values=&lt;tag value&gt;.</p>"]
#[serde(rename="Targets")]
pub targets: Targets,
#[doc="<p>The ARN of the task to execute </p>"]
#[serde(rename="TaskArn")]
pub task_arn: MaintenanceWindowTaskArn,
#[doc="<p>The parameters that should be passed to the task when it is executed.</p>"]
#[serde(rename="TaskParameters")]
pub task_parameters: Option<MaintenanceWindowTaskParameters>,
#[doc="<p>The type of task being registered.</p>"]
#[serde(rename="TaskType")]
pub task_type: MaintenanceWindowTaskType,
#[doc="<p>The id of the Maintenance Window the task should be added to.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RegisterTaskWithMaintenanceWindowResult {
                #[doc="<p>The id of the task in the Maintenance Window.</p>"]
#[serde(rename="WindowTaskId")]
pub window_task_id: Option<MaintenanceWindowTaskId>,
            }
            
pub type RegistrationLimit = i64;
pub type RegistrationsCount = i64;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct RemoveTagsFromResourceRequest {
                #[doc="<p>The resource ID for which you want to remove tags.</p>"]
#[serde(rename="ResourceId")]
pub resource_id: ResourceId,
#[doc="<p>The type of resource of which you want to remove a tag.</p>"]
#[serde(rename="ResourceType")]
pub resource_type: ResourceTypeForTagging,
#[doc="<p>Tag keys that you want to remove from the specified resource.</p>"]
#[serde(rename="TagKeys")]
pub tag_keys: KeyList,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct RemoveTagsFromResourceResult;
            
pub type ResourceId = String;
pub type ResourceType = String;
pub type ResourceTypeForTagging = String;
pub type ResponseCode = i64;
#[doc="<p>The inventory item result attribute.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
            pub struct ResultAttribute {
                #[doc="<p>Name of the inventory item type. Valid value: “AWS:InstanceInformation”. Default Value: “AWS:InstanceInformation”.</p>"]
#[serde(rename="TypeName")]
pub type_name: InventoryItemTypeName,
            }
            
pub type ResultAttributeList = Vec<ResultAttribute>;
pub type S3BucketName = String;
pub type S3KeyPrefix = String;
#[doc="<p>An Amazon S3 bucket where you want to store the results of this request.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct S3OutputLocation {
                #[doc="<p>The name of the Amazon S3 bucket.</p>"]
#[serde(rename="OutputS3BucketName")]
pub output_s3_bucket_name: Option<S3BucketName>,
#[doc="<p>The Amazon S3 bucket subfolder.</p>"]
#[serde(rename="OutputS3KeyPrefix")]
pub output_s3_key_prefix: Option<S3KeyPrefix>,
#[doc="<p>The Amazon S3 region where the association information is stored.</p>"]
#[serde(rename="OutputS3Region")]
pub output_s3_region: Option<S3Region>,
            }
            
#[doc="<p>A URL for the Amazon S3 bucket where you want to store the results of this request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct S3OutputUrl {
                #[doc="<p>A URL for an Amazon S3 bucket where you want to store the results of this request.</p>"]
#[serde(rename="OutputUrl")]
pub output_url: Option<Url>,
            }
            
pub type S3Region = String;
pub type ScheduleExpression = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct SendCommandRequest {
                #[doc="<p>User-specified information about the command, such as a brief description of what the command should do.</p>"]
#[serde(rename="Comment")]
pub comment: Option<Comment>,
#[doc="<p>The Sha256 or Sha1 hash created by the system when the document was created. </p> <note> <p>Sha1 hashes have been deprecated.</p> </note>"]
#[serde(rename="DocumentHash")]
pub document_hash: Option<DocumentHash>,
#[doc="<p>Sha256 or Sha1.</p> <note> <p>Sha1 hashes have been deprecated.</p> </note>"]
#[serde(rename="DocumentHashType")]
pub document_hash_type: Option<DocumentHashType>,
#[doc="<p>Required. The name of the Systems Manager document to execute. This can be a public document or a custom document.</p>"]
#[serde(rename="DocumentName")]
pub document_name: DocumentARN,
#[doc="<p>Required. The instance IDs where the command should execute. You can specify a maximum of 50 IDs.</p>"]
#[serde(rename="InstanceIds")]
pub instance_ids: Option<InstanceIdList>,
#[doc="<p>(Optional) The maximum number of instances that are allowed to execute the command at the same time. You can specify a number such as “10” or a percentage such as “10%”. The default value is 50. For more information about how to use <code>MaxConcurrency</code>, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="MaxConcurrency")]
pub max_concurrency: Option<MaxConcurrency>,
#[doc="<p>The maximum number of errors allowed without the command failing. When the command fails one more time beyond the value of <code>MaxErrors</code>, the systems stops sending the command to additional targets. You can specify a number like “10” or a percentage like “10%”. The default value is 50. For more information about how to use <code>MaxErrors</code>, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="MaxErrors")]
pub max_errors: Option<MaxErrors>,
#[doc="<p>Configurations for sending notifications.</p>"]
#[serde(rename="NotificationConfig")]
pub notification_config: Option<NotificationConfig>,
#[doc="<p>The name of the S3 bucket where command execution responses should be stored.</p>"]
#[serde(rename="OutputS3BucketName")]
pub output_s3_bucket_name: Option<S3BucketName>,
#[doc="<p>The directory structure within the S3 bucket where the responses should be stored.</p>"]
#[serde(rename="OutputS3KeyPrefix")]
pub output_s3_key_prefix: Option<S3KeyPrefix>,
#[doc="<p>(Optional) The region where the Amazon Simple Storage Service (Amazon S3) output bucket is located. The default value is the region where Run Command is being called.</p>"]
#[serde(rename="OutputS3Region")]
pub output_s3_region: Option<S3Region>,
#[doc="<p>The required and optional parameters specified in the document being executed.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>The IAM role that Systems Manager uses to send notifications. </p>"]
#[serde(rename="ServiceRoleArn")]
pub service_role_arn: Option<ServiceRole>,
#[doc="<p>(Optional) An array of search criteria that targets instances using a <code>Key</code>,<code>Value</code> combination that you specify. <code>Targets</code> is required if you don't provide one or more instance IDs in the call. For more information about how to use <code>Targets</code>, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="Targets")]
pub targets: Option<Targets>,
#[doc="<p>If this time is reached and the command has not already started executing, it will not execute.</p>"]
#[serde(rename="TimeoutSeconds")]
pub timeout_seconds: Option<TimeoutSeconds>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct SendCommandResult {
                #[doc="<p>The request as it was received by Systems Manager. Also provides the command ID which can be used future references to this request.</p>"]
#[serde(rename="Command")]
pub command: Option<Command>,
            }
            
pub type ServiceRole = String;
pub type SnapshotDownloadUrl = String;
pub type SnapshotId = String;
pub type StandardErrorContent = String;
pub type StandardOutputContent = String;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct StartAutomationExecutionRequest {
                #[doc="<p>The name of the Automation document to use for this execution.</p>"]
#[serde(rename="DocumentName")]
pub document_name: DocumentARN,
#[doc="<p>The version of the Automation document to use for this execution.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>A key-value map of execution parameters, which match the declared parameters in the Automation document.</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<AutomationParameterMap>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StartAutomationExecutionResult {
                #[doc="<p>The unique ID of a newly scheduled automation execution.</p>"]
#[serde(rename="AutomationExecutionId")]
pub automation_execution_id: Option<AutomationExecutionId>,
            }
            
pub type StatusAdditionalInfo = String;
pub type StatusDetails = String;
pub type StatusMessage = String;
pub type StatusName = String;
#[doc="<p>Detailed information about an the execution state of an Automation step.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StepExecution {
                #[doc="<p>The action this step performs. The action determines the behavior of the step.</p>"]
#[serde(rename="Action")]
pub action: Option<AutomationActionName>,
#[doc="<p>If a step has finished execution, this contains the time the execution ended. If the step has not yet concluded, this field is not populated.</p>"]
#[serde(rename="ExecutionEndTime")]
pub execution_end_time: Option<DateTime>,
#[doc="<p>If a step has begun execution, this contains the time the step started. If the step is in <code>Pending</code> status, this field is not populated.</p>"]
#[serde(rename="ExecutionStartTime")]
pub execution_start_time: Option<DateTime>,
#[doc="<p>If a step failed, this message explains why the execution failed.</p>"]
#[serde(rename="FailureMessage")]
pub failure_message: Option<String>,
#[doc="<p>Fully-resolved values passed into the step before execution.</p>"]
#[serde(rename="Inputs")]
pub inputs: Option<NormalStringMap>,
#[doc="<p>Returned values from the execution of the step.</p>"]
#[serde(rename="Outputs")]
pub outputs: Option<AutomationParameterMap>,
#[doc="<p>A message associated with the response code for an execution.</p>"]
#[serde(rename="Response")]
pub response: Option<String>,
#[doc="<p>The response code returned by the execution of the step.</p>"]
#[serde(rename="ResponseCode")]
pub response_code: Option<String>,
#[doc="<p>The name of this execution step.</p>"]
#[serde(rename="StepName")]
pub step_name: Option<String>,
#[doc="<p>The execution status for this step. Valid values include: <code>Pending</code>, <code>InProgress</code>, <code>Success</code>, <code>Cancelled</code>, <code>Failed</code>, and <code>TimedOut</code>.</p>"]
#[serde(rename="StepStatus")]
pub step_status: Option<AutomationExecutionStatus>,
            }
            
pub type StepExecutionList = Vec<StepExecution>;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct StopAutomationExecutionRequest {
                #[doc="<p>The execution ID of the Automation to stop.</p>"]
#[serde(rename="AutomationExecutionId")]
pub automation_execution_id: AutomationExecutionId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct StopAutomationExecutionResult;
            
pub type StringDateTime = String;
pub type StringList = Vec<String>;
#[doc="<p>Metadata that you assign to your managed instances. Tags enable you to categorize your managed instances in different ways, for example, by purpose, owner, or environment.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Tag {
                #[doc="<p>The name of the tag.</p>"]
#[serde(rename="Key")]
pub key: TagKey,
#[doc="<p>The value of the tag.</p>"]
#[serde(rename="Value")]
pub value: TagValue,
            }
            
pub type TagKey = String;
pub type TagList = Vec<Tag>;
pub type TagValue = String;
#[doc="<p>An array of search criteria that targets instances using a <code>Key</code>,<code>Value</code> combination that you specify. <code>Targets</code> is required if you don't provide one or more instance IDs in the call.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
            pub struct Target {
                #[doc="<p>User-defined criteria for sending commands that target instances that meet the criteria. <code>Key</code> can be <code>tag:&lt;Amazon EC2 tag&gt;</code> or <code>InstanceIds</code>. For more information about how to send commands that target instances using <code>Key</code>,<code>Value</code> parameters, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="Key")]
pub key: Option<TargetKey>,
#[doc="<p>User-defined criteria that maps to <code>Key</code>. For example, if you specified <code>tag:ServerRole</code>, you could specify <code>value:WebServer</code> to execute a command on instances that include Amazon EC2 tags of ServerRole,WebServer. For more information about how to send commands that target instances using <code>Key</code>,<code>Value</code> parameters, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/run-command.html\">Executing a Command Using Systems Manager Run Command</a>.</p>"]
#[serde(rename="Values")]
pub values: Option<TargetValues>,
            }
            
pub type TargetCount = i64;
pub type TargetKey = String;
pub type TargetValue = String;
pub type TargetValues = Vec<TargetValue>;
pub type Targets = Vec<Target>;
pub type TimeoutSeconds = i64;
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateAssociationRequest {
                #[doc="<p>The ID of the association you want to update. </p>"]
#[serde(rename="AssociationId")]
pub association_id: AssociationId,
#[doc="<p>The document version you want update for the association. </p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>An Amazon S3 bucket where you want to store the results of this request.</p> <p> <code>\"{ \\\"S3Location\\\": { \\\"OutputS3Region\\\": \\\"&lt;region&gt;\\\", \\\"OutputS3BucketName\\\": \\\"bucket name\\\", \\\"OutputS3KeyPrefix\\\": \\\"folder name\\\" } }\"</code> </p>"]
#[serde(rename="OutputLocation")]
pub output_location: Option<InstanceAssociationOutputLocation>,
#[doc="<p>The parameters you want to update for the association. If you create a parameter using Parameter Store, you can reference the parameter using {{ssm:parameter-name}}</p>"]
#[serde(rename="Parameters")]
pub parameters: Option<Parameters>,
#[doc="<p>The cron expression used to schedule the association that you want to update. Supported expressions are every half, 1, 2, 4, 8 or 12 hour(s); every specified day and time of the week. For example: cron(0 0/30 * 1/1 * ? *) to run every thirty minutes; cron(0 0 0/4 1/1 * ? *) to run every four hours; and cron(0 0 10 ? * SUN *) to run every Sunday at 10 a.m.</p>"]
#[serde(rename="ScheduleExpression")]
pub schedule_expression: Option<ScheduleExpression>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateAssociationResult {
                #[doc="<p>The description of the association that was updated.</p>"]
#[serde(rename="AssociationDescription")]
pub association_description: Option<AssociationDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateAssociationStatusRequest {
                #[doc="<p>The association status.</p>"]
#[serde(rename="AssociationStatus")]
pub association_status: AssociationStatus,
#[doc="<p>The ID of the instance.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: InstanceId,
#[doc="<p>The name of the SSM document.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateAssociationStatusResult {
                #[doc="<p>Information about the association.</p>"]
#[serde(rename="AssociationDescription")]
pub association_description: Option<AssociationDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateDocumentDefaultVersionRequest {
                #[doc="<p>The version of a custom document that you want to set as the default version.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: DocumentVersionNumber,
#[doc="<p>The name of a custom document that you want to set as the default version.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateDocumentDefaultVersionResult {
                #[doc="<p>The description of a custom document that you want to set as the default version.</p>"]
#[serde(rename="Description")]
pub description: Option<DocumentDefaultVersionDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateDocumentRequest {
                #[doc="<p>The content in a document that you want to update.</p>"]
#[serde(rename="Content")]
pub content: DocumentContent,
#[doc="<p>The version of the document that you want to update.</p>"]
#[serde(rename="DocumentVersion")]
pub document_version: Option<DocumentVersion>,
#[doc="<p>The name of the document that you want to update.</p>"]
#[serde(rename="Name")]
pub name: DocumentName,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateDocumentResult {
                #[doc="<p>A description of the document that was updated.</p>"]
#[serde(rename="DocumentDescription")]
pub document_description: Option<DocumentDescription>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateMaintenanceWindowRequest {
                #[doc="<p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>"]
#[serde(rename="AllowUnassociatedTargets")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub allow_unassociated_targets: Option<MaintenanceWindowAllowUnassociatedTargets>,
#[doc="<p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>"]
#[serde(rename="Cutoff")]
pub cutoff: Option<MaintenanceWindowCutoff>,
#[doc="<p>The duration of the Maintenance Window in hours.</p>"]
#[serde(rename="Duration")]
pub duration: Option<MaintenanceWindowDurationHours>,
#[doc="<p>Whether the Maintenance Window is enabled.</p>"]
#[serde(rename="Enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<MaintenanceWindowEnabled>,
#[doc="<p>The name of the Maintenance Window.</p>"]
#[serde(rename="Name")]
pub name: Option<MaintenanceWindowName>,
#[doc="<p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>"]
#[serde(rename="Schedule")]
pub schedule: Option<MaintenanceWindowSchedule>,
#[doc="<p>The ID of the Maintenance Window to update.</p>"]
#[serde(rename="WindowId")]
pub window_id: MaintenanceWindowId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateMaintenanceWindowResult {
                #[doc="<p>Whether targets must be registered with the Maintenance Window before tasks can be defined for those targets.</p>"]
#[serde(rename="AllowUnassociatedTargets")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub allow_unassociated_targets: Option<MaintenanceWindowAllowUnassociatedTargets>,
#[doc="<p>The number of hours before the end of the Maintenance Window that Systems Manager stops scheduling new tasks for execution.</p>"]
#[serde(rename="Cutoff")]
pub cutoff: Option<MaintenanceWindowCutoff>,
#[doc="<p>The duration of the Maintenance Window in hours.</p>"]
#[serde(rename="Duration")]
pub duration: Option<MaintenanceWindowDurationHours>,
#[doc="<p>Whether the Maintenance Window is enabled.</p>"]
#[serde(rename="Enabled")]
#[serde(skip_serializing_if="::std::option::Option::is_none")]
pub enabled: Option<MaintenanceWindowEnabled>,
#[doc="<p>The name of the Maintenance Window.</p>"]
#[serde(rename="Name")]
pub name: Option<MaintenanceWindowName>,
#[doc="<p>The schedule of the Maintenance Window in the form of a cron or rate expression.</p>"]
#[serde(rename="Schedule")]
pub schedule: Option<MaintenanceWindowSchedule>,
#[doc="<p>The ID of the created Maintenance Window.</p>"]
#[serde(rename="WindowId")]
pub window_id: Option<MaintenanceWindowId>,
            }
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdateManagedInstanceRoleRequest {
                #[doc="<p>The IAM role you want to assign or change.</p>"]
#[serde(rename="IamRole")]
pub iam_role: IamRole,
#[doc="<p>The ID of the managed instance where you want to update the role.</p>"]
#[serde(rename="InstanceId")]
pub instance_id: ManagedInstanceId,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdateManagedInstanceRoleResult;
            
#[derive(Default,Debug,Clone,Serialize)]
            pub struct UpdatePatchBaselineRequest {
                #[doc="<p>A set of rules used to include patches in the baseline.</p>"]
#[serde(rename="ApprovalRules")]
pub approval_rules: Option<PatchRuleGroup>,
#[doc="<p>A list of explicitly approved patches for the baseline.</p>"]
#[serde(rename="ApprovedPatches")]
pub approved_patches: Option<PatchIdList>,
#[doc="<p>The ID of the patch baseline to update.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: BaselineId,
#[doc="<p>A description of the patch baseline.</p>"]
#[serde(rename="Description")]
pub description: Option<BaselineDescription>,
#[doc="<p>A set of global filters used to exclude patches from the baseline.</p>"]
#[serde(rename="GlobalFilters")]
pub global_filters: Option<PatchFilterGroup>,
#[doc="<p>The name of the patch baseline.</p>"]
#[serde(rename="Name")]
pub name: Option<BaselineName>,
#[doc="<p>A list of explicitly rejected patches for the baseline.</p>"]
#[serde(rename="RejectedPatches")]
pub rejected_patches: Option<PatchIdList>,
            }
            
#[derive(Default,Debug,Clone,Deserialize)]
            pub struct UpdatePatchBaselineResult {
                #[doc="<p>A set of rules used to include patches in the baseline.</p>"]
#[serde(rename="ApprovalRules")]
pub approval_rules: Option<PatchRuleGroup>,
#[doc="<p>A list of explicitly approved patches for the baseline.</p>"]
#[serde(rename="ApprovedPatches")]
pub approved_patches: Option<PatchIdList>,
#[doc="<p>The ID of the deleted patch baseline.</p>"]
#[serde(rename="BaselineId")]
pub baseline_id: Option<BaselineId>,
#[doc="<p>The date when the patch baseline was created.</p>"]
#[serde(rename="CreatedDate")]
pub created_date: Option<DateTime>,
#[doc="<p>A description of the Patch Baseline.</p>"]
#[serde(rename="Description")]
pub description: Option<BaselineDescription>,
#[doc="<p>A set of global filters used to exclude patches from the baseline.</p>"]
#[serde(rename="GlobalFilters")]
pub global_filters: Option<PatchFilterGroup>,
#[doc="<p>The date when the patch baseline was last modified.</p>"]
#[serde(rename="ModifiedDate")]
pub modified_date: Option<DateTime>,
#[doc="<p>The name of the patch baseline.</p>"]
#[serde(rename="Name")]
pub name: Option<BaselineName>,
#[doc="<p>A list of explicitly rejected patches for the baseline.</p>"]
#[serde(rename="RejectedPatches")]
pub rejected_patches: Option<PatchIdList>,
            }
            
pub type Url = String;
pub type Version = String;
/// Errors returned by AddTagsToResource
                #[derive(Debug, PartialEq)]
                pub enum AddTagsToResourceError {
                    
///<p>The resource type is not valid. If you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
InvalidResourceType(String),
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The <code>Targets</code> parameter includes too many tags. Remove one or more tags and try the command again.</p>
TooManyTagsError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AddTagsToResourceError {
                    pub fn from_body(body: &str) -> AddTagsToResourceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceType" => AddTagsToResourceError::InvalidResourceType(String::from(error_message)),"InvalidResourceId" => AddTagsToResourceError::InvalidResourceId(String::from(error_message)),"InternalServerError" => AddTagsToResourceError::InternalServerError(String::from(error_message)),"TooManyTagsError" => AddTagsToResourceError::TooManyTagsError(String::from(error_message)),"ValidationException" => AddTagsToResourceError::Validation(error_message.to_string()),_ => AddTagsToResourceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => AddTagsToResourceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for AddTagsToResourceError {
                    fn from(err: serde_json::error::Error) -> AddTagsToResourceError {
                        AddTagsToResourceError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for AddTagsToResourceError {
                    fn from(err: CredentialsError) -> AddTagsToResourceError {
                        AddTagsToResourceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AddTagsToResourceError {
                    fn from(err: HttpDispatchError) -> AddTagsToResourceError {
                        AddTagsToResourceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AddTagsToResourceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AddTagsToResourceError {
                    fn description(&self) -> &str {
                        match *self {
                            AddTagsToResourceError::InvalidResourceType(ref cause) => cause,AddTagsToResourceError::InvalidResourceId(ref cause) => cause,AddTagsToResourceError::InternalServerError(ref cause) => cause,AddTagsToResourceError::TooManyTagsError(ref cause) => cause,AddTagsToResourceError::Validation(ref cause) => cause,AddTagsToResourceError::Credentials(ref err) => err.description(),AddTagsToResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),AddTagsToResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CancelCommand
                #[derive(Debug, PartialEq)]
                pub enum CancelCommandError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///
InvalidCommandId(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>You cannot specify an instance ID in more than one association.</p>
DuplicateInstanceId(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CancelCommandError {
                    pub fn from_body(body: &str) -> CancelCommandError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => CancelCommandError::InternalServerError(String::from(error_message)),"InvalidCommandId" => CancelCommandError::InvalidCommandId(String::from(error_message)),"InvalidInstanceId" => CancelCommandError::InvalidInstanceId(String::from(error_message)),"DuplicateInstanceId" => CancelCommandError::DuplicateInstanceId(String::from(error_message)),"ValidationException" => CancelCommandError::Validation(error_message.to_string()),_ => CancelCommandError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CancelCommandError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CancelCommandError {
                    fn from(err: serde_json::error::Error) -> CancelCommandError {
                        CancelCommandError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CancelCommandError {
                    fn from(err: CredentialsError) -> CancelCommandError {
                        CancelCommandError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CancelCommandError {
                    fn from(err: HttpDispatchError) -> CancelCommandError {
                        CancelCommandError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CancelCommandError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CancelCommandError {
                    fn description(&self) -> &str {
                        match *self {
                            CancelCommandError::InternalServerError(ref cause) => cause,CancelCommandError::InvalidCommandId(ref cause) => cause,CancelCommandError::InvalidInstanceId(ref cause) => cause,CancelCommandError::DuplicateInstanceId(ref cause) => cause,CancelCommandError::Validation(ref cause) => cause,CancelCommandError::Credentials(ref err) => err.description(),CancelCommandError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CancelCommandError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateActivation
                #[derive(Debug, PartialEq)]
                pub enum CreateActivationError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateActivationError {
                    pub fn from_body(body: &str) -> CreateActivationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => CreateActivationError::InternalServerError(String::from(error_message)),"ValidationException" => CreateActivationError::Validation(error_message.to_string()),_ => CreateActivationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateActivationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateActivationError {
                    fn from(err: serde_json::error::Error) -> CreateActivationError {
                        CreateActivationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateActivationError {
                    fn from(err: CredentialsError) -> CreateActivationError {
                        CreateActivationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateActivationError {
                    fn from(err: HttpDispatchError) -> CreateActivationError {
                        CreateActivationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateActivationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateActivationError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateActivationError::InternalServerError(ref cause) => cause,CreateActivationError::Validation(ref cause) => cause,CreateActivationError::Credentials(ref err) => err.description(),CreateActivationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateActivationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateAssociation
                #[derive(Debug, PartialEq)]
                pub enum CreateAssociationError {
                    
///<p>The specified association already exists.</p>
AssociationAlreadyExists(String),
///<p>You can have at most 2,000 active associations.</p>
AssociationLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
UnsupportedPlatformType(String),
///<p>The output location is not valid or does not exist.</p>
InvalidOutputLocation(String),
///<p>You must specify values for all required parameters in the SSM document. You can only supply values to parameters defined in the SSM document.</p>
InvalidParameters(String),
///<p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
InvalidTarget(String),
///<p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
InvalidSchedule(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateAssociationError {
                    pub fn from_body(body: &str) -> CreateAssociationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AssociationAlreadyExists" => CreateAssociationError::AssociationAlreadyExists(String::from(error_message)),"AssociationLimitExceeded" => CreateAssociationError::AssociationLimitExceeded(String::from(error_message)),"InternalServerError" => CreateAssociationError::InternalServerError(String::from(error_message)),"InvalidDocument" => CreateAssociationError::InvalidDocument(String::from(error_message)),"InvalidDocumentVersion" => CreateAssociationError::InvalidDocumentVersion(String::from(error_message)),"InvalidInstanceId" => CreateAssociationError::InvalidInstanceId(String::from(error_message)),"UnsupportedPlatformType" => CreateAssociationError::UnsupportedPlatformType(String::from(error_message)),"InvalidOutputLocation" => CreateAssociationError::InvalidOutputLocation(String::from(error_message)),"InvalidParameters" => CreateAssociationError::InvalidParameters(String::from(error_message)),"InvalidTarget" => CreateAssociationError::InvalidTarget(String::from(error_message)),"InvalidSchedule" => CreateAssociationError::InvalidSchedule(String::from(error_message)),"ValidationException" => CreateAssociationError::Validation(error_message.to_string()),_ => CreateAssociationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateAssociationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateAssociationError {
                    fn from(err: serde_json::error::Error) -> CreateAssociationError {
                        CreateAssociationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateAssociationError {
                    fn from(err: CredentialsError) -> CreateAssociationError {
                        CreateAssociationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateAssociationError {
                    fn from(err: HttpDispatchError) -> CreateAssociationError {
                        CreateAssociationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateAssociationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateAssociationError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateAssociationError::AssociationAlreadyExists(ref cause) => cause,CreateAssociationError::AssociationLimitExceeded(ref cause) => cause,CreateAssociationError::InternalServerError(ref cause) => cause,CreateAssociationError::InvalidDocument(ref cause) => cause,CreateAssociationError::InvalidDocumentVersion(ref cause) => cause,CreateAssociationError::InvalidInstanceId(ref cause) => cause,CreateAssociationError::UnsupportedPlatformType(ref cause) => cause,CreateAssociationError::InvalidOutputLocation(ref cause) => cause,CreateAssociationError::InvalidParameters(ref cause) => cause,CreateAssociationError::InvalidTarget(ref cause) => cause,CreateAssociationError::InvalidSchedule(ref cause) => cause,CreateAssociationError::Validation(ref cause) => cause,CreateAssociationError::Credentials(ref err) => err.description(),CreateAssociationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateAssociationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateAssociationBatch
                #[derive(Debug, PartialEq)]
                pub enum CreateAssociationBatchError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>You must specify values for all required parameters in the SSM document. You can only supply values to parameters defined in the SSM document.</p>
InvalidParameters(String),
///<p>You cannot specify an instance ID in more than one association.</p>
DuplicateInstanceId(String),
///<p>You can have at most 2,000 active associations.</p>
AssociationLimitExceeded(String),
///<p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
UnsupportedPlatformType(String),
///<p>The output location is not valid or does not exist.</p>
InvalidOutputLocation(String),
///<p>The target is not valid or does not exist. It might not be configured for EC2 Systems Manager or you might not have permission to perform the operation.</p>
InvalidTarget(String),
///<p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
InvalidSchedule(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateAssociationBatchError {
                    pub fn from_body(body: &str) -> CreateAssociationBatchError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => CreateAssociationBatchError::InternalServerError(String::from(error_message)),"InvalidDocument" => CreateAssociationBatchError::InvalidDocument(String::from(error_message)),"InvalidDocumentVersion" => CreateAssociationBatchError::InvalidDocumentVersion(String::from(error_message)),"InvalidInstanceId" => CreateAssociationBatchError::InvalidInstanceId(String::from(error_message)),"InvalidParameters" => CreateAssociationBatchError::InvalidParameters(String::from(error_message)),"DuplicateInstanceId" => CreateAssociationBatchError::DuplicateInstanceId(String::from(error_message)),"AssociationLimitExceeded" => CreateAssociationBatchError::AssociationLimitExceeded(String::from(error_message)),"UnsupportedPlatformType" => CreateAssociationBatchError::UnsupportedPlatformType(String::from(error_message)),"InvalidOutputLocation" => CreateAssociationBatchError::InvalidOutputLocation(String::from(error_message)),"InvalidTarget" => CreateAssociationBatchError::InvalidTarget(String::from(error_message)),"InvalidSchedule" => CreateAssociationBatchError::InvalidSchedule(String::from(error_message)),"ValidationException" => CreateAssociationBatchError::Validation(error_message.to_string()),_ => CreateAssociationBatchError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateAssociationBatchError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateAssociationBatchError {
                    fn from(err: serde_json::error::Error) -> CreateAssociationBatchError {
                        CreateAssociationBatchError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateAssociationBatchError {
                    fn from(err: CredentialsError) -> CreateAssociationBatchError {
                        CreateAssociationBatchError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateAssociationBatchError {
                    fn from(err: HttpDispatchError) -> CreateAssociationBatchError {
                        CreateAssociationBatchError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateAssociationBatchError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateAssociationBatchError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateAssociationBatchError::InternalServerError(ref cause) => cause,CreateAssociationBatchError::InvalidDocument(ref cause) => cause,CreateAssociationBatchError::InvalidDocumentVersion(ref cause) => cause,CreateAssociationBatchError::InvalidInstanceId(ref cause) => cause,CreateAssociationBatchError::InvalidParameters(ref cause) => cause,CreateAssociationBatchError::DuplicateInstanceId(ref cause) => cause,CreateAssociationBatchError::AssociationLimitExceeded(ref cause) => cause,CreateAssociationBatchError::UnsupportedPlatformType(ref cause) => cause,CreateAssociationBatchError::InvalidOutputLocation(ref cause) => cause,CreateAssociationBatchError::InvalidTarget(ref cause) => cause,CreateAssociationBatchError::InvalidSchedule(ref cause) => cause,CreateAssociationBatchError::Validation(ref cause) => cause,CreateAssociationBatchError::Credentials(ref err) => err.description(),CreateAssociationBatchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateAssociationBatchError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateDocument
                #[derive(Debug, PartialEq)]
                pub enum CreateDocumentError {
                    
///<p>The specified document already exists.</p>
DocumentAlreadyExists(String),
///<p>The size limit of a document is 64 KB.</p>
MaxDocumentSizeExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The content for the document is not valid.</p>
InvalidDocumentContent(String),
///<p>You can have at most 200 active SSM documents.</p>
DocumentLimitExceeded(String),
///<p>The version of the document schema is not supported.</p>
InvalidDocumentSchemaVersion(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateDocumentError {
                    pub fn from_body(body: &str) -> CreateDocumentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DocumentAlreadyExists" => CreateDocumentError::DocumentAlreadyExists(String::from(error_message)),"MaxDocumentSizeExceeded" => CreateDocumentError::MaxDocumentSizeExceeded(String::from(error_message)),"InternalServerError" => CreateDocumentError::InternalServerError(String::from(error_message)),"InvalidDocumentContent" => CreateDocumentError::InvalidDocumentContent(String::from(error_message)),"DocumentLimitExceeded" => CreateDocumentError::DocumentLimitExceeded(String::from(error_message)),"InvalidDocumentSchemaVersion" => CreateDocumentError::InvalidDocumentSchemaVersion(String::from(error_message)),"ValidationException" => CreateDocumentError::Validation(error_message.to_string()),_ => CreateDocumentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateDocumentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateDocumentError {
                    fn from(err: serde_json::error::Error) -> CreateDocumentError {
                        CreateDocumentError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateDocumentError {
                    fn from(err: CredentialsError) -> CreateDocumentError {
                        CreateDocumentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateDocumentError {
                    fn from(err: HttpDispatchError) -> CreateDocumentError {
                        CreateDocumentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateDocumentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateDocumentError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateDocumentError::DocumentAlreadyExists(ref cause) => cause,CreateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,CreateDocumentError::InternalServerError(ref cause) => cause,CreateDocumentError::InvalidDocumentContent(ref cause) => cause,CreateDocumentError::DocumentLimitExceeded(ref cause) => cause,CreateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,CreateDocumentError::Validation(ref cause) => cause,CreateDocumentError::Credentials(ref err) => err.description(),CreateDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateDocumentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum CreateMaintenanceWindowError {
                    
///<p>Error returned when an idempotent operation is retried and the parameters don’t match the original call to the API with the same idempotency token. </p>
IdempotentParameterMismatch(String),
///<p>Error returned when the caller has exceeded the default resource limits (e.g. too many Maintenance Windows have been created).</p>
ResourceLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateMaintenanceWindowError {
                    pub fn from_body(body: &str) -> CreateMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "IdempotentParameterMismatch" => CreateMaintenanceWindowError::IdempotentParameterMismatch(String::from(error_message)),"ResourceLimitExceededException" => CreateMaintenanceWindowError::ResourceLimitExceeded(String::from(error_message)),"InternalServerError" => CreateMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => CreateMaintenanceWindowError::Validation(error_message.to_string()),_ => CreateMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreateMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreateMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> CreateMaintenanceWindowError {
                        CreateMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreateMaintenanceWindowError {
                    fn from(err: CredentialsError) -> CreateMaintenanceWindowError {
                        CreateMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> CreateMaintenanceWindowError {
                        CreateMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,CreateMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,CreateMaintenanceWindowError::InternalServerError(ref cause) => cause,CreateMaintenanceWindowError::Validation(ref cause) => cause,CreateMaintenanceWindowError::Credentials(ref err) => err.description(),CreateMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreatePatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum CreatePatchBaselineError {
                    
///<p>Error returned when an idempotent operation is retried and the parameters don’t match the original call to the API with the same idempotency token. </p>
IdempotentParameterMismatch(String),
///<p>Error returned when the caller has exceeded the default resource limits (e.g. too many Maintenance Windows have been created).</p>
ResourceLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreatePatchBaselineError {
                    pub fn from_body(body: &str) -> CreatePatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "IdempotentParameterMismatch" => CreatePatchBaselineError::IdempotentParameterMismatch(String::from(error_message)),"ResourceLimitExceededException" => CreatePatchBaselineError::ResourceLimitExceeded(String::from(error_message)),"InternalServerError" => CreatePatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => CreatePatchBaselineError::Validation(error_message.to_string()),_ => CreatePatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => CreatePatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for CreatePatchBaselineError {
                    fn from(err: serde_json::error::Error) -> CreatePatchBaselineError {
                        CreatePatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for CreatePatchBaselineError {
                    fn from(err: CredentialsError) -> CreatePatchBaselineError {
                        CreatePatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreatePatchBaselineError {
                    fn from(err: HttpDispatchError) -> CreatePatchBaselineError {
                        CreatePatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreatePatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreatePatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            CreatePatchBaselineError::IdempotentParameterMismatch(ref cause) => cause,CreatePatchBaselineError::ResourceLimitExceeded(ref cause) => cause,CreatePatchBaselineError::InternalServerError(ref cause) => cause,CreatePatchBaselineError::Validation(ref cause) => cause,CreatePatchBaselineError::Credentials(ref err) => err.description(),CreatePatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreatePatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteActivation
                #[derive(Debug, PartialEq)]
                pub enum DeleteActivationError {
                    
///<p>The activation ID is not valid. Verify the you entered the correct ActivationId or ActivationCode and try again.</p>
InvalidActivationId(String),
///<p>The activation is not valid. The activation might have been deleted, or the ActivationId and the ActivationCode do not match.</p>
InvalidActivation(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteActivationError {
                    pub fn from_body(body: &str) -> DeleteActivationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidActivationId" => DeleteActivationError::InvalidActivationId(String::from(error_message)),"InvalidActivation" => DeleteActivationError::InvalidActivation(String::from(error_message)),"InternalServerError" => DeleteActivationError::InternalServerError(String::from(error_message)),"ValidationException" => DeleteActivationError::Validation(error_message.to_string()),_ => DeleteActivationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteActivationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteActivationError {
                    fn from(err: serde_json::error::Error) -> DeleteActivationError {
                        DeleteActivationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteActivationError {
                    fn from(err: CredentialsError) -> DeleteActivationError {
                        DeleteActivationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteActivationError {
                    fn from(err: HttpDispatchError) -> DeleteActivationError {
                        DeleteActivationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteActivationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteActivationError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteActivationError::InvalidActivationId(ref cause) => cause,DeleteActivationError::InvalidActivation(ref cause) => cause,DeleteActivationError::InternalServerError(ref cause) => cause,DeleteActivationError::Validation(ref cause) => cause,DeleteActivationError::Credentials(ref err) => err.description(),DeleteActivationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteActivationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteAssociation
                #[derive(Debug, PartialEq)]
                pub enum DeleteAssociationError {
                    
///<p>The specified association does not exist.</p>
AssociationDoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>There are concurrent updates for a resource that supports one update at a time.</p>
TooManyUpdates(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteAssociationError {
                    pub fn from_body(body: &str) -> DeleteAssociationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AssociationDoesNotExist" => DeleteAssociationError::AssociationDoesNotExist(String::from(error_message)),"InternalServerError" => DeleteAssociationError::InternalServerError(String::from(error_message)),"InvalidDocument" => DeleteAssociationError::InvalidDocument(String::from(error_message)),"InvalidInstanceId" => DeleteAssociationError::InvalidInstanceId(String::from(error_message)),"TooManyUpdates" => DeleteAssociationError::TooManyUpdates(String::from(error_message)),"ValidationException" => DeleteAssociationError::Validation(error_message.to_string()),_ => DeleteAssociationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteAssociationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteAssociationError {
                    fn from(err: serde_json::error::Error) -> DeleteAssociationError {
                        DeleteAssociationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteAssociationError {
                    fn from(err: CredentialsError) -> DeleteAssociationError {
                        DeleteAssociationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteAssociationError {
                    fn from(err: HttpDispatchError) -> DeleteAssociationError {
                        DeleteAssociationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteAssociationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteAssociationError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteAssociationError::AssociationDoesNotExist(ref cause) => cause,DeleteAssociationError::InternalServerError(ref cause) => cause,DeleteAssociationError::InvalidDocument(ref cause) => cause,DeleteAssociationError::InvalidInstanceId(ref cause) => cause,DeleteAssociationError::TooManyUpdates(ref cause) => cause,DeleteAssociationError::Validation(ref cause) => cause,DeleteAssociationError::Credentials(ref err) => err.description(),DeleteAssociationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteAssociationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteDocument
                #[derive(Debug, PartialEq)]
                pub enum DeleteDocumentError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it.</p>
InvalidDocumentOperation(String),
///<p>You must disassociate a document from all instances before you can delete it.</p>
AssociatedInstances(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteDocumentError {
                    pub fn from_body(body: &str) -> DeleteDocumentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DeleteDocumentError::InternalServerError(String::from(error_message)),"InvalidDocument" => DeleteDocumentError::InvalidDocument(String::from(error_message)),"InvalidDocumentOperation" => DeleteDocumentError::InvalidDocumentOperation(String::from(error_message)),"AssociatedInstances" => DeleteDocumentError::AssociatedInstances(String::from(error_message)),"ValidationException" => DeleteDocumentError::Validation(error_message.to_string()),_ => DeleteDocumentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteDocumentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteDocumentError {
                    fn from(err: serde_json::error::Error) -> DeleteDocumentError {
                        DeleteDocumentError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteDocumentError {
                    fn from(err: CredentialsError) -> DeleteDocumentError {
                        DeleteDocumentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteDocumentError {
                    fn from(err: HttpDispatchError) -> DeleteDocumentError {
                        DeleteDocumentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteDocumentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteDocumentError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteDocumentError::InternalServerError(ref cause) => cause,DeleteDocumentError::InvalidDocument(ref cause) => cause,DeleteDocumentError::InvalidDocumentOperation(ref cause) => cause,DeleteDocumentError::AssociatedInstances(ref cause) => cause,DeleteDocumentError::Validation(ref cause) => cause,DeleteDocumentError::Credentials(ref err) => err.description(),DeleteDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteDocumentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum DeleteMaintenanceWindowError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteMaintenanceWindowError {
                    pub fn from_body(body: &str) -> DeleteMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DeleteMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => DeleteMaintenanceWindowError::Validation(error_message.to_string()),_ => DeleteMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> DeleteMaintenanceWindowError {
                        DeleteMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteMaintenanceWindowError {
                    fn from(err: CredentialsError) -> DeleteMaintenanceWindowError {
                        DeleteMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> DeleteMaintenanceWindowError {
                        DeleteMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteMaintenanceWindowError::InternalServerError(ref cause) => cause,DeleteMaintenanceWindowError::Validation(ref cause) => cause,DeleteMaintenanceWindowError::Credentials(ref err) => err.description(),DeleteMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteParameter
                #[derive(Debug, PartialEq)]
                pub enum DeleteParameterError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The parameter could not be found. Verify the name and try again.</p>
ParameterNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteParameterError {
                    pub fn from_body(body: &str) -> DeleteParameterError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DeleteParameterError::InternalServerError(String::from(error_message)),"ParameterNotFound" => DeleteParameterError::ParameterNotFound(String::from(error_message)),"ValidationException" => DeleteParameterError::Validation(error_message.to_string()),_ => DeleteParameterError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeleteParameterError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeleteParameterError {
                    fn from(err: serde_json::error::Error) -> DeleteParameterError {
                        DeleteParameterError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeleteParameterError {
                    fn from(err: CredentialsError) -> DeleteParameterError {
                        DeleteParameterError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteParameterError {
                    fn from(err: HttpDispatchError) -> DeleteParameterError {
                        DeleteParameterError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteParameterError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteParameterError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteParameterError::InternalServerError(ref cause) => cause,DeleteParameterError::ParameterNotFound(ref cause) => cause,DeleteParameterError::Validation(ref cause) => cause,DeleteParameterError::Credentials(ref err) => err.description(),DeleteParameterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteParameterError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeletePatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum DeletePatchBaselineError {
                    
///<p>Error returned if an attempt is made to delete a patch baseline that is registered for a patch group.</p>
ResourceInUse(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeletePatchBaselineError {
                    pub fn from_body(body: &str) -> DeletePatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "ResourceInUseException" => DeletePatchBaselineError::ResourceInUse(String::from(error_message)),"InternalServerError" => DeletePatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => DeletePatchBaselineError::Validation(error_message.to_string()),_ => DeletePatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeletePatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeletePatchBaselineError {
                    fn from(err: serde_json::error::Error) -> DeletePatchBaselineError {
                        DeletePatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeletePatchBaselineError {
                    fn from(err: CredentialsError) -> DeletePatchBaselineError {
                        DeletePatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeletePatchBaselineError {
                    fn from(err: HttpDispatchError) -> DeletePatchBaselineError {
                        DeletePatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeletePatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeletePatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            DeletePatchBaselineError::ResourceInUse(ref cause) => cause,DeletePatchBaselineError::InternalServerError(ref cause) => cause,DeletePatchBaselineError::Validation(ref cause) => cause,DeletePatchBaselineError::Credentials(ref err) => err.description(),DeletePatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeletePatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeregisterManagedInstance
                #[derive(Debug, PartialEq)]
                pub enum DeregisterManagedInstanceError {
                    
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeregisterManagedInstanceError {
                    pub fn from_body(body: &str) -> DeregisterManagedInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidInstanceId" => DeregisterManagedInstanceError::InvalidInstanceId(String::from(error_message)),"InternalServerError" => DeregisterManagedInstanceError::InternalServerError(String::from(error_message)),"ValidationException" => DeregisterManagedInstanceError::Validation(error_message.to_string()),_ => DeregisterManagedInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeregisterManagedInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeregisterManagedInstanceError {
                    fn from(err: serde_json::error::Error) -> DeregisterManagedInstanceError {
                        DeregisterManagedInstanceError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeregisterManagedInstanceError {
                    fn from(err: CredentialsError) -> DeregisterManagedInstanceError {
                        DeregisterManagedInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeregisterManagedInstanceError {
                    fn from(err: HttpDispatchError) -> DeregisterManagedInstanceError {
                        DeregisterManagedInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeregisterManagedInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeregisterManagedInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            DeregisterManagedInstanceError::InvalidInstanceId(ref cause) => cause,DeregisterManagedInstanceError::InternalServerError(ref cause) => cause,DeregisterManagedInstanceError::Validation(ref cause) => cause,DeregisterManagedInstanceError::Credentials(ref err) => err.description(),DeregisterManagedInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeregisterManagedInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeregisterPatchBaselineForPatchGroup
                #[derive(Debug, PartialEq)]
                pub enum DeregisterPatchBaselineForPatchGroupError {
                    
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeregisterPatchBaselineForPatchGroupError {
                    pub fn from_body(body: &str) -> DeregisterPatchBaselineForPatchGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceId" => DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(String::from(error_message)),"InternalServerError" => DeregisterPatchBaselineForPatchGroupError::InternalServerError(String::from(error_message)),"ValidationException" => DeregisterPatchBaselineForPatchGroupError::Validation(error_message.to_string()),_ => DeregisterPatchBaselineForPatchGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeregisterPatchBaselineForPatchGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeregisterPatchBaselineForPatchGroupError {
                    fn from(err: serde_json::error::Error) -> DeregisterPatchBaselineForPatchGroupError {
                        DeregisterPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeregisterPatchBaselineForPatchGroupError {
                    fn from(err: CredentialsError) -> DeregisterPatchBaselineForPatchGroupError {
                        DeregisterPatchBaselineForPatchGroupError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeregisterPatchBaselineForPatchGroupError {
                    fn from(err: HttpDispatchError) -> DeregisterPatchBaselineForPatchGroupError {
                        DeregisterPatchBaselineForPatchGroupError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeregisterPatchBaselineForPatchGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeregisterPatchBaselineForPatchGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            DeregisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,DeregisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,DeregisterPatchBaselineForPatchGroupError::Validation(ref cause) => cause,DeregisterPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),DeregisterPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeregisterPatchBaselineForPatchGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeregisterTargetFromMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum DeregisterTargetFromMaintenanceWindowError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeregisterTargetFromMaintenanceWindowError {
                    pub fn from_body(body: &str) -> DeregisterTargetFromMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DeregisterTargetFromMaintenanceWindowError::DoesNotExist(String::from(error_message)),"InternalServerError" => DeregisterTargetFromMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => DeregisterTargetFromMaintenanceWindowError::Validation(error_message.to_string()),_ => DeregisterTargetFromMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeregisterTargetFromMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeregisterTargetFromMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> DeregisterTargetFromMaintenanceWindowError {
                        DeregisterTargetFromMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeregisterTargetFromMaintenanceWindowError {
                    fn from(err: CredentialsError) -> DeregisterTargetFromMaintenanceWindowError {
                        DeregisterTargetFromMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeregisterTargetFromMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> DeregisterTargetFromMaintenanceWindowError {
                        DeregisterTargetFromMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeregisterTargetFromMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeregisterTargetFromMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            DeregisterTargetFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,DeregisterTargetFromMaintenanceWindowError::InternalServerError(ref cause) => cause,DeregisterTargetFromMaintenanceWindowError::Validation(ref cause) => cause,DeregisterTargetFromMaintenanceWindowError::Credentials(ref err) => err.description(),DeregisterTargetFromMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeregisterTargetFromMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeregisterTaskFromMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum DeregisterTaskFromMaintenanceWindowError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeregisterTaskFromMaintenanceWindowError {
                    pub fn from_body(body: &str) -> DeregisterTaskFromMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DeregisterTaskFromMaintenanceWindowError::DoesNotExist(String::from(error_message)),"InternalServerError" => DeregisterTaskFromMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => DeregisterTaskFromMaintenanceWindowError::Validation(error_message.to_string()),_ => DeregisterTaskFromMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DeregisterTaskFromMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DeregisterTaskFromMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> DeregisterTaskFromMaintenanceWindowError {
                        DeregisterTaskFromMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DeregisterTaskFromMaintenanceWindowError {
                    fn from(err: CredentialsError) -> DeregisterTaskFromMaintenanceWindowError {
                        DeregisterTaskFromMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeregisterTaskFromMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> DeregisterTaskFromMaintenanceWindowError {
                        DeregisterTaskFromMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeregisterTaskFromMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeregisterTaskFromMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            DeregisterTaskFromMaintenanceWindowError::DoesNotExist(ref cause) => cause,DeregisterTaskFromMaintenanceWindowError::InternalServerError(ref cause) => cause,DeregisterTaskFromMaintenanceWindowError::Validation(ref cause) => cause,DeregisterTaskFromMaintenanceWindowError::Credentials(ref err) => err.description(),DeregisterTaskFromMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeregisterTaskFromMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeActivations
                #[derive(Debug, PartialEq)]
                pub enum DescribeActivationsError {
                    
///<p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
InvalidFilter(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeActivationsError {
                    pub fn from_body(body: &str) -> DescribeActivationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidFilter" => DescribeActivationsError::InvalidFilter(String::from(error_message)),"InvalidNextToken" => DescribeActivationsError::InvalidNextToken(String::from(error_message)),"InternalServerError" => DescribeActivationsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeActivationsError::Validation(error_message.to_string()),_ => DescribeActivationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeActivationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeActivationsError {
                    fn from(err: serde_json::error::Error) -> DescribeActivationsError {
                        DescribeActivationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeActivationsError {
                    fn from(err: CredentialsError) -> DescribeActivationsError {
                        DescribeActivationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeActivationsError {
                    fn from(err: HttpDispatchError) -> DescribeActivationsError {
                        DescribeActivationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeActivationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeActivationsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeActivationsError::InvalidFilter(ref cause) => cause,DescribeActivationsError::InvalidNextToken(ref cause) => cause,DescribeActivationsError::InternalServerError(ref cause) => cause,DescribeActivationsError::Validation(ref cause) => cause,DescribeActivationsError::Credentials(ref err) => err.description(),DescribeActivationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeActivationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeAssociation
                #[derive(Debug, PartialEq)]
                pub enum DescribeAssociationError {
                    
///<p>The specified association does not exist.</p>
AssociationDoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeAssociationError {
                    pub fn from_body(body: &str) -> DescribeAssociationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AssociationDoesNotExist" => DescribeAssociationError::AssociationDoesNotExist(String::from(error_message)),"InternalServerError" => DescribeAssociationError::InternalServerError(String::from(error_message)),"InvalidDocument" => DescribeAssociationError::InvalidDocument(String::from(error_message)),"InvalidInstanceId" => DescribeAssociationError::InvalidInstanceId(String::from(error_message)),"ValidationException" => DescribeAssociationError::Validation(error_message.to_string()),_ => DescribeAssociationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeAssociationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeAssociationError {
                    fn from(err: serde_json::error::Error) -> DescribeAssociationError {
                        DescribeAssociationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeAssociationError {
                    fn from(err: CredentialsError) -> DescribeAssociationError {
                        DescribeAssociationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeAssociationError {
                    fn from(err: HttpDispatchError) -> DescribeAssociationError {
                        DescribeAssociationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeAssociationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeAssociationError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeAssociationError::AssociationDoesNotExist(ref cause) => cause,DescribeAssociationError::InternalServerError(ref cause) => cause,DescribeAssociationError::InvalidDocument(ref cause) => cause,DescribeAssociationError::InvalidInstanceId(ref cause) => cause,DescribeAssociationError::Validation(ref cause) => cause,DescribeAssociationError::Credentials(ref err) => err.description(),DescribeAssociationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeAssociationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeAutomationExecutions
                #[derive(Debug, PartialEq)]
                pub enum DescribeAutomationExecutionsError {
                    
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeAutomationExecutionsError {
                    pub fn from_body(body: &str) -> DescribeAutomationExecutionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidNextToken" => DescribeAutomationExecutionsError::InvalidNextToken(String::from(error_message)),"InternalServerError" => DescribeAutomationExecutionsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeAutomationExecutionsError::Validation(error_message.to_string()),_ => DescribeAutomationExecutionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeAutomationExecutionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeAutomationExecutionsError {
                    fn from(err: serde_json::error::Error) -> DescribeAutomationExecutionsError {
                        DescribeAutomationExecutionsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeAutomationExecutionsError {
                    fn from(err: CredentialsError) -> DescribeAutomationExecutionsError {
                        DescribeAutomationExecutionsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeAutomationExecutionsError {
                    fn from(err: HttpDispatchError) -> DescribeAutomationExecutionsError {
                        DescribeAutomationExecutionsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeAutomationExecutionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeAutomationExecutionsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeAutomationExecutionsError::InvalidNextToken(ref cause) => cause,DescribeAutomationExecutionsError::InternalServerError(ref cause) => cause,DescribeAutomationExecutionsError::Validation(ref cause) => cause,DescribeAutomationExecutionsError::Credentials(ref err) => err.description(),DescribeAutomationExecutionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeAutomationExecutionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeAvailablePatches
                #[derive(Debug, PartialEq)]
                pub enum DescribeAvailablePatchesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeAvailablePatchesError {
                    pub fn from_body(body: &str) -> DescribeAvailablePatchesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeAvailablePatchesError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeAvailablePatchesError::Validation(error_message.to_string()),_ => DescribeAvailablePatchesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeAvailablePatchesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeAvailablePatchesError {
                    fn from(err: serde_json::error::Error) -> DescribeAvailablePatchesError {
                        DescribeAvailablePatchesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeAvailablePatchesError {
                    fn from(err: CredentialsError) -> DescribeAvailablePatchesError {
                        DescribeAvailablePatchesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeAvailablePatchesError {
                    fn from(err: HttpDispatchError) -> DescribeAvailablePatchesError {
                        DescribeAvailablePatchesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeAvailablePatchesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeAvailablePatchesError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeAvailablePatchesError::InternalServerError(ref cause) => cause,DescribeAvailablePatchesError::Validation(ref cause) => cause,DescribeAvailablePatchesError::Credentials(ref err) => err.description(),DescribeAvailablePatchesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeAvailablePatchesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeDocument
                #[derive(Debug, PartialEq)]
                pub enum DescribeDocumentError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeDocumentError {
                    pub fn from_body(body: &str) -> DescribeDocumentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeDocumentError::InternalServerError(String::from(error_message)),"InvalidDocument" => DescribeDocumentError::InvalidDocument(String::from(error_message)),"InvalidDocumentVersion" => DescribeDocumentError::InvalidDocumentVersion(String::from(error_message)),"ValidationException" => DescribeDocumentError::Validation(error_message.to_string()),_ => DescribeDocumentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeDocumentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeDocumentError {
                    fn from(err: serde_json::error::Error) -> DescribeDocumentError {
                        DescribeDocumentError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeDocumentError {
                    fn from(err: CredentialsError) -> DescribeDocumentError {
                        DescribeDocumentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeDocumentError {
                    fn from(err: HttpDispatchError) -> DescribeDocumentError {
                        DescribeDocumentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeDocumentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeDocumentError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeDocumentError::InternalServerError(ref cause) => cause,DescribeDocumentError::InvalidDocument(ref cause) => cause,DescribeDocumentError::InvalidDocumentVersion(ref cause) => cause,DescribeDocumentError::Validation(ref cause) => cause,DescribeDocumentError::Credentials(ref err) => err.description(),DescribeDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeDocumentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeDocumentPermission
                #[derive(Debug, PartialEq)]
                pub enum DescribeDocumentPermissionError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
InvalidPermissionType(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeDocumentPermissionError {
                    pub fn from_body(body: &str) -> DescribeDocumentPermissionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeDocumentPermissionError::InternalServerError(String::from(error_message)),"InvalidDocument" => DescribeDocumentPermissionError::InvalidDocument(String::from(error_message)),"InvalidPermissionType" => DescribeDocumentPermissionError::InvalidPermissionType(String::from(error_message)),"ValidationException" => DescribeDocumentPermissionError::Validation(error_message.to_string()),_ => DescribeDocumentPermissionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeDocumentPermissionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeDocumentPermissionError {
                    fn from(err: serde_json::error::Error) -> DescribeDocumentPermissionError {
                        DescribeDocumentPermissionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeDocumentPermissionError {
                    fn from(err: CredentialsError) -> DescribeDocumentPermissionError {
                        DescribeDocumentPermissionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeDocumentPermissionError {
                    fn from(err: HttpDispatchError) -> DescribeDocumentPermissionError {
                        DescribeDocumentPermissionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeDocumentPermissionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeDocumentPermissionError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeDocumentPermissionError::InternalServerError(ref cause) => cause,DescribeDocumentPermissionError::InvalidDocument(ref cause) => cause,DescribeDocumentPermissionError::InvalidPermissionType(ref cause) => cause,DescribeDocumentPermissionError::Validation(ref cause) => cause,DescribeDocumentPermissionError::Credentials(ref err) => err.description(),DescribeDocumentPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeDocumentPermissionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeEffectiveInstanceAssociations
                #[derive(Debug, PartialEq)]
                pub enum DescribeEffectiveInstanceAssociationsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeEffectiveInstanceAssociationsError {
                    pub fn from_body(body: &str) -> DescribeEffectiveInstanceAssociationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeEffectiveInstanceAssociationsError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(String::from(error_message)),"InvalidNextToken" => DescribeEffectiveInstanceAssociationsError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeEffectiveInstanceAssociationsError::Validation(error_message.to_string()),_ => DescribeEffectiveInstanceAssociationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeEffectiveInstanceAssociationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeEffectiveInstanceAssociationsError {
                    fn from(err: serde_json::error::Error) -> DescribeEffectiveInstanceAssociationsError {
                        DescribeEffectiveInstanceAssociationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeEffectiveInstanceAssociationsError {
                    fn from(err: CredentialsError) -> DescribeEffectiveInstanceAssociationsError {
                        DescribeEffectiveInstanceAssociationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeEffectiveInstanceAssociationsError {
                    fn from(err: HttpDispatchError) -> DescribeEffectiveInstanceAssociationsError {
                        DescribeEffectiveInstanceAssociationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeEffectiveInstanceAssociationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeEffectiveInstanceAssociationsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeEffectiveInstanceAssociationsError::InternalServerError(ref cause) => cause,DescribeEffectiveInstanceAssociationsError::InvalidInstanceId(ref cause) => cause,DescribeEffectiveInstanceAssociationsError::InvalidNextToken(ref cause) => cause,DescribeEffectiveInstanceAssociationsError::Validation(ref cause) => cause,DescribeEffectiveInstanceAssociationsError::Credentials(ref err) => err.description(),DescribeEffectiveInstanceAssociationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeEffectiveInstanceAssociationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeEffectivePatchesForPatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum DescribeEffectivePatchesForPatchBaselineError {
                    
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeEffectivePatchesForPatchBaselineError {
                    pub fn from_body(body: &str) -> DescribeEffectivePatchesForPatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceId" => DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(String::from(error_message)),"DoesNotExistException" => DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(String::from(error_message)),"InternalServerError" => DescribeEffectivePatchesForPatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeEffectivePatchesForPatchBaselineError::Validation(error_message.to_string()),_ => DescribeEffectivePatchesForPatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeEffectivePatchesForPatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeEffectivePatchesForPatchBaselineError {
                    fn from(err: serde_json::error::Error) -> DescribeEffectivePatchesForPatchBaselineError {
                        DescribeEffectivePatchesForPatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeEffectivePatchesForPatchBaselineError {
                    fn from(err: CredentialsError) -> DescribeEffectivePatchesForPatchBaselineError {
                        DescribeEffectivePatchesForPatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeEffectivePatchesForPatchBaselineError {
                    fn from(err: HttpDispatchError) -> DescribeEffectivePatchesForPatchBaselineError {
                        DescribeEffectivePatchesForPatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeEffectivePatchesForPatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeEffectivePatchesForPatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeEffectivePatchesForPatchBaselineError::InvalidResourceId(ref cause) => cause,DescribeEffectivePatchesForPatchBaselineError::DoesNotExist(ref cause) => cause,DescribeEffectivePatchesForPatchBaselineError::InternalServerError(ref cause) => cause,DescribeEffectivePatchesForPatchBaselineError::Validation(ref cause) => cause,DescribeEffectivePatchesForPatchBaselineError::Credentials(ref err) => err.description(),DescribeEffectivePatchesForPatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeEffectivePatchesForPatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInstanceAssociationsStatus
                #[derive(Debug, PartialEq)]
                pub enum DescribeInstanceAssociationsStatusError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInstanceAssociationsStatusError {
                    pub fn from_body(body: &str) -> DescribeInstanceAssociationsStatusError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeInstanceAssociationsStatusError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => DescribeInstanceAssociationsStatusError::InvalidInstanceId(String::from(error_message)),"InvalidNextToken" => DescribeInstanceAssociationsStatusError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeInstanceAssociationsStatusError::Validation(error_message.to_string()),_ => DescribeInstanceAssociationsStatusError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInstanceAssociationsStatusError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeInstanceAssociationsStatusError {
                    fn from(err: serde_json::error::Error) -> DescribeInstanceAssociationsStatusError {
                        DescribeInstanceAssociationsStatusError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeInstanceAssociationsStatusError {
                    fn from(err: CredentialsError) -> DescribeInstanceAssociationsStatusError {
                        DescribeInstanceAssociationsStatusError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeInstanceAssociationsStatusError {
                    fn from(err: HttpDispatchError) -> DescribeInstanceAssociationsStatusError {
                        DescribeInstanceAssociationsStatusError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeInstanceAssociationsStatusError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeInstanceAssociationsStatusError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeInstanceAssociationsStatusError::InternalServerError(ref cause) => cause,DescribeInstanceAssociationsStatusError::InvalidInstanceId(ref cause) => cause,DescribeInstanceAssociationsStatusError::InvalidNextToken(ref cause) => cause,DescribeInstanceAssociationsStatusError::Validation(ref cause) => cause,DescribeInstanceAssociationsStatusError::Credentials(ref err) => err.description(),DescribeInstanceAssociationsStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeInstanceAssociationsStatusError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInstanceInformation
                #[derive(Debug, PartialEq)]
                pub enum DescribeInstanceInformationError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>The specified filter value is not valid.</p>
InvalidInstanceInformationFilterValue(String),
///<p>The specified key is not valid.</p>
InvalidFilterKey(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInstanceInformationError {
                    pub fn from_body(body: &str) -> DescribeInstanceInformationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeInstanceInformationError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => DescribeInstanceInformationError::InvalidInstanceId(String::from(error_message)),"InvalidNextToken" => DescribeInstanceInformationError::InvalidNextToken(String::from(error_message)),"InvalidInstanceInformationFilterValue" => DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(String::from(error_message)),"InvalidFilterKey" => DescribeInstanceInformationError::InvalidFilterKey(String::from(error_message)),"ValidationException" => DescribeInstanceInformationError::Validation(error_message.to_string()),_ => DescribeInstanceInformationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInstanceInformationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeInstanceInformationError {
                    fn from(err: serde_json::error::Error) -> DescribeInstanceInformationError {
                        DescribeInstanceInformationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeInstanceInformationError {
                    fn from(err: CredentialsError) -> DescribeInstanceInformationError {
                        DescribeInstanceInformationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeInstanceInformationError {
                    fn from(err: HttpDispatchError) -> DescribeInstanceInformationError {
                        DescribeInstanceInformationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeInstanceInformationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeInstanceInformationError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeInstanceInformationError::InternalServerError(ref cause) => cause,DescribeInstanceInformationError::InvalidInstanceId(ref cause) => cause,DescribeInstanceInformationError::InvalidNextToken(ref cause) => cause,DescribeInstanceInformationError::InvalidInstanceInformationFilterValue(ref cause) => cause,DescribeInstanceInformationError::InvalidFilterKey(ref cause) => cause,DescribeInstanceInformationError::Validation(ref cause) => cause,DescribeInstanceInformationError::Credentials(ref err) => err.description(),DescribeInstanceInformationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeInstanceInformationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInstancePatchStates
                #[derive(Debug, PartialEq)]
                pub enum DescribeInstancePatchStatesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInstancePatchStatesError {
                    pub fn from_body(body: &str) -> DescribeInstancePatchStatesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeInstancePatchStatesError::InternalServerError(String::from(error_message)),"InvalidNextToken" => DescribeInstancePatchStatesError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeInstancePatchStatesError::Validation(error_message.to_string()),_ => DescribeInstancePatchStatesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInstancePatchStatesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeInstancePatchStatesError {
                    fn from(err: serde_json::error::Error) -> DescribeInstancePatchStatesError {
                        DescribeInstancePatchStatesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeInstancePatchStatesError {
                    fn from(err: CredentialsError) -> DescribeInstancePatchStatesError {
                        DescribeInstancePatchStatesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeInstancePatchStatesError {
                    fn from(err: HttpDispatchError) -> DescribeInstancePatchStatesError {
                        DescribeInstancePatchStatesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeInstancePatchStatesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeInstancePatchStatesError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeInstancePatchStatesError::InternalServerError(ref cause) => cause,DescribeInstancePatchStatesError::InvalidNextToken(ref cause) => cause,DescribeInstancePatchStatesError::Validation(ref cause) => cause,DescribeInstancePatchStatesError::Credentials(ref err) => err.description(),DescribeInstancePatchStatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeInstancePatchStatesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInstancePatchStatesForPatchGroup
                #[derive(Debug, PartialEq)]
                pub enum DescribeInstancePatchStatesForPatchGroupError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
InvalidFilter(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInstancePatchStatesForPatchGroupError {
                    pub fn from_body(body: &str) -> DescribeInstancePatchStatesForPatchGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeInstancePatchStatesForPatchGroupError::InternalServerError(String::from(error_message)),"InvalidFilter" => DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(String::from(error_message)),"InvalidNextToken" => DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeInstancePatchStatesForPatchGroupError::Validation(error_message.to_string()),_ => DescribeInstancePatchStatesForPatchGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInstancePatchStatesForPatchGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeInstancePatchStatesForPatchGroupError {
                    fn from(err: serde_json::error::Error) -> DescribeInstancePatchStatesForPatchGroupError {
                        DescribeInstancePatchStatesForPatchGroupError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeInstancePatchStatesForPatchGroupError {
                    fn from(err: CredentialsError) -> DescribeInstancePatchStatesForPatchGroupError {
                        DescribeInstancePatchStatesForPatchGroupError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeInstancePatchStatesForPatchGroupError {
                    fn from(err: HttpDispatchError) -> DescribeInstancePatchStatesForPatchGroupError {
                        DescribeInstancePatchStatesForPatchGroupError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeInstancePatchStatesForPatchGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeInstancePatchStatesForPatchGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeInstancePatchStatesForPatchGroupError::InternalServerError(ref cause) => cause,DescribeInstancePatchStatesForPatchGroupError::InvalidFilter(ref cause) => cause,DescribeInstancePatchStatesForPatchGroupError::InvalidNextToken(ref cause) => cause,DescribeInstancePatchStatesForPatchGroupError::Validation(ref cause) => cause,DescribeInstancePatchStatesForPatchGroupError::Credentials(ref err) => err.description(),DescribeInstancePatchStatesForPatchGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeInstancePatchStatesForPatchGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeInstancePatches
                #[derive(Debug, PartialEq)]
                pub enum DescribeInstancePatchesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
InvalidFilter(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeInstancePatchesError {
                    pub fn from_body(body: &str) -> DescribeInstancePatchesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeInstancePatchesError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => DescribeInstancePatchesError::InvalidInstanceId(String::from(error_message)),"InvalidFilter" => DescribeInstancePatchesError::InvalidFilter(String::from(error_message)),"InvalidNextToken" => DescribeInstancePatchesError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeInstancePatchesError::Validation(error_message.to_string()),_ => DescribeInstancePatchesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeInstancePatchesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeInstancePatchesError {
                    fn from(err: serde_json::error::Error) -> DescribeInstancePatchesError {
                        DescribeInstancePatchesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeInstancePatchesError {
                    fn from(err: CredentialsError) -> DescribeInstancePatchesError {
                        DescribeInstancePatchesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeInstancePatchesError {
                    fn from(err: HttpDispatchError) -> DescribeInstancePatchesError {
                        DescribeInstancePatchesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeInstancePatchesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeInstancePatchesError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeInstancePatchesError::InternalServerError(ref cause) => cause,DescribeInstancePatchesError::InvalidInstanceId(ref cause) => cause,DescribeInstancePatchesError::InvalidFilter(ref cause) => cause,DescribeInstancePatchesError::InvalidNextToken(ref cause) => cause,DescribeInstancePatchesError::Validation(ref cause) => cause,DescribeInstancePatchesError::Credentials(ref err) => err.description(),DescribeInstancePatchesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeInstancePatchesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindowExecutionTaskInvocations
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(String::from(error_message)),"InternalServerError" => DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowExecutionTaskInvocationsError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionTaskInvocationsError {
                        DescribeMaintenanceWindowExecutionTaskInvocationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowExecutionTaskInvocationsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowExecutionTaskInvocationsError::DoesNotExist(ref cause) => cause,DescribeMaintenanceWindowExecutionTaskInvocationsError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowExecutionTaskInvocationsError::Validation(ref cause) => cause,DescribeMaintenanceWindowExecutionTaskInvocationsError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowExecutionTaskInvocationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowExecutionTaskInvocationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindowExecutionTasks
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowExecutionTasksError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowExecutionTasksError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionTasksError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(String::from(error_message)),"InternalServerError" => DescribeMaintenanceWindowExecutionTasksError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowExecutionTasksError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowExecutionTasksError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowExecutionTasksError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionTasksError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowExecutionTasksError {
                        DescribeMaintenanceWindowExecutionTasksError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowExecutionTasksError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionTasksError {
                        DescribeMaintenanceWindowExecutionTasksError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionTasksError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionTasksError {
                        DescribeMaintenanceWindowExecutionTasksError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowExecutionTasksError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowExecutionTasksError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowExecutionTasksError::DoesNotExist(ref cause) => cause,DescribeMaintenanceWindowExecutionTasksError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowExecutionTasksError::Validation(ref cause) => cause,DescribeMaintenanceWindowExecutionTasksError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowExecutionTasksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowExecutionTasksError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindowExecutions
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowExecutionsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowExecutionsError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowExecutionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeMaintenanceWindowExecutionsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowExecutionsError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowExecutionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowExecutionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowExecutionsError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowExecutionsError {
                        DescribeMaintenanceWindowExecutionsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowExecutionsError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowExecutionsError {
                        DescribeMaintenanceWindowExecutionsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowExecutionsError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowExecutionsError {
                        DescribeMaintenanceWindowExecutionsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowExecutionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowExecutionsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowExecutionsError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowExecutionsError::Validation(ref cause) => cause,DescribeMaintenanceWindowExecutionsError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowExecutionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowExecutionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindowTargets
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowTargetsError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowTargetsError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowTargetsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DescribeMaintenanceWindowTargetsError::DoesNotExist(String::from(error_message)),"InternalServerError" => DescribeMaintenanceWindowTargetsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowTargetsError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowTargetsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowTargetsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowTargetsError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowTargetsError {
                        DescribeMaintenanceWindowTargetsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowTargetsError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowTargetsError {
                        DescribeMaintenanceWindowTargetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowTargetsError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowTargetsError {
                        DescribeMaintenanceWindowTargetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowTargetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowTargetsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowTargetsError::DoesNotExist(ref cause) => cause,DescribeMaintenanceWindowTargetsError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowTargetsError::Validation(ref cause) => cause,DescribeMaintenanceWindowTargetsError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowTargetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowTargetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindowTasks
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowTasksError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowTasksError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowTasksError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => DescribeMaintenanceWindowTasksError::DoesNotExist(String::from(error_message)),"InternalServerError" => DescribeMaintenanceWindowTasksError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowTasksError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowTasksError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowTasksError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowTasksError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowTasksError {
                        DescribeMaintenanceWindowTasksError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowTasksError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowTasksError {
                        DescribeMaintenanceWindowTasksError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowTasksError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowTasksError {
                        DescribeMaintenanceWindowTasksError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowTasksError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowTasksError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowTasksError::DoesNotExist(ref cause) => cause,DescribeMaintenanceWindowTasksError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowTasksError::Validation(ref cause) => cause,DescribeMaintenanceWindowTasksError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowTasksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowTasksError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeMaintenanceWindows
                #[derive(Debug, PartialEq)]
                pub enum DescribeMaintenanceWindowsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeMaintenanceWindowsError {
                    pub fn from_body(body: &str) -> DescribeMaintenanceWindowsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeMaintenanceWindowsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribeMaintenanceWindowsError::Validation(error_message.to_string()),_ => DescribeMaintenanceWindowsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeMaintenanceWindowsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeMaintenanceWindowsError {
                    fn from(err: serde_json::error::Error) -> DescribeMaintenanceWindowsError {
                        DescribeMaintenanceWindowsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeMaintenanceWindowsError {
                    fn from(err: CredentialsError) -> DescribeMaintenanceWindowsError {
                        DescribeMaintenanceWindowsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeMaintenanceWindowsError {
                    fn from(err: HttpDispatchError) -> DescribeMaintenanceWindowsError {
                        DescribeMaintenanceWindowsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeMaintenanceWindowsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeMaintenanceWindowsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeMaintenanceWindowsError::InternalServerError(ref cause) => cause,DescribeMaintenanceWindowsError::Validation(ref cause) => cause,DescribeMaintenanceWindowsError::Credentials(ref err) => err.description(),DescribeMaintenanceWindowsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeMaintenanceWindowsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribeParameters
                #[derive(Debug, PartialEq)]
                pub enum DescribeParametersError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The filter value is not valid. Verify the value and try again.</p>
InvalidFilterValue(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribeParametersError {
                    pub fn from_body(body: &str) -> DescribeParametersError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribeParametersError::InternalServerError(String::from(error_message)),"InvalidFilterValue" => DescribeParametersError::InvalidFilterValue(String::from(error_message)),"InvalidNextToken" => DescribeParametersError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribeParametersError::Validation(error_message.to_string()),_ => DescribeParametersError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribeParametersError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribeParametersError {
                    fn from(err: serde_json::error::Error) -> DescribeParametersError {
                        DescribeParametersError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribeParametersError {
                    fn from(err: CredentialsError) -> DescribeParametersError {
                        DescribeParametersError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribeParametersError {
                    fn from(err: HttpDispatchError) -> DescribeParametersError {
                        DescribeParametersError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribeParametersError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribeParametersError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribeParametersError::InternalServerError(ref cause) => cause,DescribeParametersError::InvalidFilterValue(ref cause) => cause,DescribeParametersError::InvalidNextToken(ref cause) => cause,DescribeParametersError::Validation(ref cause) => cause,DescribeParametersError::Credentials(ref err) => err.description(),DescribeParametersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribeParametersError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribePatchBaselines
                #[derive(Debug, PartialEq)]
                pub enum DescribePatchBaselinesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribePatchBaselinesError {
                    pub fn from_body(body: &str) -> DescribePatchBaselinesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribePatchBaselinesError::InternalServerError(String::from(error_message)),"ValidationException" => DescribePatchBaselinesError::Validation(error_message.to_string()),_ => DescribePatchBaselinesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribePatchBaselinesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribePatchBaselinesError {
                    fn from(err: serde_json::error::Error) -> DescribePatchBaselinesError {
                        DescribePatchBaselinesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribePatchBaselinesError {
                    fn from(err: CredentialsError) -> DescribePatchBaselinesError {
                        DescribePatchBaselinesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribePatchBaselinesError {
                    fn from(err: HttpDispatchError) -> DescribePatchBaselinesError {
                        DescribePatchBaselinesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribePatchBaselinesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribePatchBaselinesError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribePatchBaselinesError::InternalServerError(ref cause) => cause,DescribePatchBaselinesError::Validation(ref cause) => cause,DescribePatchBaselinesError::Credentials(ref err) => err.description(),DescribePatchBaselinesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribePatchBaselinesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribePatchGroupState
                #[derive(Debug, PartialEq)]
                pub enum DescribePatchGroupStateError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribePatchGroupStateError {
                    pub fn from_body(body: &str) -> DescribePatchGroupStateError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribePatchGroupStateError::InternalServerError(String::from(error_message)),"InvalidNextToken" => DescribePatchGroupStateError::InvalidNextToken(String::from(error_message)),"ValidationException" => DescribePatchGroupStateError::Validation(error_message.to_string()),_ => DescribePatchGroupStateError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribePatchGroupStateError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribePatchGroupStateError {
                    fn from(err: serde_json::error::Error) -> DescribePatchGroupStateError {
                        DescribePatchGroupStateError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribePatchGroupStateError {
                    fn from(err: CredentialsError) -> DescribePatchGroupStateError {
                        DescribePatchGroupStateError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribePatchGroupStateError {
                    fn from(err: HttpDispatchError) -> DescribePatchGroupStateError {
                        DescribePatchGroupStateError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribePatchGroupStateError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribePatchGroupStateError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribePatchGroupStateError::InternalServerError(ref cause) => cause,DescribePatchGroupStateError::InvalidNextToken(ref cause) => cause,DescribePatchGroupStateError::Validation(ref cause) => cause,DescribePatchGroupStateError::Credentials(ref err) => err.description(),DescribePatchGroupStateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribePatchGroupStateError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DescribePatchGroups
                #[derive(Debug, PartialEq)]
                pub enum DescribePatchGroupsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DescribePatchGroupsError {
                    pub fn from_body(body: &str) -> DescribePatchGroupsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => DescribePatchGroupsError::InternalServerError(String::from(error_message)),"ValidationException" => DescribePatchGroupsError::Validation(error_message.to_string()),_ => DescribePatchGroupsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => DescribePatchGroupsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for DescribePatchGroupsError {
                    fn from(err: serde_json::error::Error) -> DescribePatchGroupsError {
                        DescribePatchGroupsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for DescribePatchGroupsError {
                    fn from(err: CredentialsError) -> DescribePatchGroupsError {
                        DescribePatchGroupsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DescribePatchGroupsError {
                    fn from(err: HttpDispatchError) -> DescribePatchGroupsError {
                        DescribePatchGroupsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DescribePatchGroupsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DescribePatchGroupsError {
                    fn description(&self) -> &str {
                        match *self {
                            DescribePatchGroupsError::InternalServerError(ref cause) => cause,DescribePatchGroupsError::Validation(ref cause) => cause,DescribePatchGroupsError::Credentials(ref err) => err.description(),DescribePatchGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DescribePatchGroupsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetAutomationExecution
                #[derive(Debug, PartialEq)]
                pub enum GetAutomationExecutionError {
                    
///<p>There is no automation execution information for the requested automation execution ID.</p>
AutomationExecutionNotFound(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetAutomationExecutionError {
                    pub fn from_body(body: &str) -> GetAutomationExecutionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AutomationExecutionNotFoundException" => GetAutomationExecutionError::AutomationExecutionNotFound(String::from(error_message)),"InternalServerError" => GetAutomationExecutionError::InternalServerError(String::from(error_message)),"ValidationException" => GetAutomationExecutionError::Validation(error_message.to_string()),_ => GetAutomationExecutionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetAutomationExecutionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetAutomationExecutionError {
                    fn from(err: serde_json::error::Error) -> GetAutomationExecutionError {
                        GetAutomationExecutionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetAutomationExecutionError {
                    fn from(err: CredentialsError) -> GetAutomationExecutionError {
                        GetAutomationExecutionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetAutomationExecutionError {
                    fn from(err: HttpDispatchError) -> GetAutomationExecutionError {
                        GetAutomationExecutionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetAutomationExecutionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetAutomationExecutionError {
                    fn description(&self) -> &str {
                        match *self {
                            GetAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,GetAutomationExecutionError::InternalServerError(ref cause) => cause,GetAutomationExecutionError::Validation(ref cause) => cause,GetAutomationExecutionError::Credentials(ref err) => err.description(),GetAutomationExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetAutomationExecutionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetCommandInvocation
                #[derive(Debug, PartialEq)]
                pub enum GetCommandInvocationError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///
InvalidCommandId(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The plugin name is not valid.</p>
InvalidPluginName(String),
///<p>The command ID and instance ID you specified did not match any invocations. Verify the command ID adn the instance ID and try again. </p>
InvocationDoesNotExist(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetCommandInvocationError {
                    pub fn from_body(body: &str) -> GetCommandInvocationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetCommandInvocationError::InternalServerError(String::from(error_message)),"InvalidCommandId" => GetCommandInvocationError::InvalidCommandId(String::from(error_message)),"InvalidInstanceId" => GetCommandInvocationError::InvalidInstanceId(String::from(error_message)),"InvalidPluginName" => GetCommandInvocationError::InvalidPluginName(String::from(error_message)),"InvocationDoesNotExist" => GetCommandInvocationError::InvocationDoesNotExist(String::from(error_message)),"ValidationException" => GetCommandInvocationError::Validation(error_message.to_string()),_ => GetCommandInvocationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetCommandInvocationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetCommandInvocationError {
                    fn from(err: serde_json::error::Error) -> GetCommandInvocationError {
                        GetCommandInvocationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetCommandInvocationError {
                    fn from(err: CredentialsError) -> GetCommandInvocationError {
                        GetCommandInvocationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetCommandInvocationError {
                    fn from(err: HttpDispatchError) -> GetCommandInvocationError {
                        GetCommandInvocationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetCommandInvocationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetCommandInvocationError {
                    fn description(&self) -> &str {
                        match *self {
                            GetCommandInvocationError::InternalServerError(ref cause) => cause,GetCommandInvocationError::InvalidCommandId(ref cause) => cause,GetCommandInvocationError::InvalidInstanceId(ref cause) => cause,GetCommandInvocationError::InvalidPluginName(ref cause) => cause,GetCommandInvocationError::InvocationDoesNotExist(ref cause) => cause,GetCommandInvocationError::Validation(ref cause) => cause,GetCommandInvocationError::Credentials(ref err) => err.description(),GetCommandInvocationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetCommandInvocationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDefaultPatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum GetDefaultPatchBaselineError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDefaultPatchBaselineError {
                    pub fn from_body(body: &str) -> GetDefaultPatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetDefaultPatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => GetDefaultPatchBaselineError::Validation(error_message.to_string()),_ => GetDefaultPatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDefaultPatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDefaultPatchBaselineError {
                    fn from(err: serde_json::error::Error) -> GetDefaultPatchBaselineError {
                        GetDefaultPatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetDefaultPatchBaselineError {
                    fn from(err: CredentialsError) -> GetDefaultPatchBaselineError {
                        GetDefaultPatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetDefaultPatchBaselineError {
                    fn from(err: HttpDispatchError) -> GetDefaultPatchBaselineError {
                        GetDefaultPatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetDefaultPatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDefaultPatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDefaultPatchBaselineError::InternalServerError(ref cause) => cause,GetDefaultPatchBaselineError::Validation(ref cause) => cause,GetDefaultPatchBaselineError::Credentials(ref err) => err.description(),GetDefaultPatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDefaultPatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDeployablePatchSnapshotForInstance
                #[derive(Debug, PartialEq)]
                pub enum GetDeployablePatchSnapshotForInstanceError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDeployablePatchSnapshotForInstanceError {
                    pub fn from_body(body: &str) -> GetDeployablePatchSnapshotForInstanceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetDeployablePatchSnapshotForInstanceError::InternalServerError(String::from(error_message)),"ValidationException" => GetDeployablePatchSnapshotForInstanceError::Validation(error_message.to_string()),_ => GetDeployablePatchSnapshotForInstanceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDeployablePatchSnapshotForInstanceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDeployablePatchSnapshotForInstanceError {
                    fn from(err: serde_json::error::Error) -> GetDeployablePatchSnapshotForInstanceError {
                        GetDeployablePatchSnapshotForInstanceError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetDeployablePatchSnapshotForInstanceError {
                    fn from(err: CredentialsError) -> GetDeployablePatchSnapshotForInstanceError {
                        GetDeployablePatchSnapshotForInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetDeployablePatchSnapshotForInstanceError {
                    fn from(err: HttpDispatchError) -> GetDeployablePatchSnapshotForInstanceError {
                        GetDeployablePatchSnapshotForInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetDeployablePatchSnapshotForInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDeployablePatchSnapshotForInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDeployablePatchSnapshotForInstanceError::InternalServerError(ref cause) => cause,GetDeployablePatchSnapshotForInstanceError::Validation(ref cause) => cause,GetDeployablePatchSnapshotForInstanceError::Credentials(ref err) => err.description(),GetDeployablePatchSnapshotForInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDeployablePatchSnapshotForInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetDocument
                #[derive(Debug, PartialEq)]
                pub enum GetDocumentError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetDocumentError {
                    pub fn from_body(body: &str) -> GetDocumentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetDocumentError::InternalServerError(String::from(error_message)),"InvalidDocument" => GetDocumentError::InvalidDocument(String::from(error_message)),"InvalidDocumentVersion" => GetDocumentError::InvalidDocumentVersion(String::from(error_message)),"ValidationException" => GetDocumentError::Validation(error_message.to_string()),_ => GetDocumentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetDocumentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetDocumentError {
                    fn from(err: serde_json::error::Error) -> GetDocumentError {
                        GetDocumentError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetDocumentError {
                    fn from(err: CredentialsError) -> GetDocumentError {
                        GetDocumentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetDocumentError {
                    fn from(err: HttpDispatchError) -> GetDocumentError {
                        GetDocumentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetDocumentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetDocumentError {
                    fn description(&self) -> &str {
                        match *self {
                            GetDocumentError::InternalServerError(ref cause) => cause,GetDocumentError::InvalidDocument(ref cause) => cause,GetDocumentError::InvalidDocumentVersion(ref cause) => cause,GetDocumentError::Validation(ref cause) => cause,GetDocumentError::Credentials(ref err) => err.description(),GetDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetDocumentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetInventory
                #[derive(Debug, PartialEq)]
                pub enum GetInventoryError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
InvalidFilter(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>The parameter type name is not valid.</p>
InvalidTypeName(String),
///<p>The specified inventory item result attribute is not valid.</p>
InvalidResultAttribute(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetInventoryError {
                    pub fn from_body(body: &str) -> GetInventoryError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetInventoryError::InternalServerError(String::from(error_message)),"InvalidFilter" => GetInventoryError::InvalidFilter(String::from(error_message)),"InvalidNextToken" => GetInventoryError::InvalidNextToken(String::from(error_message)),"InvalidTypeNameException" => GetInventoryError::InvalidTypeName(String::from(error_message)),"InvalidResultAttributeException" => GetInventoryError::InvalidResultAttribute(String::from(error_message)),"ValidationException" => GetInventoryError::Validation(error_message.to_string()),_ => GetInventoryError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetInventoryError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetInventoryError {
                    fn from(err: serde_json::error::Error) -> GetInventoryError {
                        GetInventoryError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetInventoryError {
                    fn from(err: CredentialsError) -> GetInventoryError {
                        GetInventoryError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetInventoryError {
                    fn from(err: HttpDispatchError) -> GetInventoryError {
                        GetInventoryError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetInventoryError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetInventoryError {
                    fn description(&self) -> &str {
                        match *self {
                            GetInventoryError::InternalServerError(ref cause) => cause,GetInventoryError::InvalidFilter(ref cause) => cause,GetInventoryError::InvalidNextToken(ref cause) => cause,GetInventoryError::InvalidTypeName(ref cause) => cause,GetInventoryError::InvalidResultAttribute(ref cause) => cause,GetInventoryError::Validation(ref cause) => cause,GetInventoryError::Credentials(ref err) => err.description(),GetInventoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetInventoryError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetInventorySchema
                #[derive(Debug, PartialEq)]
                pub enum GetInventorySchemaError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The parameter type name is not valid.</p>
InvalidTypeName(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetInventorySchemaError {
                    pub fn from_body(body: &str) -> GetInventorySchemaError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetInventorySchemaError::InternalServerError(String::from(error_message)),"InvalidTypeNameException" => GetInventorySchemaError::InvalidTypeName(String::from(error_message)),"InvalidNextToken" => GetInventorySchemaError::InvalidNextToken(String::from(error_message)),"ValidationException" => GetInventorySchemaError::Validation(error_message.to_string()),_ => GetInventorySchemaError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetInventorySchemaError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetInventorySchemaError {
                    fn from(err: serde_json::error::Error) -> GetInventorySchemaError {
                        GetInventorySchemaError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetInventorySchemaError {
                    fn from(err: CredentialsError) -> GetInventorySchemaError {
                        GetInventorySchemaError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetInventorySchemaError {
                    fn from(err: HttpDispatchError) -> GetInventorySchemaError {
                        GetInventorySchemaError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetInventorySchemaError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetInventorySchemaError {
                    fn description(&self) -> &str {
                        match *self {
                            GetInventorySchemaError::InternalServerError(ref cause) => cause,GetInventorySchemaError::InvalidTypeName(ref cause) => cause,GetInventorySchemaError::InvalidNextToken(ref cause) => cause,GetInventorySchemaError::Validation(ref cause) => cause,GetInventorySchemaError::Credentials(ref err) => err.description(),GetInventorySchemaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetInventorySchemaError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum GetMaintenanceWindowError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetMaintenanceWindowError {
                    pub fn from_body(body: &str) -> GetMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => GetMaintenanceWindowError::DoesNotExist(String::from(error_message)),"InternalServerError" => GetMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => GetMaintenanceWindowError::Validation(error_message.to_string()),_ => GetMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowError {
                        GetMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetMaintenanceWindowError {
                    fn from(err: CredentialsError) -> GetMaintenanceWindowError {
                        GetMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> GetMaintenanceWindowError {
                        GetMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            GetMaintenanceWindowError::DoesNotExist(ref cause) => cause,GetMaintenanceWindowError::InternalServerError(ref cause) => cause,GetMaintenanceWindowError::Validation(ref cause) => cause,GetMaintenanceWindowError::Credentials(ref err) => err.description(),GetMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetMaintenanceWindowExecution
                #[derive(Debug, PartialEq)]
                pub enum GetMaintenanceWindowExecutionError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetMaintenanceWindowExecutionError {
                    pub fn from_body(body: &str) -> GetMaintenanceWindowExecutionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => GetMaintenanceWindowExecutionError::DoesNotExist(String::from(error_message)),"InternalServerError" => GetMaintenanceWindowExecutionError::InternalServerError(String::from(error_message)),"ValidationException" => GetMaintenanceWindowExecutionError::Validation(error_message.to_string()),_ => GetMaintenanceWindowExecutionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetMaintenanceWindowExecutionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetMaintenanceWindowExecutionError {
                    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowExecutionError {
                        GetMaintenanceWindowExecutionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetMaintenanceWindowExecutionError {
                    fn from(err: CredentialsError) -> GetMaintenanceWindowExecutionError {
                        GetMaintenanceWindowExecutionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetMaintenanceWindowExecutionError {
                    fn from(err: HttpDispatchError) -> GetMaintenanceWindowExecutionError {
                        GetMaintenanceWindowExecutionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetMaintenanceWindowExecutionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetMaintenanceWindowExecutionError {
                    fn description(&self) -> &str {
                        match *self {
                            GetMaintenanceWindowExecutionError::DoesNotExist(ref cause) => cause,GetMaintenanceWindowExecutionError::InternalServerError(ref cause) => cause,GetMaintenanceWindowExecutionError::Validation(ref cause) => cause,GetMaintenanceWindowExecutionError::Credentials(ref err) => err.description(),GetMaintenanceWindowExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetMaintenanceWindowExecutionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetMaintenanceWindowExecutionTask
                #[derive(Debug, PartialEq)]
                pub enum GetMaintenanceWindowExecutionTaskError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetMaintenanceWindowExecutionTaskError {
                    pub fn from_body(body: &str) -> GetMaintenanceWindowExecutionTaskError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => GetMaintenanceWindowExecutionTaskError::DoesNotExist(String::from(error_message)),"InternalServerError" => GetMaintenanceWindowExecutionTaskError::InternalServerError(String::from(error_message)),"ValidationException" => GetMaintenanceWindowExecutionTaskError::Validation(error_message.to_string()),_ => GetMaintenanceWindowExecutionTaskError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetMaintenanceWindowExecutionTaskError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetMaintenanceWindowExecutionTaskError {
                    fn from(err: serde_json::error::Error) -> GetMaintenanceWindowExecutionTaskError {
                        GetMaintenanceWindowExecutionTaskError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetMaintenanceWindowExecutionTaskError {
                    fn from(err: CredentialsError) -> GetMaintenanceWindowExecutionTaskError {
                        GetMaintenanceWindowExecutionTaskError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetMaintenanceWindowExecutionTaskError {
                    fn from(err: HttpDispatchError) -> GetMaintenanceWindowExecutionTaskError {
                        GetMaintenanceWindowExecutionTaskError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetMaintenanceWindowExecutionTaskError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetMaintenanceWindowExecutionTaskError {
                    fn description(&self) -> &str {
                        match *self {
                            GetMaintenanceWindowExecutionTaskError::DoesNotExist(ref cause) => cause,GetMaintenanceWindowExecutionTaskError::InternalServerError(ref cause) => cause,GetMaintenanceWindowExecutionTaskError::Validation(ref cause) => cause,GetMaintenanceWindowExecutionTaskError::Credentials(ref err) => err.description(),GetMaintenanceWindowExecutionTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetMaintenanceWindowExecutionTaskError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetParameterHistory
                #[derive(Debug, PartialEq)]
                pub enum GetParameterHistoryError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The parameter could not be found. Verify the name and try again.</p>
ParameterNotFound(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetParameterHistoryError {
                    pub fn from_body(body: &str) -> GetParameterHistoryError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetParameterHistoryError::InternalServerError(String::from(error_message)),"ParameterNotFound" => GetParameterHistoryError::ParameterNotFound(String::from(error_message)),"InvalidNextToken" => GetParameterHistoryError::InvalidNextToken(String::from(error_message)),"ValidationException" => GetParameterHistoryError::Validation(error_message.to_string()),_ => GetParameterHistoryError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetParameterHistoryError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetParameterHistoryError {
                    fn from(err: serde_json::error::Error) -> GetParameterHistoryError {
                        GetParameterHistoryError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetParameterHistoryError {
                    fn from(err: CredentialsError) -> GetParameterHistoryError {
                        GetParameterHistoryError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetParameterHistoryError {
                    fn from(err: HttpDispatchError) -> GetParameterHistoryError {
                        GetParameterHistoryError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetParameterHistoryError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetParameterHistoryError {
                    fn description(&self) -> &str {
                        match *self {
                            GetParameterHistoryError::InternalServerError(ref cause) => cause,GetParameterHistoryError::ParameterNotFound(ref cause) => cause,GetParameterHistoryError::InvalidNextToken(ref cause) => cause,GetParameterHistoryError::Validation(ref cause) => cause,GetParameterHistoryError::Credentials(ref err) => err.description(),GetParameterHistoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetParameterHistoryError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetParameters
                #[derive(Debug, PartialEq)]
                pub enum GetParametersError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetParametersError {
                    pub fn from_body(body: &str) -> GetParametersError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetParametersError::InternalServerError(String::from(error_message)),"ValidationException" => GetParametersError::Validation(error_message.to_string()),_ => GetParametersError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetParametersError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetParametersError {
                    fn from(err: serde_json::error::Error) -> GetParametersError {
                        GetParametersError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetParametersError {
                    fn from(err: CredentialsError) -> GetParametersError {
                        GetParametersError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetParametersError {
                    fn from(err: HttpDispatchError) -> GetParametersError {
                        GetParametersError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetParametersError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetParametersError {
                    fn description(&self) -> &str {
                        match *self {
                            GetParametersError::InternalServerError(ref cause) => cause,GetParametersError::Validation(ref cause) => cause,GetParametersError::Credentials(ref err) => err.description(),GetParametersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetParametersError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetPatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum GetPatchBaselineError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetPatchBaselineError {
                    pub fn from_body(body: &str) -> GetPatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => GetPatchBaselineError::DoesNotExist(String::from(error_message)),"InvalidResourceId" => GetPatchBaselineError::InvalidResourceId(String::from(error_message)),"InternalServerError" => GetPatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => GetPatchBaselineError::Validation(error_message.to_string()),_ => GetPatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetPatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetPatchBaselineError {
                    fn from(err: serde_json::error::Error) -> GetPatchBaselineError {
                        GetPatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetPatchBaselineError {
                    fn from(err: CredentialsError) -> GetPatchBaselineError {
                        GetPatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetPatchBaselineError {
                    fn from(err: HttpDispatchError) -> GetPatchBaselineError {
                        GetPatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetPatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetPatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            GetPatchBaselineError::DoesNotExist(ref cause) => cause,GetPatchBaselineError::InvalidResourceId(ref cause) => cause,GetPatchBaselineError::InternalServerError(ref cause) => cause,GetPatchBaselineError::Validation(ref cause) => cause,GetPatchBaselineError::Credentials(ref err) => err.description(),GetPatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetPatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetPatchBaselineForPatchGroup
                #[derive(Debug, PartialEq)]
                pub enum GetPatchBaselineForPatchGroupError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetPatchBaselineForPatchGroupError {
                    pub fn from_body(body: &str) -> GetPatchBaselineForPatchGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => GetPatchBaselineForPatchGroupError::InternalServerError(String::from(error_message)),"ValidationException" => GetPatchBaselineForPatchGroupError::Validation(error_message.to_string()),_ => GetPatchBaselineForPatchGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => GetPatchBaselineForPatchGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for GetPatchBaselineForPatchGroupError {
                    fn from(err: serde_json::error::Error) -> GetPatchBaselineForPatchGroupError {
                        GetPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for GetPatchBaselineForPatchGroupError {
                    fn from(err: CredentialsError) -> GetPatchBaselineForPatchGroupError {
                        GetPatchBaselineForPatchGroupError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetPatchBaselineForPatchGroupError {
                    fn from(err: HttpDispatchError) -> GetPatchBaselineForPatchGroupError {
                        GetPatchBaselineForPatchGroupError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetPatchBaselineForPatchGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetPatchBaselineForPatchGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            GetPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,GetPatchBaselineForPatchGroupError::Validation(ref cause) => cause,GetPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),GetPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetPatchBaselineForPatchGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListAssociations
                #[derive(Debug, PartialEq)]
                pub enum ListAssociationsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListAssociationsError {
                    pub fn from_body(body: &str) -> ListAssociationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListAssociationsError::InternalServerError(String::from(error_message)),"InvalidNextToken" => ListAssociationsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListAssociationsError::Validation(error_message.to_string()),_ => ListAssociationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListAssociationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListAssociationsError {
                    fn from(err: serde_json::error::Error) -> ListAssociationsError {
                        ListAssociationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListAssociationsError {
                    fn from(err: CredentialsError) -> ListAssociationsError {
                        ListAssociationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListAssociationsError {
                    fn from(err: HttpDispatchError) -> ListAssociationsError {
                        ListAssociationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListAssociationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListAssociationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListAssociationsError::InternalServerError(ref cause) => cause,ListAssociationsError::InvalidNextToken(ref cause) => cause,ListAssociationsError::Validation(ref cause) => cause,ListAssociationsError::Credentials(ref err) => err.description(),ListAssociationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListAssociationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListCommandInvocations
                #[derive(Debug, PartialEq)]
                pub enum ListCommandInvocationsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///
InvalidCommandId(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified key is not valid.</p>
InvalidFilterKey(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListCommandInvocationsError {
                    pub fn from_body(body: &str) -> ListCommandInvocationsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListCommandInvocationsError::InternalServerError(String::from(error_message)),"InvalidCommandId" => ListCommandInvocationsError::InvalidCommandId(String::from(error_message)),"InvalidInstanceId" => ListCommandInvocationsError::InvalidInstanceId(String::from(error_message)),"InvalidFilterKey" => ListCommandInvocationsError::InvalidFilterKey(String::from(error_message)),"InvalidNextToken" => ListCommandInvocationsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListCommandInvocationsError::Validation(error_message.to_string()),_ => ListCommandInvocationsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListCommandInvocationsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListCommandInvocationsError {
                    fn from(err: serde_json::error::Error) -> ListCommandInvocationsError {
                        ListCommandInvocationsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListCommandInvocationsError {
                    fn from(err: CredentialsError) -> ListCommandInvocationsError {
                        ListCommandInvocationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListCommandInvocationsError {
                    fn from(err: HttpDispatchError) -> ListCommandInvocationsError {
                        ListCommandInvocationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListCommandInvocationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListCommandInvocationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListCommandInvocationsError::InternalServerError(ref cause) => cause,ListCommandInvocationsError::InvalidCommandId(ref cause) => cause,ListCommandInvocationsError::InvalidInstanceId(ref cause) => cause,ListCommandInvocationsError::InvalidFilterKey(ref cause) => cause,ListCommandInvocationsError::InvalidNextToken(ref cause) => cause,ListCommandInvocationsError::Validation(ref cause) => cause,ListCommandInvocationsError::Credentials(ref err) => err.description(),ListCommandInvocationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListCommandInvocationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListCommands
                #[derive(Debug, PartialEq)]
                pub enum ListCommandsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///
InvalidCommandId(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified key is not valid.</p>
InvalidFilterKey(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListCommandsError {
                    pub fn from_body(body: &str) -> ListCommandsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListCommandsError::InternalServerError(String::from(error_message)),"InvalidCommandId" => ListCommandsError::InvalidCommandId(String::from(error_message)),"InvalidInstanceId" => ListCommandsError::InvalidInstanceId(String::from(error_message)),"InvalidFilterKey" => ListCommandsError::InvalidFilterKey(String::from(error_message)),"InvalidNextToken" => ListCommandsError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListCommandsError::Validation(error_message.to_string()),_ => ListCommandsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListCommandsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListCommandsError {
                    fn from(err: serde_json::error::Error) -> ListCommandsError {
                        ListCommandsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListCommandsError {
                    fn from(err: CredentialsError) -> ListCommandsError {
                        ListCommandsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListCommandsError {
                    fn from(err: HttpDispatchError) -> ListCommandsError {
                        ListCommandsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListCommandsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListCommandsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListCommandsError::InternalServerError(ref cause) => cause,ListCommandsError::InvalidCommandId(ref cause) => cause,ListCommandsError::InvalidInstanceId(ref cause) => cause,ListCommandsError::InvalidFilterKey(ref cause) => cause,ListCommandsError::InvalidNextToken(ref cause) => cause,ListCommandsError::Validation(ref cause) => cause,ListCommandsError::Credentials(ref err) => err.description(),ListCommandsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListCommandsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDocumentVersions
                #[derive(Debug, PartialEq)]
                pub enum ListDocumentVersionsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDocumentVersionsError {
                    pub fn from_body(body: &str) -> ListDocumentVersionsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListDocumentVersionsError::InternalServerError(String::from(error_message)),"InvalidNextToken" => ListDocumentVersionsError::InvalidNextToken(String::from(error_message)),"InvalidDocument" => ListDocumentVersionsError::InvalidDocument(String::from(error_message)),"ValidationException" => ListDocumentVersionsError::Validation(error_message.to_string()),_ => ListDocumentVersionsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDocumentVersionsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDocumentVersionsError {
                    fn from(err: serde_json::error::Error) -> ListDocumentVersionsError {
                        ListDocumentVersionsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListDocumentVersionsError {
                    fn from(err: CredentialsError) -> ListDocumentVersionsError {
                        ListDocumentVersionsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListDocumentVersionsError {
                    fn from(err: HttpDispatchError) -> ListDocumentVersionsError {
                        ListDocumentVersionsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListDocumentVersionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDocumentVersionsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDocumentVersionsError::InternalServerError(ref cause) => cause,ListDocumentVersionsError::InvalidNextToken(ref cause) => cause,ListDocumentVersionsError::InvalidDocument(ref cause) => cause,ListDocumentVersionsError::Validation(ref cause) => cause,ListDocumentVersionsError::Credentials(ref err) => err.description(),ListDocumentVersionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDocumentVersionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListDocuments
                #[derive(Debug, PartialEq)]
                pub enum ListDocumentsError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),
///<p>The specified key is not valid.</p>
InvalidFilterKey(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListDocumentsError {
                    pub fn from_body(body: &str) -> ListDocumentsError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListDocumentsError::InternalServerError(String::from(error_message)),"InvalidNextToken" => ListDocumentsError::InvalidNextToken(String::from(error_message)),"InvalidFilterKey" => ListDocumentsError::InvalidFilterKey(String::from(error_message)),"ValidationException" => ListDocumentsError::Validation(error_message.to_string()),_ => ListDocumentsError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListDocumentsError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListDocumentsError {
                    fn from(err: serde_json::error::Error) -> ListDocumentsError {
                        ListDocumentsError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListDocumentsError {
                    fn from(err: CredentialsError) -> ListDocumentsError {
                        ListDocumentsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListDocumentsError {
                    fn from(err: HttpDispatchError) -> ListDocumentsError {
                        ListDocumentsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListDocumentsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListDocumentsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListDocumentsError::InternalServerError(ref cause) => cause,ListDocumentsError::InvalidNextToken(ref cause) => cause,ListDocumentsError::InvalidFilterKey(ref cause) => cause,ListDocumentsError::Validation(ref cause) => cause,ListDocumentsError::Credentials(ref err) => err.description(),ListDocumentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListDocumentsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListInventoryEntries
                #[derive(Debug, PartialEq)]
                pub enum ListInventoryEntriesError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The parameter type name is not valid.</p>
InvalidTypeName(String),
///<p>The filter name is not valid. Verify the you entered the correct name and try again.</p>
InvalidFilter(String),
///<p>The specified token is not valid.</p>
InvalidNextToken(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListInventoryEntriesError {
                    pub fn from_body(body: &str) -> ListInventoryEntriesError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ListInventoryEntriesError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => ListInventoryEntriesError::InvalidInstanceId(String::from(error_message)),"InvalidTypeNameException" => ListInventoryEntriesError::InvalidTypeName(String::from(error_message)),"InvalidFilter" => ListInventoryEntriesError::InvalidFilter(String::from(error_message)),"InvalidNextToken" => ListInventoryEntriesError::InvalidNextToken(String::from(error_message)),"ValidationException" => ListInventoryEntriesError::Validation(error_message.to_string()),_ => ListInventoryEntriesError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListInventoryEntriesError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListInventoryEntriesError {
                    fn from(err: serde_json::error::Error) -> ListInventoryEntriesError {
                        ListInventoryEntriesError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListInventoryEntriesError {
                    fn from(err: CredentialsError) -> ListInventoryEntriesError {
                        ListInventoryEntriesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListInventoryEntriesError {
                    fn from(err: HttpDispatchError) -> ListInventoryEntriesError {
                        ListInventoryEntriesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListInventoryEntriesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListInventoryEntriesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListInventoryEntriesError::InternalServerError(ref cause) => cause,ListInventoryEntriesError::InvalidInstanceId(ref cause) => cause,ListInventoryEntriesError::InvalidTypeName(ref cause) => cause,ListInventoryEntriesError::InvalidFilter(ref cause) => cause,ListInventoryEntriesError::InvalidNextToken(ref cause) => cause,ListInventoryEntriesError::Validation(ref cause) => cause,ListInventoryEntriesError::Credentials(ref err) => err.description(),ListInventoryEntriesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListInventoryEntriesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTagsForResource
                #[derive(Debug, PartialEq)]
                pub enum ListTagsForResourceError {
                    
///<p>The resource type is not valid. If you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
InvalidResourceType(String),
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTagsForResourceError {
                    pub fn from_body(body: &str) -> ListTagsForResourceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceType" => ListTagsForResourceError::InvalidResourceType(String::from(error_message)),"InvalidResourceId" => ListTagsForResourceError::InvalidResourceId(String::from(error_message)),"InternalServerError" => ListTagsForResourceError::InternalServerError(String::from(error_message)),"ValidationException" => ListTagsForResourceError::Validation(error_message.to_string()),_ => ListTagsForResourceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ListTagsForResourceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ListTagsForResourceError {
                    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
                        ListTagsForResourceError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ListTagsForResourceError {
                    fn from(err: CredentialsError) -> ListTagsForResourceError {
                        ListTagsForResourceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTagsForResourceError {
                    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
                        ListTagsForResourceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTagsForResourceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTagsForResourceError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTagsForResourceError::InvalidResourceType(ref cause) => cause,ListTagsForResourceError::InvalidResourceId(ref cause) => cause,ListTagsForResourceError::InternalServerError(ref cause) => cause,ListTagsForResourceError::Validation(ref cause) => cause,ListTagsForResourceError::Credentials(ref err) => err.description(),ListTagsForResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTagsForResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ModifyDocumentPermission
                #[derive(Debug, PartialEq)]
                pub enum ModifyDocumentPermissionError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The permission type is not supported. <i>Share</i> is the only supported permission type.</p>
InvalidPermissionType(String),
///<p>The document cannot be shared with more AWS user accounts. You can share a document with a maximum of 20 accounts. You can publicly share up to five documents. If you need to increase this limit, contact AWS Support.</p>
DocumentPermissionLimit(String),
///<p>You can have at most 200 active SSM documents.</p>
DocumentLimitExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ModifyDocumentPermissionError {
                    pub fn from_body(body: &str) -> ModifyDocumentPermissionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => ModifyDocumentPermissionError::InternalServerError(String::from(error_message)),"InvalidDocument" => ModifyDocumentPermissionError::InvalidDocument(String::from(error_message)),"InvalidPermissionType" => ModifyDocumentPermissionError::InvalidPermissionType(String::from(error_message)),"DocumentPermissionLimit" => ModifyDocumentPermissionError::DocumentPermissionLimit(String::from(error_message)),"DocumentLimitExceeded" => ModifyDocumentPermissionError::DocumentLimitExceeded(String::from(error_message)),"ValidationException" => ModifyDocumentPermissionError::Validation(error_message.to_string()),_ => ModifyDocumentPermissionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => ModifyDocumentPermissionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for ModifyDocumentPermissionError {
                    fn from(err: serde_json::error::Error) -> ModifyDocumentPermissionError {
                        ModifyDocumentPermissionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for ModifyDocumentPermissionError {
                    fn from(err: CredentialsError) -> ModifyDocumentPermissionError {
                        ModifyDocumentPermissionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ModifyDocumentPermissionError {
                    fn from(err: HttpDispatchError) -> ModifyDocumentPermissionError {
                        ModifyDocumentPermissionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ModifyDocumentPermissionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ModifyDocumentPermissionError {
                    fn description(&self) -> &str {
                        match *self {
                            ModifyDocumentPermissionError::InternalServerError(ref cause) => cause,ModifyDocumentPermissionError::InvalidDocument(ref cause) => cause,ModifyDocumentPermissionError::InvalidPermissionType(ref cause) => cause,ModifyDocumentPermissionError::DocumentPermissionLimit(ref cause) => cause,ModifyDocumentPermissionError::DocumentLimitExceeded(ref cause) => cause,ModifyDocumentPermissionError::Validation(ref cause) => cause,ModifyDocumentPermissionError::Credentials(ref err) => err.description(),ModifyDocumentPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ModifyDocumentPermissionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutInventory
                #[derive(Debug, PartialEq)]
                pub enum PutInventoryError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The parameter type name is not valid.</p>
InvalidTypeName(String),
///<p>One or more content items is not valid.</p>
InvalidItemContent(String),
///<p>The size of inventory data has exceeded the total size limit for the resource.</p>
TotalSizeLimitExceeded(String),
///<p>The inventory item size has exceeded the size limit.</p>
ItemSizeLimitExceeded(String),
///<p>The inventory item has invalid content. </p>
ItemContentMismatch(String),
///<p>You have exceeded the limit for custom schemas. Delete one or more custom schemas and try again.</p>
CustomSchemaCountLimitExceeded(String),
///<p>Inventory item type schema version has to match supported versions in the service. Check output of <code>GetInventorySchema</code> to see the available schema version for each type.</p>
UnsupportedInventorySchemaVersion(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutInventoryError {
                    pub fn from_body(body: &str) -> PutInventoryError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => PutInventoryError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => PutInventoryError::InvalidInstanceId(String::from(error_message)),"InvalidTypeNameException" => PutInventoryError::InvalidTypeName(String::from(error_message)),"InvalidItemContentException" => PutInventoryError::InvalidItemContent(String::from(error_message)),"TotalSizeLimitExceededException" => PutInventoryError::TotalSizeLimitExceeded(String::from(error_message)),"ItemSizeLimitExceededException" => PutInventoryError::ItemSizeLimitExceeded(String::from(error_message)),"ItemContentMismatchException" => PutInventoryError::ItemContentMismatch(String::from(error_message)),"CustomSchemaCountLimitExceededException" => PutInventoryError::CustomSchemaCountLimitExceeded(String::from(error_message)),"UnsupportedInventorySchemaVersionException" => PutInventoryError::UnsupportedInventorySchemaVersion(String::from(error_message)),"ValidationException" => PutInventoryError::Validation(error_message.to_string()),_ => PutInventoryError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutInventoryError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutInventoryError {
                    fn from(err: serde_json::error::Error) -> PutInventoryError {
                        PutInventoryError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutInventoryError {
                    fn from(err: CredentialsError) -> PutInventoryError {
                        PutInventoryError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutInventoryError {
                    fn from(err: HttpDispatchError) -> PutInventoryError {
                        PutInventoryError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutInventoryError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutInventoryError {
                    fn description(&self) -> &str {
                        match *self {
                            PutInventoryError::InternalServerError(ref cause) => cause,PutInventoryError::InvalidInstanceId(ref cause) => cause,PutInventoryError::InvalidTypeName(ref cause) => cause,PutInventoryError::InvalidItemContent(ref cause) => cause,PutInventoryError::TotalSizeLimitExceeded(ref cause) => cause,PutInventoryError::ItemSizeLimitExceeded(ref cause) => cause,PutInventoryError::ItemContentMismatch(ref cause) => cause,PutInventoryError::CustomSchemaCountLimitExceeded(ref cause) => cause,PutInventoryError::UnsupportedInventorySchemaVersion(ref cause) => cause,PutInventoryError::Validation(ref cause) => cause,PutInventoryError::Credentials(ref err) => err.description(),PutInventoryError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutInventoryError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by PutParameter
                #[derive(Debug, PartialEq)]
                pub enum PutParameterError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The query key ID is not valid.</p>
InvalidKeyId(String),
///<p>You have exceeded the number of parameters for this AWS account. Delete one or more parameters and try again.</p>
ParameterLimitExceeded(String),
///<p>There are concurrent updates for a resource that supports one update at a time.</p>
TooManyUpdates(String),
///<p>The parameter already exists. You can't create duplicate parameters.</p>
ParameterAlreadyExists(String),
///<p>The parameter type is not supported.</p>
UnsupportedParameterType(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl PutParameterError {
                    pub fn from_body(body: &str) -> PutParameterError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => PutParameterError::InternalServerError(String::from(error_message)),"InvalidKeyId" => PutParameterError::InvalidKeyId(String::from(error_message)),"ParameterLimitExceeded" => PutParameterError::ParameterLimitExceeded(String::from(error_message)),"TooManyUpdates" => PutParameterError::TooManyUpdates(String::from(error_message)),"ParameterAlreadyExists" => PutParameterError::ParameterAlreadyExists(String::from(error_message)),"UnsupportedParameterType" => PutParameterError::UnsupportedParameterType(String::from(error_message)),"ValidationException" => PutParameterError::Validation(error_message.to_string()),_ => PutParameterError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => PutParameterError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for PutParameterError {
                    fn from(err: serde_json::error::Error) -> PutParameterError {
                        PutParameterError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for PutParameterError {
                    fn from(err: CredentialsError) -> PutParameterError {
                        PutParameterError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for PutParameterError {
                    fn from(err: HttpDispatchError) -> PutParameterError {
                        PutParameterError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for PutParameterError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for PutParameterError {
                    fn description(&self) -> &str {
                        match *self {
                            PutParameterError::InternalServerError(ref cause) => cause,PutParameterError::InvalidKeyId(ref cause) => cause,PutParameterError::ParameterLimitExceeded(ref cause) => cause,PutParameterError::TooManyUpdates(ref cause) => cause,PutParameterError::ParameterAlreadyExists(ref cause) => cause,PutParameterError::UnsupportedParameterType(ref cause) => cause,PutParameterError::Validation(ref cause) => cause,PutParameterError::Credentials(ref err) => err.description(),PutParameterError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),PutParameterError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterDefaultPatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum RegisterDefaultPatchBaselineError {
                    
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterDefaultPatchBaselineError {
                    pub fn from_body(body: &str) -> RegisterDefaultPatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceId" => RegisterDefaultPatchBaselineError::InvalidResourceId(String::from(error_message)),"DoesNotExistException" => RegisterDefaultPatchBaselineError::DoesNotExist(String::from(error_message)),"InternalServerError" => RegisterDefaultPatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => RegisterDefaultPatchBaselineError::Validation(error_message.to_string()),_ => RegisterDefaultPatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterDefaultPatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterDefaultPatchBaselineError {
                    fn from(err: serde_json::error::Error) -> RegisterDefaultPatchBaselineError {
                        RegisterDefaultPatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RegisterDefaultPatchBaselineError {
                    fn from(err: CredentialsError) -> RegisterDefaultPatchBaselineError {
                        RegisterDefaultPatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RegisterDefaultPatchBaselineError {
                    fn from(err: HttpDispatchError) -> RegisterDefaultPatchBaselineError {
                        RegisterDefaultPatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RegisterDefaultPatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterDefaultPatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterDefaultPatchBaselineError::InvalidResourceId(ref cause) => cause,RegisterDefaultPatchBaselineError::DoesNotExist(ref cause) => cause,RegisterDefaultPatchBaselineError::InternalServerError(ref cause) => cause,RegisterDefaultPatchBaselineError::Validation(ref cause) => cause,RegisterDefaultPatchBaselineError::Credentials(ref err) => err.description(),RegisterDefaultPatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterDefaultPatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterPatchBaselineForPatchGroup
                #[derive(Debug, PartialEq)]
                pub enum RegisterPatchBaselineForPatchGroupError {
                    
///<p>Error returned if an attempt is made to register a patch group with a patch baseline that is already registered with a different patch baseline.</p>
AlreadyExists(String),
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>Error returned when the caller has exceeded the default resource limits (e.g. too many Maintenance Windows have been created).</p>
ResourceLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterPatchBaselineForPatchGroupError {
                    pub fn from_body(body: &str) -> RegisterPatchBaselineForPatchGroupError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AlreadyExistsException" => RegisterPatchBaselineForPatchGroupError::AlreadyExists(String::from(error_message)),"DoesNotExistException" => RegisterPatchBaselineForPatchGroupError::DoesNotExist(String::from(error_message)),"InvalidResourceId" => RegisterPatchBaselineForPatchGroupError::InvalidResourceId(String::from(error_message)),"ResourceLimitExceededException" => RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(String::from(error_message)),"InternalServerError" => RegisterPatchBaselineForPatchGroupError::InternalServerError(String::from(error_message)),"ValidationException" => RegisterPatchBaselineForPatchGroupError::Validation(error_message.to_string()),_ => RegisterPatchBaselineForPatchGroupError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterPatchBaselineForPatchGroupError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterPatchBaselineForPatchGroupError {
                    fn from(err: serde_json::error::Error) -> RegisterPatchBaselineForPatchGroupError {
                        RegisterPatchBaselineForPatchGroupError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RegisterPatchBaselineForPatchGroupError {
                    fn from(err: CredentialsError) -> RegisterPatchBaselineForPatchGroupError {
                        RegisterPatchBaselineForPatchGroupError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RegisterPatchBaselineForPatchGroupError {
                    fn from(err: HttpDispatchError) -> RegisterPatchBaselineForPatchGroupError {
                        RegisterPatchBaselineForPatchGroupError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RegisterPatchBaselineForPatchGroupError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterPatchBaselineForPatchGroupError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterPatchBaselineForPatchGroupError::AlreadyExists(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::DoesNotExist(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::InvalidResourceId(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::ResourceLimitExceeded(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::InternalServerError(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::Validation(ref cause) => cause,RegisterPatchBaselineForPatchGroupError::Credentials(ref err) => err.description(),RegisterPatchBaselineForPatchGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterPatchBaselineForPatchGroupError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterTargetWithMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum RegisterTargetWithMaintenanceWindowError {
                    
///<p>Error returned when an idempotent operation is retried and the parameters don’t match the original call to the API with the same idempotency token. </p>
IdempotentParameterMismatch(String),
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>Error returned when the caller has exceeded the default resource limits (e.g. too many Maintenance Windows have been created).</p>
ResourceLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterTargetWithMaintenanceWindowError {
                    pub fn from_body(body: &str) -> RegisterTargetWithMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "IdempotentParameterMismatch" => RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(String::from(error_message)),"DoesNotExistException" => RegisterTargetWithMaintenanceWindowError::DoesNotExist(String::from(error_message)),"ResourceLimitExceededException" => RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(String::from(error_message)),"InternalServerError" => RegisterTargetWithMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => RegisterTargetWithMaintenanceWindowError::Validation(error_message.to_string()),_ => RegisterTargetWithMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterTargetWithMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterTargetWithMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> RegisterTargetWithMaintenanceWindowError {
                        RegisterTargetWithMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RegisterTargetWithMaintenanceWindowError {
                    fn from(err: CredentialsError) -> RegisterTargetWithMaintenanceWindowError {
                        RegisterTargetWithMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RegisterTargetWithMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> RegisterTargetWithMaintenanceWindowError {
                        RegisterTargetWithMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RegisterTargetWithMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterTargetWithMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterTargetWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,RegisterTargetWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,RegisterTargetWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,RegisterTargetWithMaintenanceWindowError::InternalServerError(ref cause) => cause,RegisterTargetWithMaintenanceWindowError::Validation(ref cause) => cause,RegisterTargetWithMaintenanceWindowError::Credentials(ref err) => err.description(),RegisterTargetWithMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterTargetWithMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RegisterTaskWithMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum RegisterTaskWithMaintenanceWindowError {
                    
///<p>Error returned when an idempotent operation is retried and the parameters don’t match the original call to the API with the same idempotency token. </p>
IdempotentParameterMismatch(String),
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>Error returned when the caller has exceeded the default resource limits (e.g. too many Maintenance Windows have been created).</p>
ResourceLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RegisterTaskWithMaintenanceWindowError {
                    pub fn from_body(body: &str) -> RegisterTaskWithMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "IdempotentParameterMismatch" => RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(String::from(error_message)),"DoesNotExistException" => RegisterTaskWithMaintenanceWindowError::DoesNotExist(String::from(error_message)),"ResourceLimitExceededException" => RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(String::from(error_message)),"InternalServerError" => RegisterTaskWithMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => RegisterTaskWithMaintenanceWindowError::Validation(error_message.to_string()),_ => RegisterTaskWithMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RegisterTaskWithMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RegisterTaskWithMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> RegisterTaskWithMaintenanceWindowError {
                        RegisterTaskWithMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RegisterTaskWithMaintenanceWindowError {
                    fn from(err: CredentialsError) -> RegisterTaskWithMaintenanceWindowError {
                        RegisterTaskWithMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RegisterTaskWithMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> RegisterTaskWithMaintenanceWindowError {
                        RegisterTaskWithMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RegisterTaskWithMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RegisterTaskWithMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            RegisterTaskWithMaintenanceWindowError::IdempotentParameterMismatch(ref cause) => cause,RegisterTaskWithMaintenanceWindowError::DoesNotExist(ref cause) => cause,RegisterTaskWithMaintenanceWindowError::ResourceLimitExceeded(ref cause) => cause,RegisterTaskWithMaintenanceWindowError::InternalServerError(ref cause) => cause,RegisterTaskWithMaintenanceWindowError::Validation(ref cause) => cause,RegisterTaskWithMaintenanceWindowError::Credentials(ref err) => err.description(),RegisterTaskWithMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RegisterTaskWithMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by RemoveTagsFromResource
                #[derive(Debug, PartialEq)]
                pub enum RemoveTagsFromResourceError {
                    
///<p>The resource type is not valid. If you are attempting to tag an instance, the instance must be a registered, managed instance.</p>
InvalidResourceType(String),
///<p>The resource ID is not valid. Verify that you entered the correct ID and try again.</p>
InvalidResourceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl RemoveTagsFromResourceError {
                    pub fn from_body(body: &str) -> RemoveTagsFromResourceError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidResourceType" => RemoveTagsFromResourceError::InvalidResourceType(String::from(error_message)),"InvalidResourceId" => RemoveTagsFromResourceError::InvalidResourceId(String::from(error_message)),"InternalServerError" => RemoveTagsFromResourceError::InternalServerError(String::from(error_message)),"ValidationException" => RemoveTagsFromResourceError::Validation(error_message.to_string()),_ => RemoveTagsFromResourceError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => RemoveTagsFromResourceError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for RemoveTagsFromResourceError {
                    fn from(err: serde_json::error::Error) -> RemoveTagsFromResourceError {
                        RemoveTagsFromResourceError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for RemoveTagsFromResourceError {
                    fn from(err: CredentialsError) -> RemoveTagsFromResourceError {
                        RemoveTagsFromResourceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for RemoveTagsFromResourceError {
                    fn from(err: HttpDispatchError) -> RemoveTagsFromResourceError {
                        RemoveTagsFromResourceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for RemoveTagsFromResourceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for RemoveTagsFromResourceError {
                    fn description(&self) -> &str {
                        match *self {
                            RemoveTagsFromResourceError::InvalidResourceType(ref cause) => cause,RemoveTagsFromResourceError::InvalidResourceId(ref cause) => cause,RemoveTagsFromResourceError::InternalServerError(ref cause) => cause,RemoveTagsFromResourceError::Validation(ref cause) => cause,RemoveTagsFromResourceError::Credentials(ref err) => err.description(),RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),RemoveTagsFromResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by SendCommand
                #[derive(Debug, PartialEq)]
                pub enum SendCommandError {
                    
///<p>You cannot specify an instance ID in more than one association.</p>
DuplicateInstanceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The S3 bucket does not exist.</p>
InvalidOutputFolder(String),
///<p>You must specify values for all required parameters in the SSM document. You can only supply values to parameters defined in the SSM document.</p>
InvalidParameters(String),
///<p>The document does not support the platform type of the given instance ID(s). For example, you sent an document for a Windows instance to a Linux instance.</p>
UnsupportedPlatformType(String),
///<p>The size limit of a document is 64 KB.</p>
MaxDocumentSizeExceeded(String),
///<p>The role name can't contain invalid characters. Also verify that you specified an IAM role for notifications that includes the required trust policy. For information about configuring the IAM role for Run Command notifications, see <a href="http://docs.aws.amazon.com/systems-manager/latest/userguide/rc-sns-notifications.html">Configuring Amazon SNS Notifications for Run Command</a> in the <i>Amazon EC2 Systems Manager User Guide</i>.</p>
InvalidRole(String),
///<p>One or more configuration items is not valid. Verify that a valid Amazon Resource Name (ARN) was provided for an Amazon SNS topic.</p>
InvalidNotificationConfig(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl SendCommandError {
                    pub fn from_body(body: &str) -> SendCommandError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DuplicateInstanceId" => SendCommandError::DuplicateInstanceId(String::from(error_message)),"InternalServerError" => SendCommandError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => SendCommandError::InvalidInstanceId(String::from(error_message)),"InvalidDocument" => SendCommandError::InvalidDocument(String::from(error_message)),"InvalidOutputFolder" => SendCommandError::InvalidOutputFolder(String::from(error_message)),"InvalidParameters" => SendCommandError::InvalidParameters(String::from(error_message)),"UnsupportedPlatformType" => SendCommandError::UnsupportedPlatformType(String::from(error_message)),"MaxDocumentSizeExceeded" => SendCommandError::MaxDocumentSizeExceeded(String::from(error_message)),"InvalidRole" => SendCommandError::InvalidRole(String::from(error_message)),"InvalidNotificationConfig" => SendCommandError::InvalidNotificationConfig(String::from(error_message)),"ValidationException" => SendCommandError::Validation(error_message.to_string()),_ => SendCommandError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => SendCommandError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for SendCommandError {
                    fn from(err: serde_json::error::Error) -> SendCommandError {
                        SendCommandError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for SendCommandError {
                    fn from(err: CredentialsError) -> SendCommandError {
                        SendCommandError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for SendCommandError {
                    fn from(err: HttpDispatchError) -> SendCommandError {
                        SendCommandError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for SendCommandError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for SendCommandError {
                    fn description(&self) -> &str {
                        match *self {
                            SendCommandError::DuplicateInstanceId(ref cause) => cause,SendCommandError::InternalServerError(ref cause) => cause,SendCommandError::InvalidInstanceId(ref cause) => cause,SendCommandError::InvalidDocument(ref cause) => cause,SendCommandError::InvalidOutputFolder(ref cause) => cause,SendCommandError::InvalidParameters(ref cause) => cause,SendCommandError::UnsupportedPlatformType(ref cause) => cause,SendCommandError::MaxDocumentSizeExceeded(ref cause) => cause,SendCommandError::InvalidRole(ref cause) => cause,SendCommandError::InvalidNotificationConfig(ref cause) => cause,SendCommandError::Validation(ref cause) => cause,SendCommandError::Credentials(ref err) => err.description(),SendCommandError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),SendCommandError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by StartAutomationExecution
                #[derive(Debug, PartialEq)]
                pub enum StartAutomationExecutionError {
                    
///<p>An Automation document with the specified name could not be found.</p>
AutomationDefinitionNotFound(String),
///<p>The supplied parameters for invoking the specified Automation document are incorrect. For example, they may not match the set of parameters permitted for the specified Automation document.</p>
InvalidAutomationExecutionParameters(String),
///<p>The number of simultaneously running Automation executions exceeded the allowable limit.</p>
AutomationExecutionLimitExceeded(String),
///<p>An Automation document with the specified name and version could not be found.</p>
AutomationDefinitionVersionNotFound(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl StartAutomationExecutionError {
                    pub fn from_body(body: &str) -> StartAutomationExecutionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AutomationDefinitionNotFoundException" => StartAutomationExecutionError::AutomationDefinitionNotFound(String::from(error_message)),"InvalidAutomationExecutionParametersException" => StartAutomationExecutionError::InvalidAutomationExecutionParameters(String::from(error_message)),"AutomationExecutionLimitExceededException" => StartAutomationExecutionError::AutomationExecutionLimitExceeded(String::from(error_message)),"AutomationDefinitionVersionNotFoundException" => StartAutomationExecutionError::AutomationDefinitionVersionNotFound(String::from(error_message)),"InternalServerError" => StartAutomationExecutionError::InternalServerError(String::from(error_message)),"ValidationException" => StartAutomationExecutionError::Validation(error_message.to_string()),_ => StartAutomationExecutionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => StartAutomationExecutionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for StartAutomationExecutionError {
                    fn from(err: serde_json::error::Error) -> StartAutomationExecutionError {
                        StartAutomationExecutionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for StartAutomationExecutionError {
                    fn from(err: CredentialsError) -> StartAutomationExecutionError {
                        StartAutomationExecutionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for StartAutomationExecutionError {
                    fn from(err: HttpDispatchError) -> StartAutomationExecutionError {
                        StartAutomationExecutionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for StartAutomationExecutionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for StartAutomationExecutionError {
                    fn description(&self) -> &str {
                        match *self {
                            StartAutomationExecutionError::AutomationDefinitionNotFound(ref cause) => cause,StartAutomationExecutionError::InvalidAutomationExecutionParameters(ref cause) => cause,StartAutomationExecutionError::AutomationExecutionLimitExceeded(ref cause) => cause,StartAutomationExecutionError::AutomationDefinitionVersionNotFound(ref cause) => cause,StartAutomationExecutionError::InternalServerError(ref cause) => cause,StartAutomationExecutionError::Validation(ref cause) => cause,StartAutomationExecutionError::Credentials(ref err) => err.description(),StartAutomationExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),StartAutomationExecutionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by StopAutomationExecution
                #[derive(Debug, PartialEq)]
                pub enum StopAutomationExecutionError {
                    
///<p>There is no automation execution information for the requested automation execution ID.</p>
AutomationExecutionNotFound(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl StopAutomationExecutionError {
                    pub fn from_body(body: &str) -> StopAutomationExecutionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "AutomationExecutionNotFoundException" => StopAutomationExecutionError::AutomationExecutionNotFound(String::from(error_message)),"InternalServerError" => StopAutomationExecutionError::InternalServerError(String::from(error_message)),"ValidationException" => StopAutomationExecutionError::Validation(error_message.to_string()),_ => StopAutomationExecutionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => StopAutomationExecutionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for StopAutomationExecutionError {
                    fn from(err: serde_json::error::Error) -> StopAutomationExecutionError {
                        StopAutomationExecutionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for StopAutomationExecutionError {
                    fn from(err: CredentialsError) -> StopAutomationExecutionError {
                        StopAutomationExecutionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for StopAutomationExecutionError {
                    fn from(err: HttpDispatchError) -> StopAutomationExecutionError {
                        StopAutomationExecutionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for StopAutomationExecutionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for StopAutomationExecutionError {
                    fn description(&self) -> &str {
                        match *self {
                            StopAutomationExecutionError::AutomationExecutionNotFound(ref cause) => cause,StopAutomationExecutionError::InternalServerError(ref cause) => cause,StopAutomationExecutionError::Validation(ref cause) => cause,StopAutomationExecutionError::Credentials(ref err) => err.description(),StopAutomationExecutionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),StopAutomationExecutionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateAssociation
                #[derive(Debug, PartialEq)]
                pub enum UpdateAssociationError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The schedule is invalid. Verify your cron or rate expression and try again.</p>
InvalidSchedule(String),
///<p>You must specify values for all required parameters in the SSM document. You can only supply values to parameters defined in the SSM document.</p>
InvalidParameters(String),
///<p>The output location is not valid or does not exist.</p>
InvalidOutputLocation(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),
///<p>The specified association does not exist.</p>
AssociationDoesNotExist(String),
///<p>The update is not valid.</p>
InvalidUpdate(String),
///<p>There are concurrent updates for a resource that supports one update at a time.</p>
TooManyUpdates(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateAssociationError {
                    pub fn from_body(body: &str) -> UpdateAssociationError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => UpdateAssociationError::InternalServerError(String::from(error_message)),"InvalidSchedule" => UpdateAssociationError::InvalidSchedule(String::from(error_message)),"InvalidParameters" => UpdateAssociationError::InvalidParameters(String::from(error_message)),"InvalidOutputLocation" => UpdateAssociationError::InvalidOutputLocation(String::from(error_message)),"InvalidDocumentVersion" => UpdateAssociationError::InvalidDocumentVersion(String::from(error_message)),"AssociationDoesNotExist" => UpdateAssociationError::AssociationDoesNotExist(String::from(error_message)),"InvalidUpdate" => UpdateAssociationError::InvalidUpdate(String::from(error_message)),"TooManyUpdates" => UpdateAssociationError::TooManyUpdates(String::from(error_message)),"ValidationException" => UpdateAssociationError::Validation(error_message.to_string()),_ => UpdateAssociationError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateAssociationError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateAssociationError {
                    fn from(err: serde_json::error::Error) -> UpdateAssociationError {
                        UpdateAssociationError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateAssociationError {
                    fn from(err: CredentialsError) -> UpdateAssociationError {
                        UpdateAssociationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateAssociationError {
                    fn from(err: HttpDispatchError) -> UpdateAssociationError {
                        UpdateAssociationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateAssociationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateAssociationError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateAssociationError::InternalServerError(ref cause) => cause,UpdateAssociationError::InvalidSchedule(ref cause) => cause,UpdateAssociationError::InvalidParameters(ref cause) => cause,UpdateAssociationError::InvalidOutputLocation(ref cause) => cause,UpdateAssociationError::InvalidDocumentVersion(ref cause) => cause,UpdateAssociationError::AssociationDoesNotExist(ref cause) => cause,UpdateAssociationError::InvalidUpdate(ref cause) => cause,UpdateAssociationError::TooManyUpdates(ref cause) => cause,UpdateAssociationError::Validation(ref cause) => cause,UpdateAssociationError::Credentials(ref err) => err.description(),UpdateAssociationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateAssociationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateAssociationStatus
                #[derive(Debug, PartialEq)]
                pub enum UpdateAssociationStatusError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The specified association does not exist.</p>
AssociationDoesNotExist(String),
///<p>The updated status is the same as the current status.</p>
StatusUnchanged(String),
///<p>There are concurrent updates for a resource that supports one update at a time.</p>
TooManyUpdates(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateAssociationStatusError {
                    pub fn from_body(body: &str) -> UpdateAssociationStatusError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => UpdateAssociationStatusError::InternalServerError(String::from(error_message)),"InvalidInstanceId" => UpdateAssociationStatusError::InvalidInstanceId(String::from(error_message)),"InvalidDocument" => UpdateAssociationStatusError::InvalidDocument(String::from(error_message)),"AssociationDoesNotExist" => UpdateAssociationStatusError::AssociationDoesNotExist(String::from(error_message)),"StatusUnchanged" => UpdateAssociationStatusError::StatusUnchanged(String::from(error_message)),"TooManyUpdates" => UpdateAssociationStatusError::TooManyUpdates(String::from(error_message)),"ValidationException" => UpdateAssociationStatusError::Validation(error_message.to_string()),_ => UpdateAssociationStatusError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateAssociationStatusError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateAssociationStatusError {
                    fn from(err: serde_json::error::Error) -> UpdateAssociationStatusError {
                        UpdateAssociationStatusError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateAssociationStatusError {
                    fn from(err: CredentialsError) -> UpdateAssociationStatusError {
                        UpdateAssociationStatusError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateAssociationStatusError {
                    fn from(err: HttpDispatchError) -> UpdateAssociationStatusError {
                        UpdateAssociationStatusError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateAssociationStatusError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateAssociationStatusError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateAssociationStatusError::InternalServerError(ref cause) => cause,UpdateAssociationStatusError::InvalidInstanceId(ref cause) => cause,UpdateAssociationStatusError::InvalidDocument(ref cause) => cause,UpdateAssociationStatusError::AssociationDoesNotExist(ref cause) => cause,UpdateAssociationStatusError::StatusUnchanged(ref cause) => cause,UpdateAssociationStatusError::TooManyUpdates(ref cause) => cause,UpdateAssociationStatusError::Validation(ref cause) => cause,UpdateAssociationStatusError::Credentials(ref err) => err.description(),UpdateAssociationStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateAssociationStatusError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateDocument
                #[derive(Debug, PartialEq)]
                pub enum UpdateDocumentError {
                    
///<p>The size limit of a document is 64 KB.</p>
MaxDocumentSizeExceeded(String),
///<p>The document has too many versions. Delete one or more document versions and try again.</p>
DocumentVersionLimitExceeded(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The content of the association document matches another document. Change the content of the document and try again.</p>
DuplicateDocumentContent(String),
///<p>The content for the document is not valid.</p>
InvalidDocumentContent(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),
///<p>The version of the document schema is not supported.</p>
InvalidDocumentSchemaVersion(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateDocumentError {
                    pub fn from_body(body: &str) -> UpdateDocumentError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "MaxDocumentSizeExceeded" => UpdateDocumentError::MaxDocumentSizeExceeded(String::from(error_message)),"DocumentVersionLimitExceeded" => UpdateDocumentError::DocumentVersionLimitExceeded(String::from(error_message)),"InternalServerError" => UpdateDocumentError::InternalServerError(String::from(error_message)),"DuplicateDocumentContent" => UpdateDocumentError::DuplicateDocumentContent(String::from(error_message)),"InvalidDocumentContent" => UpdateDocumentError::InvalidDocumentContent(String::from(error_message)),"InvalidDocumentVersion" => UpdateDocumentError::InvalidDocumentVersion(String::from(error_message)),"InvalidDocumentSchemaVersion" => UpdateDocumentError::InvalidDocumentSchemaVersion(String::from(error_message)),"InvalidDocument" => UpdateDocumentError::InvalidDocument(String::from(error_message)),"ValidationException" => UpdateDocumentError::Validation(error_message.to_string()),_ => UpdateDocumentError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateDocumentError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateDocumentError {
                    fn from(err: serde_json::error::Error) -> UpdateDocumentError {
                        UpdateDocumentError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateDocumentError {
                    fn from(err: CredentialsError) -> UpdateDocumentError {
                        UpdateDocumentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateDocumentError {
                    fn from(err: HttpDispatchError) -> UpdateDocumentError {
                        UpdateDocumentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateDocumentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateDocumentError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateDocumentError::MaxDocumentSizeExceeded(ref cause) => cause,UpdateDocumentError::DocumentVersionLimitExceeded(ref cause) => cause,UpdateDocumentError::InternalServerError(ref cause) => cause,UpdateDocumentError::DuplicateDocumentContent(ref cause) => cause,UpdateDocumentError::InvalidDocumentContent(ref cause) => cause,UpdateDocumentError::InvalidDocumentVersion(ref cause) => cause,UpdateDocumentError::InvalidDocumentSchemaVersion(ref cause) => cause,UpdateDocumentError::InvalidDocument(ref cause) => cause,UpdateDocumentError::Validation(ref cause) => cause,UpdateDocumentError::Credentials(ref err) => err.description(),UpdateDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateDocumentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateDocumentDefaultVersion
                #[derive(Debug, PartialEq)]
                pub enum UpdateDocumentDefaultVersionError {
                    
///<p>An error occurred on the server side.</p>
InternalServerError(String),
///<p>The specified document does not exist.</p>
InvalidDocument(String),
///<p>The document version is not valid or does not exist.</p>
InvalidDocumentVersion(String),
///<p>The version of the document schema is not supported.</p>
InvalidDocumentSchemaVersion(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateDocumentDefaultVersionError {
                    pub fn from_body(body: &str) -> UpdateDocumentDefaultVersionError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InternalServerError" => UpdateDocumentDefaultVersionError::InternalServerError(String::from(error_message)),"InvalidDocument" => UpdateDocumentDefaultVersionError::InvalidDocument(String::from(error_message)),"InvalidDocumentVersion" => UpdateDocumentDefaultVersionError::InvalidDocumentVersion(String::from(error_message)),"InvalidDocumentSchemaVersion" => UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(String::from(error_message)),"ValidationException" => UpdateDocumentDefaultVersionError::Validation(error_message.to_string()),_ => UpdateDocumentDefaultVersionError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateDocumentDefaultVersionError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateDocumentDefaultVersionError {
                    fn from(err: serde_json::error::Error) -> UpdateDocumentDefaultVersionError {
                        UpdateDocumentDefaultVersionError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateDocumentDefaultVersionError {
                    fn from(err: CredentialsError) -> UpdateDocumentDefaultVersionError {
                        UpdateDocumentDefaultVersionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateDocumentDefaultVersionError {
                    fn from(err: HttpDispatchError) -> UpdateDocumentDefaultVersionError {
                        UpdateDocumentDefaultVersionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateDocumentDefaultVersionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateDocumentDefaultVersionError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateDocumentDefaultVersionError::InternalServerError(ref cause) => cause,UpdateDocumentDefaultVersionError::InvalidDocument(ref cause) => cause,UpdateDocumentDefaultVersionError::InvalidDocumentVersion(ref cause) => cause,UpdateDocumentDefaultVersionError::InvalidDocumentSchemaVersion(ref cause) => cause,UpdateDocumentDefaultVersionError::Validation(ref cause) => cause,UpdateDocumentDefaultVersionError::Credentials(ref err) => err.description(),UpdateDocumentDefaultVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateDocumentDefaultVersionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateMaintenanceWindow
                #[derive(Debug, PartialEq)]
                pub enum UpdateMaintenanceWindowError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateMaintenanceWindowError {
                    pub fn from_body(body: &str) -> UpdateMaintenanceWindowError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => UpdateMaintenanceWindowError::DoesNotExist(String::from(error_message)),"InternalServerError" => UpdateMaintenanceWindowError::InternalServerError(String::from(error_message)),"ValidationException" => UpdateMaintenanceWindowError::Validation(error_message.to_string()),_ => UpdateMaintenanceWindowError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateMaintenanceWindowError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateMaintenanceWindowError {
                    fn from(err: serde_json::error::Error) -> UpdateMaintenanceWindowError {
                        UpdateMaintenanceWindowError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateMaintenanceWindowError {
                    fn from(err: CredentialsError) -> UpdateMaintenanceWindowError {
                        UpdateMaintenanceWindowError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateMaintenanceWindowError {
                    fn from(err: HttpDispatchError) -> UpdateMaintenanceWindowError {
                        UpdateMaintenanceWindowError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateMaintenanceWindowError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateMaintenanceWindowError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateMaintenanceWindowError::DoesNotExist(ref cause) => cause,UpdateMaintenanceWindowError::InternalServerError(ref cause) => cause,UpdateMaintenanceWindowError::Validation(ref cause) => cause,UpdateMaintenanceWindowError::Credentials(ref err) => err.description(),UpdateMaintenanceWindowError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateMaintenanceWindowError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateManagedInstanceRole
                #[derive(Debug, PartialEq)]
                pub enum UpdateManagedInstanceRoleError {
                    
///<p>The following problems can cause this exception:</p> <p>You do not have permission to access the instance.</p> <p>The SSM Agent is not running. On managed instances and Linux instances, verify that the SSM Agent is running. On EC2 Windows instances, verify that the EC2Config service is running.</p> <p>The SSM Agent or EC2Config service is not registered to the SSM endpoint. Try reinstalling the SSM Agent or EC2Config service.</p> <p>The instance is not in valid state. Valid states are: Running, Pending, Stopped, Stopping. Invalid states are: Shutting-down and Terminated.</p>
InvalidInstanceId(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateManagedInstanceRoleError {
                    pub fn from_body(body: &str) -> UpdateManagedInstanceRoleError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "InvalidInstanceId" => UpdateManagedInstanceRoleError::InvalidInstanceId(String::from(error_message)),"InternalServerError" => UpdateManagedInstanceRoleError::InternalServerError(String::from(error_message)),"ValidationException" => UpdateManagedInstanceRoleError::Validation(error_message.to_string()),_ => UpdateManagedInstanceRoleError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdateManagedInstanceRoleError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdateManagedInstanceRoleError {
                    fn from(err: serde_json::error::Error) -> UpdateManagedInstanceRoleError {
                        UpdateManagedInstanceRoleError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdateManagedInstanceRoleError {
                    fn from(err: CredentialsError) -> UpdateManagedInstanceRoleError {
                        UpdateManagedInstanceRoleError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateManagedInstanceRoleError {
                    fn from(err: HttpDispatchError) -> UpdateManagedInstanceRoleError {
                        UpdateManagedInstanceRoleError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateManagedInstanceRoleError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateManagedInstanceRoleError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateManagedInstanceRoleError::InvalidInstanceId(ref cause) => cause,UpdateManagedInstanceRoleError::InternalServerError(ref cause) => cause,UpdateManagedInstanceRoleError::Validation(ref cause) => cause,UpdateManagedInstanceRoleError::Credentials(ref err) => err.description(),UpdateManagedInstanceRoleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateManagedInstanceRoleError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdatePatchBaseline
                #[derive(Debug, PartialEq)]
                pub enum UpdatePatchBaselineError {
                    
///<p>Error returned when the ID specified for a resource (e.g. a Maintenance Window) doesn’t exist.</p>
DoesNotExist(String),
///<p>An error occurred on the server side.</p>
InternalServerError(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdatePatchBaselineError {
                    pub fn from_body(body: &str) -> UpdatePatchBaselineError {
                        match from_str::<SerdeJsonValue>(body) {
                            Ok(json) => {
                                let raw_error_type = json.get("__type").and_then(|e| e.as_str()).unwrap_or("Unknown");
                                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                                let error_type = pieces.last().expect("Expected error type");

                                match *error_type {
                                    "DoesNotExistException" => UpdatePatchBaselineError::DoesNotExist(String::from(error_message)),"InternalServerError" => UpdatePatchBaselineError::InternalServerError(String::from(error_message)),"ValidationException" => UpdatePatchBaselineError::Validation(error_message.to_string()),_ => UpdatePatchBaselineError::Unknown(String::from(body))
                                }
                            },
                            Err(_) => UpdatePatchBaselineError::Unknown(String::from(body))
                        }
                    }
                }
                
                impl From<serde_json::error::Error> for UpdatePatchBaselineError {
                    fn from(err: serde_json::error::Error) -> UpdatePatchBaselineError {
                        UpdatePatchBaselineError::Unknown(err.description().to_string())
                    }
                }
                impl From<CredentialsError> for UpdatePatchBaselineError {
                    fn from(err: CredentialsError) -> UpdatePatchBaselineError {
                        UpdatePatchBaselineError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdatePatchBaselineError {
                    fn from(err: HttpDispatchError) -> UpdatePatchBaselineError {
                        UpdatePatchBaselineError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdatePatchBaselineError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdatePatchBaselineError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdatePatchBaselineError::DoesNotExist(ref cause) => cause,UpdatePatchBaselineError::InternalServerError(ref cause) => cause,UpdatePatchBaselineError::Validation(ref cause) => cause,UpdatePatchBaselineError::Credentials(ref err) => err.description(),UpdatePatchBaselineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdatePatchBaselineError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// A client for the Amazon SSM API.
        pub struct SsmClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> SsmClient<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  SsmClient {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }

        

                #[doc="<p>Adds or overwrites one or more tags for the specified resource. Tags are metadata that you assign to your managed instances. Tags enable you to categorize your managed instances in different ways, for example, by purpose, owner, or environment. Each tag consists of a key and an optional value, both of which you define. For example, you could define a set of tags for your account's managed instances that helps you track each instance's owner and stack level. For example: Key=Owner and Value=DbAdmin, SysAdmin, or Dev. Or Key=Stack and Value=Production, Pre-Production, or Test. Each resource can have a maximum of 10 tags. </p> <p>We recommend that you devise a set of tag keys that meets your needs for each resource type. Using a consistent set of tag keys makes it easier for you to manage your resources. You can search and filter the resources based on the tags you add. Tags don't have any semantic meaning to Amazon EC2 and are interpreted strictly as a string of characters. </p> <p>For more information about tags, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html\">Tagging Your Amazon EC2 Resources</a> in the <i>Amazon EC2 User Guide</i>.</p>"]
                pub fn add_tags_to_resource(&self, input: &AddTagsToResourceRequest)  -> Result<AddTagsToResourceResult, AddTagsToResourceError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.AddTagsToResource");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddTagsToResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(AddTagsToResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped.</p>"]
                pub fn cancel_command(&self, input: &CancelCommandRequest)  -> Result<CancelCommandResult, CancelCommandError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CancelCommand");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CancelCommandResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CancelCommandError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Registers your on-premises server or virtual machine with Amazon EC2 so that you can manage these resources using Run Command. An on-premises server or virtual machine that has been registered with EC2 is called a managed instance. For more information about activations, see <a href=\"http://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-managedinstances.html\">Setting Up Systems Manager in Hybrid Environments</a>.</p>"]
                pub fn create_activation(&self, input: &CreateActivationRequest)  -> Result<CreateActivationResult, CreateActivationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreateActivation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateActivationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateActivationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>"]
                pub fn create_association(&self, input: &CreateAssociationRequest)  -> Result<CreateAssociationResult, CreateAssociationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreateAssociation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateAssociationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateAssociationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Associates the specified Systems Manager document with the specified instances or targets.</p> <p>When you associate a document with one or more instances using instance IDs or tags, the SSM Agent running on the instance processes the document and configures the instance as specified.</p> <p>If you associate a document with an instance that already has an associated document, the system throws the AssociationAlreadyExists exception.</p>"]
                pub fn create_association_batch(&self, input: &CreateAssociationBatchRequest)  -> Result<CreateAssociationBatchResult, CreateAssociationBatchError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreateAssociationBatch");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateAssociationBatchResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateAssociationBatchError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a Systems Manager document.</p> <p>After you create a document, you can use CreateAssociation to associate it with one or more running instances.</p>"]
                pub fn create_document(&self, input: &CreateDocumentRequest)  -> Result<CreateDocumentResult, CreateDocumentError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreateDocument");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateDocumentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateDocumentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a new Maintenance Window.</p>"]
                pub fn create_maintenance_window(&self, input: &CreateMaintenanceWindowRequest)  -> Result<CreateMaintenanceWindowResult, CreateMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreateMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreateMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Creates a patch baseline.</p>"]
                pub fn create_patch_baseline(&self, input: &CreatePatchBaselineRequest)  -> Result<CreatePatchBaselineResult, CreatePatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.CreatePatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreatePatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(CreatePatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes an activation. You are not required to delete an activation. If you delete an activation, you can no longer use it to register additional managed instances. Deleting an activation does not de-register managed instances. You must manually de-register managed instances.</p>"]
                pub fn delete_activation(&self, input: &DeleteActivationRequest)  -> Result<DeleteActivationResult, DeleteActivationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeleteActivation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteActivationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteActivationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Disassociates the specified Systems Manager document from the specified instance.</p> <p>When you disassociate a document from an instance, it does not change the configuration of the instance. To change the configuration state of an instance after you disassociate a document, you must create a new document with the desired configuration and associate it with the instance.</p>"]
                pub fn delete_association(&self, input: &DeleteAssociationRequest)  -> Result<DeleteAssociationResult, DeleteAssociationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeleteAssociation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteAssociationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteAssociationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes the Systems Manager document and all instance associations to the document.</p> <p>Before you delete the document, we recommend that you use <a>DeleteAssociation</a> to disassociate all instances that are associated with the document.</p>"]
                pub fn delete_document(&self, input: &DeleteDocumentRequest)  -> Result<DeleteDocumentResult, DeleteDocumentError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeleteDocument");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteDocumentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteDocumentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a Maintenance Window.</p>"]
                pub fn delete_maintenance_window(&self, input: &DeleteMaintenanceWindowRequest)  -> Result<DeleteMaintenanceWindowResult, DeleteMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeleteMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Delete a parameter from the system.</p>"]
                pub fn delete_parameter(&self, input: &DeleteParameterRequest)  -> Result<DeleteParameterResult, DeleteParameterError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeleteParameter");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteParameterResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeleteParameterError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Deletes a patch baseline.</p>"]
                pub fn delete_patch_baseline(&self, input: &DeletePatchBaselineRequest)  -> Result<DeletePatchBaselineResult, DeletePatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeletePatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeletePatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeletePatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes the server or virtual machine from the list of registered servers. You can reregister the instance again at any time. If you don’t plan to use Run Command on the server, we suggest uninstalling the SSM Agent first.</p>"]
                pub fn deregister_managed_instance(&self, input: &DeregisterManagedInstanceRequest)  -> Result<DeregisterManagedInstanceResult, DeregisterManagedInstanceError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeregisterManagedInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeregisterManagedInstanceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeregisterManagedInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes a patch group from a patch baseline.</p>"]
                pub fn deregister_patch_baseline_for_patch_group(&self, input: &DeregisterPatchBaselineForPatchGroupRequest)  -> Result<DeregisterPatchBaselineForPatchGroupResult, DeregisterPatchBaselineForPatchGroupError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeregisterPatchBaselineForPatchGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeregisterPatchBaselineForPatchGroupResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeregisterPatchBaselineForPatchGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes a target from a Maintenance Window.</p>"]
                pub fn deregister_target_from_maintenance_window(&self, input: &DeregisterTargetFromMaintenanceWindowRequest)  -> Result<DeregisterTargetFromMaintenanceWindowResult, DeregisterTargetFromMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeregisterTargetFromMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeregisterTargetFromMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeregisterTargetFromMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes a task from a Maintenance Window.</p>"]
                pub fn deregister_task_from_maintenance_window(&self, input: &DeregisterTaskFromMaintenanceWindowRequest)  -> Result<DeregisterTaskFromMaintenanceWindowResult, DeregisterTaskFromMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DeregisterTaskFromMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeregisterTaskFromMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DeregisterTaskFromMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Details about the activation, including: the date and time the activation was created, the expiration date, the IAM role assigned to the instances in the activation, and the number of instances activated by this registration.</p>"]
                pub fn describe_activations(&self, input: &DescribeActivationsRequest)  -> Result<DescribeActivationsResult, DescribeActivationsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeActivations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeActivationsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeActivationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the associations for the specified Systems Manager document or instance.</p>"]
                pub fn describe_association(&self, input: &DescribeAssociationRequest)  -> Result<DescribeAssociationResult, DescribeAssociationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeAssociation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAssociationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeAssociationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Provides details about all active and terminated Automation executions.</p>"]
                pub fn describe_automation_executions(&self, input: &DescribeAutomationExecutionsRequest)  -> Result<DescribeAutomationExecutionsResult, DescribeAutomationExecutionsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeAutomationExecutions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAutomationExecutionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeAutomationExecutionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists all patches that could possibly be included in a patch baseline.</p>"]
                pub fn describe_available_patches(&self, input: &DescribeAvailablePatchesRequest)  -> Result<DescribeAvailablePatchesResult, DescribeAvailablePatchesError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeAvailablePatches");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAvailablePatchesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeAvailablePatchesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the specified SSM document.</p>"]
                pub fn describe_document(&self, input: &DescribeDocumentRequest)  -> Result<DescribeDocumentResult, DescribeDocumentError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeDocument");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDocumentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeDocumentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes the permissions for a Systems Manager document. If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a user’s AWS account ID) or publicly (<i>All</i>). </p>"]
                pub fn describe_document_permission(&self, input: &DescribeDocumentPermissionRequest)  -> Result<DescribeDocumentPermissionResponse, DescribeDocumentPermissionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeDocumentPermission");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeDocumentPermissionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeDocumentPermissionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>All associations for the instance(s).</p>"]
                pub fn describe_effective_instance_associations(&self, input: &DescribeEffectiveInstanceAssociationsRequest)  -> Result<DescribeEffectiveInstanceAssociationsResult, DescribeEffectiveInstanceAssociationsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeEffectiveInstanceAssociations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEffectiveInstanceAssociationsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeEffectiveInstanceAssociationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline.</p>"]
                pub fn describe_effective_patches_for_patch_baseline(&self, input: &DescribeEffectivePatchesForPatchBaselineRequest)  -> Result<DescribeEffectivePatchesForPatchBaselineResult, DescribeEffectivePatchesForPatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeEffectivePatchesForPatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEffectivePatchesForPatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeEffectivePatchesForPatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The status of the associations for the instance(s).</p>"]
                pub fn describe_instance_associations_status(&self, input: &DescribeInstanceAssociationsStatusRequest)  -> Result<DescribeInstanceAssociationsStatusResult, DescribeInstanceAssociationsStatusError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeInstanceAssociationsStatus");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstanceAssociationsStatusResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInstanceAssociationsStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes one or more of your instances. You can use this to get information about instances like the operating system platform, the SSM Agent version (Linux), status etc. If you specify one or more instance IDs, it returns information for those instances. If you do not specify instance IDs, it returns information for all your instances. If you specify an instance ID that is not valid or an instance that you do not own, you receive an error. </p>"]
                pub fn describe_instance_information(&self, input: &DescribeInstanceInformationRequest)  -> Result<DescribeInstanceInformationResult, DescribeInstanceInformationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeInstanceInformation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstanceInformationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInstanceInformationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the high-level patch state of one or more instances.</p>"]
                pub fn describe_instance_patch_states(&self, input: &DescribeInstancePatchStatesRequest)  -> Result<DescribeInstancePatchStatesResult, DescribeInstancePatchStatesError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatchStates");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstancePatchStatesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInstancePatchStatesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the high-level patch state for the instances in the specified patch group.</p>"]
                pub fn describe_instance_patch_states_for_patch_group(&self, input: &DescribeInstancePatchStatesForPatchGroupRequest)  -> Result<DescribeInstancePatchStatesForPatchGroupResult, DescribeInstancePatchStatesForPatchGroupError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatchStatesForPatchGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstancePatchStatesForPatchGroupResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInstancePatchStatesForPatchGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves information about the patches on the specified instance and their state relative to the patch baseline being used for the instance.</p>"]
                pub fn describe_instance_patches(&self, input: &DescribeInstancePatchesRequest)  -> Result<DescribeInstancePatchesResult, DescribeInstancePatchesError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeInstancePatches");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeInstancePatchesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeInstancePatchesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the individual task executions (one per target) for a particular task executed as part of a Maintenance Window execution.</p>"]
                pub fn describe_maintenance_window_execution_task_invocations(&self, input: &DescribeMaintenanceWindowExecutionTaskInvocationsRequest)  -> Result<DescribeMaintenanceWindowExecutionTaskInvocationsResult, DescribeMaintenanceWindowExecutionTaskInvocationsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowExecutionTaskInvocations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowExecutionTaskInvocationsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowExecutionTaskInvocationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>For a given Maintenance Window execution, lists the tasks that were executed.</p>"]
                pub fn describe_maintenance_window_execution_tasks(&self, input: &DescribeMaintenanceWindowExecutionTasksRequest)  -> Result<DescribeMaintenanceWindowExecutionTasksResult, DescribeMaintenanceWindowExecutionTasksError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowExecutionTasks");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowExecutionTasksResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowExecutionTasksError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the executions of a Maintenance Window (meaning, information about when the Maintenance Window was scheduled to be active and information about tasks registered and run with the Maintenance Window).</p>"]
                pub fn describe_maintenance_window_executions(&self, input: &DescribeMaintenanceWindowExecutionsRequest)  -> Result<DescribeMaintenanceWindowExecutionsResult, DescribeMaintenanceWindowExecutionsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowExecutions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowExecutionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowExecutionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the targets registered with the Maintenance Window.</p>"]
                pub fn describe_maintenance_window_targets(&self, input: &DescribeMaintenanceWindowTargetsRequest)  -> Result<DescribeMaintenanceWindowTargetsResult, DescribeMaintenanceWindowTargetsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTargets");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowTargetsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowTargetsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the tasks in a Maintenance Window.</p>"]
                pub fn describe_maintenance_window_tasks(&self, input: &DescribeMaintenanceWindowTasksRequest)  -> Result<DescribeMaintenanceWindowTasksResult, DescribeMaintenanceWindowTasksError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindowTasks");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowTasksResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowTasksError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the Maintenance Windows in an AWS account.</p>"]
                pub fn describe_maintenance_windows(&self, input: &DescribeMaintenanceWindowsRequest)  -> Result<DescribeMaintenanceWindowsResult, DescribeMaintenanceWindowsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeMaintenanceWindows");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeMaintenanceWindowsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeMaintenanceWindowsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Get information about a parameter.</p>"]
                pub fn describe_parameters(&self, input: &DescribeParametersRequest)  -> Result<DescribeParametersResult, DescribeParametersError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribeParameters");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeParametersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribeParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the patch baselines in your AWS account.</p>"]
                pub fn describe_patch_baselines(&self, input: &DescribePatchBaselinesRequest)  -> Result<DescribePatchBaselinesResult, DescribePatchBaselinesError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribePatchBaselines");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribePatchBaselinesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribePatchBaselinesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns high-level aggregated patch compliance state for a patch group.</p>"]
                pub fn describe_patch_group_state(&self, input: &DescribePatchGroupStateRequest)  -> Result<DescribePatchGroupStateResult, DescribePatchGroupStateError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroupState");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribePatchGroupStateResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribePatchGroupStateError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists all patch groups that have been registered with patch baselines.</p>"]
                pub fn describe_patch_groups(&self, input: &DescribePatchGroupsRequest)  -> Result<DescribePatchGroupsResult, DescribePatchGroupsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.DescribePatchGroups");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribePatchGroupsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(DescribePatchGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Get detailed information about a particular Automation execution.</p>"]
                pub fn get_automation_execution(&self, input: &GetAutomationExecutionRequest)  -> Result<GetAutomationExecutionResult, GetAutomationExecutionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetAutomationExecution");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetAutomationExecutionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetAutomationExecutionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns detailed information about command execution for an invocation or plugin. </p>"]
                pub fn get_command_invocation(&self, input: &GetCommandInvocationRequest)  -> Result<GetCommandInvocationResult, GetCommandInvocationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetCommandInvocation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetCommandInvocationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetCommandInvocationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the default patch baseline.</p>"]
                pub fn get_default_patch_baseline(&self, input: &GetDefaultPatchBaselineRequest)  -> Result<GetDefaultPatchBaselineResult, GetDefaultPatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetDefaultPatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDefaultPatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDefaultPatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the current snapshot for the patch baseline the instance uses. This API is primarily used by the AWS-ApplyPatchBaseline Systems Manager document. </p>"]
                pub fn get_deployable_patch_snapshot_for_instance(&self, input: &GetDeployablePatchSnapshotForInstanceRequest)  -> Result<GetDeployablePatchSnapshotForInstanceResult, GetDeployablePatchSnapshotForInstanceError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetDeployablePatchSnapshotForInstance");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDeployablePatchSnapshotForInstanceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDeployablePatchSnapshotForInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Gets the contents of the specified SSM document.</p>"]
                pub fn get_document(&self, input: &GetDocumentRequest)  -> Result<GetDocumentResult, GetDocumentError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetDocument");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetDocumentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetDocumentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Query inventory information.</p>"]
                pub fn get_inventory(&self, input: &GetInventoryRequest)  -> Result<GetInventoryResult, GetInventoryError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetInventory");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetInventoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetInventoryError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. </p>"]
                pub fn get_inventory_schema(&self, input: &GetInventorySchemaRequest)  -> Result<GetInventorySchemaResult, GetInventorySchemaError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetInventorySchema");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetInventorySchemaResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetInventorySchemaError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves a Maintenance Window.</p>"]
                pub fn get_maintenance_window(&self, input: &GetMaintenanceWindowRequest)  -> Result<GetMaintenanceWindowResult, GetMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves details about a specific task executed as part of a Maintenance Window execution.</p>"]
                pub fn get_maintenance_window_execution(&self, input: &GetMaintenanceWindowExecutionRequest)  -> Result<GetMaintenanceWindowExecutionResult, GetMaintenanceWindowExecutionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowExecution");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetMaintenanceWindowExecutionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetMaintenanceWindowExecutionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the details about a specific task executed as part of a Maintenance Window execution.</p>"]
                pub fn get_maintenance_window_execution_task(&self, input: &GetMaintenanceWindowExecutionTaskRequest)  -> Result<GetMaintenanceWindowExecutionTaskResult, GetMaintenanceWindowExecutionTaskError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetMaintenanceWindowExecutionTask");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetMaintenanceWindowExecutionTaskResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetMaintenanceWindowExecutionTaskError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Query a list of all parameters used by the AWS account.</p>"]
                pub fn get_parameter_history(&self, input: &GetParameterHistoryRequest)  -> Result<GetParameterHistoryResult, GetParameterHistoryError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetParameterHistory");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetParameterHistoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetParameterHistoryError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Get details of a parameter.</p>"]
                pub fn get_parameters(&self, input: &GetParametersRequest)  -> Result<GetParametersResult, GetParametersError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetParameters");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetParametersResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves information about a patch baseline.</p>"]
                pub fn get_patch_baseline(&self, input: &GetPatchBaselineRequest)  -> Result<GetPatchBaselineResult, GetPatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetPatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetPatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetPatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Retrieves the patch baseline that should be used for the specified patch group.</p>"]
                pub fn get_patch_baseline_for_patch_group(&self, input: &GetPatchBaselineForPatchGroupRequest)  -> Result<GetPatchBaselineForPatchGroupResult, GetPatchBaselineForPatchGroupError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.GetPatchBaselineForPatchGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<GetPatchBaselineForPatchGroupResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(GetPatchBaselineForPatchGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the associations for the specified Systems Manager document or instance.</p>"]
                pub fn list_associations(&self, input: &ListAssociationsRequest)  -> Result<ListAssociationsResult, ListAssociationsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListAssociations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListAssociationsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListAssociationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>An invocation is copy of a command sent to a specific instance. A command can apply to one or more instances. A command invocation applies to one instance. For example, if a user executes SendCommand against three instances, then a command invocation is created for each requested instance ID. ListCommandInvocations provide status about command execution.</p>"]
                pub fn list_command_invocations(&self, input: &ListCommandInvocationsRequest)  -> Result<ListCommandInvocationsResult, ListCommandInvocationsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListCommandInvocations");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListCommandInvocationsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListCommandInvocationsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Lists the commands requested by users of the AWS account.</p>"]
                pub fn list_commands(&self, input: &ListCommandsRequest)  -> Result<ListCommandsResult, ListCommandsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListCommands");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListCommandsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListCommandsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>List all versions for a document.</p>"]
                pub fn list_document_versions(&self, input: &ListDocumentVersionsRequest)  -> Result<ListDocumentVersionsResult, ListDocumentVersionsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListDocumentVersions");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDocumentVersionsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDocumentVersionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Describes one or more of your SSM documents.</p>"]
                pub fn list_documents(&self, input: &ListDocumentsRequest)  -> Result<ListDocumentsResult, ListDocumentsError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListDocuments");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListDocumentsResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListDocumentsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>A list of inventory items returned by the request.</p>"]
                pub fn list_inventory_entries(&self, input: &ListInventoryEntriesRequest)  -> Result<ListInventoryEntriesResult, ListInventoryEntriesError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListInventoryEntries");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListInventoryEntriesResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListInventoryEntriesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Returns a list of the tags assigned to the specified resource.</p>"]
                pub fn list_tags_for_resource(&self, input: &ListTagsForResourceRequest)  -> Result<ListTagsForResourceResult, ListTagsForResourceError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ListTagsForResource");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsForResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Shares a Systems Manager document publicly or privately. If you share a document privately, you must specify the AWS user account IDs for those people who can use the document. If you share a document publicly, you must specify <i>All</i> as the account ID.</p>"]
                pub fn modify_document_permission(&self, input: &ModifyDocumentPermissionRequest)  -> Result<ModifyDocumentPermissionResponse, ModifyDocumentPermissionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.ModifyDocumentPermission");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyDocumentPermissionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(ModifyDocumentPermissionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Bulk update custom inventory items on one more instance. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist.</p>"]
                pub fn put_inventory(&self, input: &PutInventoryRequest)  -> Result<PutInventoryResult, PutInventoryError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.PutInventory");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutInventoryResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutInventoryError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Add one or more paramaters to the system.</p>"]
                pub fn put_parameter(&self, input: &PutParameterRequest)  -> Result<PutParameterResult, PutParameterError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.PutParameter");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<PutParameterResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(PutParameterError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Defines the default patch baseline.</p>"]
                pub fn register_default_patch_baseline(&self, input: &RegisterDefaultPatchBaselineRequest)  -> Result<RegisterDefaultPatchBaselineResult, RegisterDefaultPatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.RegisterDefaultPatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterDefaultPatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RegisterDefaultPatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Registers a patch baseline for a patch group.</p>"]
                pub fn register_patch_baseline_for_patch_group(&self, input: &RegisterPatchBaselineForPatchGroupRequest)  -> Result<RegisterPatchBaselineForPatchGroupResult, RegisterPatchBaselineForPatchGroupError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.RegisterPatchBaselineForPatchGroup");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterPatchBaselineForPatchGroupResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RegisterPatchBaselineForPatchGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Registers a target with a Maintenance Window.</p>"]
                pub fn register_target_with_maintenance_window(&self, input: &RegisterTargetWithMaintenanceWindowRequest)  -> Result<RegisterTargetWithMaintenanceWindowResult, RegisterTargetWithMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.RegisterTargetWithMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterTargetWithMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RegisterTargetWithMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Adds a new task to a Maintenance Window.</p>"]
                pub fn register_task_with_maintenance_window(&self, input: &RegisterTaskWithMaintenanceWindowRequest)  -> Result<RegisterTaskWithMaintenanceWindowResult, RegisterTaskWithMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.RegisterTaskWithMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RegisterTaskWithMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RegisterTaskWithMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Removes all tags from the specified resource.</p>"]
                pub fn remove_tags_from_resource(&self, input: &RemoveTagsFromResourceRequest)  -> Result<RemoveTagsFromResourceResult, RemoveTagsFromResourceError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.RemoveTagsFromResource");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTagsFromResourceResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(RemoveTagsFromResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Executes commands on one or more remote instances.</p>"]
                pub fn send_command(&self, input: &SendCommandRequest)  -> Result<SendCommandResult, SendCommandError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.SendCommand");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<SendCommandResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(SendCommandError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Initiates execution of an Automation document.</p>"]
                pub fn start_automation_execution(&self, input: &StartAutomationExecutionRequest)  -> Result<StartAutomationExecutionResult, StartAutomationExecutionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.StartAutomationExecution");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartAutomationExecutionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(StartAutomationExecutionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Stop an Automation that is currently executing.</p>"]
                pub fn stop_automation_execution(&self, input: &StopAutomationExecutionRequest)  -> Result<StopAutomationExecutionResult, StopAutomationExecutionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.StopAutomationExecution");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<StopAutomationExecutionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(StopAutomationExecutionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Updates an association. You can only update the document version, schedule, parameters, and Amazon S3 output of an association.</p>"]
                pub fn update_association(&self, input: &UpdateAssociationRequest)  -> Result<UpdateAssociationResult, UpdateAssociationError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateAssociation");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateAssociationResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateAssociationError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Updates the status of the Systems Manager document associated with the specified instance.</p>"]
                pub fn update_association_status(&self, input: &UpdateAssociationStatusRequest)  -> Result<UpdateAssociationStatusResult, UpdateAssociationStatusError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateAssociationStatus");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateAssociationStatusResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateAssociationStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>The document you want to update.</p>"]
                pub fn update_document(&self, input: &UpdateDocumentRequest)  -> Result<UpdateDocumentResult, UpdateDocumentError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateDocument");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateDocumentResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateDocumentError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Set the default version of a document. </p>"]
                pub fn update_document_default_version(&self, input: &UpdateDocumentDefaultVersionRequest)  -> Result<UpdateDocumentDefaultVersionResult, UpdateDocumentDefaultVersionError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateDocumentDefaultVersion");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateDocumentDefaultVersionResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateDocumentDefaultVersionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Updates an existing Maintenance Window. Only specified parameters are modified.</p>"]
                pub fn update_maintenance_window(&self, input: &UpdateMaintenanceWindowRequest)  -> Result<UpdateMaintenanceWindowResult, UpdateMaintenanceWindowError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateMaintenanceWindow");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateMaintenanceWindowResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateMaintenanceWindowError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Assigns or changes an Amazon Identity and Access Management (IAM) role to the managed instance.</p>"]
                pub fn update_managed_instance_role(&self, input: &UpdateManagedInstanceRoleRequest)  -> Result<UpdateManagedInstanceRoleResult, UpdateManagedInstanceRoleError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdateManagedInstanceRole");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateManagedInstanceRoleResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdateManagedInstanceRoleError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                

                #[doc="<p>Modifies an existing patch baseline. Fields not specified in the request are left unchanged.</p>"]
                pub fn update_patch_baseline(&self, input: &UpdatePatchBaselineRequest)  -> Result<UpdatePatchBaselineResult, UpdatePatchBaselineError> {
                    let mut request = SignedRequest::new("POST", "ssm", self.region, "/");
                    
                    request.set_content_type("application/x-amz-json-1.1".to_owned());
                    request.add_header("x-amz-target", "AmazonSSM.UpdatePatchBaseline");
                    let encoded = serde_json::to_string(input).unwrap();
         request.set_payload(Some(encoded.into_bytes()));
         
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdatePatchBaselineResult>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
                        _ => Err(UpdatePatchBaselineError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
                    }
                }
                
}
