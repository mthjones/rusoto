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
pub type ArtifactNamespace = String;
pub type ArtifactPackaging = String;
pub type ArtifactsType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchGetBuildsInput {
    #[doc="<p>The IDs of the builds.</p>"]
    #[serde(rename="ids")]
    pub ids: BuildIds,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchGetBuildsOutput {
    #[doc="<p>Information about the requested builds.</p>"]
    #[serde(rename="builds")]
    pub builds: Option<Builds>,
    #[doc="<p>The IDs of builds for which information could not be found.</p>"]
    #[serde(rename="buildsNotFound")]
    pub builds_not_found: Option<BuildIds>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct BatchGetProjectsInput {
    #[doc="<p>The names of the build projects.</p>"]
    #[serde(rename="names")]
    pub names: ProjectNames,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct BatchGetProjectsOutput {
    #[doc="<p>Information about the requested build projects.</p>"]
    #[serde(rename="projects")]
    pub projects: Option<Projects>,
    #[doc="<p>The names of build projects for which information could not be found.</p>"]
    #[serde(rename="projectsNotFound")]
    pub projects_not_found: Option<ProjectNames>,
}

pub type Boolean = bool;
#[doc="<p>Information about a build.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Build {
    #[doc="<p>The Amazon Resource Name (ARN) of the build.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<NonEmptyString>,
    #[doc="<p>Information about the output artifacts for the build.</p>"]
    #[serde(rename="artifacts")]
    pub artifacts: Option<BuildArtifacts>,
    #[doc="<p>Whether the build has finished. True if completed; otherwise, false.</p>"]
    #[serde(rename="buildComplete")]
    #[serde(skip_serializing_if="::std::option::Option::is_none")]
    pub build_complete: Option<Boolean>,
    #[doc="<p>The current status of the build. Valid values include:</p> <ul> <li> <p> <code>FAILED</code>: The build failed.</p> </li> <li> <p> <code>FAULT</code>: The build faulted.</p> </li> <li> <p> <code>IN_PROGRESS</code>: The build is still in progress.</p> </li> <li> <p> <code>STOPPED</code>: The build stopped.</p> </li> <li> <p> <code>SUCCEEDED</code>: The build succeeded.</p> </li> <li> <p> <code>TIMED_OUT</code>: The build timed out.</p> </li> </ul>"]
    #[serde(rename="buildStatus")]
    pub build_status: Option<StatusType>,
    #[doc="<p>The current build phase.</p>"]
    #[serde(rename="currentPhase")]
    pub current_phase: Option<String>,
    #[doc="<p>When the build process ended, expressed in Unix time format.</p>"]
    #[serde(rename="endTime")]
    pub end_time: Option<Timestamp>,
    #[doc="<p>Information about the build environment for this build.</p>"]
    #[serde(rename="environment")]
    pub environment: Option<ProjectEnvironment>,
    #[doc="<p>The unique ID for the build.</p>"]
    #[serde(rename="id")]
    pub id: Option<NonEmptyString>,
    #[doc="<p>The entity that started the build. Valid values include:</p> <ul> <li> <p>If AWS CodePipeline started the build, the pipeline's name (for example, <code>codepipeline/my-demo-pipeline</code>).</p> </li> <li> <p>If an AWS Identity and Access Management (IAM) user started the build, the user's name (for example <code>MyUserName</code>).</p> </li> <li> <p>If the Jenkins plugin for AWS CodeBuild started the build, the string <code>CodeBuild-Jenkins-Plugin</code>.</p> </li> </ul>"]
    #[serde(rename="initiator")]
    pub initiator: Option<String>,
    #[doc="<p>Information about the build's logs in Amazon CloudWatch Logs.</p>"]
    #[serde(rename="logs")]
    pub logs: Option<LogsLocation>,
    #[doc="<p>Information about all previous build phases that are completed and information about any current build phase that is not yet complete.</p>"]
    #[serde(rename="phases")]
    pub phases: Option<BuildPhases>,
    #[doc="<p>The name of the build project.</p>"]
    #[serde(rename="projectName")]
    pub project_name: Option<NonEmptyString>,
    #[doc="<p>Information about the source code to be built.</p>"]
    #[serde(rename="source")]
    pub source: Option<ProjectSource>,
    #[doc="<p>Any version identifier for the version of the source code to be built.</p>"]
    #[serde(rename="sourceVersion")]
    pub source_version: Option<NonEmptyString>,
    #[doc="<p>When the build process started, expressed in Unix time format.</p>"]
    #[serde(rename="startTime")]
    pub start_time: Option<Timestamp>,
    #[doc="<p>How long, in minutes, for AWS CodeBuild to wait before timing out this build if it does not get marked as completed.</p>"]
    #[serde(rename="timeoutInMinutes")]
    pub timeout_in_minutes: Option<WrapperInt>,
}

#[doc="<p>Information about build output artifacts.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BuildArtifacts {
    #[doc="<p>Information about the location of the build artifacts.</p>"]
    #[serde(rename="location")]
    pub location: Option<String>,
    #[doc="<p>The MD5 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm both file integrity and authenticity.</p> <note> <p>This value is available only if the build project's <code>packaging</code> value is set to <code>ZIP</code>.</p> </note>"]
    #[serde(rename="md5sum")]
    pub md_5sum: Option<String>,
    #[doc="<p>The SHA-256 hash of the build artifact.</p> <p>You can use this hash along with a checksum tool to confirm both file integrity and authenticity.</p> <note> <p>This value is available only if the build project's <code>packaging</code> value is set to <code>ZIP</code>.</p> </note>"]
    #[serde(rename="sha256sum")]
    pub sha_25_6sum: Option<String>,
}

pub type BuildIds = Vec<NonEmptyString>;
#[doc="<p>Information about a stage for a build.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BuildPhase {
    #[doc="<p>Additional information about a build phase, especially to help troubleshoot a failed build.</p>"]
    #[serde(rename="contexts")]
    pub contexts: Option<PhaseContexts>,
    #[doc="<p>How long, in seconds, between the starting and ending times of the build's phase.</p>"]
    #[serde(rename="durationInSeconds")]
    pub duration_in_seconds: Option<WrapperLong>,
    #[doc="<p>When the build phase ended, expressed in Unix time format.</p>"]
    #[serde(rename="endTime")]
    pub end_time: Option<Timestamp>,
    #[doc="<p>The current status of the build phase. Valid values include:</p> <ul> <li> <p> <code>FAILED</code>: The build phase failed.</p> </li> <li> <p> <code>FAULT</code>: The build phase faulted.</p> </li> <li> <p> <code>IN_PROGRESS</code>: The build phase is still in progress.</p> </li> <li> <p> <code>STOPPED</code>: The build phase stopped.</p> </li> <li> <p> <code>SUCCEEDED</code>: The build phase succeeded.</p> </li> <li> <p> <code>TIMED_OUT</code>: The build phase timed out.</p> </li> </ul>"]
    #[serde(rename="phaseStatus")]
    pub phase_status: Option<StatusType>,
    #[doc="<p>The name of the build phase. Valid values include:</p> <ul> <li> <p> <code>BUILD</code>: Core build activities typically occur in this build phase.</p> </li> <li> <p> <code>COMPLETED</code>: The build has been completed.</p> </li> <li> <p> <code>DOWNLOAD_SOURCE</code>: Source code is being downloaded in this build phase.</p> </li> <li> <p> <code>FINALIZING</code>: The build process is completing in this build phase.</p> </li> <li> <p> <code>INSTALL</code>: Installation activities typically occur in this build phase.</p> </li> <li> <p> <code>POST_BUILD</code>: Post-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PRE_BUILD</code>: Pre-build activities typically occur in this build phase.</p> </li> <li> <p> <code>PROVISIONING</code>: The build environment is being set up.</p> </li> <li> <p> <code>SUBMITTED</code>: The build has been submitted.</p> </li> <li> <p> <code>UPLOAD_ARTIFACTS</code>: Build output artifacts are being uploaded to the output location.</p> </li> </ul>"]
    #[serde(rename="phaseType")]
    pub phase_type: Option<BuildPhaseType>,
    #[doc="<p>When the build phase started, expressed in Unix time format.</p>"]
    #[serde(rename="startTime")]
    pub start_time: Option<Timestamp>,
}

