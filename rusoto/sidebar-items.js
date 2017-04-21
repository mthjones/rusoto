initSidebarItems({"enum":[["Region","An AWS region. `CnNorth1` is currently untested due to Rusoto maintainers not having access to AWS China."]],"fn":[["default_region","Get the region from `AWS_DEFAULT_REGION` environment variable. Uses us-east-1 if not set or value is malformed."],["default_tls_client","Helper method for creating an http client with tls. Makes a `hyper` client with `NativeTls` for HTTPS support."]],"mod":[["acm","AWS Certificate Manager"],["autoscaling","AWS Auto Scaling"],["claims","Credential Claims module."],["cloudformation","AWS CloudFormation"],["cloudfront","The CloudFront API."],["cloudhsm","AWS CloudHSM"],["cloudsearch","Amazon CloudSearch"],["cloudtrail","AWS CloudTrail"],["cloudwatch","AWS CloudWatch"],["codecommit","AWS CodeCommit"],["codedeploy","AWS CodeDeploy"],["codepipeline","AWS CodePipeline"],["cognitoidentity","AWS Cognito Identity"],["config","AWS Config"],["datapipeline","AWS Data Pipeline"],["devicefarm","AWS Device Farm"],["directconnect","AWS Direct Connect"],["ds","AWS Directory Service"],["dynamodb","Amazon DynamoDB"],["dynamodbstreams","AWS DynamoDB Streams"],["ec2","Amazon EC2 Container Service"],["ecr","Amazon EC2 Container Registry"],["ecs","Amazon EC2 Container Service"],["elasticache","Amazon ElastiCache"],["elasticbeanstalk","AWS Elastic Beanstalk"],["elastictranscoder","Amazon Elastic Transcoder"],["elb","AWS Elastic Load Balancing"],["elbv2","AWS Application Load Balancing"],["emr","Amazon Elastic MapReduce"],["events","Amazon CloudWatch Events"],["firehose","Amazon Kinesis Firehose"],["iam","Amazon Identity and Access Management"],["importexport","AWS Import/Export"],["inspector","Amazon Inspector"],["iot","Amazon IoT"],["kinesis","Amazon Kinesis"],["kms","AWS Key Management Service"],["lambda","AWS Lambda"],["logs","Amazon CloudWatch Logs"],["machinelearning","Amazon Machine Learning"],["marketplacecommerceanalytics","Amazon Marketplace Commerce Analytics"],["opsworks","AWS OpsWorks"],["rds","AWS Relational Database Service"],["redshift","AWS Redshift"],["route53","AWS Route 53"],["route53domains","AWS Route 53 Domains"],["s3","The AWS S3 API."],["sdb","Amazon SimpleDB"],["ses","Amazon Simple Email Service"],["sns","Amazon Simple Notification Service"],["sqs","The AWS SQS API."],["ssm","Amazon EC2 Simple Systems Manager"],["storagegateway","Amazon Storage Gateway"],["sts","AWS Security Token Service"],["swf","Amazon Simple Workflow Service"],["waf","AWS Web Application Firewall"],["workspaces","Amazon Workspaces"]],"struct":[["AwsCredentials","AWS API access credentials, including access key, secret key, token (for IAM profiles), expiration timestamp, and claims from federated login."],["BaseAutoRefreshingProvider","Wrapper for `ProvideAwsCredentials` that caches the credentials returned by the wrapped provider.  Each time the credentials are accessed, they are checked to see if they have expired, in which case they are retrieved from the wrapped provider again."],["ChainProvider","Provides AWS credentials from multiple possible sources using a priority order."],["ContainerProvider","Provides AWS credentials from a task's IAM role."],["CredentialsError",""],["EnvironmentProvider","Provides AWS credentials from environment variables."],["HttpDispatchError",""],["HttpResponse",""],["InstanceMetadataProvider","Provides AWS credentials from a resource's IAM role."],["ParseRegionError","An error produced when attempting to convert a `str` into a `Region` fails."],["ProfileProvider","Provides AWS credentials from a profile in a credentials file."],["SignedRequest","A data structure for all the elements of an HTTP request that are involved in the Amazon Signature Version 4 signing process"],["TlsError",""]],"trait":[["DispatchSignedRequest",""],["ProvideAwsCredentials","A trait for types that produce `AwsCredentials`."]],"type":[["AutoRefreshingProvider","`!Sync` `AutoRefreshingProvider` that caches credentials in a `RefCell`"],["AutoRefreshingProviderSync","Threadsafe `AutoRefreshingProvider` that locks cached credentials with a `Mutex`"],["DefaultCredentialsProvider","The credentials provider you probably want to use if you don't require Sync for your AWS services. Wraps a `ChainProvider` in an `AutoRefreshingProvider` that uses a `RefCell` to cache credentials"],["DefaultCredentialsProviderSync","The credentials provider you probably want to use if you do require Sync for your AWS services. Wraps a `ChainProvider` in an `AutoRefreshingProvider` that uses a `Mutex` to lock credentials in a threadsafe manner."]]});