pub type BuildPhaseType = String;
pub type BuildPhases = Vec<BuildPhase>;
pub type Builds = Vec<Build>;
pub type ComputeType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateProjectInput {
    #[doc="<p>Information about the build output artifacts for the build project.</p>"]
    #[serde(rename="artifacts")]
    pub artifacts: ProjectArtifacts,
    #[doc="<p>A description that makes the build project easy to identify.</p>"]
    #[serde(rename="description")]
    pub description: Option<ProjectDescription>,
    #[doc="<p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>You can specify either the CMK's Amazon Resource Name (ARN) or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>"]
    #[serde(rename="encryptionKey")]
    pub encryption_key: Option<NonEmptyString>,
    #[doc="<p>Information about the build environment for the build project.</p>"]
    #[serde(rename="environment")]
    pub environment: ProjectEnvironment,
    #[doc="<p>The name of the build project.</p>"]
    #[serde(rename="name")]
    pub name: ProjectName,
    #[doc="<p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>"]
    #[serde(rename="serviceRole")]
    pub service_role: Option<NonEmptyString>,
    #[doc="<p>Information about the build input source code for the build project.</p>"]
    #[serde(rename="source")]
    pub source: ProjectSource,
    #[doc="<p>A set of tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<TagList>,
    #[doc="<p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait until timing out any build that has not been marked as completed. The default is 60 minutes.</p>"]
    #[serde(rename="timeoutInMinutes")]
    pub timeout_in_minutes: Option<TimeOut>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateProjectOutput {
    #[doc="<p>Information about the build project that was created.</p>"]
    #[serde(rename="project")]
    pub project: Option<Project>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteProjectInput {
    #[doc="<p>The name of the build project.</p>"]
    #[serde(rename="name")]
    pub name: NonEmptyString,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteProjectOutput;

#[doc="<p>Information about a Docker image that is managed by AWS CodeBuild.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnvironmentImage {
    #[doc="<p>The description of the Docker image.</p>"]
    #[serde(rename="description")]
    pub description: Option<String>,
    #[doc="<p>The name of the Docker image.</p>"]
    #[serde(rename="name")]
    pub name: Option<String>,
}

pub type EnvironmentImages = Vec<EnvironmentImage>;
#[doc="<p>A set of Docker images that are related by programming language and are managed by AWS CodeBuild.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnvironmentLanguage {
    #[doc="<p>The list of Docker images that are related by the specified programming language.</p>"]
    #[serde(rename="images")]
    pub images: Option<EnvironmentImages>,
    #[doc="<p>The programming language for the Docker images.</p>"]
    #[serde(rename="language")]
    pub language: Option<LanguageType>,
}

pub type EnvironmentLanguages = Vec<EnvironmentLanguage>;
#[doc="<p>A set of Docker images that are related by platform and are managed by AWS CodeBuild.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EnvironmentPlatform {
    #[doc="<p>The list of programming languages that are available for the specified platform.</p>"]
    #[serde(rename="languages")]
    pub languages: Option<EnvironmentLanguages>,
    #[doc="<p>The platform's name.</p>"]
    #[serde(rename="platform")]
    pub platform: Option<PlatformType>,
}

pub type EnvironmentPlatforms = Vec<EnvironmentPlatform>;
pub type EnvironmentType = String;
#[doc="<p>Information about an environment variable for a build project or a build.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EnvironmentVariable {
    #[doc="<p>The name or key of the environment variable.</p>"]
    #[serde(rename="name")]
    pub name: NonEmptyString,
    #[doc="<p>The value of the environment variable.</p>"]
    #[serde(rename="value")]
    pub value: String,
}

pub type EnvironmentVariables = Vec<EnvironmentVariable>;
pub type KeyInput = String;
pub type LanguageType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListBuildsForProjectInput {
    #[doc="<p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<String>,
    #[doc="<p>The name of the build project.</p>"]
    #[serde(rename="projectName")]
    pub project_name: NonEmptyString,
    #[doc="<p>The order to list build IDs. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build IDs in ascending order by build ID.</p> </li> <li> <p> <code>DESCENDING</code>: List the build IDs in descending order by build ID.</p> </li> </ul>"]
    #[serde(rename="sortOrder")]
    pub sort_order: Option<SortOrderType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListBuildsForProjectOutput {
    #[doc="<p>A list of build IDs for the specified build project, with each build ID representing a single build.</p>"]
    #[serde(rename="ids")]
    pub ids: Option<BuildIds>,
    #[doc="<p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListBuildsInput {
    #[doc="<p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<String>,
    #[doc="<p>The order to list build IDs. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build IDs in ascending order by build ID.</p> </li> <li> <p> <code>DESCENDING</code>: List the build IDs in descending order by build ID.</p> </li> </ul>"]
    #[serde(rename="sortOrder")]
    pub sort_order: Option<SortOrderType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListBuildsOutput {
    #[doc="<p>A list of build IDs, with each build ID representing a single build.</p>"]
    #[serde(rename="ids")]
    pub ids: Option<BuildIds>,
    #[doc="<p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListCuratedEnvironmentImagesInput;

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListCuratedEnvironmentImagesOutput {
    #[doc="<p>Information about supported platforms for Docker images that are managed by AWS CodeBuild.</p>"]
    #[serde(rename="platforms")]
    pub platforms: Option<EnvironmentPlatforms>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListProjectsInput {
    #[doc="<p>During a previous call, if there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call. To get all of the items in the list, keep calling this operation with each subsequent next token that is returned, until no more next tokens are returned.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<NonEmptyString>,
    #[doc="<p>The criterion to be used to list build project names. Valid values include:</p> <ul> <li> <p> <code>CREATED_TIME</code>: List the build project names based on when each build project was created.</p> </li> <li> <p> <code>LAST_MODIFIED_TIME</code>: List the build project names based on when information about each build project was last changed.</p> </li> <li> <p> <code>NAME</code>: List the build project names based on each build project's name.</p> </li> </ul> <p>Use <code>sortOrder</code> to specify in what order to list the build project names based on the preceding criteria.</p>"]
    #[serde(rename="sortBy")]
    pub sort_by: Option<ProjectSortByType>,
    #[doc="<p>The order in which to list build projects. Valid values include:</p> <ul> <li> <p> <code>ASCENDING</code>: List the build project names in ascending order.</p> </li> <li> <p> <code>DESCENDING</code>: List the build project names in descending order.</p> </li> </ul> <p>Use <code>sortBy</code> to specify the criterion to be used to list build project names.</p>"]
    #[serde(rename="sortOrder")]
    pub sort_order: Option<SortOrderType>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListProjectsOutput {
    #[doc="<p>If there are more than 100 items in the list, only the first 100 items are returned, along with a unique string called a <i>next token</i>. To get the next batch of items in the list, call this operation again, adding the next token to the call.</p>"]
    #[serde(rename="nextToken")]
    pub next_token: Option<String>,
    #[doc="<p>The list of build project names, with each build project name representing a single build project.</p>"]
    #[serde(rename="projects")]
    pub projects: Option<ProjectNames>,
}

#[doc="<p>Information about build logs in Amazon CloudWatch Logs.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct LogsLocation {
    #[doc="<p>The URL to an individual build log in Amazon CloudWatch Logs.</p>"]
    #[serde(rename="deepLink")]
    pub deep_link: Option<String>,
    #[doc="<p>The name of the Amazon CloudWatch Logs group for the build logs.</p>"]
    #[serde(rename="groupName")]
    pub group_name: Option<String>,
    #[doc="<p>The name of the Amazon CloudWatch Logs stream for the build logs.</p>"]
    #[serde(rename="streamName")]
    pub stream_name: Option<String>,
}

pub type NonEmptyString = String;
#[doc="<p>Additional information about a build phase that has an error. You can use this information to help troubleshoot a failed build.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PhaseContext {
    #[doc="<p>An explanation of the build phase's context. This explanation might include a command ID and an exit code.</p>"]
    #[serde(rename="message")]
    pub message: Option<String>,
    #[doc="<p>The status code for the context of the build phase.</p>"]
    #[serde(rename="statusCode")]
    pub status_code: Option<String>,
}

pub type PhaseContexts = Vec<PhaseContext>;
pub type PlatformType = String;
#[doc="<p>Information about a build project.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Project {
    #[doc="<p>The Amazon Resource Name (ARN) of the build project.</p>"]
    #[serde(rename="arn")]
    pub arn: Option<String>,
    #[doc="<p>Information about the build output artifacts for the build project.</p>"]
    #[serde(rename="artifacts")]
    pub artifacts: Option<ProjectArtifacts>,
    #[doc="<p>When the build project was created, expressed in Unix time format.</p>"]
    #[serde(rename="created")]
    pub created: Option<Timestamp>,
    #[doc="<p>A description that makes the build project easy to identify.</p>"]
    #[serde(rename="description")]
    pub description: Option<ProjectDescription>,
    #[doc="<p>The AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>This is expressed either as the CMK's Amazon Resource Name (ARN) or, if specified, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>"]
    #[serde(rename="encryptionKey")]
    pub encryption_key: Option<NonEmptyString>,
    #[doc="<p>Information about the build environment for this build project.</p>"]
    #[serde(rename="environment")]
    pub environment: Option<ProjectEnvironment>,
    #[doc="<p>When the build project's settings were last modified, expressed in Unix time format.</p>"]
    #[serde(rename="lastModified")]
    pub last_modified: Option<Timestamp>,
    #[doc="<p>The name of the build project.</p>"]
    #[serde(rename="name")]
    pub name: Option<ProjectName>,
    #[doc="<p>The ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>"]
    #[serde(rename="serviceRole")]
    pub service_role: Option<NonEmptyString>,
    #[doc="<p>Information about the build input source code for this build project.</p>"]
    #[serde(rename="source")]
    pub source: Option<ProjectSource>,
    #[doc="<p>The tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<TagList>,
    #[doc="<p>How long, in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any related build that did not get marked as completed. The default is 60 minutes.</p>"]
    #[serde(rename="timeoutInMinutes")]
    pub timeout_in_minutes: Option<TimeOut>,
}

#[doc="<p>Information about the build output artifacts for the build project.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ProjectArtifacts {
    #[doc="<p>Information about the build output artifact location, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output locations instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output bucket.</p> </li> </ul>"]
    #[serde(rename="location")]
    pub location: Option<String>,
    #[doc="<p>Along with <code>path</code> and <code>namespaceType</code>, the pattern that AWS CodeBuild will use to name and store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the name of the output artifact object.</p> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>.</p>"]
    #[serde(rename="name")]
    pub name: Option<String>,
    #[doc="<p>Along with <code>path</code> and <code>name</code>, the pattern that AWS CodeBuild will use to determine the name and location to store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, then valid values include:</p> <ul> <li> <p> <code>BUILD_ID</code>: Include the build ID in the location of the build output artifact.</p> </li> <li> <p> <code>NONE</code>: Do not include the build ID. This is the default if <code>namespaceType</code> is not specified.</p> </li> </ul> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>BUILD_ID</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in <code>MyArtifacts/<i>build-ID</i>/MyArtifact.zip</code>.</p>"]
    #[serde(rename="namespaceType")]
    pub namespace_type: Option<ArtifactNamespace>,
    #[doc="<p>The type of build output artifact to create, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output artifacts instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, valid values include:</p> <ul> <li> <p> <code>NONE</code>: AWS CodeBuild will create in the output bucket a folder containing the build output. This is the default if <code>packaging</code> is not specified.</p> </li> <li> <p> <code>ZIP</code>: AWS CodeBuild will create in the output bucket a ZIP file containing the build output.</p> </li> </ul> </li> </ul>"]
    #[serde(rename="packaging")]
    pub packaging: Option<ArtifactPackaging>,
    #[doc="<p>Along with <code>namespaceType</code> and <code>name</code>, the pattern that AWS CodeBuild will use to name and store the output artifact, as follows:</p> <ul> <li> <p>If <code>type</code> is set to <code>CODEPIPELINE</code>, then AWS CodePipeline will ignore this value if specified. This is because AWS CodePipeline manages its build output names instead of AWS CodeBuild.</p> </li> <li> <p>If <code>type</code> is set to <code>NO_ARTIFACTS</code>, then this value will be ignored if specified, because no build output will be produced.</p> </li> <li> <p>If <code>type</code> is set to <code>S3</code>, this is the path to the output artifact. If <code>path</code> is not specified, then <code>path</code> will not be used.</p> </li> </ul> <p>For example, if <code>path</code> is set to <code>MyArtifacts</code>, <code>namespaceType</code> is set to <code>NONE</code>, and <code>name</code> is set to <code>MyArtifact.zip</code>, then the output artifact would be stored in the output bucket at <code>MyArtifacts/MyArtifact.zip</code>.</p>"]
    #[serde(rename="path")]
    pub path: Option<String>,
    #[doc="<p>The type of build output artifact. Valid values include:</p> <ul> <li> <p> <code>CODEPIPELINE</code>: The build project will have build output generated through AWS CodePipeline.</p> </li> <li> <p> <code>NO_ARTIFACTS</code>: The build project will not produce any build output.</p> </li> <li> <p> <code>S3</code>: The build project will store build output in Amazon Simple Storage Service (Amazon S3).</p> </li> </ul>"]
    #[serde(rename="type")]
    pub type_: ArtifactsType,
}

pub type ProjectDescription = String;
#[doc="<p>Information about the build environment of the build project.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ProjectEnvironment {
    #[doc="<p>Information about the compute resources the build project will use. Available values include:</p> <ul> <li> <p> <code>BUILD_GENERAL1_SMALL</code>: Use up to 3 GB memory and 2 vCPUs for builds.</p> </li> <li> <p> <code>BUILD_GENERAL1_MEDIUM</code>: Use up to 7 GB memory and 4 vCPUs for builds.</p> </li> <li> <p> <code>BUILD_GENERAL1_LARGE</code>: Use up to 15 GB memory and 8 vCPUs for builds.</p> </li> </ul>"]
    #[serde(rename="computeType")]
    pub compute_type: ComputeType,
    #[doc="<p>A set of environment variables to make available to builds for this build project.</p>"]
    #[serde(rename="environmentVariables")]
    pub environment_variables: Option<EnvironmentVariables>,
    #[doc="<p>The ID of the Docker image to use for this build project.</p>"]
    #[serde(rename="image")]
    pub image: NonEmptyString,
    #[doc="<p>The type of build environment to use for related builds.</p>"]
    #[serde(rename="type")]
    pub type_: EnvironmentType,
}

pub type ProjectName = String;
pub type ProjectNames = Vec<NonEmptyString>;
pub type ProjectSortByType = String;
#[doc="<p>Information about the build input source code for the build project.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ProjectSource {
    #[doc="<p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly (unless the build project's source <code>type</code> value is <code>GITHUB</code>).</p>"]
    #[serde(rename="auth")]
    pub auth: Option<SourceAuth>,
    #[doc="<p>The build spec declaration to use for the builds in this build project.</p> <p>If this value is not specified, a build spec must be included along with the source code to be built.</p>"]
    #[serde(rename="buildspec")]
    pub buildspec: Option<String>,
    #[doc="<p>Information about the location of the source code to be built. Valid values include:</p> <ul> <li> <p>For source code settings that are specified in the source action of a pipeline in AWS CodePipeline, <code>location</code> should not be specified. If it is specified, AWS CodePipeline will ignore it. This is because AWS CodePipeline uses the settings in a pipeline's source action instead of this value.</p> </li> <li> <p>For source code in an AWS CodeCommit repository, the HTTPS clone URL to the repository that contains the source code and the build spec (for example, <code>https://git-codecommit.<i>region-ID</i>.amazonaws.com/v1/repos/<i>repo-name</i> </code>).</p> </li> <li> <p>For source code in an Amazon Simple Storage Service (Amazon S3) input bucket, the path to the ZIP file that contains the source code (for example, <code> <i>bucket-name</i>/<i>path</i>/<i>to</i>/<i>object-name</i>.zip</code>)</p> </li> <li> <p>For source code in a GitHub repository, the HTTPS clone URL to the repository that contains the source and the build spec. Also, you must connect your AWS account to your GitHub account. To do this, use the AWS CodeBuild console to begin creating a build project, and follow the on-screen instructions to complete the connection. (After you have connected to your GitHub account, you do not need to finish creating the build project, and you may then leave the AWS CodeBuild console.) To instruct AWS CodeBuild to then use this connection, in the <code>source</code> object, set the <code>auth</code> object's <code>type</code> value to <code>OAUTH</code>.</p> </li> </ul>"]
    #[serde(rename="location")]
    pub location: Option<String>,
    #[doc="<p>The type of repository that contains the source code to be built. Valid values include:</p> <ul> <li> <p> <code>CODECOMMIT</code>: The source code is in an AWS CodeCommit repository.</p> </li> <li> <p> <code>CODEPIPELINE</code>: The source code settings are specified in the source action of a pipeline in AWS CodePipeline.</p> </li> <li> <p> <code>GITHUB</code>: The source code is in a GitHub repository.</p> </li> <li> <p> <code>S3</code>: The source code is in an Amazon Simple Storage Service (Amazon S3) input bucket.</p> </li> </ul>"]
    #[serde(rename="type")]
    pub type_: SourceType,
}

pub type Projects = Vec<Project>;
pub type SortOrderType = String;
#[doc="<p>Information about the authorization settings for AWS CodeBuild to access the source code to be built.</p> <p>This information is for the AWS CodeBuild console's use only. Your code should not get or set this information directly (unless the build project's source <code>type</code> value is <code>GITHUB</code>).</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SourceAuth {
    #[doc="<p>The resource value that applies to the specified authorization type.</p>"]
    #[serde(rename="resource")]
    pub resource: Option<String>,
    #[doc="<p>The authorization type to use. The only valid value is <code>OAUTH</code>, which represents the OAuth authorization type.</p>"]
    #[serde(rename="type")]
    pub type_: SourceAuthType,
}

pub type SourceAuthType = String;
pub type SourceType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartBuildInput {
    #[doc="<p>Build output artifact settings that override, for this build only, the latest ones already defined in the build project.</p>"]
    #[serde(rename="artifactsOverride")]
    pub artifacts_override: Option<ProjectArtifacts>,
    #[doc="<p>A build spec declaration that overrides, for this build only, the latest one already defined in the build project.</p>"]
    #[serde(rename="buildspecOverride")]
    pub buildspec_override: Option<String>,
    #[doc="<p>A set of environment variables that overrides, for this build only, the latest ones already defined in the build project.</p>"]
    #[serde(rename="environmentVariablesOverride")]
    pub environment_variables_override: Option<EnvironmentVariables>,
    #[doc="<p>The name of the build project to start running a build.</p>"]
    #[serde(rename="projectName")]
    pub project_name: NonEmptyString,
    #[doc="<p>A version of the build input to be built, for this build only. If not specified, the latest version will be used. If specified, must be one of:</p> <ul> <li> <p>For AWS CodeCommit or GitHub: the commit ID to use.</p> </li> <li> <p>For Amazon Simple Storage Service (Amazon S3): the version ID of the object representing the build input ZIP file to use.</p> </li> </ul>"]
    #[serde(rename="sourceVersion")]
    pub source_version: Option<String>,
    #[doc="<p>The number of build timeout minutes, from 5 to 480 (8 hours), that overrides, for this build only, the latest setting already defined in the build project.</p>"]
    #[serde(rename="timeoutInMinutesOverride")]
    pub timeout_in_minutes_override: Option<TimeOut>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartBuildOutput {
    #[doc="<p>Information about the build to be run.</p>"]
    #[serde(rename="build")]
    pub build: Option<Build>,
}

pub type StatusType = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopBuildInput {
    #[doc="<p>The ID of the build.</p>"]
    #[serde(rename="id")]
    pub id: NonEmptyString,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopBuildOutput {
    #[doc="<p>Information about the build.</p>"]
    #[serde(rename="build")]
    pub build: Option<Build>,
}

#[doc="<p>A tag, consisting of a key and a value.</p> <p>This tag is available for use by AWS services that support tags in AWS CodeBuild.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>The tag's key.</p>"]
    #[serde(rename="key")]
    pub key: Option<KeyInput>,
    #[doc="<p>The tag's value.</p>"]
    #[serde(rename="value")]
    pub value: Option<ValueInput>,
}

pub type TagList = Vec<Tag>;
pub type TimeOut = i64;
pub type Timestamp = f64;
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateProjectInput {
    #[doc="<p>Information to be changed about the build output artifacts for the build project.</p>"]
    #[serde(rename="artifacts")]
    pub artifacts: Option<ProjectArtifacts>,
    #[doc="<p>A new or replacement description of the build project.</p>"]
    #[serde(rename="description")]
    pub description: Option<ProjectDescription>,
    #[doc="<p>The replacement AWS Key Management Service (AWS KMS) customer master key (CMK) to be used for encrypting the build output artifacts.</p> <p>You can specify either the CMK's Amazon Resource Name (ARN) or, if available, the CMK's alias (using the format <code>alias/<i>alias-name</i> </code>).</p>"]
    #[serde(rename="encryptionKey")]
    pub encryption_key: Option<NonEmptyString>,
    #[doc="<p>Information to be changed about the build environment for the build project.</p>"]
    #[serde(rename="environment")]
    pub environment: Option<ProjectEnvironment>,
    #[doc="<p>The name of the build project.</p> <note> <p>You cannot change a build project's name.</p> </note>"]
    #[serde(rename="name")]
    pub name: NonEmptyString,
    #[doc="<p>The replacement ARN of the AWS Identity and Access Management (IAM) role that enables AWS CodeBuild to interact with dependent AWS services on behalf of the AWS account.</p>"]
    #[serde(rename="serviceRole")]
    pub service_role: Option<NonEmptyString>,
    #[doc="<p>Information to be changed about the build input source code for the build project.</p>"]
    #[serde(rename="source")]
    pub source: Option<ProjectSource>,
    #[doc="<p>The replacement set of tags for this build project.</p> <p>These tags are available for use by AWS services that support AWS CodeBuild build project tags.</p>"]
    #[serde(rename="tags")]
    pub tags: Option<TagList>,
    #[doc="<p>The replacement value in minutes, from 5 to 480 (8 hours), for AWS CodeBuild to wait before timing out any related build that did not get marked as completed.</p>"]
    #[serde(rename="timeoutInMinutes")]
    pub timeout_in_minutes: Option<TimeOut>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateProjectOutput {
    #[doc="<p>Information about the build project that was changed.</p>"]
    #[serde(rename="project")]
    pub project: Option<Project>,
}

pub type ValueInput = String;
pub type WrapperInt = i64;
pub type WrapperLong = i64;
/// Errors returned by BatchGetBuilds
#[derive(Debug, PartialEq)]
pub enum BatchGetBuildsError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchGetBuildsError {
    pub fn from_body(body: &str) -> BatchGetBuildsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        BatchGetBuildsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetBuildsError::Validation(error_message.to_string())
                    }
                    _ => BatchGetBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetBuildsError {
    fn from(err: serde_json::error::Error) -> BatchGetBuildsError {
        BatchGetBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetBuildsError {
    fn from(err: CredentialsError) -> BatchGetBuildsError {
        BatchGetBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetBuildsError {
    fn from(err: HttpDispatchError) -> BatchGetBuildsError {
        BatchGetBuildsError::HttpDispatch(err)
    }
}
impl fmt::Display for BatchGetBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetBuildsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetBuildsError::InvalidInput(ref cause) => cause,
            BatchGetBuildsError::Validation(ref cause) => cause,
            BatchGetBuildsError::Credentials(ref err) => err.description(),
            BatchGetBuildsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetProjects
#[derive(Debug, PartialEq)]
pub enum BatchGetProjectsError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BatchGetProjectsError {
    pub fn from_body(body: &str) -> BatchGetProjectsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        BatchGetProjectsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        BatchGetProjectsError::Validation(error_message.to_string())
                    }
                    _ => BatchGetProjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => BatchGetProjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BatchGetProjectsError {
    fn from(err: serde_json::error::Error) -> BatchGetProjectsError {
        BatchGetProjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchGetProjectsError {
    fn from(err: CredentialsError) -> BatchGetProjectsError {
        BatchGetProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchGetProjectsError {
    fn from(err: HttpDispatchError) -> BatchGetProjectsError {
        BatchGetProjectsError::HttpDispatch(err)
    }
}
impl fmt::Display for BatchGetProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetProjectsError {
    fn description(&self) -> &str {
        match *self {
            BatchGetProjectsError::InvalidInput(ref cause) => cause,
            BatchGetProjectsError::Validation(ref cause) => cause,
            BatchGetProjectsError::Credentials(ref err) => err.description(),
            BatchGetProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchGetProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateProject
#[derive(Debug, PartialEq)]
pub enum CreateProjectError {
    ///<p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    ///<p>The specified AWS resource cannot be created, because an AWS resource with the same settings already exists.</p>
    ResourceAlreadyExists(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateProjectError {
    pub fn from_body(body: &str) -> CreateProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccountLimitExceededException" => {
                        CreateProjectError::AccountLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        CreateProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateProjectError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateProjectError::Validation(error_message.to_string())
                    }
                    _ => CreateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateProjectError {
    fn from(err: serde_json::error::Error) -> CreateProjectError {
        CreateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateProjectError {
    fn from(err: CredentialsError) -> CreateProjectError {
        CreateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateProjectError {
    fn from(err: HttpDispatchError) -> CreateProjectError {
        CreateProjectError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateProjectError {
    fn description(&self) -> &str {
        match *self {
            CreateProjectError::AccountLimitExceeded(ref cause) => cause,
            CreateProjectError::InvalidInput(ref cause) => cause,
            CreateProjectError::ResourceAlreadyExists(ref cause) => cause,
            CreateProjectError::Validation(ref cause) => cause,
            CreateProjectError::Credentials(ref err) => err.description(),
            CreateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteProject
#[derive(Debug, PartialEq)]
pub enum DeleteProjectError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteProjectError {
    pub fn from_body(body: &str) -> DeleteProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        DeleteProjectError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteProjectError::Validation(error_message.to_string())
                    }
                    _ => DeleteProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteProjectError {
    fn from(err: serde_json::error::Error) -> DeleteProjectError {
        DeleteProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteProjectError {
    fn from(err: CredentialsError) -> DeleteProjectError {
        DeleteProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteProjectError {
    fn from(err: HttpDispatchError) -> DeleteProjectError {
        DeleteProjectError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteProjectError {
    fn description(&self) -> &str {
        match *self {
            DeleteProjectError::InvalidInput(ref cause) => cause,
            DeleteProjectError::Validation(ref cause) => cause,
            DeleteProjectError::Credentials(ref err) => err.description(),
            DeleteProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuilds
#[derive(Debug, PartialEq)]
pub enum ListBuildsError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListBuildsError {
    pub fn from_body(body: &str) -> ListBuildsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        ListBuildsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => ListBuildsError::Validation(error_message.to_string()),
                    _ => ListBuildsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBuildsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBuildsError {
    fn from(err: serde_json::error::Error) -> ListBuildsError {
        ListBuildsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBuildsError {
    fn from(err: CredentialsError) -> ListBuildsError {
        ListBuildsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBuildsError {
    fn from(err: HttpDispatchError) -> ListBuildsError {
        ListBuildsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListBuildsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBuildsError {
    fn description(&self) -> &str {
        match *self {
            ListBuildsError::InvalidInput(ref cause) => cause,
            ListBuildsError::Validation(ref cause) => cause,
            ListBuildsError::Credentials(ref err) => err.description(),
            ListBuildsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListBuildsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListBuildsForProject
#[derive(Debug, PartialEq)]
pub enum ListBuildsForProjectError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    ///<p>The specified AWS resource cannot be found.</p>
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


impl ListBuildsForProjectError {
    pub fn from_body(body: &str) -> ListBuildsForProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        ListBuildsForProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListBuildsForProjectError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListBuildsForProjectError::Validation(error_message.to_string())
                    }
                    _ => ListBuildsForProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListBuildsForProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListBuildsForProjectError {
    fn from(err: serde_json::error::Error) -> ListBuildsForProjectError {
        ListBuildsForProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListBuildsForProjectError {
    fn from(err: CredentialsError) -> ListBuildsForProjectError {
        ListBuildsForProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListBuildsForProjectError {
    fn from(err: HttpDispatchError) -> ListBuildsForProjectError {
        ListBuildsForProjectError::HttpDispatch(err)
    }
}
impl fmt::Display for ListBuildsForProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListBuildsForProjectError {
    fn description(&self) -> &str {
        match *self {
            ListBuildsForProjectError::InvalidInput(ref cause) => cause,
            ListBuildsForProjectError::ResourceNotFound(ref cause) => cause,
            ListBuildsForProjectError::Validation(ref cause) => cause,
            ListBuildsForProjectError::Credentials(ref err) => err.description(),
            ListBuildsForProjectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListBuildsForProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCuratedEnvironmentImages
#[derive(Debug, PartialEq)]
pub enum ListCuratedEnvironmentImagesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListCuratedEnvironmentImagesError {
    pub fn from_body(body: &str) -> ListCuratedEnvironmentImagesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        ListCuratedEnvironmentImagesError::Validation(error_message.to_string())
                    }
                    _ => ListCuratedEnvironmentImagesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCuratedEnvironmentImagesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCuratedEnvironmentImagesError {
    fn from(err: serde_json::error::Error) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCuratedEnvironmentImagesError {
    fn from(err: CredentialsError) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCuratedEnvironmentImagesError {
    fn from(err: HttpDispatchError) -> ListCuratedEnvironmentImagesError {
        ListCuratedEnvironmentImagesError::HttpDispatch(err)
    }
}
impl fmt::Display for ListCuratedEnvironmentImagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCuratedEnvironmentImagesError {
    fn description(&self) -> &str {
        match *self {
            ListCuratedEnvironmentImagesError::Validation(ref cause) => cause,
            ListCuratedEnvironmentImagesError::Credentials(ref err) => err.description(),
            ListCuratedEnvironmentImagesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCuratedEnvironmentImagesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListProjects
#[derive(Debug, PartialEq)]
pub enum ListProjectsError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListProjectsError {
    pub fn from_body(body: &str) -> ListProjectsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        ListProjectsError::InvalidInput(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListProjectsError::Validation(error_message.to_string())
                    }
                    _ => ListProjectsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListProjectsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListProjectsError {
    fn from(err: serde_json::error::Error) -> ListProjectsError {
        ListProjectsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListProjectsError {
    fn from(err: CredentialsError) -> ListProjectsError {
        ListProjectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListProjectsError {
    fn from(err: HttpDispatchError) -> ListProjectsError {
        ListProjectsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListProjectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListProjectsError {
    fn description(&self) -> &str {
        match *self {
            ListProjectsError::InvalidInput(ref cause) => cause,
            ListProjectsError::Validation(ref cause) => cause,
            ListProjectsError::Credentials(ref err) => err.description(),
            ListProjectsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListProjectsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartBuild
#[derive(Debug, PartialEq)]
pub enum StartBuildError {
    ///<p>An AWS service limit was exceeded for the calling AWS account.</p>
    AccountLimitExceeded(String),
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    ///<p>The specified AWS resource cannot be found.</p>
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


impl StartBuildError {
    pub fn from_body(body: &str) -> StartBuildError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccountLimitExceededException" => {
                        StartBuildError::AccountLimitExceeded(String::from(error_message))
                    }
                    "InvalidInputException" => {
                        StartBuildError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartBuildError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StartBuildError::Validation(error_message.to_string()),
                    _ => StartBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartBuildError {
    fn from(err: serde_json::error::Error) -> StartBuildError {
        StartBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartBuildError {
    fn from(err: CredentialsError) -> StartBuildError {
        StartBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartBuildError {
    fn from(err: HttpDispatchError) -> StartBuildError {
        StartBuildError::HttpDispatch(err)
    }
}
impl fmt::Display for StartBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartBuildError {
    fn description(&self) -> &str {
        match *self {
            StartBuildError::AccountLimitExceeded(ref cause) => cause,
            StartBuildError::InvalidInput(ref cause) => cause,
            StartBuildError::ResourceNotFound(ref cause) => cause,
            StartBuildError::Validation(ref cause) => cause,
            StartBuildError::Credentials(ref err) => err.description(),
            StartBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopBuild
#[derive(Debug, PartialEq)]
pub enum StopBuildError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    ///<p>The specified AWS resource cannot be found.</p>
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


impl StopBuildError {
    pub fn from_body(body: &str) -> StopBuildError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        StopBuildError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopBuildError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => StopBuildError::Validation(error_message.to_string()),
                    _ => StopBuildError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopBuildError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopBuildError {
    fn from(err: serde_json::error::Error) -> StopBuildError {
        StopBuildError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopBuildError {
    fn from(err: CredentialsError) -> StopBuildError {
        StopBuildError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopBuildError {
    fn from(err: HttpDispatchError) -> StopBuildError {
        StopBuildError::HttpDispatch(err)
    }
}
impl fmt::Display for StopBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopBuildError {
    fn description(&self) -> &str {
        match *self {
            StopBuildError::InvalidInput(ref cause) => cause,
            StopBuildError::ResourceNotFound(ref cause) => cause,
            StopBuildError::Validation(ref cause) => cause,
            StopBuildError::Credentials(ref err) => err.description(),
            StopBuildError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopBuildError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateProject
#[derive(Debug, PartialEq)]
pub enum UpdateProjectError {
    ///<p>The input value that was provided is not valid.</p>
    InvalidInput(String),
    ///<p>The specified AWS resource cannot be found.</p>
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


impl UpdateProjectError {
    pub fn from_body(body: &str) -> UpdateProjectError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidInputException" => {
                        UpdateProjectError::InvalidInput(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateProjectError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateProjectError::Validation(error_message.to_string())
                    }
                    _ => UpdateProjectError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateProjectError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateProjectError {
    fn from(err: serde_json::error::Error) -> UpdateProjectError {
        UpdateProjectError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateProjectError {
    fn from(err: CredentialsError) -> UpdateProjectError {
        UpdateProjectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateProjectError {
    fn from(err: HttpDispatchError) -> UpdateProjectError {
        UpdateProjectError::HttpDispatch(err)
    }
}
impl fmt::Display for UpdateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateProjectError {
    fn description(&self) -> &str {
        match *self {
            UpdateProjectError::InvalidInput(ref cause) => cause,
            UpdateProjectError::ResourceNotFound(ref cause) => cause,
            UpdateProjectError::Validation(ref cause) => cause,
            UpdateProjectError::Credentials(ref err) => err.description(),
            UpdateProjectError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateProjectError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS CodeBuild API. AWS CodeBuild clients implement this trait.
pub trait CodeBuild {
    #[doc="<p>Gets information about builds.</p>"]
    fn batch_get_builds(&self,
                        input: &BatchGetBuildsInput)
                        -> Result<BatchGetBuildsOutput, BatchGetBuildsError>;


    #[doc="<p>Gets information about build projects.</p>"]
    fn batch_get_projects(&self,
                          input: &BatchGetProjectsInput)
                          -> Result<BatchGetProjectsOutput, BatchGetProjectsError>;


    #[doc="<p>Creates a build project.</p>"]
    fn create_project(&self,
                      input: &CreateProjectInput)
                      -> Result<CreateProjectOutput, CreateProjectError>;


    #[doc="<p>Deletes a build project.</p>"]
    fn delete_project(&self,
                      input: &DeleteProjectInput)
                      -> Result<DeleteProjectOutput, DeleteProjectError>;


    #[doc="<p>Gets a list of build IDs, with each build ID representing a single build.</p>"]
    fn list_builds(&self, input: &ListBuildsInput) -> Result<ListBuildsOutput, ListBuildsError>;


    #[doc="<p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>"]
    fn list_builds_for_project(&self,
                               input: &ListBuildsForProjectInput)
                               -> Result<ListBuildsForProjectOutput, ListBuildsForProjectError>;


    #[doc="<p>Gets information about Docker images that are managed by AWS CodeBuild.</p>"]
    fn list_curated_environment_images
        (&self,
         input: &ListCuratedEnvironmentImagesInput)
         -> Result<ListCuratedEnvironmentImagesOutput, ListCuratedEnvironmentImagesError>;


    #[doc="<p>Gets a list of build project names, with each build project name representing a single build project.</p>"]
    fn list_projects(&self,
                     input: &ListProjectsInput)
                     -> Result<ListProjectsOutput, ListProjectsError>;


    #[doc="<p>Starts running a build.</p>"]
    fn start_build(&self, input: &StartBuildInput) -> Result<StartBuildOutput, StartBuildError>;


    #[doc="<p>Attempts to stop running a build.</p>"]
    fn stop_build(&self, input: &StopBuildInput) -> Result<StopBuildOutput, StopBuildError>;


    #[doc="<p>Changes the settings of a build project.</p>"]
    fn update_project(&self,
                      input: &UpdateProjectInput)
                      -> Result<UpdateProjectOutput, UpdateProjectError>;
}
/// A client for the AWS CodeBuild API.
pub struct CodeBuildClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CodeBuildClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CodeBuildClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CodeBuild for CodeBuildClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Gets information about builds.</p>"]
    fn batch_get_builds(&self,
                        input: &BatchGetBuildsInput)
                        -> Result<BatchGetBuildsOutput, BatchGetBuildsError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetBuilds");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetBuildsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(BatchGetBuildsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about build projects.</p>"]
    fn batch_get_projects(&self,
                          input: &BatchGetProjectsInput)
                          -> Result<BatchGetProjectsOutput, BatchGetProjectsError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.BatchGetProjects");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<BatchGetProjectsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(BatchGetProjectsError::from_body(String::from_utf8_lossy(&response.body)
                                                         .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a build project.</p>"]
    fn create_project(&self,
                      input: &CreateProjectInput)
                      -> Result<CreateProjectOutput, CreateProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.CreateProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateProjectOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateProjectError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a build project.</p>"]
    fn delete_project(&self,
                      input: &DeleteProjectInput)
                      -> Result<DeleteProjectOutput, DeleteProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.DeleteProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteProjectOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteProjectError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets a list of build IDs, with each build ID representing a single build.</p>"]
    fn list_builds(&self, input: &ListBuildsInput) -> Result<ListBuildsOutput, ListBuildsError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuilds");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListBuildsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListBuildsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets a list of build IDs for the specified build project, with each build ID representing a single build.</p>"]
    fn list_builds_for_project(&self,
                               input: &ListBuildsForProjectInput)
                               -> Result<ListBuildsForProjectOutput, ListBuildsForProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListBuildsForProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListBuildsForProjectOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListBuildsForProjectError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Gets information about Docker images that are managed by AWS CodeBuild.</p>"]
    fn list_curated_environment_images
        (&self,
         input: &ListCuratedEnvironmentImagesInput)
         -> Result<ListCuratedEnvironmentImagesOutput, ListCuratedEnvironmentImagesError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "CodeBuild_20161006.ListCuratedEnvironmentImages");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListCuratedEnvironmentImagesOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ListCuratedEnvironmentImagesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Gets a list of build project names, with each build project name representing a single build project.</p>"]
    fn list_projects(&self,
                     input: &ListProjectsInput)
                     -> Result<ListProjectsOutput, ListProjectsError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.ListProjects");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListProjectsOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListProjectsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Starts running a build.</p>"]
    fn start_build(&self, input: &StartBuildInput) -> Result<StartBuildOutput, StartBuildError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StartBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartBuildOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(StartBuildError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Attempts to stop running a build.</p>"]
    fn stop_build(&self, input: &StopBuildInput) -> Result<StopBuildOutput, StopBuildError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.StopBuild");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                Ok(serde_json::from_str::<StopBuildOutput>(String::from_utf8_lossy(&response.body)
                                                               .as_ref())
                           .unwrap())
            }
            _ => Err(StopBuildError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Changes the settings of a build project.</p>"]
    fn update_project(&self,
                      input: &UpdateProjectInput)
                      -> Result<UpdateProjectOutput, UpdateProjectError> {
        let mut request = SignedRequest::new("POST", "codebuild", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "CodeBuild_20161006.UpdateProject");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<UpdateProjectOutput>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(UpdateProjectError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
