extern crate hyper;
extern crate rusoto_core;
extern crate xml;
#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;

        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::{CredentialsError, ProvideAwsCredentials};
    

            use std::str::{FromStr};
            use xml::reader::ParserConfig;
            use rusoto_core::param::{Params, ServiceParams};
            use rusoto_core::signature::SignedRequest;
            use xml::EventReader;
            use xml::reader::XmlEvent;
            use rusoto_core::xmlerror::*;
            use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
            use rusoto_core::xmlutil::{peek_at_name, characters, end_element, start_element, skip_tree};
            enum DeserializerNext {
                Close,
                Skip,
                Element(String),
            }
#[doc="<p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct AlarmIdentifier {
                #[doc="<p>The name of the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>"]
pub name: AlarmName,
#[doc="<p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p> <p>For the current list of CloudWatch regions, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html#cw_region\">Amazon CloudWatch</a> in <i>AWS Regions and Endpoints</i> in the <i>Amazon Web Services General Reference</i>.</p>"]
pub region: CloudWatchRegion,
            }
            
struct AlarmIdentifierDeserializer;
            impl AlarmIdentifierDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AlarmIdentifier, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = AlarmIdentifier::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Name" => {
                obj.name = try!(AlarmNameDeserializer::deserialize("Name", stack));
            }
"Region" => {
                obj.region = try!(CloudWatchRegionDeserializer::deserialize("Region", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct AlarmIdentifierSerializer;
                impl AlarmIdentifierSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &AlarmIdentifier) -> String {
                        let mut serialized = format!("<{name}>", name=name);serialized += &format!("<Name>{value}</Name>", value=obj.name);serialized += &format!("<Region>{value}</Region>", value=obj.region);serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type AlarmName = String;
struct AlarmNameDeserializer;
            impl AlarmNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AlarmName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct AlarmNameSerializer;
                impl AlarmNameSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &AlarmName) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type AliasHealthEnabled = bool;
struct AliasHealthEnabledDeserializer;
            impl AliasHealthEnabledDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AliasHealthEnabled, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct AliasHealthEnabledSerializer;
                impl AliasHealthEnabledSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &AliasHealthEnabled) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p> <i>Alias resource record sets only:</i> Information about the CloudFront distribution, Elastic Beanstalk environment, ELB load balancer, Amazon S3 bucket, or Amazon Route 53 resource record set that you're redirecting queries to. The Elastic Beanstalk environment must have a regionalized subdomain.</p> <p>When creating resource record sets for a private hosted zone, note the following:</p> <ul> <li> <p>Resource record sets can't be created for CloudFront distributions in a private hosted zone.</p> </li> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html\">Configuring Failover in a Private Hosted Zone</a>.</p> </li> </ul>"]
#[derive(Default,Clone,Debug)]
            pub struct AliasTarget {
                #[doc="<p> <i>Alias resource record sets only:</i> The value that you specify depends on where you want to route queries:</p> <ul> <li> <p> <b>A CloudFront distribution:</b> Specify the domain name that CloudFront assigned when you created your distribution.</p> <p>Your CloudFront distribution must include an alternate domain name that matches the name of the resource record set. For example, if the name of the resource record set is <i>acme.example.com</i>, your CloudFront distribution must include <i>acme.example.com</i> as one of the alternate domain names. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/CNAMEs.html\">Using Alternate Domain Names (CNAMEs)</a> in the <i>Amazon CloudFront Developer Guide</i>.</p> </li> <li> <p> <b>Elastic Beanstalk environment</b>: Specify the <code>CNAME</code> attribute for the environment. (The environment must have a regionalized domain name.) You can use the following methods to get the value of the CNAME attribute:</p> <ul> <li> <p> <i>AWS Management Console</i>: For information about how to get the value by using the console, see <a href=\"http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/customdomains.html\">Using Custom Domains with AWS Elastic Beanstalk</a> in the <i>AWS Elastic Beanstalk Developer Guide</i>.</p> </li> <li> <p> <i>Elastic Beanstalk API</i>: Use the <code>DescribeEnvironments</code> action to get the value of the <code>CNAME</code> attribute. For more information, see <a href=\"http://docs.aws.amazon.com/elasticbeanstalk/latest/api/API_DescribeEnvironments.html\">DescribeEnvironments</a> in the <i>AWS Elastic Beanstalk API Reference</i>.</p> </li> <li> <p> <i>AWS CLI</i>: Use the <code>describe-environments</code> command to get the value of the <code>CNAME</code> attribute. For more information, see <a href=\"http://docs.aws.amazon.com/cli/latest/reference/elasticbeanstalk/describe-environments.html\">describe-environments</a> in the <i>AWS Command Line Interface Reference</i>.</p> </li> </ul> </li> <li> <p> <b>An ELB load balancer:</b> Specify the DNS name that is associated with the load balancer. Get the DNS name by using the AWS Management Console, the ELB API, or the AWS CLI. Use the same method to get values for <code>HostedZoneId</code> and <code>DNSName</code>. If you get one value from the console and the other value from the API or the CLI, creating the resource record set will fail.</p> <ul> <li> <p> <i>AWS Management Console</i>: Go to the EC2 page, click <b>Load Balancers</b> in the navigation pane, choose the load balancer, choose the <b>Description</b> tab, and get the value of the <b>DNS name</b> field. (If you're routing traffic to a Classic Load Balancer, get the value that begins with <b>dualstack</b>.) Use the same process to get the value of the <b>Hosted zone</b> field. See <a>AliasTarget$HostedZoneId</a>.</p> </li> <li> <p> <i>Elastic Load Balancing API</i>: Use <code>DescribeLoadBalancers</code> to get the value of <code>DNSName</code> and <code>CanonicalHostedZoneNameId</code>. (You specify the value of <code>CanonicalHostedZoneNameId</code> for <a>AliasTarget$HostedZoneId</a>.) For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancer: <a href=\"http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html\">DescribeLoadBalancers</a> </p> </li> <li> <p>Application Load Balancer: <a href=\"http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html\">DescribeLoadBalancers</a> </p> </li> </ul> </li> <li> <p> <i>AWS CLI</i>: Use <code> <a href=\"http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html\">describe-load-balancers</a> </code> to get the value of <code>DNSName</code> and <code>CanonicalHostedZoneNameId</code>. (You specify the value of <code>CanonicalHostedZoneNameId</code> for <a>AliasTarget$HostedZoneId</a>.)</p> </li> </ul> </li> <li> <p> <b>An Amazon S3 bucket that is configured as a static website:</b> Specify the domain name of the Amazon S3 website endpoint in which you created the bucket, for example, <code>s3-website-us-east-1.amazonaws.com</code>. For more information about valid values, see the table <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region\">Amazon Simple Storage Service (S3) Website Endpoints</a> in the <i>Amazon Web Services General Reference</i>. For more information about using S3 buckets for websites, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/getting-started.html\">Getting Started with Amazon Route 53</a> in the <i>Amazon Route 53 Developer Guide.</i> </p> </li> <li> <p> <b>Another Amazon Route 53 resource record set</b>: Specify the value of the <code>Name</code> element for a resource record set in the current hosted zone.</p> </li> </ul>"]
pub dns_name: DNSName,
#[doc="<p> <i>Applies only to alias, weighted alias, latency alias, and failover alias record sets:</i> If you set the value of <code>EvaluateTargetHealth</code> to <code>true</code> for the resource record set or sets in an alias, weighted alias, latency alias, or failover alias resource record set, and if you specify a value for <code> <a>HealthCheck$Id</a> </code> for every resource record set that is referenced by these alias resource record sets, the alias resource record sets inherit the health of the referenced resource record sets.</p> <p>In this configuration, when Amazon Route 53 receives a DNS query for an alias resource record set:</p> <ul> <li> <p>Amazon Route 53 looks at the resource record sets that are referenced by the alias resource record sets to determine which health checks they're using.</p> </li> <li> <p>Amazon Route 53 checks the current status of each health check. (Amazon Route 53 periodically checks the health of the endpoint that is specified in a health check; it doesn't perform the health check when the DNS query arrives.)</p> </li> <li> <p>Based on the status of the health checks, Amazon Route 53 determines which resource record sets are healthy. Unhealthy resource record sets are immediately removed from consideration. In addition, if all of the resource record sets that are referenced by an alias resource record set are unhealthy, that alias resource record set also is immediately removed from consideration.</p> </li> <li> <p>Based on the configuration of the alias resource record sets (weighted alias or latency alias, for example) and the configuration of the resource record sets that they reference, Amazon Route 53 chooses a resource record set from the healthy resource record sets, and responds to the query.</p> </li> </ul> <p>Note the following:</p> <ul> <li> <p>You can't set <code>EvaluateTargetHealth</code> to <code>true</code> when the alias target is a CloudFront distribution.</p> </li> <li> <p>If the AWS resource that you specify in <code>AliasTarget</code> is a resource record set or a group of resource record sets (for example, a group of weighted resource record sets), but it is not another alias resource record set, we recommend that you associate a health check with all of the resource record sets in the alias target.For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-complex-configs.html#dns-failover-complex-configs-hc-omitting\">What Happens When You Omit Health Checks?</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> <li> <p>If you specify an Elastic Beanstalk environment in <code>HostedZoneId</code> and <code>DNSName</code>, and if the environment contains an ELB load balancer, Elastic Load Balancing routes queries only to the healthy Amazon EC2 instances that are registered with the load balancer. (An environment automatically contains an ELB load balancer if it includes more than one EC2 instance.) If you set <code>EvaluateTargetHealth</code> to <code>true</code> and either no EC2 instances are healthy or the load balancer itself is unhealthy, Amazon Route 53 routes queries to other available resources that are healthy, if any.</p> <p>If the environment contains a single EC2 instance, there are no special requirements.</p> </li> <li> <p>If you specify an ELB load balancer in <code> <a>AliasTarget</a> </code>, Elastic Load Balancing routes queries only to the healthy EC2 instances that are registered with the load balancer. If no EC2 instances are healthy or if the load balancer itself is unhealthy, and if <code>EvaluateTargetHealth</code> is true for the corresponding alias resource record set, Amazon Route 53 routes queries to other resources. When you create a load balancer, you configure settings for Elastic Load Balancing health checks; they're not Amazon Route 53 health checks, but they perform a similar function. Do not create Amazon Route 53 health checks for the EC2 instances that you register with an ELB load balancer.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-complex-configs.html\">How Health Checks Work in More Complex Amazon Route 53 Configurations</a> in the <i>Amazon Route 53 Developers Guide</i>.</p> </li> <li> <p>We recommend that you set <code>EvaluateTargetHealth</code> to true only when you have enough idle capacity to handle the failure of one or more endpoints.</p> </li> </ul> <p>For more information and examples, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
pub evaluate_target_health: AliasHealthEnabled,
#[doc="<p> <i>Alias resource records sets only</i>: The value used depends on where the queries are routed:</p> <dl> <dt>A CloudFront distribution</dt> <dd> <p>Specify <code>Z2FDTNDATAQYW2</code>.</p> <note> <p>Alias resource record sets for CloudFront can't be created in a private zone.</p> </note> </dd> <dt>Elastic Beanstalk environment</dt> <dd> <p>Specify the hosted zone ID for the region in which you created the environment. The environment must have a regionalized subdomain. For a list of regions and the corresponding hosted zone IDs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html#elasticbeanstalk_region\">AWS Elastic Beanstalk</a> in the Regions and Endpoints chapter of the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>ELB load balancer</dt> <dd> <p>Specify the value of the hosted zone ID for the load balancer. Use the following methods to get the hosted zone ID:</p> <ul> <li> <p>AWS Management Console: Go to the Amazon EC2 page, click <b>Load Balancers</b> in the navigation pane, select the load balancer, and get the value of the <b>Hosted zone</b> field on the <b>Description</b> tab. Use the same process to get the value of <b>DNS name</b>. (You specify the value of <b>DNS name</b> for <a>AliasTarget$DNSName</a>.)</p> </li> <li> <p> <i>Elastic Load Balancing API</i>: Use <code>DescribeLoadBalancers</code> to get the value of <code>CanonicalHostedZoneNameId</code> and <code>DNSName</code>. (You specify the value of <code>DNSName</code> for <a>AliasTarget$DNSName</a>.) For more information, see the applicable guide:</p> <ul> <li> <p>Classic Load Balancer: <a href=\"http://docs.aws.amazon.com/elasticloadbalancing/2012-06-01/APIReference/API_DescribeLoadBalancers.html\">DescribeLoadBalancers</a> </p> </li> <li> <p>Application Load Balancer: <a href=\"http://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html\">DescribeLoadBalancers</a> </p> </li> </ul> </li> <li> <p>AWS CLI: Use <code> <a href=\"http://docs.aws.amazon.com/cli/latest/reference/elb/describe-load-balancers.html\">describe-load-balancers</a> </code> to get the value of <code>CanonicalHostedZoneNameID</code> and <code>DNSName</code>. (You specify the value of <code>DNSName</code> for <a>AliasTarget$DNSName</a>.)</p> </li> </ul> </dd> <dt>An Amazon S3 bucket configured as a static website</dt> <dd> <p>Specify the hosted zone ID for the region that you created the bucket in. For more information about valid values, see the table <a href=\"http://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region\">Amazon Simple Storage Service Website Endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> </dd> <dt>Another Amazon Route 53 resource record set in your hosted zone</dt> <dd> <p>Specify the hosted zone ID of your hosted zone. (An alias resource record set can't reference a resource record set in a different hosted zone.)</p> </dd> </dl>"]
pub hosted_zone_id: ResourceId,
            }
            
struct AliasTargetDeserializer;
            impl AliasTargetDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AliasTarget, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = AliasTarget::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DNSName" => {
                obj.dns_name = try!(DNSNameDeserializer::deserialize("DNSName", stack));
            }
"EvaluateTargetHealth" => {
                obj.evaluate_target_health = try!(AliasHealthEnabledDeserializer::deserialize("EvaluateTargetHealth", stack));
            }
"HostedZoneId" => {
                obj.hosted_zone_id = try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct AliasTargetSerializer;
                impl AliasTargetSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &AliasTarget) -> String {
                        let mut serialized = format!("<{name}>", name=name);serialized += &format!("<DNSName>{value}</DNSName>", value=obj.dns_name);serialized += &format!("<EvaluateTargetHealth>{value}</EvaluateTargetHealth>", value=obj.evaluate_target_health);serialized += &format!("<HostedZoneId>{value}</HostedZoneId>", value=obj.hosted_zone_id);serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type AssociateVPCComment = String;

                pub struct AssociateVPCCommentSerializer;
                impl AssociateVPCCommentSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &AssociateVPCComment) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains information about the request to associate a VPC with a private hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct AssociateVPCWithHostedZoneRequest {
                #[doc="<p> <i>Optional:</i> A comment about the association request.</p>"]
pub comment: Option<AssociateVPCComment>,
#[doc="<p>The ID of the private hosted zone that you want to associate an Amazon VPC with.</p> <p>Note that you can't associate a VPC with a hosted zone that doesn't have an existing VPC association.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>A complex type that contains information about the VPC that you want to associate with a private hosted zone.</p>"]
pub vpc: VPC,
            }
            
#[doc="<p>A complex type that contains the response information for the <code>AssociateVPCWithHostedZone</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct AssociateVPCWithHostedZoneResponse {
                #[doc="<p>A complex type that describes the changes made to your hosted zone.</p>"]
pub change_info: ChangeInfo,
            }
            
struct AssociateVPCWithHostedZoneResponseDeserializer;
            impl AssociateVPCWithHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<AssociateVPCWithHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = AssociateVPCWithHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>The information for each resource record set that you want to change.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct Change {
                #[doc="<p>The action to perform:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes a existing resource record set.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <code> <a>DeleteTrafficPolicyInstance</a> </code>. Amazon Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Amazon Route 53 doesn't automatically delete the traffic policy instance, and you'll continue to be charged for it even though it's no longer in use. </p> </important> </li> <li> <p> <code>UPSERT</code>: If a resource record set doesn't already exist, Amazon Route 53 creates it. If a resource record set does exist, Amazon Route 53 updates it with the values in the request.</p> </li> </ul> <p>The values that you need to include in the request depend on the type of resource record set that you're creating, deleting, or updating:</p> <p> <b>Basic resource record sets (excluding alias, failover, geolocation, latency, and weighted resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>TTL</code> </p> </li> </ul> <p> <b>Failover, geolocation, latency, or weighted resource record sets (excluding alias resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>TTL</code> </p> </li> <li> <p> <code>SetIdentifier</code> </p> </li> </ul> <p> <b>Alias resource record sets (including failover alias, geolocation alias, latency alias, and weighted alias resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>AliasTarget</code> (includes <code>DNSName</code>, <code>EvaluateTargetHealth</code>, and <code>HostedZoneId</code>)</p> </li> <li> <p> <code>SetIdentifier</code> (for failover, geolocation, latency, and weighted resource record sets)</p> </li> </ul>"]
pub action: ChangeAction,
#[doc="<p>Information about the resource record set to create, delete, or update.</p>"]
pub resource_record_set: ResourceRecordSet,
            }
            

                pub struct ChangeSerializer;
                impl ChangeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Change) -> String {
                        let mut serialized = format!("<{name}>", name=name);serialized += &format!("<Action>{value}</Action>", value=obj.action);serialized += &ResourceRecordSetSerializer::serialize("ResourceRecordSet", &obj.resource_record_set);serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type ChangeAction = String;

                pub struct ChangeActionSerializer;
                impl ChangeActionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ChangeAction) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>The information for a change request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeBatch {
                #[doc="<p>Information about the changes to make to the record sets.</p>"]
pub changes: Changes,
#[doc="<p> <i>Optional:</i> Any comments you want to include about a change batch request.</p>"]
pub comment: Option<ResourceDescription>,
            }
            

                pub struct ChangeBatchSerializer;
                impl ChangeBatchSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ChangeBatch) -> String {
                        let mut serialized = format!("<{name}>", name=name);serialized += &ChangesSerializer::serialize("Changes", &obj.changes);if let Some(ref value) = obj.comment {
                serialized += &format!("<Comment>{value}</Comment>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
#[doc="<p>A complex type that describes change information about changes made to your hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeInfo {
                #[doc="<p>A complex type that describes change information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a>GetChange</a> action to get detailed information about the change.</p>"]
pub comment: Option<ResourceDescription>,
#[doc="<p>The ID of the request.</p>"]
pub id: ResourceId,
#[doc="<p>The current state of the request. <code>PENDING</code> indicates that this request has not yet been applied to all Amazon Route 53 DNS servers.</p>"]
pub status: ChangeStatus,
#[doc="<p>The date and time the change request was submitted, in Coordinated Universal Time (UTC) format: <code>YYYY-MM-DDThh:mm:ssZ</code>. For more information, see the Wikipedia entry <a href=\"https://en.wikipedia.org/wiki/ISO_8601\">ISO 8601</a>.</p>"]
pub submitted_at: TimeStamp,
            }
            
struct ChangeInfoDeserializer;
            impl ChangeInfoDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ChangeInfo, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ChangeInfo::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Comment" => {
                obj.comment = Some(try!(ResourceDescriptionDeserializer::deserialize("Comment", stack)));
            }
"Id" => {
                obj.id = try!(ResourceIdDeserializer::deserialize("Id", stack));
            }
"Status" => {
                obj.status = try!(ChangeStatusDeserializer::deserialize("Status", stack));
            }
"SubmittedAt" => {
                obj.submitted_at = try!(TimeStampDeserializer::deserialize("SubmittedAt", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains change information for the resource record set.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeResourceRecordSetsRequest {
                #[doc="<p>A complex type that contains an optional comment and the <code>Changes</code> element.</p>"]
pub change_batch: ChangeBatch,
#[doc="<p>The ID of the hosted zone that contains the resource record sets that you want to change.</p>"]
pub hosted_zone_id: ResourceId,
            }
            
#[doc="<p>A complex type containing the response for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeResourceRecordSetsResponse {
                #[doc="<p>A complex type that contains information about changes made to your hosted zone.</p> <p>This element contains an ID that you use when performing a <a>GetChange</a> action to get detailed information about the change.</p>"]
pub change_info: ChangeInfo,
            }
            
struct ChangeResourceRecordSetsResponseDeserializer;
            impl ChangeResourceRecordSetsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ChangeResourceRecordSetsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ChangeResourceRecordSetsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type ChangeStatus = String;
struct ChangeStatusDeserializer;
            impl ChangeStatusDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ChangeStatus, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the tags that you want to add, edit, or delete.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeTagsForResourceRequest {
                #[doc="<p>A complex type that contains a list of the tags that you want to add to the specified health check or hosted zone and/or the tags for which you want to edit the <code>Value</code> element.</p> <p>You can add a maximum of 10 tags to a health check or a hosted zone.</p>"]
pub add_tags: Option<TagList>,
#[doc="<p>A complex type that contains a list of the tags that you want to delete from the specified health check or hosted zone. You can specify up to 10 keys.</p>"]
pub remove_tag_keys: Option<TagKeyList>,
#[doc="<p>The ID of the resource for which you want to add, change, or delete tags.</p>"]
pub resource_id: TagResourceId,
#[doc="<p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul>"]
pub resource_type: TagResourceType,
            }
            
#[doc="<p>Empty response for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ChangeTagsForResourceResponse;
            
struct ChangeTagsForResourceResponseDeserializer;
            impl ChangeTagsForResourceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ChangeTagsForResourceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = ChangeTagsForResourceResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
pub type Changes = Vec<Change>;

                pub struct ChangesSerializer;
                impl ChangesSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Changes) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(ChangeSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
pub type CheckerIpRanges = Vec<IPAddressCidr>;
struct CheckerIpRangesDeserializer;
            impl CheckerIpRangesDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CheckerIpRanges, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "member" {
                        obj.push(try!(IPAddressCidrDeserializer::deserialize("member", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type ChildHealthCheckList = Vec<HealthCheckId>;
struct ChildHealthCheckListDeserializer;
            impl ChildHealthCheckListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ChildHealthCheckList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ChildHealthCheck" {
                        obj.push(try!(HealthCheckIdDeserializer::deserialize("ChildHealthCheck", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }

                pub struct ChildHealthCheckListSerializer;
                impl ChildHealthCheckListSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ChildHealthCheckList) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(HealthCheckIdSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
#[doc="<p>A complex type that contains information about the CloudWatch alarm that Amazon Route 53 is monitoring for this health check.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CloudWatchAlarmConfiguration {
                #[doc="<p>For the metric that the CloudWatch alarm is associated with, the arithmetic operation that is used for the comparison.</p>"]
pub comparison_operator: ComparisonOperator,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, a complex type that contains information about the dimensions for the metric.For information, see <a href=\" http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html\">Amazon CloudWatch Namespaces, Dimensions, and Metrics Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>"]
pub dimensions: Option<DimensionList>,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, the number of periods that the metric is compared to the threshold.</p>"]
pub evaluation_periods: EvaluationPeriods,
#[doc="<p>The name of the CloudWatch metric that the alarm is associated with.</p>"]
pub metric_name: MetricName,
#[doc="<p>The namespace of the metric that the alarm is associated with. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html\">Amazon CloudWatch Namespaces, Dimensions, and Metrics Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>"]
pub namespace: Namespace,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, the duration of one evaluation period in seconds.</p>"]
pub period: Period,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, the statistic that is applied to the metric.</p>"]
pub statistic: Statistic,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, the value the metric is compared with.</p>"]
pub threshold: Threshold,
            }
            
struct CloudWatchAlarmConfigurationDeserializer;
            impl CloudWatchAlarmConfigurationDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CloudWatchAlarmConfiguration, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CloudWatchAlarmConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ComparisonOperator" => {
                obj.comparison_operator = try!(ComparisonOperatorDeserializer::deserialize("ComparisonOperator", stack));
            }
"Dimensions" => {
                obj.dimensions = Some(try!(DimensionListDeserializer::deserialize("Dimensions", stack)));
            }
"EvaluationPeriods" => {
                obj.evaluation_periods = try!(EvaluationPeriodsDeserializer::deserialize("EvaluationPeriods", stack));
            }
"MetricName" => {
                obj.metric_name = try!(MetricNameDeserializer::deserialize("MetricName", stack));
            }
"Namespace" => {
                obj.namespace = try!(NamespaceDeserializer::deserialize("Namespace", stack));
            }
"Period" => {
                obj.period = try!(PeriodDeserializer::deserialize("Period", stack));
            }
"Statistic" => {
                obj.statistic = try!(StatisticDeserializer::deserialize("Statistic", stack));
            }
"Threshold" => {
                obj.threshold = try!(ThresholdDeserializer::deserialize("Threshold", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type CloudWatchRegion = String;
struct CloudWatchRegionDeserializer;
            impl CloudWatchRegionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CloudWatchRegion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct CloudWatchRegionSerializer;
                impl CloudWatchRegionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &CloudWatchRegion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ComparisonOperator = String;
struct ComparisonOperatorDeserializer;
            impl ComparisonOperatorDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ComparisonOperator, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the health check request information.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateHealthCheckRequest {
                #[doc="<p>A unique string that identifies the request and that allows failed <code>CreateHealthCheck</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you create a health check.</p>"]
pub caller_reference: HealthCheckNonce,
#[doc="<p>A complex type that contains the response to a <code>CreateHealthCheck</code> request. </p>"]
pub health_check_config: HealthCheckConfig,
            }
            
#[doc="<p>A complex type containing the response information for the new health check.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateHealthCheckResponse {
                #[doc="<p>A complex type that contains identifying information about the health check.</p>"]
pub health_check: HealthCheck,
#[doc="<p>The unique URL representing the new health check.</p>"]
pub location: ResourceURI,
            }
            
struct CreateHealthCheckResponseDeserializer;
            impl CreateHealthCheckResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateHealthCheckResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateHealthCheckResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheck" => {
                obj.health_check = try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type containing the hosted zone request information.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateHostedZoneRequest {
                #[doc="<p>A unique string that identifies the request and that allows failed <code>CreateHostedZone</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you create a hosted zone. <code>CallerReference</code> can be any unique string, for example, a date/time stamp.</p>"]
pub caller_reference: Nonce,
#[doc="<p>If you want to associate a reusable delegation set with this hosted zone, the ID that Amazon Route 53 assigned to the reusable delegation set when you created it. For more information about reusable delegation sets, see <a>CreateReusableDelegationSet</a>.</p> <dl> <dt>Type</dt> <dd> <p>String</p> </dd> <dt>Default</dt> <dd> <p>None</p> </dd> <dt>Parent</dt> <dd> <p> <code>CreatedHostedZoneRequest</code> </p> </dd> </dl>"]
pub delegation_set_id: Option<ResourceId>,
#[doc="<p> (Optional) A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the <code>HostedZoneConfig</code> and <code>Comment</code> elements.</p>"]
pub hosted_zone_config: Option<HostedZoneConfig>,
#[doc="<p>The name of the domain. For resource record types that include a domain name, specify a fully qualified domain name, for example, <i>www.example.com</i>. The trailing dot is optional; Amazon Route 53 assumes that the domain name is fully qualified. This means that Amazon Route 53 treats <i>www.example.com</i> (without a trailing dot) and <i>www.example.com.</i> (with a trailing dot) as identical.</p> <p>If you're creating a public hosted zone, this is the name you have registered with your DNS registrar. If your domain name is registered with a registrar other than Amazon Route 53, change the name servers for your domain to the set of <code>NameServers</code> that <code>CreateHostedZone</code> returns in the DelegationSet element.</p>"]
pub name: DNSName,
#[doc="<p>The VPC that you want your hosted zone to be associated with. By providing this parameter, your newly created hosted can't be resolved anywhere other than the given VPC.</p>"]
pub vpc: Option<VPC>,
            }
            
#[doc="<p>A complex type containing the response information for the hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateHostedZoneResponse {
                #[doc="<p>A complex type that describes the changes made to your hosted zone.</p>"]
pub change_info: ChangeInfo,
#[doc="<p>A complex type that describes the name servers for this hosted zone.</p>"]
pub delegation_set: DelegationSet,
#[doc="<p>A complex type that contains general information about the hosted zone.</p>"]
pub hosted_zone: HostedZone,
#[doc="<p>The unique URL representing the new hosted zone.</p>"]
pub location: ResourceURI,
#[doc="<p>A complex type that contains information about an Amazon VPC that you associated with this hosted zone.</p>"]
pub vpc: Option<VPC>,
            }
            
struct CreateHostedZoneResponseDeserializer;
            impl CreateHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
"DelegationSet" => {
                obj.delegation_set = try!(DelegationSetDeserializer::deserialize("DelegationSet", stack));
            }
"HostedZone" => {
                obj.hosted_zone = try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
            }
"VPC" => {
                obj.vpc = Some(try!(VPCDeserializer::deserialize("VPC", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[derive(Default,Clone,Debug)]
            pub struct CreateReusableDelegationSetRequest {
                #[doc="<p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example a date/time stamp.</p>"]
pub caller_reference: Nonce,
#[doc="<p>If you want to mark the delegation set for an existing hosted zone as reusable, the ID for that hosted zone.</p>"]
pub hosted_zone_id: Option<ResourceId>,
            }
            
#[derive(Default,Clone,Debug)]
            pub struct CreateReusableDelegationSetResponse {
                #[doc="<p>A complex type that contains name server information.</p>"]
pub delegation_set: DelegationSet,
#[doc="<p>The unique URL representing the new reusable delegation set.</p>"]
pub location: ResourceURI,
            }
            
struct CreateReusableDelegationSetResponseDeserializer;
            impl CreateReusableDelegationSetResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateReusableDelegationSetResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateReusableDelegationSetResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DelegationSet" => {
                obj.delegation_set = try!(DelegationSetDeserializer::deserialize("DelegationSet", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the resource record sets that you want to create based on a specified traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyInstanceRequest {
                #[doc="<p>The ID of the hosted zone in which you want Amazon Route 53 to create resource record sets by using the configuration in a traffic policy.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>The domain name (such as example.com) or subdomain name (such as www.example.com) for which Amazon Route 53 responds to DNS queries by using the resource record sets that Amazon Route 53 creates for this traffic policy instance.</p>"]
pub name: DNSName,
#[doc="<p>(Optional) The TTL that you want Amazon Route 53 to assign to all of the resource record sets that it creates in the specified hosted zone.</p>"]
pub ttl: TTL,
#[doc="<p>The ID of the traffic policy that you want to use to create resource record sets in the specified hosted zone.</p>"]
pub traffic_policy_id: TrafficPolicyId,
#[doc="<p>The version of the traffic policy that you want to use to create resource record sets in the specified hosted zone.</p>"]
pub traffic_policy_version: TrafficPolicyVersion,
            }
            
#[doc="<p>A complex type that contains the response information for the <code>CreateTrafficPolicyInstance</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyInstanceResponse {
                #[doc="<p>A unique URL that represents a new traffic policy instance.</p>"]
pub location: ResourceURI,
#[doc="<p>A complex type that contains settings for the new traffic policy instance.</p>"]
pub traffic_policy_instance: TrafficPolicyInstance,
            }
            
struct CreateTrafficPolicyInstanceResponseDeserializer;
            impl CreateTrafficPolicyInstanceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateTrafficPolicyInstanceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyInstanceResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicyInstance" => {
                obj.traffic_policy_instance = try!(TrafficPolicyInstanceDeserializer::deserialize("TrafficPolicyInstance", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the traffic policy that you want to create.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyRequest {
                #[doc="<p>(Optional) Any comments that you want to include about the traffic policy.</p>"]
pub comment: Option<TrafficPolicyComment>,
#[doc="<p>The definition of this traffic policy in JSON format. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html\">Traffic Policy Document Format</a>.</p>"]
pub document: TrafficPolicyDocument,
#[doc="<p>The name of the traffic policy.</p>"]
pub name: TrafficPolicyName,
            }
            
#[doc="<p>A complex type that contains the response information for the <code>CreateTrafficPolicy</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyResponse {
                #[doc="<p>A unique URL that represents a new traffic policy.</p>"]
pub location: ResourceURI,
#[doc="<p>A complex type that contains settings for the new traffic policy.</p>"]
pub traffic_policy: TrafficPolicy,
            }
            
struct CreateTrafficPolicyResponseDeserializer;
            impl CreateTrafficPolicyResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateTrafficPolicyResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicy" => {
                obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the traffic policy for which you want to create a new version.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyVersionRequest {
                #[doc="<p>The comment that you specified in the <code>CreateTrafficPolicyVersion</code> request, if any.</p>"]
pub comment: Option<TrafficPolicyComment>,
#[doc="<p>The definition of this version of the traffic policy, in JSON format. You specified the JSON in the <code>CreateTrafficPolicyVersion</code> request. For more information about the JSON format, see <a>CreateTrafficPolicy</a>.</p>"]
pub document: TrafficPolicyDocument,
#[doc="<p>The ID of the traffic policy for which you want to create a new version.</p>"]
pub id: TrafficPolicyId,
            }
            
#[doc="<p>A complex type that contains the response information for the <code>CreateTrafficPolicyVersion</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateTrafficPolicyVersionResponse {
                #[doc="<p>A unique URL that represents a new traffic policy version.</p>"]
pub location: ResourceURI,
#[doc="<p>A complex type that contains settings for the new version of the traffic policy.</p>"]
pub traffic_policy: TrafficPolicy,
            }
            
struct CreateTrafficPolicyVersionResponseDeserializer;
            impl CreateTrafficPolicyVersionResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateTrafficPolicyVersionResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateTrafficPolicyVersionResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicy" => {
                obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the request to authorize associating a VPC with your private hosted zone. Authorization is only required when a private hosted zone and a VPC were created by using different accounts.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateVPCAssociationAuthorizationRequest {
                #[doc="<p>The ID of the private hosted zone that you want to authorize associating a VPC with.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>A complex type that contains the VPC ID and region for the VPC that you want to authorize associating with your hosted zone.</p>"]
pub vpc: VPC,
            }
            
#[doc="<p>A complex type that contains the response information from a CreateVPCAssociationAuthorization request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct CreateVPCAssociationAuthorizationResponse {
                #[doc="<p>The ID of the hosted zone that you authorized associating a VPC with.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>The VPC that you authorized associating with a hosted zone.</p>"]
pub vpc: VPC,
            }
            
struct CreateVPCAssociationAuthorizationResponseDeserializer;
            impl CreateVPCAssociationAuthorizationResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<CreateVPCAssociationAuthorizationResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = CreateVPCAssociationAuthorizationResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneId" => {
                obj.hosted_zone_id = try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
            }
"VPC" => {
                obj.vpc = try!(VPCDeserializer::deserialize("VPC", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type DNSName = String;
struct DNSNameDeserializer;
            impl DNSNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DNSName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct DNSNameSerializer;
                impl DNSNameSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &DNSName) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type DNSRCode = String;
struct DNSRCodeDeserializer;
            impl DNSRCodeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DNSRCode, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that describes the name servers for this hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DelegationSet {
                #[doc="<p>A unique string that identifies the request, and that allows you to retry failed <code>CreateReusableDelegationSet</code> requests without the risk of executing the operation twice. You must use a unique <code>CallerReference</code> string every time you submit a <code>CreateReusableDelegationSet</code> request. <code>CallerReference</code> can be any unique string, for example, a date/time stamp.</p>"]
pub caller_reference: Option<Nonce>,
#[doc="<p>The ID that Amazon Route 53 assigns to a reusable delegation set.</p>"]
pub id: Option<ResourceId>,
#[doc="<p>A complex type that contains a list of the authoritative name servers for the hosted zone.</p>"]
pub name_servers: DelegationSetNameServers,
            }
            
struct DelegationSetDeserializer;
            impl DelegationSetDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DelegationSet, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = DelegationSet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CallerReference" => {
                obj.caller_reference = Some(try!(NonceDeserializer::deserialize("CallerReference", stack)));
            }
"Id" => {
                obj.id = Some(try!(ResourceIdDeserializer::deserialize("Id", stack)));
            }
"NameServers" => {
                obj.name_servers = try!(DelegationSetNameServersDeserializer::deserialize("NameServers", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type DelegationSetNameServers = Vec<DNSName>;
struct DelegationSetNameServersDeserializer;
            impl DelegationSetNameServersDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DelegationSetNameServers, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "NameServer" {
                        obj.push(try!(DNSNameDeserializer::deserialize("NameServer", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type DelegationSets = Vec<DelegationSet>;
struct DelegationSetsDeserializer;
            impl DelegationSetsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DelegationSets, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "DelegationSet" {
                        obj.push(try!(DelegationSetDeserializer::deserialize("DelegationSet", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
#[doc="<p>This action deletes a health check. Send a <code>DELETE</code> request to the <code>/2013-04-01/DeleteHealthCheckRequest</code> resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteHealthCheckRequest {
                #[doc="<p>The ID of the health check that you want to delete.</p>"]
pub health_check_id: HealthCheckId,
            }
            
#[doc="<p>An empty element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteHealthCheckResponse;
            
struct DeleteHealthCheckResponseDeserializer;
            impl DeleteHealthCheckResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteHealthCheckResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = DeleteHealthCheckResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
#[doc="<p>A complex type that contains information about the hosted zone that you want to delete.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteHostedZoneRequest {
                #[doc="<p>The ID of the hosted zone you want to delete.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>A complex type containing the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteHostedZoneResponse {
                #[doc="<p>A complex type that contains the ID, the status, and the date and time of your delete request.</p>"]
pub change_info: ChangeInfo,
            }
            
struct DeleteHostedZoneResponseDeserializer;
            impl DeleteHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = DeleteHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type containing the information for the delete request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteReusableDelegationSetRequest {
                #[doc="<p>The ID of the reusable delegation set you want to delete.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>An empty element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteReusableDelegationSetResponse;
            
struct DeleteReusableDelegationSetResponseDeserializer;
            impl DeleteReusableDelegationSetResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteReusableDelegationSetResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = DeleteReusableDelegationSetResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
#[doc="<p>A complex type that contains information about the traffic policy instance that you want to delete.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteTrafficPolicyInstanceRequest {
                #[doc="<p>The ID of the traffic policy instance that you want to delete. </p> <important> <p>When you delete a traffic policy instance, Amazon Route 53 also deletes all of the resource record sets that were created when you created the traffic policy instance.</p> </important>"]
pub id: TrafficPolicyInstanceId,
            }
            
#[doc="<p>An empty element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteTrafficPolicyInstanceResponse;
            
struct DeleteTrafficPolicyInstanceResponseDeserializer;
            impl DeleteTrafficPolicyInstanceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteTrafficPolicyInstanceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = DeleteTrafficPolicyInstanceResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
#[doc="<p>A request to delete a specified traffic policy version.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteTrafficPolicyRequest {
                #[doc="<p>The ID of the traffic policy that you want to delete.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The version number of the traffic policy that you want to delete.</p>"]
pub version: TrafficPolicyVersion,
            }
            
#[doc="<p>An empty element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteTrafficPolicyResponse;
            
struct DeleteTrafficPolicyResponseDeserializer;
            impl DeleteTrafficPolicyResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteTrafficPolicyResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = DeleteTrafficPolicyResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
#[doc="<p>A complex type that contains information about the request to remove authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account. </p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteVPCAssociationAuthorizationRequest {
                #[doc="<p>When removing authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account, the ID of the hosted zone.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>When removing authorization to associate a VPC that was created by one AWS account with a hosted zone that was created with a different AWS account, a complex type that includes the ID and region of the VPC.</p>"]
pub vpc: VPC,
            }
            
#[doc="<p>Empty response for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DeleteVPCAssociationAuthorizationResponse;
            
struct DeleteVPCAssociationAuthorizationResponseDeserializer;
            impl DeleteVPCAssociationAuthorizationResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DeleteVPCAssociationAuthorizationResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

            let obj = DeleteVPCAssociationAuthorizationResponse::default();

            try!(end_element(tag_name, stack));

            Ok(obj)
            
                }
            }
#[doc="<p>For the metric that the CloudWatch alarm is associated with, a complex type that contains information about one dimension.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct Dimension {
                #[doc="<p>For the metric that the CloudWatch alarm is associated with, the name of one dimension.</p>"]
pub name: DimensionField,
#[doc="<p>For the metric that the CloudWatch alarm is associated with, the value of one dimension.</p>"]
pub value: DimensionField,
            }
            
struct DimensionDeserializer;
            impl DimensionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Dimension, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = Dimension::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Name" => {
                obj.name = try!(DimensionFieldDeserializer::deserialize("Name", stack));
            }
"Value" => {
                obj.value = try!(DimensionFieldDeserializer::deserialize("Value", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type DimensionField = String;
struct DimensionFieldDeserializer;
            impl DimensionFieldDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DimensionField, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type DimensionList = Vec<Dimension>;
struct DimensionListDeserializer;
            impl DimensionListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DimensionList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Dimension" {
                        obj.push(try!(DimensionDeserializer::deserialize("Dimension", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type DisassociateVPCComment = String;

                pub struct DisassociateVPCCommentSerializer;
                impl DisassociateVPCCommentSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &DisassociateVPCComment) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains information about the VPC that you want to disassociate from a specified private hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DisassociateVPCFromHostedZoneRequest {
                #[doc="<p> <i>Optional:</i> A comment about the disassociation request.</p>"]
pub comment: Option<DisassociateVPCComment>,
#[doc="<p>The ID of the private hosted zone that you want to disassociate a VPC from.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>A complex type that contains information about the VPC that you're disassociating from the specified hosted zone.</p>"]
pub vpc: VPC,
            }
            
#[doc="<p>A complex type that contains the response information for the disassociate request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct DisassociateVPCFromHostedZoneResponse {
                #[doc="<p>A complex type that describes the changes made to the specified private hosted zone.</p>"]
pub change_info: ChangeInfo,
            }
            
struct DisassociateVPCFromHostedZoneResponseDeserializer;
            impl DisassociateVPCFromHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<DisassociateVPCFromHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = DisassociateVPCFromHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type EnableSNI = bool;
struct EnableSNIDeserializer;
            impl EnableSNIDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<EnableSNI, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct EnableSNISerializer;
                impl EnableSNISerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &EnableSNI) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ErrorMessage = String;
pub type ErrorMessages = Vec<ErrorMessage>;
pub type EvaluationPeriods = i64;
struct EvaluationPeriodsDeserializer;
            impl EvaluationPeriodsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<EvaluationPeriods, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type FailureThreshold = i64;
struct FailureThresholdDeserializer;
            impl FailureThresholdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<FailureThreshold, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct FailureThresholdSerializer;
                impl FailureThresholdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &FailureThreshold) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type FullyQualifiedDomainName = String;
struct FullyQualifiedDomainNameDeserializer;
            impl FullyQualifiedDomainNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<FullyQualifiedDomainName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct FullyQualifiedDomainNameSerializer;
                impl FullyQualifiedDomainNameSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &FullyQualifiedDomainName) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains information about a geo location.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GeoLocation {
                #[doc="<p>The two-letter code for the continent.</p> <p>Valid values: <code>AF</code> | <code>AN</code> | <code>AS</code> | <code>EU</code> | <code>OC</code> | <code>NA</code> | <code>SA</code> </p> <p>Constraint: Specifying <code>ContinentCode</code> with either <code>CountryCode</code> or <code>SubdivisionCode</code> returns an <code>InvalidInput</code> error.</p>"]
pub continent_code: Option<GeoLocationContinentCode>,
#[doc="<p>The two-letter code for the country.</p>"]
pub country_code: Option<GeoLocationCountryCode>,
#[doc="<p>The code for the subdivision, for example, a state in the United States or a province in Canada.</p>"]
pub subdivision_code: Option<GeoLocationSubdivisionCode>,
            }
            
struct GeoLocationDeserializer;
            impl GeoLocationDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocation, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GeoLocation::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ContinentCode" => {
                obj.continent_code = Some(try!(GeoLocationContinentCodeDeserializer::deserialize("ContinentCode", stack)));
            }
"CountryCode" => {
                obj.country_code = Some(try!(GeoLocationCountryCodeDeserializer::deserialize("CountryCode", stack)));
            }
"SubdivisionCode" => {
                obj.subdivision_code = Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize("SubdivisionCode", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct GeoLocationSerializer;
                impl GeoLocationSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &GeoLocation) -> String {
                        let mut serialized = format!("<{name}>", name=name);if let Some(ref value) = obj.continent_code {
                serialized += &format!("<ContinentCode>{value}</ContinentCode>", value=value);
            }if let Some(ref value) = obj.country_code {
                serialized += &format!("<CountryCode>{value}</CountryCode>", value=value);
            }if let Some(ref value) = obj.subdivision_code {
                serialized += &format!("<SubdivisionCode>{value}</SubdivisionCode>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type GeoLocationContinentCode = String;
struct GeoLocationContinentCodeDeserializer;
            impl GeoLocationContinentCodeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationContinentCode, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct GeoLocationContinentCodeSerializer;
                impl GeoLocationContinentCodeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &GeoLocationContinentCode) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type GeoLocationContinentName = String;
struct GeoLocationContinentNameDeserializer;
            impl GeoLocationContinentNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationContinentName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type GeoLocationCountryCode = String;
struct GeoLocationCountryCodeDeserializer;
            impl GeoLocationCountryCodeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationCountryCode, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct GeoLocationCountryCodeSerializer;
                impl GeoLocationCountryCodeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &GeoLocationCountryCode) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type GeoLocationCountryName = String;
struct GeoLocationCountryNameDeserializer;
            impl GeoLocationCountryNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationCountryName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the codes and full continent, country, and subdivision names for the specified <code>geolocation</code> code.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GeoLocationDetails {
                #[doc="<p>The two-letter code for the continent.</p>"]
pub continent_code: Option<GeoLocationContinentCode>,
#[doc="<p>The full name of the continent.</p>"]
pub continent_name: Option<GeoLocationContinentName>,
#[doc="<p>The two-letter code for the country.</p>"]
pub country_code: Option<GeoLocationCountryCode>,
#[doc="<p>The name of the country.</p>"]
pub country_name: Option<GeoLocationCountryName>,
#[doc="<p>The code for the subdivision, for example, a state in the United States or a province in Canada.</p>"]
pub subdivision_code: Option<GeoLocationSubdivisionCode>,
#[doc="<p>The full name of the subdivision, for example, a state in the United States or a province in Canada.</p>"]
pub subdivision_name: Option<GeoLocationSubdivisionName>,
            }
            
struct GeoLocationDetailsDeserializer;
            impl GeoLocationDetailsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationDetails, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GeoLocationDetails::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ContinentCode" => {
                obj.continent_code = Some(try!(GeoLocationContinentCodeDeserializer::deserialize("ContinentCode", stack)));
            }
"ContinentName" => {
                obj.continent_name = Some(try!(GeoLocationContinentNameDeserializer::deserialize("ContinentName", stack)));
            }
"CountryCode" => {
                obj.country_code = Some(try!(GeoLocationCountryCodeDeserializer::deserialize("CountryCode", stack)));
            }
"CountryName" => {
                obj.country_name = Some(try!(GeoLocationCountryNameDeserializer::deserialize("CountryName", stack)));
            }
"SubdivisionCode" => {
                obj.subdivision_code = Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize("SubdivisionCode", stack)));
            }
"SubdivisionName" => {
                obj.subdivision_name = Some(try!(GeoLocationSubdivisionNameDeserializer::deserialize("SubdivisionName", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type GeoLocationDetailsList = Vec<GeoLocationDetails>;
struct GeoLocationDetailsListDeserializer;
            impl GeoLocationDetailsListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationDetailsList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "GeoLocationDetails" {
                        obj.push(try!(GeoLocationDetailsDeserializer::deserialize("GeoLocationDetails", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type GeoLocationSubdivisionCode = String;
struct GeoLocationSubdivisionCodeDeserializer;
            impl GeoLocationSubdivisionCodeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationSubdivisionCode, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct GeoLocationSubdivisionCodeSerializer;
                impl GeoLocationSubdivisionCodeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &GeoLocationSubdivisionCode) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type GeoLocationSubdivisionName = String;
struct GeoLocationSubdivisionNameDeserializer;
            impl GeoLocationSubdivisionNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GeoLocationSubdivisionName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>The input for a GetChange request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetChangeRequest {
                #[doc="<p>The ID of the change batch request. The value that you specify here is the value that <code>ChangeResourceRecordSets</code> returned in the Id element when you submitted the request.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>A complex type that contains the <code>ChangeInfo</code> element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetChangeResponse {
                #[doc="<p>A complex type that contains information about the specified change batch.</p>"]
pub change_info: ChangeInfo,
            }
            
struct GetChangeResponseDeserializer;
            impl GetChangeResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetChangeResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetChangeResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ChangeInfo" => {
                obj.change_info = try!(ChangeInfoDeserializer::deserialize("ChangeInfo", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>Empty request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetCheckerIpRangesRequest;
            
#[doc="<p>A complex type that contains the <code>CheckerIpRanges</code> element.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetCheckerIpRangesResponse {
                #[doc="<p>A complex type that contains sorted list of IP ranges in CIDR format for Amazon Route 53 health checkers.</p>"]
pub checker_ip_ranges: CheckerIpRanges,
            }
            
struct GetCheckerIpRangesResponseDeserializer;
            impl GetCheckerIpRangesResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetCheckerIpRangesResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetCheckerIpRangesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CheckerIpRanges" => {
                obj.checker_ip_ranges = try!(CheckerIpRangesDeserializer::deserialize("CheckerIpRanges", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the request to get a geo location.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetGeoLocationRequest {
                #[doc="<p>Amazon Route 53 supports the following continent codes:</p> <ul> <li> <p> <b>AF</b>: Africa</p> </li> <li> <p> <b>AN</b>: Antarctica</p> </li> <li> <p> <b>AS</b>: Asia</p> </li> <li> <p> <b>EU</b>: Europe</p> </li> <li> <p> <b>OC</b>: Oceania</p> </li> <li> <p> <b>NA</b>: North America</p> </li> <li> <p> <b>SA</b>: South America</p> </li> </ul>"]
pub continent_code: Option<GeoLocationContinentCode>,
#[doc="<p>Amazon Route 53 uses the two-letter country codes that are specified in <a href=\"https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2\">ISO standard 3166-1 alpha-2</a>.</p>"]
pub country_code: Option<GeoLocationCountryCode>,
#[doc="<p>Amazon Route 53 uses the one- to three-letter subdivision codes that are specified in <a href=\"https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2\">ISO standard 3166-1 alpha-2</a>. Amazon Route 53 doesn't support subdivision codes for all countries. If you specify <code>SubdivisionCode</code>, you must also specify <code>CountryCode</code>. </p>"]
pub subdivision_code: Option<GeoLocationSubdivisionCode>,
            }
            
#[doc="<p>A complex type that contains the response information for the specified geolocation code.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetGeoLocationResponse {
                #[doc="<p>A complex type that contains the codes and full continent, country, and subdivision names for the specified geolocation code.</p>"]
pub geo_location_details: GeoLocationDetails,
            }
            
struct GetGeoLocationResponseDeserializer;
            impl GetGeoLocationResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetGeoLocationResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetGeoLocationResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "GeoLocationDetails" => {
                obj.geo_location_details = try!(GeoLocationDetailsDeserializer::deserialize("GeoLocationDetails", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a count of all your health checks, send a <code>GET</code> request to the <code>/2013-04-01/healthcheckcount</code> resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckCountRequest;
            
#[doc="<p>A complex type that contains the response to a <code>healthcheckcount</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckCountResponse {
                #[doc="<p>The number of health checks associated with the current AWS account.</p>"]
pub health_check_count: HealthCheckCount,
            }
            
struct GetHealthCheckCountResponseDeserializer;
            impl GetHealthCheckCountResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHealthCheckCountResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckCountResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheckCount" => {
                obj.health_check_count = try!(HealthCheckCountDeserializer::deserialize("HealthCheckCount", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>This action gets the reason that a specified health check failed most recently.</p> <p>To get the reason for the last failure of a health check, send a GET request to the /2013-04-01/healthcheck/health check ID/lastfailurereason resource. </p> <p>For information about viewing the last failure reason for a health check using the Amazon Route 53 console, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-monitor-view-status.html\">Viewing Health Check Status and the Reason for Health Check Failures</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckLastFailureReasonRequest {
                #[doc="<p>The ID for the health check for which you want the last failure reason. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p>"]
pub health_check_id: HealthCheckId,
            }
            
#[doc="<p>A complex type that contains the response to a <code>GetHealthCheckLastFailureReason</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckLastFailureReasonResponse {
                #[doc="<p>A list that contains one <code>Observation</code> element for each Amazon Route 53 health checker that is reporting a last failure reason. </p>"]
pub health_check_observations: HealthCheckObservations,
            }
            
struct GetHealthCheckLastFailureReasonResponseDeserializer;
            impl GetHealthCheckLastFailureReasonResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHealthCheckLastFailureReasonResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckLastFailureReasonResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheckObservations" => {
                obj.health_check_observations = try!(HealthCheckObservationsDeserializer::deserialize("HealthCheckObservations", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>This action gets information about a specified health check.</p> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/gethealthcheckrequest</code> resource.</p> <p>For information about getting information about a health check using the Amazon Route 53 console, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckRequest {
                #[doc="<p>The identifier that Amazon Route 53 assigned to the health check when you created it. When you add or update a resource record set, you use this value to specify which health check to use. The value can be up to 64 characters long.</p>"]
pub health_check_id: HealthCheckId,
            }
            
#[doc="<p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckResponse {
                #[doc="<p>A complex type that contains information about one health check that is associated with the current AWS account.</p>"]
pub health_check: HealthCheck,
            }
            
struct GetHealthCheckResponseDeserializer;
            impl GetHealthCheckResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHealthCheckResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheck" => {
                obj.health_check = try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the request to get health check status for a health check.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckStatusRequest {
                #[doc="<p>If you want Amazon Route 53 to return this resource record set in response to a DNS query only when a health check is passing, include the <code>HealthCheckId</code> element and specify the ID of the applicable health check.</p> <p>Amazon Route 53 determines whether a resource record set is healthy by periodically sending a request to the endpoint that is specified in the health check. If that endpoint returns an HTTP status code of 2xx or 3xx, the endpoint is healthy. If the endpoint returns an HTTP status code of 400 or greater, or if the endpoint doesn't respond for a certain amount of time, Amazon Route 53 considers the endpoint unhealthy and also considers the resource record set unhealthy.</p> <p>The <code>HealthCheckId</code> element is only useful when Amazon Route 53 is choosing between two or more resource record sets to respond to a DNS query, and you want Amazon Route 53 to base the choice in part on the status of a health check. Configuring health checks only makes sense in the following configurations:</p> <ul> <li> <p>You're checking the health of the resource record sets in a weighted, latency, geolocation, or failover resource record set, and you specify health check IDs for all of the resource record sets. If the health check for one resource record set specifies an endpoint that is not healthy, Amazon Route 53 stops responding to queries using the value for that resource record set.</p> </li> <li> <p>You set <code>EvaluateTargetHealth</code> to <code>true</code> for the resource record sets in an alias, weighted alias, latency alias, geolocation alias, or failover alias resource record set, and you specify health check IDs for all of the resource record sets that are referenced by the alias resource record sets. For more information about this configuration, see <code>EvaluateTargetHealth</code>.</p> <p>Amazon Route 53 doesn't check the health of the endpoint specified in the resource record set, for example, the endpoint specified by the IP address in the <code>Value</code> element. When you add a <code>HealthCheckId</code> element to a resource record set, Amazon Route 53 checks the health of the endpoint that you specified in the health check.</p> </li> </ul> <p>For geolocation resource record sets, if an endpoint is unhealthy, Amazon Route 53 looks for a resource record set for the larger, associated geographic region. For example, suppose you have resource record sets for a state in the United States, for the United States, for North America, and for all locations. If the endpoint for the state resource record set is unhealthy, Amazon Route 53 checks the resource record sets for the United States, for North America, and for all locations (a resource record set for which the value of CountryCode is <code>*</code>), in that order, until it finds a resource record set for which the endpoint is healthy.</p> <p>If your health checks specify the endpoint only by domain name, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-1-www.example.com</code>), not the name of the resource record sets (example.com).</p> <important> <p>In this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important>"]
pub health_check_id: HealthCheckId,
            }
            
#[doc="<p>A complex type that contains the response to a <code>GetHealthCheck</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHealthCheckStatusResponse {
                #[doc="<p>A list that contains one <code>HealthCheckObservation</code> element for each Amazon Route 53 health checker that is reporting a status about the health check endpoint.</p>"]
pub health_check_observations: HealthCheckObservations,
            }
            
struct GetHealthCheckStatusResponseDeserializer;
            impl GetHealthCheckStatusResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHealthCheckStatusResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHealthCheckStatusResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheckObservations" => {
                obj.health_check_observations = try!(HealthCheckObservationsDeserializer::deserialize("HealthCheckObservations", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a count of all your hosted zones, send a <code>GET</code> request to the <code>/2013-04-01/hostedzonecount</code> resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHostedZoneCountRequest;
            
#[doc="<p>A complex type that contains the response to a <code>hostedzonecount</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHostedZoneCountResponse {
                #[doc="<p>The total number of public and private hosted zones associated with the current AWS account.</p>"]
pub hosted_zone_count: HostedZoneCount,
            }
            
struct GetHostedZoneCountResponseDeserializer;
            impl GetHostedZoneCountResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHostedZoneCountResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHostedZoneCountResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneCount" => {
                obj.hosted_zone_count = try!(HostedZoneCountDeserializer::deserialize("HostedZoneCount", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>The input for a GetHostedZone request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHostedZoneRequest {
                #[doc="<p>The ID of the hosted zone for which you want to get a list of the name servers in the delegation set.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>A complex type containing the response information for the hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetHostedZoneResponse {
                #[doc="<p>A complex type that describes the name servers for this hosted zone.</p>"]
pub delegation_set: Option<DelegationSet>,
#[doc="<p>A complex type that contains general information about the hosted zone.</p>"]
pub hosted_zone: HostedZone,
#[doc="<p>A complex type that contains information about VPCs associated with the specified hosted zone.</p>"]
pub vp_cs: Option<VPCs>,
            }
            
struct GetHostedZoneResponseDeserializer;
            impl GetHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DelegationSet" => {
                obj.delegation_set = Some(try!(DelegationSetDeserializer::deserialize("DelegationSet", stack)));
            }
"HostedZone" => {
                obj.hosted_zone = try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
            }
"VPCs" => {
                obj.vp_cs = Some(try!(VPCsDeserializer::deserialize("VPCs", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>The input for a <code>GetReusableDelegationSet</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetReusableDelegationSetRequest {
                #[doc="<p>The ID of the reusable delegation set for which you want to get a list of the name server.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>A complex type that contains the response to the <code>GetReusableDelegationSet</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetReusableDelegationSetResponse {
                #[doc="<p>A complex type that contains information about the reusable delegation set.</p>"]
pub delegation_set: DelegationSet,
            }
            
struct GetReusableDelegationSetResponseDeserializer;
            impl GetReusableDelegationSetResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetReusableDelegationSetResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetReusableDelegationSetResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DelegationSet" => {
                obj.delegation_set = try!(DelegationSetDeserializer::deserialize("DelegationSet", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a count of all your traffic policy instances, send a <code>GET</code> request to the <code>/2013-04-01/trafficpolicyinstancecount</code> resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyInstanceCountRequest;
            
#[doc="<p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyInstanceCountResponse {
                #[doc="<p>The number of traffic policy instances that are associated with the current AWS account.</p>"]
pub traffic_policy_instance_count: TrafficPolicyInstanceCount,
            }
            
struct GetTrafficPolicyInstanceCountResponseDeserializer;
            impl GetTrafficPolicyInstanceCountResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetTrafficPolicyInstanceCountResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyInstanceCountResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicyInstanceCount" => {
                obj.traffic_policy_instance_count = try!(TrafficPolicyInstanceCountDeserializer::deserialize("TrafficPolicyInstanceCount", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>Gets information about a specified traffic policy instance.</p> <p>To get information about a traffic policy instance, send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicyinstance/<i>Id</i> </code> resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyInstanceRequest {
                #[doc="<p>The ID of the traffic policy instance that you want to get information about.</p>"]
pub id: TrafficPolicyInstanceId,
            }
            
#[doc="<p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyInstanceResponse {
                #[doc="<p>A complex type that contains settings for the traffic policy instance.</p>"]
pub traffic_policy_instance: TrafficPolicyInstance,
            }
            
struct GetTrafficPolicyInstanceResponseDeserializer;
            impl GetTrafficPolicyInstanceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetTrafficPolicyInstanceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyInstanceResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicyInstance" => {
                obj.traffic_policy_instance = try!(TrafficPolicyInstanceDeserializer::deserialize("TrafficPolicyInstance", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>Gets information about a specific traffic policy version. To get the information, send a GET request to the /2013-04-01/trafficpolicy resource, and specify the ID and the version of the traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyRequest {
                #[doc="<p>The ID of the traffic policy that you want to get information about.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The version number of the traffic policy that you want to get information about.</p>"]
pub version: TrafficPolicyVersion,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct GetTrafficPolicyResponse {
                #[doc="<p>A complex type that contains settings for the specified traffic policy.</p>"]
pub traffic_policy: TrafficPolicy,
            }
            
struct GetTrafficPolicyResponseDeserializer;
            impl GetTrafficPolicyResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<GetTrafficPolicyResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = GetTrafficPolicyResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicy" => {
                obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about one health check that is associated with the current AWS account.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct HealthCheck {
                #[doc="<p>A unique string that you specified when you created the health check.</p>"]
pub caller_reference: HealthCheckNonce,
#[doc="<p>A complex type that contains information about the CloudWatch alarm that Amazon Route 53 is monitoring for this health check.</p>"]
pub cloud_watch_alarm_configuration: Option<CloudWatchAlarmConfiguration>,
#[doc="<p>A complex type that contains detailed information about one health check.</p>"]
pub health_check_config: HealthCheckConfig,
#[doc="<p>The version of the health check. You can optionally pass this value in a call to <code>UpdateHealthCheck</code> to prevent overwriting another change to the health check.</p>"]
pub health_check_version: HealthCheckVersion,
#[doc="<p>The identifier that Amazon Route 53assigned to the health check when you created it. When you add or update a resource record set, you use this value to specify which health check to use. The value can be up to 64 characters long. </p>"]
pub id: HealthCheckId,
            }
            
struct HealthCheckDeserializer;
            impl HealthCheckDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheck, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = HealthCheck::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CallerReference" => {
                obj.caller_reference = try!(HealthCheckNonceDeserializer::deserialize("CallerReference", stack));
            }
"CloudWatchAlarmConfiguration" => {
                obj.cloud_watch_alarm_configuration = Some(try!(CloudWatchAlarmConfigurationDeserializer::deserialize("CloudWatchAlarmConfiguration", stack)));
            }
"HealthCheckConfig" => {
                obj.health_check_config = try!(HealthCheckConfigDeserializer::deserialize("HealthCheckConfig", stack));
            }
"HealthCheckVersion" => {
                obj.health_check_version = try!(HealthCheckVersionDeserializer::deserialize("HealthCheckVersion", stack));
            }
"Id" => {
                obj.id = try!(HealthCheckIdDeserializer::deserialize("Id", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the health check.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct HealthCheckConfig {
                #[doc="<p>A complex type that identifies the CloudWatch alarm that you want Amazon Route 53 health checkers to use to determine whether this health check is healthy.</p>"]
pub alarm_identifier: Option<AlarmIdentifier>,
#[doc="<p>(CALCULATED Health Checks Only) A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>"]
pub child_health_checks: Option<ChildHealthCheckList>,
#[doc="<p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during TLS negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that <code>HTTPS</code> requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be <code>SSL alert handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>"]
pub enable_sni: Option<EnableSNI>,
#[doc="<p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html\">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
pub failure_threshold: Option<FailureThreshold>,
#[doc="<p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Amazon Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Amazon Route 53 to perform health checks.</p> <p>When Amazon Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the Host header. </p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Amazon Route 53 passes <code>FullyQualifiedDomainName:Port</code> to the endpoint in the <code>Host</code> header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Amazon Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the preceding cases.</p> <p> <b>If you don't specify a value for <code>IPAddress</code> </b>:</p> <p>Amazon Route 53 sends a DNS request to the domain that you specify for <code>FullyQualifiedDomainName</code> at the interval that you specify for <code>RequestInterval</code>. Using an IPv4 address that DNS returns, Amazon Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a \"DNS resolution failed\" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as us-east-1-www.example.com), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value that you specify for <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Amazon Route 53 doesn't pass a <code>Host</code> header.</p>"]
pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
#[doc="<p>The number of child health checks that are associated with a <code>CALCULATED</code> health that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <a>HealthCheckConfig$ChildHealthChecks</a> and <a>HealthCheckConfig$ChildHealthChecks</a> elements.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Amazon Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Amazon Route 53 always considers this health check to be healthy.</p> </li> </ul>"]
pub health_threshold: Option<HealthThreshold>,
#[doc="<p>The IPv4 or IPv6 IP address of the endpoint that you want Amazon Route 53 to perform health checks on. If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance will never change.</p> <p>For more information, see <a>HealthCheckConfig$FullyQualifiedDomainName</a>.</p> <p>Constraints: Amazon Route 53 can't check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can't create health checks, see the following documents:</p> <ul> <li> <p> <a href=\"https://tools.ietf.org/html/rfc5735\">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href=\"https://tools.ietf.org/html/rfc6598\">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href=\"https://tools.ietf.org/html/rfc5156\">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul> <p>When the value of <code>Type</code> is <code>CALCULATED</code> or <code>CLOUDWATCH_METRIC</code>, omit <code>IPAddress</code>.</p>"]
pub ip_address: Option<IPAddress>,
#[doc="<p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Amazon Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Amazon Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Amazon Route 53uses the status of the health check from the last time CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul>"]
pub insufficient_data_health_status: Option<InsufficientDataHealthStatus>,
#[doc="<p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>"]
pub inverted: Option<Inverted>,
#[doc="<p>Specify whether you want Amazon Route 53 to measure the latency between health checkers in multiple AWS regions and your endpoint, and to display CloudWatch latency graphs on the <b>Health Checks</b> page in the Amazon Route 53 console.</p> <important> <p>You can't change the value of <code>MeasureLatency</code> after you create a health check.</p> </important>"]
pub measure_latency: Option<MeasureLatency>,
#[doc="<p>The port on the endpoint on which you want Amazon Route 53 to perform health checks. Specify a value for Port only when you specify a value for <code>IPAddress</code>.</p>"]
pub port: Option<Port>,
#[doc="<p>A complex type that contains one Region element for each region from which you want Amazon Route 53 health checkers to check the specified endpoint.</p>"]
pub regions: Option<HealthCheckRegionList>,
#[doc="<p>The number of seconds between the time that Amazon Route 53 gets a response from your endpoint and the time that it sends the next health-check request. Each Amazon Route 53 health checker makes requests at this interval.</p> <important> <p>You can't change the value of <code>RequestInterval</code> after you create a health check.</p> </important>"]
pub request_interval: Option<RequestInterval>,
#[doc="<p>The path, if any, that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example, the file /docs/route53-health-check.html. </p>"]
pub resource_path: Option<ResourcePath>,
#[doc="<p>If the value of Type is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Amazon Route 53 considers the resource healthy.</p> <p>Amazon Route 53 considers case when searching for <code>SearchString</code> in the response body. </p>"]
pub search_string: Option<SearchString>,
#[doc="<p>The type of health check that you want to create, which indicates how Amazon Route 53 determines whether an endpoint is healthy.</p> <important> <p>You can't change the value of <code>Type</code> after you create a health check.</p> </important> <p>You can create the following types of health checks:</p> <ul> <li> <p> <b>HTTP</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater and less than 400.</p> </li> <li> <p> <b>HTTPS</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater and less than 400.</p> <important> <p>If you specify <code>HTTPS</code> for the value of <code>Type</code>, the endpoint must support TLS v1.0 or later.</p> </important> </li> <li> <p> <b>HTTP_STR_MATCH</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an HTTP request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>HTTPS_STR_MATCH</b>: Amazon Route 53 tries to establish a TCP connection. If successful, Amazon Route 53 submits an <code>HTTPS</code> request and searches the first 5,120 bytes of the response body for the string that you specify in <code>SearchString</code>.</p> </li> <li> <p> <b>TCP</b>: Amazon Route 53 tries to establish a TCP connection.</p> </li> <li> <p> <b>CLOUDWATCH_METRIC</b>: The health check is associated with a CloudWatch alarm. If the state of the alarm is <code>OK</code>, the health check is considered healthy. If the state is <code>ALARM</code>, the health check is considered unhealthy. If CloudWatch doesn't have sufficient data to determine whether the state is <code>OK</code> or <code>ALARM</code>, the health check status depends on the setting for <code>InsufficientDataHealthStatus</code>: <code>Healthy</code>, <code>Unhealthy</code>, or <code>LastKnownStatus</code>. </p> </li> <li> <p> <b>CALCULATED</b>: For health checks that monitor the status of other health checks, Amazon Route 53 adds up the number of health checks that Amazon Route 53 health checkers consider to be healthy and compares that number with the value of <code>HealthThreshold</code>. </p> </li> </ul> <p>For more information about how Amazon Route 53 determines whether an endpoint is healthy, see the introduction to this topic.</p>"]
pub type_: HealthCheckType,
            }
            
struct HealthCheckConfigDeserializer;
            impl HealthCheckConfigDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckConfig, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = HealthCheckConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AlarmIdentifier" => {
                obj.alarm_identifier = Some(try!(AlarmIdentifierDeserializer::deserialize("AlarmIdentifier", stack)));
            }
"ChildHealthChecks" => {
                obj.child_health_checks = Some(try!(ChildHealthCheckListDeserializer::deserialize("ChildHealthChecks", stack)));
            }
"EnableSNI" => {
                obj.enable_sni = Some(try!(EnableSNIDeserializer::deserialize("EnableSNI", stack)));
            }
"FailureThreshold" => {
                obj.failure_threshold = Some(try!(FailureThresholdDeserializer::deserialize("FailureThreshold", stack)));
            }
"FullyQualifiedDomainName" => {
                obj.fully_qualified_domain_name = Some(try!(FullyQualifiedDomainNameDeserializer::deserialize("FullyQualifiedDomainName", stack)));
            }
"HealthThreshold" => {
                obj.health_threshold = Some(try!(HealthThresholdDeserializer::deserialize("HealthThreshold", stack)));
            }
"IPAddress" => {
                obj.ip_address = Some(try!(IPAddressDeserializer::deserialize("IPAddress", stack)));
            }
"InsufficientDataHealthStatus" => {
                obj.insufficient_data_health_status = Some(try!(InsufficientDataHealthStatusDeserializer::deserialize("InsufficientDataHealthStatus", stack)));
            }
"Inverted" => {
                obj.inverted = Some(try!(InvertedDeserializer::deserialize("Inverted", stack)));
            }
"MeasureLatency" => {
                obj.measure_latency = Some(try!(MeasureLatencyDeserializer::deserialize("MeasureLatency", stack)));
            }
"Port" => {
                obj.port = Some(try!(PortDeserializer::deserialize("Port", stack)));
            }
"Regions" => {
                obj.regions = Some(try!(HealthCheckRegionListDeserializer::deserialize("Regions", stack)));
            }
"RequestInterval" => {
                obj.request_interval = Some(try!(RequestIntervalDeserializer::deserialize("RequestInterval", stack)));
            }
"ResourcePath" => {
                obj.resource_path = Some(try!(ResourcePathDeserializer::deserialize("ResourcePath", stack)));
            }
"SearchString" => {
                obj.search_string = Some(try!(SearchStringDeserializer::deserialize("SearchString", stack)));
            }
"Type" => {
                obj.type_ = try!(HealthCheckTypeDeserializer::deserialize("Type", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckConfigSerializer;
                impl HealthCheckConfigSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckConfig) -> String {
                        let mut serialized = format!("<{name}>", name=name);
            if let Some(ref value) = obj.alarm_identifier {
                serialized += &AlarmIdentifierSerializer::serialize("AlarmIdentifier", value);
            }
            if let Some(ref value) = obj.child_health_checks {
                serialized += &ChildHealthCheckListSerializer::serialize("ChildHealthChecks", value);
            }if let Some(ref value) = obj.enable_sni {
                serialized += &format!("<EnableSNI>{value}</EnableSNI>", value=value);
            }if let Some(ref value) = obj.failure_threshold {
                serialized += &format!("<FailureThreshold>{value}</FailureThreshold>", value=value);
            }if let Some(ref value) = obj.fully_qualified_domain_name {
                serialized += &format!("<FullyQualifiedDomainName>{value}</FullyQualifiedDomainName>", value=value);
            }if let Some(ref value) = obj.health_threshold {
                serialized += &format!("<HealthThreshold>{value}</HealthThreshold>", value=value);
            }if let Some(ref value) = obj.ip_address {
                serialized += &format!("<IPAddress>{value}</IPAddress>", value=value);
            }if let Some(ref value) = obj.insufficient_data_health_status {
                serialized += &format!("<InsufficientDataHealthStatus>{value}</InsufficientDataHealthStatus>", value=value);
            }if let Some(ref value) = obj.inverted {
                serialized += &format!("<Inverted>{value}</Inverted>", value=value);
            }if let Some(ref value) = obj.measure_latency {
                serialized += &format!("<MeasureLatency>{value}</MeasureLatency>", value=value);
            }if let Some(ref value) = obj.port {
                serialized += &format!("<Port>{value}</Port>", value=value);
            }
            if let Some(ref value) = obj.regions {
                serialized += &HealthCheckRegionListSerializer::serialize("Regions", value);
            }if let Some(ref value) = obj.request_interval {
                serialized += &format!("<RequestInterval>{value}</RequestInterval>", value=value);
            }if let Some(ref value) = obj.resource_path {
                serialized += &format!("<ResourcePath>{value}</ResourcePath>", value=value);
            }if let Some(ref value) = obj.search_string {
                serialized += &format!("<SearchString>{value}</SearchString>", value=value);
            }serialized += &format!("<Type>{value}</Type>", value=obj.type_);serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type HealthCheckCount = i64;
struct HealthCheckCountDeserializer;
            impl HealthCheckCountDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckCount, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type HealthCheckId = String;
struct HealthCheckIdDeserializer;
            impl HealthCheckIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckIdSerializer;
                impl HealthCheckIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type HealthCheckNonce = String;
struct HealthCheckNonceDeserializer;
            impl HealthCheckNonceDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckNonce, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckNonceSerializer;
                impl HealthCheckNonceSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckNonce) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains the last failure reason as reported by one Amazon Route 53 health checker.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct HealthCheckObservation {
                #[doc="<p>The IP address of the Amazon Route 53 health checker that provided the failure reason in <code>StatusReport</code>.</p>"]
pub ip_address: Option<IPAddress>,
#[doc="<p>The region of the Amazon Route 53 health checker that provided the status in StatusReport.</p>"]
pub region: Option<HealthCheckRegion>,
#[doc="<p>A complex type that contains the last failure reason as reported by one Amazon Route 53 health checker and the time of the failed health check.</p>"]
pub status_report: Option<StatusReport>,
            }
            
struct HealthCheckObservationDeserializer;
            impl HealthCheckObservationDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckObservation, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = HealthCheckObservation::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IPAddress" => {
                obj.ip_address = Some(try!(IPAddressDeserializer::deserialize("IPAddress", stack)));
            }
"Region" => {
                obj.region = Some(try!(HealthCheckRegionDeserializer::deserialize("Region", stack)));
            }
"StatusReport" => {
                obj.status_report = Some(try!(StatusReportDeserializer::deserialize("StatusReport", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type HealthCheckObservations = Vec<HealthCheckObservation>;
struct HealthCheckObservationsDeserializer;
            impl HealthCheckObservationsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckObservations, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "HealthCheckObservation" {
                        obj.push(try!(HealthCheckObservationDeserializer::deserialize("HealthCheckObservation", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
#[doc="<p>An Amazon EC2 Region that you want Amazon Route 53 to use to perform health checks.</p>"]
pub type HealthCheckRegion = String;
struct HealthCheckRegionDeserializer;
            impl HealthCheckRegionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckRegion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckRegionSerializer;
                impl HealthCheckRegionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckRegion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type HealthCheckRegionList = Vec<HealthCheckRegion>;
struct HealthCheckRegionListDeserializer;
            impl HealthCheckRegionListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckRegionList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Region" {
                        obj.push(try!(HealthCheckRegionDeserializer::deserialize("Region", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }

                pub struct HealthCheckRegionListSerializer;
                impl HealthCheckRegionListSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckRegionList) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(HealthCheckRegionSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
pub type HealthCheckType = String;
struct HealthCheckTypeDeserializer;
            impl HealthCheckTypeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckType, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckTypeSerializer;
                impl HealthCheckTypeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckType) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type HealthCheckVersion = i64;
struct HealthCheckVersionDeserializer;
            impl HealthCheckVersionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthCheckVersion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthCheckVersionSerializer;
                impl HealthCheckVersionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthCheckVersion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type HealthChecks = Vec<HealthCheck>;
struct HealthChecksDeserializer;
            impl HealthChecksDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthChecks, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "HealthCheck" {
                        obj.push(try!(HealthCheckDeserializer::deserialize("HealthCheck", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type HealthThreshold = i64;
struct HealthThresholdDeserializer;
            impl HealthThresholdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HealthThreshold, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HealthThresholdSerializer;
                impl HealthThresholdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HealthThreshold) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains general information about the hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct HostedZone {
                #[doc="<p>The value that you specified for <code>CallerReference</code> when you created the hosted zone.</p>"]
pub caller_reference: Nonce,
#[doc="<p>A complex type that includes the <code>Comment</code> and <code>PrivateZone</code> elements. If you omitted the <code>HostedZoneConfig</code> and <code>Comment</code> elements from the request, the <code>Config</code> and <code>Comment</code> elements don't appear in the response.</p>"]
pub config: Option<HostedZoneConfig>,
#[doc="<p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>"]
pub id: ResourceId,
#[doc="<p>The name of the domain. For public hosted zones, this is the name that you have registered with your DNS registrar.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a>CreateHostedZone</a>.</p>"]
pub name: DNSName,
#[doc="<p>The number of resource record sets in the hosted zone.</p>"]
pub resource_record_set_count: Option<HostedZoneRRSetCount>,
            }
            
struct HostedZoneDeserializer;
            impl HostedZoneDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HostedZone, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = HostedZone::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CallerReference" => {
                obj.caller_reference = try!(NonceDeserializer::deserialize("CallerReference", stack));
            }
"Config" => {
                obj.config = Some(try!(HostedZoneConfigDeserializer::deserialize("Config", stack)));
            }
"Id" => {
                obj.id = try!(ResourceIdDeserializer::deserialize("Id", stack));
            }
"Name" => {
                obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
            }
"ResourceRecordSetCount" => {
                obj.resource_record_set_count = Some(try!(HostedZoneRRSetCountDeserializer::deserialize("ResourceRecordSetCount", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the <code>HostedZoneConfig</code> and <code>Comment</code> elements.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct HostedZoneConfig {
                #[doc="<p>Any comments that you want to include about the hosted zone.</p>"]
pub comment: Option<ResourceDescription>,
#[doc="<p>A value that indicates whether this is a private hosted zone.</p>"]
pub private_zone: Option<IsPrivateZone>,
            }
            
struct HostedZoneConfigDeserializer;
            impl HostedZoneConfigDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HostedZoneConfig, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = HostedZoneConfig::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Comment" => {
                obj.comment = Some(try!(ResourceDescriptionDeserializer::deserialize("Comment", stack)));
            }
"PrivateZone" => {
                obj.private_zone = Some(try!(IsPrivateZoneDeserializer::deserialize("PrivateZone", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct HostedZoneConfigSerializer;
                impl HostedZoneConfigSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &HostedZoneConfig) -> String {
                        let mut serialized = format!("<{name}>", name=name);if let Some(ref value) = obj.comment {
                serialized += &format!("<Comment>{value}</Comment>", value=value);
            }if let Some(ref value) = obj.private_zone {
                serialized += &format!("<PrivateZone>{value}</PrivateZone>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type HostedZoneCount = i64;
struct HostedZoneCountDeserializer;
            impl HostedZoneCountDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HostedZoneCount, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type HostedZoneRRSetCount = i64;
struct HostedZoneRRSetCountDeserializer;
            impl HostedZoneRRSetCountDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HostedZoneRRSetCount, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type HostedZones = Vec<HostedZone>;
struct HostedZonesDeserializer;
            impl HostedZonesDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<HostedZones, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "HostedZone" {
                        obj.push(try!(HostedZoneDeserializer::deserialize("HostedZone", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type IPAddress = String;
struct IPAddressDeserializer;
            impl IPAddressDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<IPAddress, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct IPAddressSerializer;
                impl IPAddressSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &IPAddress) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type IPAddressCidr = String;
struct IPAddressCidrDeserializer;
            impl IPAddressCidrDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<IPAddressCidr, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type InsufficientDataHealthStatus = String;
struct InsufficientDataHealthStatusDeserializer;
            impl InsufficientDataHealthStatusDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<InsufficientDataHealthStatus, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct InsufficientDataHealthStatusSerializer;
                impl InsufficientDataHealthStatusSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &InsufficientDataHealthStatus) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type Inverted = bool;
struct InvertedDeserializer;
            impl InvertedDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Inverted, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct InvertedSerializer;
                impl InvertedSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Inverted) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type IsPrivateZone = bool;
struct IsPrivateZoneDeserializer;
            impl IsPrivateZoneDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<IsPrivateZone, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct IsPrivateZoneSerializer;
                impl IsPrivateZoneSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &IsPrivateZone) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>To get a list of geographic locations that Amazon Route 53 supports for geolocation, send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/geolocations</code> resource. The response to this request includes a <code>GeoLocationDetails</code> element for each location that Amazon Route 53 supports.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country. </p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListGeoLocationsRequest {
                #[doc="<p>(Optional) The maximum number of geolocations to be included in the response body for this request. If more than <code>MaxItems</code> geolocations remain to be listed, then the value of the <code>IsTruncated</code> element in the response is <code>true</code>.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>The code for the continent with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is true, and if <code>NextContinentCode</code> from the previous response has a value, enter that value in <code>StartContinentCode</code> to return the next page of results.</p> <p>Include <code>StartContinentCode</code> only if you want to list continents. Don't include <code>StartContinentCode</code> when you're listing countries or countries with their subdivisions.</p>"]
pub start_continent_code: Option<GeoLocationContinentCode>,
#[doc="<p>The code for the country with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextCountryCode</code> from the previous response has a value, enter that value in <code>StartCountryCode</code> to return the next page of results.</p> <p>Amazon Route 53 uses the two-letter country codes that are specified in <a href=\"https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2\">ISO standard 3166-1 alpha-2</a>.</p>"]
pub start_country_code: Option<GeoLocationCountryCode>,
#[doc="<p>The code for the subdivision (for example, state or province) with which you want to start listing locations that Amazon Route 53 supports for geolocation. If Amazon Route 53 has already returned a page or more of results, if <code>IsTruncated</code> is <code>true</code>, and if <code>NextSubdivisionCode</code> from the previous response has a value, enter that value in <code>StartSubdivisionCode</code> to return the next page of results.</p> <p>To list subdivisions of a country, you must include both <code>StartCountryCode</code> and <code>StartSubdivisionCode</code>.</p>"]
pub start_subdivision_code: Option<GeoLocationSubdivisionCode>,
            }
            
#[doc="<p>A complex type containing the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListGeoLocationsResponse {
                #[doc="<p>A complex type that contains one <code>GeoLocationDetails</code> element for each location that Amazon Route 53 supports for geolocation.</p>"]
pub geo_location_details_list: GeoLocationDetailsList,
#[doc="<p>A value that indicates whether more locations remain to be listed after the last location in this response. If so, the value of <code>IsTruncated</code> is <code>true</code>. To get more values, submit another request and include the values of <code>NextContinentCode</code>, <code>NextCountryCode</code>, and <code>NextSubdivisionCode</code> in the <code>StartContinentCode</code>, <code>StartCountryCode</code>, and <code>StartSubdivisionCode</code>, as applicable.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for <code>MaxItems</code> in the request.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextContinentCode</code> in the <code>StartContinentCode</code> parameter in another <code>GET</code> <code>ListGeoLocations</code> request.</p>"]
pub next_continent_code: Option<GeoLocationContinentCode>,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextCountryCode</code> in the <code>StartCountryCode</code> parameter in another <code>GET</code> <code>ListGeoLocations</code> request.</p>"]
pub next_country_code: Option<GeoLocationCountryCode>,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, you can make a follow-up request to display more locations. Enter the value of <code>NextSubdivisionCode</code> in the <code>StartSubdivisionCode</code> parameter in another <code>GET</code> <code>ListGeoLocations</code> request.</p>"]
pub next_subdivision_code: Option<GeoLocationSubdivisionCode>,
            }
            
struct ListGeoLocationsResponseDeserializer;
            impl ListGeoLocationsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListGeoLocationsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListGeoLocationsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "GeoLocationDetailsList" => {
                obj.geo_location_details_list = try!(GeoLocationDetailsListDeserializer::deserialize("GeoLocationDetailsList", stack));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextContinentCode" => {
                obj.next_continent_code = Some(try!(GeoLocationContinentCodeDeserializer::deserialize("NextContinentCode", stack)));
            }
"NextCountryCode" => {
                obj.next_country_code = Some(try!(GeoLocationCountryCodeDeserializer::deserialize("NextCountryCode", stack)));
            }
"NextSubdivisionCode" => {
                obj.next_subdivision_code = Some(try!(GeoLocationSubdivisionCodeDeserializer::deserialize("NextSubdivisionCode", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a list of your health checks, send a <code>GET</code> request to the <code>/2013-04-01/healthcheck</code> resource. The response to this request includes a <code>HealthChecks</code> element with zero or more <code>HealthCheck</code> child elements. By default, the list of health checks is displayed on a single page. You can control the length of the page that is displayed by using the <code>MaxItems</code> parameter. You can use the <code>Marker</code> parameter to control the health check that the list begins with.</p> <note> <p> Amazon Route 53 returns a maximum of 100 items. If you set <code>MaxItems</code> to a value greater than 100, Amazon Route 53 returns only the first 100.</p> </note>"]
#[derive(Default,Clone,Debug)]
            pub struct ListHealthChecksRequest {
                #[doc="<p>If the response to a <code>ListHealthChecks</code> is more than one page, marker is the health check ID for the first health check on the next page of results. For more information, see <a>ListHealthChecksResponse$MaxItems</a>.</p>"]
pub marker: Option<PageMarker>,
#[doc="<p>The maximum number of <code>HealthCheck</code> elements you want <code>ListHealthChecks</code> to return on each page of the response body. If the AWS account includes more <code>HealthCheck</code> elements than the value of <code>maxitems</code>, the response is broken into pages. Each page contains the number of <code>HealthCheck</code> elements specified by <code>maxitems</code>.</p> <p>For example, suppose you specify <code>10</code> for <code>maxitems</code> and the current AWS account has <code>51</code> health checks. In the response, <code>ListHealthChecks</code> sets <a>ListHealthChecksResponse$IsTruncated</a> to true and includes the <a>ListHealthChecksResponse$NextMarker</a> element. To access the second and subsequent pages, you resend the <code>GET</code> <code>ListHealthChecks</code> request, add the <a>ListHealthChecksResponse$Marker</a> parameter to the request, and specify the value of the <a>ListHealthChecksResponse$NextMarker</a> element from the previous response. On the last (sixth) page of the response, which contains only one HealthCheck element:</p> <ul> <li> <p>The value of <a>ListHealthChecksResponse$IsTruncated</a> is <code>false</code>.</p> </li> <li> <p> <a>ListHealthChecksResponse$NextMarker</a> is omitted.</p> </li> </ul>"]
pub max_items: Option<PageMaxItems>,
            }
            
#[doc="<p>A complex type that contains the response to a <code>ListHealthChecks</code> request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListHealthChecksResponse {
                #[doc="<p>A complex type that contains one <code>HealthCheck</code> element for each health check that is associated with the current AWS account.</p>"]
pub health_checks: HealthChecks,
#[doc="<p>A flag that indicates whether there are more health checks to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> health checks by calling <code>ListHealthChecks</code> again and specifying the value of the <code>NextMarker</code> element in the marker parameter.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>"]
pub is_truncated: PageTruncated,
#[doc="<p>For the second and subsequent calls to <code>ListHealthChecks</code>, <code>Marker</code> is the value that you specified for the marker parameter in the previous request.</p>"]
pub marker: PageMarker,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHealthChecks</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first health check in the next group of <code>maxitems</code> health checks. Call <code>ListHealthChecks</code> again and specify the value of <code>NextMarker</code> in the marker parameter.</p>"]
pub next_marker: Option<PageMarker>,
            }
            
struct ListHealthChecksResponseDeserializer;
            impl ListHealthChecksResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListHealthChecksResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListHealthChecksResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthChecks" => {
                obj.health_checks = try!(HealthChecksDeserializer::deserialize("HealthChecks", stack));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"Marker" => {
                obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextMarker" => {
                obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize("NextMarker", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a list of your public and private hosted zones in ASCII order by domain name, send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/hostedzonesbyname</code> resource. The response to this request includes a <code>HostedZone</code> child element for each hosted zone that was created by the current AWS account. <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for exämple.com, you specify <code>ex\\344mple.com</code> for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as: <code>com.ex\\344mple</code>. The labels are reversed, and it's alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html\">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Amazon Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help you navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, that you specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current Amazon Route 53 account.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul>"]
#[derive(Default,Clone,Debug)]
            pub struct ListHostedZonesByNameRequest {
                #[doc="<p>(Optional) For your first request to <code>ListHostedZonesByName</code>, include the <code>dnsname</code> parameter only if you want to specify the name of the first hosted zone in the response. If you don't include the <code>dnsname</code> parameter, Amazon Route 53 returns all of the hosted zones that were created by the current AWS account, in ASCII order. For subsequent requests, include both <code>dnsname</code> and <code>hostedzoneid</code> parameters. For <code>dnsname</code>, specify the value of <code>NextDNSName</code> from the previous response.</p>"]
pub dns_name: Option<DNSName>,
#[doc="<p>(Optional) For your first request to <code>ListHostedZonesByName</code>, do not include the <code>hostedzoneid</code> parameter.</p> <p>If you have more hosted zones than the value of <code>maxitems</code>, <code>ListHostedZonesByName</code> returns only the first <code>maxitems</code> hosted zones. To get the next group of <code>maxitems</code> hosted zones, submit another request to <code>ListHostedZonesByName</code> and include both <code>dnsname</code> and <code>hostedzoneid</code> parameters. For the value of <code>hostedzoneid</code>, specify the value of the <code>NextHostedZoneId</code> element from the previous response.</p>"]
pub hosted_zone_id: Option<ResourceId>,
#[doc="<p>The maximum number of hosted zones to be included in the response body for this request. If you have more than <code>maxitems</code> hosted zones, then the value of the <code>IsTruncated</code> element in the response is true, and the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> specify the first hosted zone in the next group of <code>maxitems</code> hosted zones. </p>"]
pub max_items: Option<PageMaxItems>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListHostedZonesByNameResponse {
                #[doc="<p>For the second and subsequent calls to <code>ListHostedZonesByName</code>, <code>DNSName</code> is the value that you specified for the <code>dnsname</code> parameter in the request that produced the current response.</p>"]
pub dns_name: Option<DNSName>,
#[doc="<p>The ID that Amazon Route 53 assigned to the hosted zone when you created it.</p>"]
pub hosted_zone_id: Option<ResourceId>,
#[doc="<p>A complex type that contains general information about the hosted zone.</p>"]
pub hosted_zones: HostedZones,
#[doc="<p>A flag that indicates whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZonesByName</code> again and specifying the values of <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the <code>dnsname</code> and <code>hostedzoneid</code> parameters.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZonesByName</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is true, the value of <code>NextDNSName</code> is the name of the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>"]
pub next_dns_name: Option<DNSName>,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextHostedZoneId</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZonesByName</code> again and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>"]
pub next_hosted_zone_id: Option<ResourceId>,
            }
            
struct ListHostedZonesByNameResponseDeserializer;
            impl ListHostedZonesByNameResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListHostedZonesByNameResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListHostedZonesByNameResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DNSName" => {
                obj.dns_name = Some(try!(DNSNameDeserializer::deserialize("DNSName", stack)));
            }
"HostedZoneId" => {
                obj.hosted_zone_id = Some(try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack)));
            }
"HostedZones" => {
                obj.hosted_zones = try!(HostedZonesDeserializer::deserialize("HostedZones", stack));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextDNSName" => {
                obj.next_dns_name = Some(try!(DNSNameDeserializer::deserialize("NextDNSName", stack)));
            }
"NextHostedZoneId" => {
                obj.next_hosted_zone_id = Some(try!(ResourceIdDeserializer::deserialize("NextHostedZoneId", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a list of your public and private hosted zones, send a <code>GET</code> request to the <code>/2013-04-01/hostedzone</code> resource. The response to this request includes a HostedZone child element for each hosted zone that was created by the current AWS account.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the maxitems parameter to list them in groups of up to 100. The response includes four values that help you navigate from one group of maxitems hosted zones to the next:</p> <ul> <li> <p> <code>MaxItems</code> is the value that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is <code>true</code>, there are more hosted zones associated with the current AWS account.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last hosted zone that is associated with the current account.</p> </li> <li> <p> <code>NextMarker</code> is the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZones</code>, and specify the value of the <code>NextMarker</code> element in the marker parameter.</p> <p>If <code>IsTruncated</code> is <code>false</code>, the <code>NextMarker</code> element is omitted from the response.</p> </li> <li> <p>If you're making the second or subsequent call to <code>ListHostedZones</code>, the <code>Marker</code> element matches the value that you specified in the <code>marker</code> parameter in the previous request.</p> </li> </ul>"]
#[derive(Default,Clone,Debug)]
            pub struct ListHostedZonesRequest {
                #[doc="<p>If you're using reusable delegation sets and you want to list all of the hosted zones that are associated with a reusable delegation set, specify the ID of that reusable delegation set. </p>"]
pub delegation_set_id: Option<ResourceId>,
#[doc="<p>(Optional) If you have more hosted zones than the value of <code>maxitems</code>, <code>ListHostedZones</code> returns only the first <code>maxitems</code> hosted zones. To get the next group of <code>maxitems</code> hosted zones, submit another request to <code>ListHostedZones</code>. For the value of marker, specify the value of the <code>NextMarker</code> element that was returned in the previous response.</p> <p>Hosted zones are listed in the order in which they were created.</p>"]
pub marker: Option<PageMarker>,
#[doc="<p>(Optional) The maximum number of hosted zones to be included in the response body for this request. If you have more than <code>maxitems</code> hosted zones, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the value of the <code>NextMarker</code> element is the hosted zone ID of the first hosted zone in the next group of <code>maxitems</code> hosted zones.</p>"]
pub max_items: Option<PageMaxItems>,
            }
            
#[derive(Default,Clone,Debug)]
            pub struct ListHostedZonesResponse {
                #[doc="<p>A complex type that contains general information about the hosted zone.</p>"]
pub hosted_zones: HostedZones,
#[doc="<p>A flag indicating whether there are more hosted zones to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> hosted zones by calling <code>ListHostedZones</code> again and specifying the value of the <code>NextMarker</code> element in the marker parameter.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>For the second and subsequent calls to <code>ListHostedZones</code>, <code>Marker</code> is the value that you specified for the marker parameter in the request that produced the current response.</p>"]
pub marker: PageMarker,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListHostedZones</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first hosted zone in the next group of <code>maxitems</code> hosted zones. Call <code>ListHostedZones</code> again and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>"]
pub next_marker: Option<PageMarker>,
            }
            
struct ListHostedZonesResponseDeserializer;
            impl ListHostedZonesResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListHostedZonesResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListHostedZonesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZones" => {
                obj.hosted_zones = try!(HostedZonesDeserializer::deserialize("HostedZones", stack));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"Marker" => {
                obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextMarker" => {
                obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize("NextMarker", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>The input for a ListResourceRecordSets request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListResourceRecordSetsRequest {
                #[doc="<p>The ID of the hosted zone that contains the resource record sets that you want to get.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p> <i>Weighted resource record sets only:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>"]
pub start_record_identifier: Option<ResourceRecordSetIdentifier>,
#[doc="<p>The first name in the lexicographic ordering of domain names that you want the <code>ListResourceRecordSets</code> request to list.</p>"]
pub start_record_name: Option<DNSName>,
#[doc="<p>The type of resource record set to begin the record listing from.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geo, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for alias resource record sets: </p> <ul> <li> <p> <b>CloudFront distribution</b>: A or AAAA</p> </li> <li> <p> <b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p> </li> <li> <p> <b>ELB load balancer</b>: A | AAAA</p> </li> <li> <p> <b>Amazon S3 bucket</b>: A</p> </li> </ul> <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>"]
pub start_record_type: Option<RRType>,
            }
            
#[doc="<p>A complex type that contains list information for the resource record set.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListResourceRecordSetsResponse {
                #[doc="<p>A flag that indicates whether more resource record sets remain to be listed. If your results were truncated, you can make a follow-up pagination request by using the <code>NextRecordName</code> element.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The maximum number of records you requested.</p>"]
pub max_items: PageMaxItems,
#[doc="<p> <i>Weighted, latency, geolocation, and failover resource record sets only</i>: If results were truncated for a given DNS name and type, the value of <code>SetIdentifier</code> for the next resource record set that has the current DNS name and type.</p>"]
pub next_record_identifier: Option<ResourceRecordSetIdentifier>,
#[doc="<p>If the results were truncated, the name of the next record in the list.</p> <p>This element is present only if <code>IsTruncated</code> is true. </p>"]
pub next_record_name: Option<DNSName>,
#[doc="<p>If the results were truncated, the type of the next record in the list.</p> <p>This element is present only if <code>IsTruncated</code> is true. </p>"]
pub next_record_type: Option<RRType>,
#[doc="<p>Information about multiple resource record sets.</p>"]
pub resource_record_sets: ResourceRecordSets,
            }
            
struct ListResourceRecordSetsResponseDeserializer;
            impl ListResourceRecordSetsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListResourceRecordSetsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListResourceRecordSetsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextRecordIdentifier" => {
                obj.next_record_identifier = Some(try!(ResourceRecordSetIdentifierDeserializer::deserialize("NextRecordIdentifier", stack)));
            }
"NextRecordName" => {
                obj.next_record_name = Some(try!(DNSNameDeserializer::deserialize("NextRecordName", stack)));
            }
"NextRecordType" => {
                obj.next_record_type = Some(try!(RRTypeDeserializer::deserialize("NextRecordType", stack)));
            }
"ResourceRecordSets" => {
                obj.resource_record_sets = try!(ResourceRecordSetsDeserializer::deserialize("ResourceRecordSets", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>To retrieve a list of your reusable delegation sets, send a <code>GET</code> request to the <code>/2013-04-01/delegationset</code> resource. The response to this request includes a <code>DelegationSets</code> element with zero or more <code>DelegationSet</code> child elements. By default, the list of reusable delegation sets is displayed on a single page. You can control the length of the page that is displayed by using the <code>MaxItems</code> parameter. You can use the <code>Marker</code> parameter to control the delegation set that the list begins with.</p> <note> <p>Amazon Route 53 returns a maximum of 100 items. If you set <code>MaxItems</code> to a value greater than 100, Amazon Route 53 returns only the first 100.</p> </note>"]
#[derive(Default,Clone,Debug)]
            pub struct ListReusableDelegationSetsRequest {
                #[doc="<p>If you're making the second or subsequent call to <code>ListReusableDelegationSets</code>, the <code>Marker</code> element matches the value that you specified in the <code>marker</code> parameter in the previous request.</p>"]
pub marker: Option<PageMarker>,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p>"]
pub max_items: Option<PageMaxItems>,
            }
            
#[doc="<p>A complex type that contains information about the reusable delegation sets that are associated with the current AWS account.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListReusableDelegationSetsResponse {
                #[doc="<p>A complex type that contains one <code>DelegationSet</code> element for each reusable delegation set that was created by the current AWS account.</p>"]
pub delegation_sets: DelegationSets,
#[doc="<p>A flag that indicates whether there are more reusable delegation sets to be listed. If the response is truncated, you can get the next group of <code>maxitems</code> reusable delegation sets by calling <code>ListReusableDelegationSets</code> again and specifying the value of the <code>NextMarker</code> element in the <code>marker</code> parameter.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>For the second and subsequent calls to <code>ListReusableDelegationSets</code>, <code>Marker</code> is the value that you specified for the marker parameter in the request that produced the current response.</p>"]
pub marker: PageMarker,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListReusableDelegationSets</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>NextMarker</code> identifies the first reusable delegation set in the next group of <code>maxitems</code> reusable delegation sets. Call <code>ListReusableDelegationSets</code> again and specify the value of <code>NextMarker</code> in the <code>marker</code> parameter.</p>"]
pub next_marker: Option<PageMarker>,
            }
            
struct ListReusableDelegationSetsResponseDeserializer;
            impl ListReusableDelegationSetsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListReusableDelegationSetsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListReusableDelegationSetsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "DelegationSets" => {
                obj.delegation_sets = try!(DelegationSetsDeserializer::deserialize("DelegationSets", stack));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"Marker" => {
                obj.marker = try!(PageMarkerDeserializer::deserialize("Marker", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"NextMarker" => {
                obj.next_marker = Some(try!(PageMarkerDeserializer::deserialize("NextMarker", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type containing information about a request for a list of the tags that are associated with an individual resource.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTagsForResourceRequest {
                #[doc="<p>The ID of the resource for which you want to retrieve tags.</p>"]
pub resource_id: TagResourceId,
#[doc="<p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul>"]
pub resource_type: TagResourceType,
            }
            
#[doc="<p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTagsForResourceResponse {
                #[doc="<p>A <code>ResourceTagSet</code> containing tags associated with the specified resource.</p>"]
pub resource_tag_set: ResourceTagSet,
            }
            
struct ListTagsForResourceResponseDeserializer;
            impl ListTagsForResourceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTagsForResourceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTagsForResourceResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ResourceTagSet" => {
                obj.resource_tag_set = try!(ResourceTagSetDeserializer::deserialize("ResourceTagSet", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTagsForResourcesRequest {
                #[doc="<p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>"]
pub resource_ids: TagResourceIdList,
#[doc="<p>The type of the resources.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul>"]
pub resource_type: TagResourceType,
            }
            
#[doc="<p>A complex type containing tags for the specified resources.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTagsForResourcesResponse {
                #[doc="<p>A list of <code>ResourceTagSet</code>s containing tags associated with the specified resources.</p>"]
pub resource_tag_sets: ResourceTagSetList,
            }
            
struct ListTagsForResourcesResponseDeserializer;
            impl ListTagsForResourcesResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTagsForResourcesResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTagsForResourcesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ResourceTagSets" => {
                obj.resource_tag_sets = try!(ResourceTagSetListDeserializer::deserialize("ResourceTagSets", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the information about the request to list the traffic policies that are associated with the current AWS account.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPoliciesRequest {
                #[doc="<p>(Optional) The maximum number of traffic policies to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policies, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the value of the <code>TrafficPolicyIdMarker</code> element is the ID of the first traffic policy in the next group of <code>MaxItems</code> traffic policies.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>(Conditional) For your first request to <code>ListTrafficPolicies</code>, do not include the <code>TrafficPolicyIdMarker</code> parameter.</p> <p>If you have more traffic policies than the value of <code>MaxItems</code>, <code>ListTrafficPolicies</code> returns only the first <code>MaxItems</code> traffic policies. To get the next group of <code>MaxItems</code> policies, submit another request to <code>ListTrafficPolicies</code>. For the value of <code>TrafficPolicyIdMarker</code>, specify the value of the <code>TrafficPolicyIdMarker</code> element that was returned in the previous response.</p> <p>Policies are listed in the order in which they were created.</p>"]
pub traffic_policy_id_marker: Option<TrafficPolicyId>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPoliciesResponse {
                #[doc="<p>A flag that indicates whether there are more traffic policies to be listed. If the response was truncated, you can get the next group of <code>MaxItems</code> traffic policies by calling <code>ListTrafficPolicies</code> again and specifying the value of the <code>TrafficPolicyIdMarker</code> element in the <code>TrafficPolicyIdMarker</code> request parameter.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicies</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If the value of <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy in the next group of <code>MaxItems</code> traffic policies.</p>"]
pub traffic_policy_id_marker: TrafficPolicyId,
#[doc="<p>A list that contains one <code>TrafficPolicySummary</code> element for each traffic policy that was created by the current AWS account.</p>"]
pub traffic_policy_summaries: TrafficPolicySummaries,
            }
            
struct ListTrafficPoliciesResponseDeserializer;
            impl ListTrafficPoliciesResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTrafficPoliciesResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPoliciesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"TrafficPolicyIdMarker" => {
                obj.traffic_policy_id_marker = try!(TrafficPolicyIdDeserializer::deserialize("TrafficPolicyIdMarker", stack));
            }
"TrafficPolicySummaries" => {
                obj.traffic_policy_summaries = try!(TrafficPolicySummariesDeserializer::deserialize("TrafficPolicySummaries", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A request for the traffic policy instances that you created in a specified hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesByHostedZoneRequest {
                #[doc="<p>The ID of the hosted zone for which you want to list traffic policy instances.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstancesByHostedZone</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get for this hosted zone.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, omit this value.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstancesByHostedZone</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get for this hosted zone.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesByHostedZoneResponse {
                #[doc="<p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of <code>MaxItems</code> traffic policy instances by calling <code>ListTrafficPolicyInstancesByHostedZone</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> elements in the corresponding request parameters.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstancesByHostedZone</code> that produced the current response. </p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>If <code>IsTruncated</code> is true, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
#[doc="<p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request. </p>"]
pub traffic_policy_instances: TrafficPolicyInstances,
            }
            
struct ListTrafficPolicyInstancesByHostedZoneResponseDeserializer;
            impl ListTrafficPolicyInstancesByHostedZoneResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTrafficPolicyInstancesByHostedZoneResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesByHostedZoneResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"TrafficPolicyInstanceNameMarker" => {
                obj.traffic_policy_instance_name_marker = Some(try!(DNSNameDeserializer::deserialize("TrafficPolicyInstanceNameMarker", stack)));
            }
"TrafficPolicyInstanceTypeMarker" => {
                obj.traffic_policy_instance_type_marker = Some(try!(RRTypeDeserializer::deserialize("TrafficPolicyInstanceTypeMarker", stack)));
            }
"TrafficPolicyInstances" => {
                obj.traffic_policy_instances = try!(TrafficPolicyInstancesDeserializer::deserialize("TrafficPolicyInstances", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the information about the request to list your traffic policy instances.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesByPolicyRequest {
                #[doc="<p>For the first request to <code>ListTrafficPolicyInstancesByPolicy</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone for the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get for this hosted zone.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, omit this value.</p>"]
pub hosted_zone_id_marker: Option<ResourceId>,
#[doc="<p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>The ID of the traffic policy for which you want to list traffic policy instances.</p>"]
pub traffic_policy_id: TrafficPolicyId,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstancesByPolicy</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get for this hosted zone.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, omit this value.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstancesByPolicy</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get for this hosted zone.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
#[doc="<p>The version of the traffic policy for which you want to list traffic policy instances. The version must be associated with the traffic policy that is specified by <code>TrafficPolicyId</code>.</p>"]
pub traffic_policy_version: TrafficPolicyVersion,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesByPolicyResponse {
                #[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub hosted_zone_id_marker: Option<ResourceId>,
#[doc="<p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of <code>MaxItems</code> traffic policy instances by calling <code>ListTrafficPolicyInstancesByPolicy</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> elements in the corresponding request parameters.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstancesByPolicy</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
#[doc="<p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request.</p>"]
pub traffic_policy_instances: TrafficPolicyInstances,
            }
            
struct ListTrafficPolicyInstancesByPolicyResponseDeserializer;
            impl ListTrafficPolicyInstancesByPolicyResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTrafficPolicyInstancesByPolicyResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesByPolicyResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneIdMarker" => {
                obj.hosted_zone_id_marker = Some(try!(ResourceIdDeserializer::deserialize("HostedZoneIdMarker", stack)));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"TrafficPolicyInstanceNameMarker" => {
                obj.traffic_policy_instance_name_marker = Some(try!(DNSNameDeserializer::deserialize("TrafficPolicyInstanceNameMarker", stack)));
            }
"TrafficPolicyInstanceTypeMarker" => {
                obj.traffic_policy_instance_type_marker = Some(try!(RRTypeDeserializer::deserialize("TrafficPolicyInstanceTypeMarker", stack)));
            }
"TrafficPolicyInstances" => {
                obj.traffic_policy_instances = try!(TrafficPolicyInstancesDeserializer::deserialize("TrafficPolicyInstances", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the information about the request to list your traffic policy instances.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesRequest {
                #[doc="<p>For the first request to <code>ListTrafficPolicyInstances</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, you have more traffic policy instances. To get the next group of <code>MaxItems</code> traffic policy instances, submit another <code>ListTrafficPolicyInstances</code> request. For the value of <code>HostedZoneIdMarker</code>, specify the value of <code>HostedZoneIdMarker</code> from the previous response, which is the hosted zone ID of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>"]
pub hosted_zone_id_marker: Option<ResourceId>,
#[doc="<p>The maximum number of traffic policy instances to be included in the response body for this request. If you have more than <code>MaxItems</code> traffic policy instances, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstances</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>For the first request to <code>ListTrafficPolicyInstances</code>, omit this value.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p> <p>If the value of <code>IsTruncated</code> in the previous response was <code>false</code>, there are no more traffic policy instances to get.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyInstancesResponse {
                #[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>HostedZoneIdMarker</code> is the ID of the hosted zone of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub hosted_zone_id_marker: Option<ResourceId>,
#[doc="<p>A flag that indicates whether there are more traffic policy instances to be listed. If the response was truncated, you can get the next group of <code>MaxItems</code> traffic policy instances by calling <code>ListTrafficPolicyInstances</code> again and specifying the values of the <code>HostedZoneIdMarker</code>, <code>TrafficPolicyInstanceNameMarker</code>, and <code>TrafficPolicyInstanceTypeMarker</code> elements in the corresponding request parameters.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>MaxItems</code> parameter in the call to <code>ListTrafficPolicyInstances</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceNameMarker</code> is the name of the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_name_marker: Option<DNSName>,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyInstanceTypeMarker</code> is the DNS type of the resource record sets that are associated with the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances.</p>"]
pub traffic_policy_instance_type_marker: Option<RRType>,
#[doc="<p>A list that contains one <code>TrafficPolicyInstance</code> element for each traffic policy instance that matches the elements in the request.</p>"]
pub traffic_policy_instances: TrafficPolicyInstances,
            }
            
struct ListTrafficPolicyInstancesResponseDeserializer;
            impl ListTrafficPolicyInstancesResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTrafficPolicyInstancesResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyInstancesResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneIdMarker" => {
                obj.hosted_zone_id_marker = Some(try!(ResourceIdDeserializer::deserialize("HostedZoneIdMarker", stack)));
            }
"IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"TrafficPolicyInstanceNameMarker" => {
                obj.traffic_policy_instance_name_marker = Some(try!(DNSNameDeserializer::deserialize("TrafficPolicyInstanceNameMarker", stack)));
            }
"TrafficPolicyInstanceTypeMarker" => {
                obj.traffic_policy_instance_type_marker = Some(try!(RRTypeDeserializer::deserialize("TrafficPolicyInstanceTypeMarker", stack)));
            }
"TrafficPolicyInstances" => {
                obj.traffic_policy_instances = try!(TrafficPolicyInstancesDeserializer::deserialize("TrafficPolicyInstances", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the information about the request to list your traffic policies.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyVersionsRequest {
                #[doc="<p>Specify the value of <code>Id</code> of the traffic policy for which you want to list all versions.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The maximum number of traffic policy versions that you want Amazon Route 53 to include in the response body for this request. If the specified traffic policy has more than <code>MaxItems</code> versions, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the value of the <code>TrafficPolicyVersionMarker</code> element is the ID of the first version in the next group of <code>MaxItems</code> traffic policy versions.</p>"]
pub max_items: Option<PageMaxItems>,
#[doc="<p>For your first request to <code>ListTrafficPolicyVersions</code>, do not include the <code>TrafficPolicyVersionMarker</code> parameter.</p> <p>If you have more traffic policy versions than the value of <code>MaxItems</code>, <code>ListTrafficPolicyVersions</code> returns only the first group of <code>MaxItems</code> versions. To get the next group of <code>MaxItems</code> traffic policy versions, submit another request to <code>ListTrafficPolicyVersions</code>. For the value of <code>TrafficPolicyVersionMarker</code>, specify the value of the <code>TrafficPolicyVersionMarker</code> element that was returned in the previous response.</p> <p>Traffic policy versions are listed in sequential order.</p>"]
pub traffic_policy_version_marker: Option<TrafficPolicyVersionMarker>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListTrafficPolicyVersionsResponse {
                #[doc="<p>A flag that indicates whether there are more traffic policies to be listed. If the response was truncated, you can get the next group of <code>maxitems</code> traffic policies by calling <code>ListTrafficPolicyVersions</code> again and specifying the value of the <code>NextMarker</code> element in the <code>marker</code> parameter.</p>"]
pub is_truncated: PageTruncated,
#[doc="<p>The value that you specified for the <code>maxitems</code> parameter in the call to <code>ListTrafficPolicyVersions</code> that produced the current response.</p>"]
pub max_items: PageMaxItems,
#[doc="<p>A list that contains one <code>TrafficPolicy</code> element for each traffic policy version that is associated with the specified traffic policy.</p>"]
pub traffic_policies: TrafficPolicies,
#[doc="<p>If <code>IsTruncated</code> is <code>true</code>, the value of <code>TrafficPolicyVersionMarker</code> identifies the first traffic policy in the next group of <code>MaxItems</code> traffic policies. Call <code>ListTrafficPolicyVersions</code> again and specify the value of <code>TrafficPolicyVersionMarker</code> in the <code>TrafficPolicyVersionMarker</code> request parameter.</p> <p>This element is present only if <code>IsTruncated</code> is <code>true</code>.</p>"]
pub traffic_policy_version_marker: TrafficPolicyVersionMarker,
            }
            
struct ListTrafficPolicyVersionsResponseDeserializer;
            impl ListTrafficPolicyVersionsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListTrafficPolicyVersionsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListTrafficPolicyVersionsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "IsTruncated" => {
                obj.is_truncated = try!(PageTruncatedDeserializer::deserialize("IsTruncated", stack));
            }
"MaxItems" => {
                obj.max_items = try!(PageMaxItemsDeserializer::deserialize("MaxItems", stack));
            }
"TrafficPolicies" => {
                obj.traffic_policies = try!(TrafficPoliciesDeserializer::deserialize("TrafficPolicies", stack));
            }
"TrafficPolicyVersionMarker" => {
                obj.traffic_policy_version_marker = try!(TrafficPolicyVersionMarkerDeserializer::deserialize("TrafficPolicyVersionMarker", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about that can be associated with your hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListVPCAssociationAuthorizationsRequest {
                #[doc="<p>The ID of the hosted zone for which you want a list of VPCs that can be associated with the hosted zone.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p> <i>Optional</i>: An integer that specifies the maximum number of VPCs that you want Amazon Route 53 to return.</p>"]
pub max_results: Option<MaxResults>,
#[doc="<p> <i>Optional</i>: If a response includes a <code>NextToken</code> element, there are more VPCs that can be associated with the specified hosted zone. To get the next page of results, submit another request, and include the value of the <code>NextToken</code> element in from the response in the <code>NextToken</code> parameter in another <code>ListVPCAssociationAuthorizations</code> request.</p>"]
pub next_token: Option<PaginationToken>,
            }
            
#[doc="<p>A complex type that contains the response information for the request.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ListVPCAssociationAuthorizationsResponse {
                #[doc="<p>The ID of the hosted zone that you can associate the listed VPCs with.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>When the response includes a <code>NextToken</code> element, there are more VPCs that can be associated with the specified hosted zone. To get the next page of VPCs, submit another <code>ListVPCAssociationAuthorizations</code> request, and include the value of the <code>NextToken</code> element from the response in the <code>NextToken</code> request parameter:</p> <p> <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/authorizevpcassociation?MaxItems=<i>VPCs per page</i>&amp;NextToken=<i/> </code> </p>"]
pub next_token: Option<PaginationToken>,
#[doc="<p>The list of VPCs that are authorized to be associated with the specified hosted zone.</p>"]
pub vp_cs: VPCs,
            }
            
struct ListVPCAssociationAuthorizationsResponseDeserializer;
            impl ListVPCAssociationAuthorizationsResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ListVPCAssociationAuthorizationsResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ListVPCAssociationAuthorizationsResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneId" => {
                obj.hosted_zone_id = try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
            }
"NextToken" => {
                obj.next_token = Some(try!(PaginationTokenDeserializer::deserialize("NextToken", stack)));
            }
"VPCs" => {
                obj.vp_cs = try!(VPCsDeserializer::deserialize("VPCs", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type MaxResults = String;

                pub struct MaxResultsSerializer;
                impl MaxResultsSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &MaxResults) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type MeasureLatency = bool;
struct MeasureLatencyDeserializer;
            impl MeasureLatencyDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<MeasureLatency, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct MeasureLatencySerializer;
                impl MeasureLatencySerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &MeasureLatency) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type Message = String;
struct MessageDeserializer;
            impl MessageDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Message, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type MetricName = String;
struct MetricNameDeserializer;
            impl MetricNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<MetricName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Nameserver = String;
struct NameserverDeserializer;
            impl NameserverDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Nameserver, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Namespace = String;
struct NamespaceDeserializer;
            impl NamespaceDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Namespace, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Nonce = String;
struct NonceDeserializer;
            impl NonceDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Nonce, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct NonceSerializer;
                impl NonceSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Nonce) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type PageMarker = String;
struct PageMarkerDeserializer;
            impl PageMarkerDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<PageMarker, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct PageMarkerSerializer;
                impl PageMarkerSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &PageMarker) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type PageMaxItems = String;
struct PageMaxItemsDeserializer;
            impl PageMaxItemsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<PageMaxItems, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct PageMaxItemsSerializer;
                impl PageMaxItemsSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &PageMaxItems) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type PageTruncated = bool;
struct PageTruncatedDeserializer;
            impl PageTruncatedDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<PageTruncated, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type PaginationToken = String;
struct PaginationTokenDeserializer;
            impl PaginationTokenDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<PaginationToken, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct PaginationTokenSerializer;
                impl PaginationTokenSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &PaginationToken) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type Period = i64;
struct PeriodDeserializer;
            impl PeriodDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Period, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Port = i64;
struct PortDeserializer;
            impl PortDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Port, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct PortSerializer;
                impl PortSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Port) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type RData = String;
struct RDataDeserializer;
            impl RDataDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<RData, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct RDataSerializer;
                impl RDataSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &RData) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type RRType = String;
struct RRTypeDeserializer;
            impl RRTypeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<RRType, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct RRTypeSerializer;
                impl RRTypeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &RRType) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type RecordData = Vec<RecordDataEntry>;
struct RecordDataDeserializer;
            impl RecordDataDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<RecordData, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "RecordDataEntry" {
                        obj.push(try!(RecordDataEntryDeserializer::deserialize("RecordDataEntry", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
#[doc="<p>A value that Amazon Route 53 returned for this resource record set. A RecordDataEntry element is one of the following:</p> <ul> <li> <p>For non-alias resource record sets, a <code>RecordDataEntry</code> element contains one value in the resource record set. If the resource record set contains multiple values, the response includes one <code>RecordDataEntry</code> element for each value.</p> </li> <li> <p>For multiple resource record sets that have the same name and type, which includes weighted, latency, geolocation, and failover, a <code>RecordDataEntry</code> element contains the value from the appropriate resource record set based on the request.</p> </li> <li> <p>For alias resource record sets that refer to AWS resources other than another resource record set, the <code>RecordDataEntry</code> element contains an IP address or a domain name for the AWS resource, depending on the type of resource.</p> </li> <li> <p>For alias resource record sets that refer to other resource record sets, a <code>RecordDataEntry</code> element contains one value from the referenced resource record set. If the referenced resource record set contains multiple values, the response includes one <code>RecordDataEntry</code> element for each value.</p> </li> </ul>"]
pub type RecordDataEntry = String;
struct RecordDataEntryDeserializer;
            impl RecordDataEntryDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<RecordDataEntry, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type RequestInterval = i64;
struct RequestIntervalDeserializer;
            impl RequestIntervalDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<RequestInterval, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct RequestIntervalSerializer;
                impl RequestIntervalSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &RequestInterval) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceDescription = String;
struct ResourceDescriptionDeserializer;
            impl ResourceDescriptionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceDescription, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceDescriptionSerializer;
                impl ResourceDescriptionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceDescription) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceId = String;
struct ResourceIdDeserializer;
            impl ResourceIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceIdSerializer;
                impl ResourceIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourcePath = String;
struct ResourcePathDeserializer;
            impl ResourcePathDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourcePath, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourcePathSerializer;
                impl ResourcePathSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourcePath) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>Information specific to the resource record.</p> <note> <p>If you are creating an alias resource record set, omit <code>ResourceRecord</code>.</p> </note>"]
#[derive(Default,Clone,Debug)]
            pub struct ResourceRecord {
                #[doc="<p>The current or new DNS record value, not to exceed 4,000 characters. In the case of a <code>DELETE</code> action, if the current value does not match the actual value, an error is returned. For descriptions about how to format <code>Value</code> for different record types, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html\">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can specify more than one value for all record types except <code>CNAME</code> and <code>SOA</code>. </p> <note> <p>If you are creating an alias resource record set, omit <code>Value</code>.</p> </note>"]
pub value: RData,
            }
            
struct ResourceRecordDeserializer;
            impl ResourceRecordDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecord, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ResourceRecord::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Value" => {
                obj.value = try!(RDataDeserializer::deserialize("Value", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSerializer;
                impl ResourceRecordSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecord) -> String {
                        let mut serialized = format!("<{name}>", name=name);serialized += &format!("<Value>{value}</Value>", value=obj.value);serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
#[doc="<p>Information about the resource record set to create or delete.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ResourceRecordSet {
                #[doc="<p> <i>Alias resource record sets only:</i> Information about the CloudFront distribution, AWS Elastic Beanstalk environment, ELB load balancer, Amazon S3 bucket, or Amazon Route 53 resource record set to which you are redirecting queries. The AWS Elastic Beanstalk environment must have a regionalized subdomain.</p> <p>If you're creating resource records sets for a private hosted zone, note the following:</p> <ul> <li> <p>You can't create alias resource record sets for CloudFront distributions in a private hosted zone.</p> </li> <li> <p>Creating geolocation alias resource record sets or latency alias resource record sets in a private hosted zone is unsupported.</p> </li> <li> <p>For information about creating failover resource record sets in a private hosted zone, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html\">Configuring Failover in a Private Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul>"]
pub alias_target: Option<AliasTarget>,
#[doc="<p> <i>Failover resource record sets only:</i> To configure failover, you add the <code>Failover</code> element to two resource record sets. For one resource record set, you specify <code>PRIMARY</code> as the value for <code>Failover</code>; for the other resource record set, you specify <code>SECONDARY</code>. In addition, you include the <code>HealthCheckId</code> element and specify the health check that you want Amazon Route 53 to perform for each resource record set.</p> <p>Except where noted, the following failover behaviors assume that you have included the <code>HealthCheckId</code> element in both resource record sets:</p> <ul> <li> <p>When the primary resource record set is healthy, Amazon Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the secondary resource record set.</p> </li> <li> <p>When the primary resource record set is unhealthy and the secondary resource record set is healthy, Amazon Route 53 responds to DNS queries with the applicable value from the secondary resource record set.</p> </li> <li> <p>When the secondary resource record set is unhealthy, Amazon Route 53 responds to DNS queries with the applicable value from the primary resource record set regardless of the health of the primary resource record set.</p> </li> <li> <p>If you omit the <code>HealthCheckId</code> element for the secondary resource record set, and if the primary resource record set is unhealthy, Amazon Route 53 always responds to DNS queries with the applicable value from the secondary resource record set. This is true regardless of the health of the associated endpoint.</p> </li> </ul> <p>You can't create non-failover resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as failover resource record sets.</p> <p>For failover alias resource record sets, you must also include the <code>EvaluateTargetHealth</code> element and set the value to true.</p> <p>For more information about configuring failover for Amazon Route 53, see the following topics in the <i>Amazon Route 53 Developer Guide</i>: </p> <ul> <li> <p> <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html\">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul> <p>Valid values: <code>PRIMARY</code> | <code>SECONDARY</code> </p>"]
pub failover: Option<ResourceRecordSetFailover>,
#[doc="<p> <i>Geo location resource record sets only:</i> A complex type that lets you control how Amazon Route 53 responds to DNS queries based on the geographic origin of the query. For example, if you want all queries from Africa to be routed to a web server with an IP address of <code>192.0.2.111</code>, create a resource record set with a <code>Type</code> of <code>A</code> and a <code>ContinentCode</code> of <code>AF</code>.</p> <note> <p>Creating geolocation and geolocation alias resource record sets in private hosted zones is not supported.</p> </note> <p>If you create separate resource record sets for overlapping geographic regions (for example, one resource record set for a continent and one for a country on the same continent), priority goes to the smallest geographic region. This allows you to route most queries for a continent to one resource and to route queries for a country on that continent to a different resource.</p> <p>You can't create two geolocation resource record sets that specify the same geographic location.</p> <p>The value <code>*</code> in the <code>CountryCode</code> element matches all geographic locations that aren't specified in other geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> <important> <p>Geolocation works by mapping IP addresses to locations. However, some IP addresses aren't mapped to geographic locations, so even if you create geolocation resource record sets that cover all seven continents, Amazon Route 53 will receive some DNS queries from locations that it can't identify. We recommend that you create a resource record set for which the value of <code>CountryCode</code> is <code>*</code>, which handles both queries that come from locations for which you haven't created geolocation resource record sets and queries from IP addresses that aren't mapped to a location. If you don't create a <code>*</code> resource record set, Amazon Route 53 returns a \"no answer\" response for queries from those locations.</p> </important> <p>You can't create non-geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as geolocation resource record sets.</p>"]
pub geo_location: Option<GeoLocation>,
#[doc="<p>If you want Amazon Route 53 to return this resource record set in response to a DNS query only when a health check is passing, include the <code>HealthCheckId</code> element and specify the ID of the applicable health check.</p> <p>Amazon Route 53 determines whether a resource record set is healthy based on one of the following:</p> <ul> <li> <p>By periodically sending a request to the endpoint that is specified in the health check</p> </li> <li> <p>By aggregating the status of a specified group of health checks (calculated health checks)</p> </li> <li> <p>By determining the current state of a CloudWatch alarm (CloudWatch metric health checks)</p> </li> </ul> <p>For information about how Amazon Route 53 determines whether a health check is healthy, see <a>CreateHealthCheck</a>.</p> <p>The <code>HealthCheckId</code> element is only useful when Amazon Route 53 is choosing between two or more resource record sets to respond to a DNS query, and you want Amazon Route 53 to base the choice in part on the status of a health check. Configuring health checks only makes sense in the following configurations:</p> <ul> <li> <p>You're checking the health of the resource record sets in a weighted, latency, geolocation, or failover resource record set, and you specify health check IDs for all of the resource record sets. If the health check for one resource record set specifies an endpoint that is not healthy, Amazon Route 53 stops responding to queries using the value for that resource record set.</p> </li> <li> <p>You set <code>EvaluateTargetHealth</code> to true for the resource record sets in an alias, weighted alias, latency alias, geolocation alias, or failover alias resource record set, and you specify health check IDs for all of the resource record sets that are referenced by the alias resource record sets.</p> </li> </ul> <important> <p>Amazon Route 53 doesn't check the health of the endpoint specified in the resource record set, for example, the endpoint specified by the IP address in the <code>Value</code> element. When you add a <code>HealthCheckId</code> element to a resource record set, Amazon Route 53 checks the health of the endpoint that you specified in the health check. </p> </important> <p>For geolocation resource record sets, if an endpoint is unhealthy, Amazon Route 53 looks for a resource record set for the larger, associated geographic region. For example, suppose you have resource record sets for a state in the United States, for the United States, for North America, and for all locations. If the endpoint for the state resource record set is unhealthy, Amazon Route 53 checks the resource record sets for the United States, for North America, and for all locations (a resource record set for which the value of <code>CountryCode</code> is <code>*</code>), in that order, until it finds a resource record set for which the endpoint is healthy. </p> <p>If your health checks specify the endpoint only by domain name, we recommend that you create a separate health check for each endpoint. For example, create a health check for each <code>HTTP</code> server that is serving content for <code>www.example.com</code>. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-1-www.example.com</code>), not the name of the resource record sets (example.com).</p> <important> <p>n this configuration, if you create a health check for which the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>For more information, see the following topics in the <i>Amazon Route 53 Developer Guide</i>:</p> <ul> <li> <p> <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a> </p> </li> <li> <p> <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-private-hosted-zones.html\">Configuring Failover in a Private Hosted Zone</a> </p> </li> </ul>"]
pub health_check_id: Option<HealthCheckId>,
#[doc="<p>The name of the domain you want to perform the action on.</p> <p>Enter a fully qualified domain name, for example, <code>www.example.com</code>. You can optionally include a trailing dot. If you omit the trailing dot, Amazon Route 53 still assumes that the domain name that you specify is fully qualified. This means that Amazon Route 53 treats <code>www.example.com</code> (without a trailing dot) and <code>www.example.com.</code> (with a trailing dot) as identical.</p> <p>For information about how to specify characters other than <code>a-z</code>, <code>0-9</code>, and <code>-</code> (hyphen) and how to specify internationalized domain names, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html\">DNS Domain Name Format</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>You can use the asterisk (*) wildcard to replace the leftmost label in a domain name. For example, <code>*.example.com</code>. Note the following:</p> <ul> <li> <p>The * must replace the entire label. For example, you can't specify <code>*prod.example.com</code> or <code>prod*.example.com</code>.</p> </li> <li> <p>The * can't replace any of the middle labels, for example, marketing.*.example.com.</p> </li> <li> <p>If you include * in any position other than the leftmost label in a domain name, DNS treats it as an * character (ASCII 42), not as a wildcard.</p> <important> <p>You can't use the * wildcard for resource records sets that have a type of NS.</p> </important> </li> </ul> <p>You can use the * wildcard as the leftmost label in a domain name, for example, <code>*.example.com</code>. You can't use an * for one of the middle labels, for example, <code>marketing.*.example.com</code>. In addition, the * must replace the entire label; for example, you can't specify <code>prod*.example.com</code>.</p>"]
pub name: DNSName,
#[doc="<p> <i>Latency-based resource record sets only:</i> The Amazon EC2 Region where the resource that is specified in this resource record set resides. The resource typically is an AWS resource, such as an EC2 instance or an ELB load balancer, and is referred to by an IP address or a DNS domain name, depending on the record type.</p> <note> <p>Creating latency and latency alias resource record sets in private hosted zones is not supported.</p> </note> <p>When Amazon Route 53 receives a DNS query for a domain name and type for which you have created latency resource record sets, Amazon Route 53 selects the latency resource record set that has the lowest latency between the end user and the associated Amazon EC2 Region. Amazon Route 53 then returns the value that is associated with the selected resource record set.</p> <p>Note the following:</p> <ul> <li> <p>You can only specify one <code>ResourceRecord</code> per latency resource record set.</p> </li> <li> <p>You can only create one latency resource record set for each Amazon EC2 Region.</p> </li> <li> <p>You are not required to create latency resource record sets for all Amazon EC2 Regions. Amazon Route 53 will choose the region with the best latency from among the regions for which you create latency resource record sets.</p> </li> <li> <p>You can't create non-latency resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as latency resource record sets.</p> </li> </ul>"]
pub region: Option<ResourceRecordSetRegion>,
#[doc="<p>Information about the resource records to act upon.</p> <note> <p>If you are creating an alias resource record set, omit <code>ResourceRecords</code>.</p> </note>"]
pub resource_records: Option<ResourceRecords>,
#[doc="<p> <i>Weighted, Latency, Geo, and Failover resource record sets only:</i> An identifier that differentiates among multiple resource record sets that have the same combination of DNS name and type. The value of <code>SetIdentifier</code> must be unique for each resource record set that has the same combination of DNS name and type. Omit <code>SetIdentifier</code> for any other types of record sets.</p>"]
pub set_identifier: Option<ResourceRecordSetIdentifier>,
#[doc="<p>The resource record cache time to live (TTL), in seconds. Note the following:</p> <ul> <li> <p>If you're creating an alias resource record set, omit <code>TTL</code>. Amazon Route 53 uses the value of <code>TTL</code> for the alias target. </p> </li> <li> <p>If you're associating this resource record set with a health check (if you're adding a <code>HealthCheckId</code> element), we recommend that you specify a <code>TTL</code> of 60 seconds or less so clients respond quickly to changes in health status.</p> </li> <li> <p>All of the resource record sets in a group of weighted, latency, geolocation, or failover resource record sets must have the same value for <code>TTL</code>.</p> </li> <li> <p>If a group of weighted resource record sets includes one or more weighted alias resource record sets for which the alias target is an ELB load balancer, we recommend that you specify a <code>TTL</code> of 60 seconds for all of the non-alias weighted resource record sets that have the same name and type. Values other than 60 seconds (the TTL for load balancers) will change the effect of the values that you specify for <code>Weight</code>.</p> </li> </ul>"]
pub ttl: Option<TTL>,
#[doc="<p>When you create a traffic policy instance, Amazon Route 53 automatically creates a resource record set. <code>TrafficPolicyInstanceId</code> is the ID of the traffic policy instance that Amazon Route 53 created this resource record set for.</p> <important> <p>To delete the resource record set that is associated with a traffic policy instance, use <code>DeleteTrafficPolicyInstance</code>. Amazon Route 53 will delete the resource record set automatically. If you delete the resource record set by using <code>ChangeResourceRecordSets</code>, Amazon Route 53 doesn't automatically delete the traffic policy instance, and you'll continue to be charged for it even though it's no longer in use. </p> </important>"]
pub traffic_policy_instance_id: Option<TrafficPolicyInstanceId>,
#[doc="<p>The DNS record type. For information about different record types and how data is encoded for them, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/ResourceRecordTypes.html\">Supported DNS Resource Record Types</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code> </p> <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code>. When creating a group of weighted, latency, geolocation, or failover resource record sets, specify the same value for all of the resource record sets in the group.</p> <note> <p>SPF records were formerly used to verify the identity of the sender of email messages. However, we no longer recommend that you create resource record sets for which the value of <code>Type</code> is <code>SPF</code>. RFC 7208, <i>Sender Policy Framework (SPF) for Authorizing Use of Domains in Email, Version 1</i>, has been updated to say, \"...[I]ts existence and mechanism defined in [RFC4408] have led to some interoperability issues. Accordingly, its use is no longer appropriate for SPF version 1; implementations are not to use it.\" In RFC 7208, see section 14.1, <a href=\"http://tools.ietf.org/html/rfc7208#section-14.1\">The SPF DNS Record Type</a>.</p> </note> <p>Values for alias resource record sets:</p> <ul> <li> <p> <b>CloudFront distributions:</b> <code>A</code> </p> <p>If IPv6 is enabled for the distribution, create two resource record sets to route traffic to your distribution, one with a value of <code>A</code> and one with a value of <code>AAAA</code>. </p> </li> <li> <p> <b>AWS Elastic Beanstalk environment that has a regionalized subdomain</b>: <code>A</code> </p> </li> <li> <p> <b>ELB load balancers:</b> <code>A</code> | <code>AAAA</code> </p> </li> <li> <p> <b>Amazon S3 buckets:</b> <code>A</code> </p> </li> <li> <p> <b>Another resource record set in this hosted zone:</b> Specify the type of the resource record set for which you're creating the alias. Specify any value except <code>NS</code> or <code>SOA</code>.</p> </li> </ul>"]
pub type_: RRType,
#[doc="<p> <i>Weighted resource record sets only:</i> Among resource record sets that have the same combination of DNS name and type, a value that determines the proportion of DNS queries that Amazon Route 53 responds to using the current resource record set. Amazon Route 53 calculates the sum of the weights for the resource record sets that have the same combination of DNS name and type. Amazon Route 53 then responds to queries based on the ratio of a resource's weight to the total. Note the following:</p> <ul> <li> <p>You must specify a value for the <code>Weight</code> element for every weighted resource record set.</p> </li> <li> <p>You can only specify one <code>ResourceRecord</code> per weighted resource record set.</p> </li> <li> <p>You can't create latency, failover, or geolocation resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements as weighted resource record sets.</p> </li> <li> <p>You can create a maximum of 100 weighted resource record sets that have the same values for the <code>Name</code> and <code>Type</code> elements.</p> </li> <li> <p>For weighted (but not weighted alias) resource record sets, if you set <code>Weight</code> to <code>0</code> for a resource record set, Amazon Route 53 never responds to queries with the applicable value for that resource record set. However, if you set <code>Weight</code> to <code>0</code> for all resource record sets that have the same combination of DNS name and type, traffic is routed to all resources with equal probability.</p> <p>The effect of setting <code>Weight</code> to <code>0</code> is different when you associate health checks with weighted resource record sets. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-configuring-options.html\">Options for Configuring Amazon Route 53 Active-Active and Active-Passive Failover</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> </ul>"]
pub weight: Option<ResourceRecordSetWeight>,
            }
            
struct ResourceRecordSetDeserializer;
            impl ResourceRecordSetDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSet, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ResourceRecordSet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AliasTarget" => {
                obj.alias_target = Some(try!(AliasTargetDeserializer::deserialize("AliasTarget", stack)));
            }
"Failover" => {
                obj.failover = Some(try!(ResourceRecordSetFailoverDeserializer::deserialize("Failover", stack)));
            }
"GeoLocation" => {
                obj.geo_location = Some(try!(GeoLocationDeserializer::deserialize("GeoLocation", stack)));
            }
"HealthCheckId" => {
                obj.health_check_id = Some(try!(HealthCheckIdDeserializer::deserialize("HealthCheckId", stack)));
            }
"Name" => {
                obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
            }
"Region" => {
                obj.region = Some(try!(ResourceRecordSetRegionDeserializer::deserialize("Region", stack)));
            }
"ResourceRecords" => {
                obj.resource_records = Some(try!(ResourceRecordsDeserializer::deserialize("ResourceRecords", stack)));
            }
"SetIdentifier" => {
                obj.set_identifier = Some(try!(ResourceRecordSetIdentifierDeserializer::deserialize("SetIdentifier", stack)));
            }
"TTL" => {
                obj.ttl = Some(try!(TTLDeserializer::deserialize("TTL", stack)));
            }
"TrafficPolicyInstanceId" => {
                obj.traffic_policy_instance_id = Some(try!(TrafficPolicyInstanceIdDeserializer::deserialize("TrafficPolicyInstanceId", stack)));
            }
"Type" => {
                obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
            }
"Weight" => {
                obj.weight = Some(try!(ResourceRecordSetWeightDeserializer::deserialize("Weight", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSetSerializer;
                impl ResourceRecordSetSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecordSet) -> String {
                        let mut serialized = format!("<{name}>", name=name);
            if let Some(ref value) = obj.alias_target {
                serialized += &AliasTargetSerializer::serialize("AliasTarget", value);
            }if let Some(ref value) = obj.failover {
                serialized += &format!("<Failover>{value}</Failover>", value=value);
            }
            if let Some(ref value) = obj.geo_location {
                serialized += &GeoLocationSerializer::serialize("GeoLocation", value);
            }if let Some(ref value) = obj.health_check_id {
                serialized += &format!("<HealthCheckId>{value}</HealthCheckId>", value=value);
            }serialized += &format!("<Name>{value}</Name>", value=obj.name);if let Some(ref value) = obj.region {
                serialized += &format!("<Region>{value}</Region>", value=value);
            }
            if let Some(ref value) = obj.resource_records {
                serialized += &ResourceRecordsSerializer::serialize("ResourceRecords", value);
            }if let Some(ref value) = obj.set_identifier {
                serialized += &format!("<SetIdentifier>{value}</SetIdentifier>", value=value);
            }if let Some(ref value) = obj.ttl {
                serialized += &format!("<TTL>{value}</TTL>", value=value);
            }if let Some(ref value) = obj.traffic_policy_instance_id {
                serialized += &format!("<TrafficPolicyInstanceId>{value}</TrafficPolicyInstanceId>", value=value);
            }serialized += &format!("<Type>{value}</Type>", value=obj.type_);if let Some(ref value) = obj.weight {
                serialized += &format!("<Weight>{value}</Weight>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type ResourceRecordSetFailover = String;
struct ResourceRecordSetFailoverDeserializer;
            impl ResourceRecordSetFailoverDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSetFailover, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSetFailoverSerializer;
                impl ResourceRecordSetFailoverSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecordSetFailover) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceRecordSetIdentifier = String;
struct ResourceRecordSetIdentifierDeserializer;
            impl ResourceRecordSetIdentifierDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSetIdentifier, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSetIdentifierSerializer;
                impl ResourceRecordSetIdentifierSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecordSetIdentifier) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceRecordSetRegion = String;
struct ResourceRecordSetRegionDeserializer;
            impl ResourceRecordSetRegionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSetRegion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSetRegionSerializer;
                impl ResourceRecordSetRegionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecordSetRegion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceRecordSetWeight = i64;
struct ResourceRecordSetWeightDeserializer;
            impl ResourceRecordSetWeightDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSetWeight, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordSetWeightSerializer;
                impl ResourceRecordSetWeightSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecordSetWeight) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type ResourceRecordSets = Vec<ResourceRecordSet>;
struct ResourceRecordSetsDeserializer;
            impl ResourceRecordSetsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecordSets, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ResourceRecordSet" {
                        obj.push(try!(ResourceRecordSetDeserializer::deserialize("ResourceRecordSet", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type ResourceRecords = Vec<ResourceRecord>;
struct ResourceRecordsDeserializer;
            impl ResourceRecordsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceRecords, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ResourceRecord" {
                        obj.push(try!(ResourceRecordDeserializer::deserialize("ResourceRecord", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }

                pub struct ResourceRecordsSerializer;
                impl ResourceRecordsSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &ResourceRecords) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(ResourceRecordSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
#[doc="<p>A complex type containing a resource and its associated tags.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct ResourceTagSet {
                #[doc="<p>The ID for the specified resource.</p>"]
pub resource_id: Option<TagResourceId>,
#[doc="<p>The type of the resource.</p> <ul> <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li> <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li> </ul>"]
pub resource_type: Option<TagResourceType>,
#[doc="<p>The tags associated with the specified resource.</p>"]
pub tags: Option<TagList>,
            }
            
struct ResourceTagSetDeserializer;
            impl ResourceTagSetDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceTagSet, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = ResourceTagSet::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ResourceId" => {
                obj.resource_id = Some(try!(TagResourceIdDeserializer::deserialize("ResourceId", stack)));
            }
"ResourceType" => {
                obj.resource_type = Some(try!(TagResourceTypeDeserializer::deserialize("ResourceType", stack)));
            }
"Tags" => {
                obj.tags = Some(try!(TagListDeserializer::deserialize("Tags", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type ResourceTagSetList = Vec<ResourceTagSet>;
struct ResourceTagSetListDeserializer;
            impl ResourceTagSetListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<ResourceTagSetList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "ResourceTagSet" {
                        obj.push(try!(ResourceTagSetDeserializer::deserialize("ResourceTagSet", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type ResourceURI = String;
pub type SearchString = String;
struct SearchStringDeserializer;
            impl SearchStringDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<SearchString, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct SearchStringSerializer;
                impl SearchStringSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &SearchString) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type Statistic = String;
struct StatisticDeserializer;
            impl StatisticDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Statistic, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Status = String;
struct StatusDeserializer;
            impl StatusDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Status, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the status that one Amazon Route 53 health checker reports and the time of the health check.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct StatusReport {
                #[doc="<p>The time at which the health checker performed the health check in <a href=\"https://en.wikipedia.org/wiki/ISO_8601\">ISO 8601 format</a> and Coordinated Universal Time (UTC). For example, the value <code>2014-10-27T17:48:16.751Z</code> represents October 27, 2014 at 17:48:16.751 UTC.</p>"]
pub checked_time: Option<TimeStamp>,
#[doc="<p>A description of the status of the health check endpoint as reported by one of the Amazon Route 53 health checkers.</p>"]
pub status: Option<Status>,
            }
            
struct StatusReportDeserializer;
            impl StatusReportDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<StatusReport, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = StatusReport::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CheckedTime" => {
                obj.checked_time = Some(try!(TimeStampDeserializer::deserialize("CheckedTime", stack)));
            }
"Status" => {
                obj.status = Some(try!(StatusDeserializer::deserialize("Status", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type SubnetMask = String;

                pub struct SubnetMaskSerializer;
                impl SubnetMaskSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &SubnetMask) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TTL = i64;
struct TTLDeserializer;
            impl TTLDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TTL, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TTLSerializer;
                impl TTLSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TTL) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains information about a tag that you want to add or edit for the specified health check or hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct Tag {
                #[doc="<p>The value of <code>Key</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Key</code> is the name that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Key</code> is the name of the tag whose <code>Value</code> element you want to remove.</p> </li> <li> <p> <b> Delete a key</b>: <code>Key</code> is the name of the tag you want to remove.</p> </li> <li> <p> <b>Give a name to a health check</b>: Edit the default <code>Name</code> tag. In the Amazon Route 53 console, the list of your health checks includes a <b>Name</b> column that lets you see the name that you've given to each health check.</p> </li> </ul>"]
pub key: Option<TagKey>,
#[doc="<p>The value of <code>Value</code> depends on the operation that you want to perform:</p> <ul> <li> <p> <b>Add a tag to a health check or hosted zone</b>: <code>Value</code> is the value that you want to give the new tag.</p> </li> <li> <p> <b>Edit a tag</b>: <code>Value</code> is the new value that you want to assign the tag.</p> </li> </ul>"]
pub value: Option<TagValue>,
            }
            
struct TagDeserializer;
            impl TagDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Tag, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = Tag::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Key" => {
                obj.key = Some(try!(TagKeyDeserializer::deserialize("Key", stack)));
            }
"Value" => {
                obj.value = Some(try!(TagValueDeserializer::deserialize("Value", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TagSerializer;
                impl TagSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &Tag) -> String {
                        let mut serialized = format!("<{name}>", name=name);if let Some(ref value) = obj.key {
                serialized += &format!("<Key>{value}</Key>", value=value);
            }if let Some(ref value) = obj.value {
                serialized += &format!("<Value>{value}</Value>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
pub type TagKey = String;
struct TagKeyDeserializer;
            impl TagKeyDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TagKey, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TagKeySerializer;
                impl TagKeySerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagKey) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TagKeyList = Vec<TagKey>;

                pub struct TagKeyListSerializer;
                impl TagKeyListSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagKeyList) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(TagKeySerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
pub type TagList = Vec<Tag>;
struct TagListDeserializer;
            impl TagListDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TagList, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "Tag" {
                        obj.push(try!(TagDeserializer::deserialize("Tag", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }

                pub struct TagListSerializer;
                impl TagListSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagList) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(TagSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
pub type TagResourceId = String;
struct TagResourceIdDeserializer;
            impl TagResourceIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TagResourceId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TagResourceIdSerializer;
                impl TagResourceIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagResourceId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TagResourceIdList = Vec<TagResourceId>;

                pub struct TagResourceIdListSerializer;
                impl TagResourceIdListSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagResourceIdList) -> String {
                        let mut parts: Vec<String> = Vec::new();parts.push(format!("<{}>", name));
        for element in obj {
            parts.push(TagResourceIdSerializer::serialize(name, element));
        }parts.push(format!("</{}>", name));parts.join("")
                    }
                }
                
pub type TagResourceType = String;
struct TagResourceTypeDeserializer;
            impl TagResourceTypeDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TagResourceType, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TagResourceTypeSerializer;
                impl TagResourceTypeSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagResourceType) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TagValue = String;
struct TagValueDeserializer;
            impl TagValueDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TagValue, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TagValueSerializer;
                impl TagValueSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TagValue) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p> <p> <b>Parameters</b> </p> <dl> <dt>hostedzoneid</dt> <dd> <p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p> </dd> <dt>recordname</dt> <dd> <p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p> </dd> <dt>recordtype</dt> <dd> <p>The type of the resource record set.</p> </dd> <dt>resolverip (optional)</dt> <dd> <p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDNSAnswer</code> uses the IP address of a DNS resolver in the AWS US East region. </p> </dd> <dt>edns0clientsubnetip (optional)</dt> <dd> <p>If the resolver that you specified for <code>resolverip</code> supports EDNS0, specify the IP address of a client in the applicable location. </p> </dd> <dt>edns0clientsubnetmask (optional)</dt> <dd> <p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from <code>192.0.2.0/24</code>. The default value is 24 bits. </p> </dd> </dl>"]
#[derive(Default,Clone,Debug)]
            pub struct TestDNSAnswerRequest {
                #[doc="<p>If the resolver that you specified for resolverip supports EDNS0, specify the IP address of a client in the applicable location.</p>"]
pub edns0_client_subnet_ip: Option<IPAddress>,
#[doc="<p>If you specify an IP address for <code>edns0clientsubnetip</code>, you can optionally specify the number of bits of the IP address that you want the checking tool to include in the DNS query. For example, if you specify <code>192.0.2.44</code> for <code>edns0clientsubnetip</code> and <code>24</code> for <code>edns0clientsubnetmask</code>, the checking tool will simulate a request from 192.0.2.0/24. The default value is 24 bits.</p>"]
pub edns0_client_subnet_mask: Option<SubnetMask>,
#[doc="<p>The ID of the hosted zone that you want Amazon Route 53 to simulate a query for.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>The name of the resource record set that you want Amazon Route 53 to simulate a query for.</p>"]
pub record_name: DNSName,
#[doc="<p>The type of the resource record set.</p>"]
pub record_type: RRType,
#[doc="<p>If you want to simulate a request from a specific DNS resolver, specify the IP address for that resolver. If you omit this value, <code>TestDnsAnswer</code> uses the IP address of a DNS resolver in the AWS US East region.</p>"]
pub resolver_ip: Option<IPAddress>,
            }
            
#[doc="<p>A complex type that contains the response to a <code>TestDNSAnswer</code> request. </p>"]
#[derive(Default,Clone,Debug)]
            pub struct TestDNSAnswerResponse {
                #[doc="<p>The Amazon Route 53 name server used to respond to the request.</p>"]
pub nameserver: Nameserver,
#[doc="<p>The protocol that Amazon Route 53 used to respond to the request, either <code>UDP</code> or <code>TCP</code>. </p>"]
pub protocol: TransportProtocol,
#[doc="<p>A list that contains values that Amazon Route 53 returned for this resource record set.</p>"]
pub record_data: RecordData,
#[doc="<p>The name of the resource record set that you submitted a request for.</p>"]
pub record_name: DNSName,
#[doc="<p>The type of the resource record set that you submitted a request for.</p>"]
pub record_type: RRType,
#[doc="<p>A code that indicates whether the request is valid or not. The most common response code is <code>NOERROR</code>, meaning that the request is valid. If the response is not valid, Amazon Route 53 returns a response code that describes the error. For a list of possible response codes, see <a href=\"http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6\">DNS RCODES</a> on the IANA website. </p>"]
pub response_code: DNSRCode,
            }
            
struct TestDNSAnswerResponseDeserializer;
            impl TestDNSAnswerResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TestDNSAnswerResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = TestDNSAnswerResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Nameserver" => {
                obj.nameserver = try!(NameserverDeserializer::deserialize("Nameserver", stack));
            }
"Protocol" => {
                obj.protocol = try!(TransportProtocolDeserializer::deserialize("Protocol", stack));
            }
"RecordData" => {
                obj.record_data = try!(RecordDataDeserializer::deserialize("RecordData", stack));
            }
"RecordName" => {
                obj.record_name = try!(DNSNameDeserializer::deserialize("RecordName", stack));
            }
"RecordType" => {
                obj.record_type = try!(RRTypeDeserializer::deserialize("RecordType", stack));
            }
"ResponseCode" => {
                obj.response_code = try!(DNSRCodeDeserializer::deserialize("ResponseCode", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type Threshold = f64;
struct ThresholdDeserializer;
            impl ThresholdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<Threshold, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TimeStamp = String;
struct TimeStampDeserializer;
            impl TimeStampDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TimeStamp, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicies = Vec<TrafficPolicy>;
struct TrafficPoliciesDeserializer;
            impl TrafficPoliciesDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicies, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "TrafficPolicy" {
                        obj.push(try!(TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains settings for a traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct TrafficPolicy {
                #[doc="<p>The comment that you specify in the <code>CreateTrafficPolicy</code> request, if any.</p>"]
pub comment: Option<TrafficPolicyComment>,
#[doc="<p>The definition of a traffic policy in JSON format. You specify the JSON document to use for a new traffic policy in the <code>CreateTrafficPolicy</code> request. For more information about the JSON format, see <a href=\"http://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html\">Traffic Policy Document Format</a>.</p>"]
pub document: TrafficPolicyDocument,
#[doc="<p>The ID that Amazon Route 53 assigned to a traffic policy when you created it.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The name that you specified when you created the traffic policy.</p>"]
pub name: TrafficPolicyName,
#[doc="<p>The DNS type of the resource record sets that Amazon Route 53 creates when you use a traffic policy to create a traffic policy instance.</p>"]
pub type_: RRType,
#[doc="<p>The version number that Amazon Route 53 assigns to a traffic policy. For a new traffic policy, the value of <code>Version</code> is always 1.</p>"]
pub version: TrafficPolicyVersion,
            }
            
struct TrafficPolicyDeserializer;
            impl TrafficPolicyDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicy, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicy::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Comment" => {
                obj.comment = Some(try!(TrafficPolicyCommentDeserializer::deserialize("Comment", stack)));
            }
"Document" => {
                obj.document = try!(TrafficPolicyDocumentDeserializer::deserialize("Document", stack));
            }
"Id" => {
                obj.id = try!(TrafficPolicyIdDeserializer::deserialize("Id", stack));
            }
"Name" => {
                obj.name = try!(TrafficPolicyNameDeserializer::deserialize("Name", stack));
            }
"Type" => {
                obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
            }
"Version" => {
                obj.version = try!(TrafficPolicyVersionDeserializer::deserialize("Version", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicyComment = String;
struct TrafficPolicyCommentDeserializer;
            impl TrafficPolicyCommentDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyComment, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyCommentSerializer;
                impl TrafficPolicyCommentSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyComment) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TrafficPolicyDocument = String;
struct TrafficPolicyDocumentDeserializer;
            impl TrafficPolicyDocumentDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyDocument, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyDocumentSerializer;
                impl TrafficPolicyDocumentSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyDocument) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TrafficPolicyId = String;
struct TrafficPolicyIdDeserializer;
            impl TrafficPolicyIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyIdSerializer;
                impl TrafficPolicyIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A complex type that contains settings for the new traffic policy instance.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct TrafficPolicyInstance {
                #[doc="<p>The ID of the hosted zone that Amazon Route 53 created resource record sets in.</p>"]
pub hosted_zone_id: ResourceId,
#[doc="<p>The ID that Amazon Route 53 assigned to the new traffic policy instance.</p>"]
pub id: TrafficPolicyInstanceId,
#[doc="<p>If <code>State</code> is <code>Failed</code>, an explanation of the reason for the failure. If <code>State</code> is another value, <code>Message</code> is empty.</p>"]
pub message: Message,
#[doc="<p>The DNS name, such as www.example.com, for which Amazon Route 53 responds to queries by using the resource record sets that are associated with this traffic policy instance. </p>"]
pub name: DNSName,
#[doc="<p>The value of <code>State</code> is one of the following values:</p> <dl> <dt>Applied</dt> <dd> <p>Amazon Route 53 has finished creating resource record sets, and changes have propagated to all Amazon Route 53 edge locations.</p> </dd> <dt>Creating</dt> <dd> <p>Amazon Route 53 is creating the resource record sets. Use <code>GetTrafficPolicyInstance</code> to confirm that the <code>CreateTrafficPolicyInstance</code> request completed successfully.</p> </dd> <dt>Failed</dt> <dd> <p>Amazon Route 53 wasn't able to create or update the resource record sets. When the value of <code>State</code> is <code>Failed</code>, see <code>Message</code> for an explanation of what caused the request to fail.</p> </dd> </dl>"]
pub state: TrafficPolicyInstanceState,
#[doc="<p>The TTL that Amazon Route 53 assigned to all of the resource record sets that it created in the specified hosted zone.</p>"]
pub ttl: TTL,
#[doc="<p>The ID of the traffic policy that Amazon Route 53 used to create resource record sets in the specified hosted zone.</p>"]
pub traffic_policy_id: TrafficPolicyId,
#[doc="<p>The DNS type that Amazon Route 53 assigned to all of the resource record sets that it created for this traffic policy instance. </p>"]
pub traffic_policy_type: RRType,
#[doc="<p>The version of the traffic policy that Amazon Route 53 used to create resource record sets in the specified hosted zone.</p>"]
pub traffic_policy_version: TrafficPolicyVersion,
            }
            
struct TrafficPolicyInstanceDeserializer;
            impl TrafficPolicyInstanceDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyInstance, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicyInstance::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZoneId" => {
                obj.hosted_zone_id = try!(ResourceIdDeserializer::deserialize("HostedZoneId", stack));
            }
"Id" => {
                obj.id = try!(TrafficPolicyInstanceIdDeserializer::deserialize("Id", stack));
            }
"Message" => {
                obj.message = try!(MessageDeserializer::deserialize("Message", stack));
            }
"Name" => {
                obj.name = try!(DNSNameDeserializer::deserialize("Name", stack));
            }
"State" => {
                obj.state = try!(TrafficPolicyInstanceStateDeserializer::deserialize("State", stack));
            }
"TTL" => {
                obj.ttl = try!(TTLDeserializer::deserialize("TTL", stack));
            }
"TrafficPolicyId" => {
                obj.traffic_policy_id = try!(TrafficPolicyIdDeserializer::deserialize("TrafficPolicyId", stack));
            }
"TrafficPolicyType" => {
                obj.traffic_policy_type = try!(RRTypeDeserializer::deserialize("TrafficPolicyType", stack));
            }
"TrafficPolicyVersion" => {
                obj.traffic_policy_version = try!(TrafficPolicyVersionDeserializer::deserialize("TrafficPolicyVersion", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicyInstanceCount = i64;
struct TrafficPolicyInstanceCountDeserializer;
            impl TrafficPolicyInstanceCountDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyInstanceCount, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicyInstanceId = String;
struct TrafficPolicyInstanceIdDeserializer;
            impl TrafficPolicyInstanceIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyInstanceId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyInstanceIdSerializer;
                impl TrafficPolicyInstanceIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyInstanceId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TrafficPolicyInstanceState = String;
struct TrafficPolicyInstanceStateDeserializer;
            impl TrafficPolicyInstanceStateDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyInstanceState, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicyInstances = Vec<TrafficPolicyInstance>;
struct TrafficPolicyInstancesDeserializer;
            impl TrafficPolicyInstancesDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyInstances, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "TrafficPolicyInstance" {
                        obj.push(try!(TrafficPolicyInstanceDeserializer::deserialize("TrafficPolicyInstance", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
pub type TrafficPolicyName = String;
struct TrafficPolicyNameDeserializer;
            impl TrafficPolicyNameDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyName, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyNameSerializer;
                impl TrafficPolicyNameSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyName) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TrafficPolicySummaries = Vec<TrafficPolicySummary>;
struct TrafficPolicySummariesDeserializer;
            impl TrafficPolicySummariesDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicySummaries, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "TrafficPolicySummary" {
                        obj.push(try!(TrafficPolicySummaryDeserializer::deserialize("TrafficPolicySummary", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the latest version of one traffic policy that is associated with the current AWS account.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct TrafficPolicySummary {
                #[doc="<p>The ID that Amazon Route 53 assigned to the traffic policy when you created it.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The version number of the latest version of the traffic policy.</p>"]
pub latest_version: TrafficPolicyVersion,
#[doc="<p>The name that you specified for the traffic policy when you created it.</p>"]
pub name: TrafficPolicyName,
#[doc="<p>The number of traffic policies that are associated with the current AWS account.</p>"]
pub traffic_policy_count: TrafficPolicyVersion,
#[doc="<p>The DNS type of the resource record sets that Amazon Route 53 creates when you use a traffic policy to create a traffic policy instance.</p>"]
pub type_: RRType,
            }
            
struct TrafficPolicySummaryDeserializer;
            impl TrafficPolicySummaryDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicySummary, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = TrafficPolicySummary::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Id" => {
                obj.id = try!(TrafficPolicyIdDeserializer::deserialize("Id", stack));
            }
"LatestVersion" => {
                obj.latest_version = try!(TrafficPolicyVersionDeserializer::deserialize("LatestVersion", stack));
            }
"Name" => {
                obj.name = try!(TrafficPolicyNameDeserializer::deserialize("Name", stack));
            }
"TrafficPolicyCount" => {
                obj.traffic_policy_count = try!(TrafficPolicyVersionDeserializer::deserialize("TrafficPolicyCount", stack));
            }
"Type" => {
                obj.type_ = try!(RRTypeDeserializer::deserialize("Type", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
pub type TrafficPolicyVersion = i64;
struct TrafficPolicyVersionDeserializer;
            impl TrafficPolicyVersionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyVersion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyVersionSerializer;
                impl TrafficPolicyVersionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyVersion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TrafficPolicyVersionMarker = String;
struct TrafficPolicyVersionMarkerDeserializer;
            impl TrafficPolicyVersionMarkerDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TrafficPolicyVersionMarker, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct TrafficPolicyVersionMarkerSerializer;
                impl TrafficPolicyVersionMarkerSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &TrafficPolicyVersionMarker) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type TransportProtocol = String;
struct TransportProtocolDeserializer;
            impl TransportProtocolDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<TransportProtocol, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the health check request information.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateHealthCheckRequest {
                pub alarm_identifier: Option<AlarmIdentifier>,
#[doc="<p>A complex type that contains one <code>ChildHealthCheck</code> element for each health check that you want to associate with a <code>CALCULATED</code> health check.</p>"]
pub child_health_checks: Option<ChildHealthCheckList>,
#[doc="<p>Specify whether you want Amazon Route 53 to send the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>client_hello</code> message during <code>TLS</code> negotiation. This allows the endpoint to respond to <code>HTTPS</code> health check requests with the applicable SSL/TLS certificate.</p> <p>Some endpoints require that HTTPS requests include the host name in the <code>client_hello</code> message. If you don't enable SNI, the status of the health check will be SSL alert <code>handshake_failure</code>. A health check can also have that status for other reasons. If SNI is enabled and you're still getting the error, check the SSL/TLS configuration on your endpoint and confirm that your certificate is valid.</p> <p>The SSL/TLS certificate on your endpoint includes a domain name in the <code>Common Name</code> field and possibly several more in the <code>Subject Alternative Names</code> field. One of the domain names in the certificate should match the value that you specify for <code>FullyQualifiedDomainName</code>. If the endpoint responds to the <code>client_hello</code> message with a certificate that does not include the domain name that you specified in <code>FullyQualifiedDomainName</code>, a health checker will retry the handshake. In the second attempt, the health checker will omit <code>FullyQualifiedDomainName</code> from the <code>client_hello</code> message.</p>"]
pub enable_sni: Option<EnableSNI>,
#[doc="<p>The number of consecutive health checks that an endpoint must pass or fail for Amazon Route 53 to change the current status of the endpoint from unhealthy to healthy or vice versa. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html\">How Amazon Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
pub failure_threshold: Option<FailureThreshold>,
#[doc="<p>Amazon Route 53 behavior depends on whether you specify a value for <code>IPAddress</code>.</p> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can't update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p> <b>If you specify a value for</b> <code>IPAddress</code>:</p> <p>Amazon Route 53 sends health check requests to the specified IPv4 or IPv6 address and passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header for all health checks except TCP health checks. This is typically the fully qualified DNS name of the endpoint on which you want Amazon Route 53 to perform health checks.</p> <p>When Amazon Route 53 checks the health of an endpoint, here is how it constructs the <code>Host</code> header:</p> <ul> <li> <p>If you specify a value of <code>80</code> for <code>Port</code> and <code>HTTP</code> or <code>HTTP_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the <code>Host</code> header.</p> </li> <li> <p>If you specify a value of <code>443</code> for <code>Port</code> and <code>HTTPS</code> or <code>HTTPS_STR_MATCH</code> for <code>Type</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> to the endpoint in the Host header.</p> </li> <li> <p>If you specify another value for <code>Port</code> and any value except <code>TCP</code> for <code>Type</code>, Amazon Route 53 passes <i> <code>FullyQualifiedDomainName</code>:<code>Port</code> </i> to the endpoint in the Host header.</p> </li> </ul> <p>If you don't specify a value for <code>FullyQualifiedDomainName</code>, Amazon Route 53 substitutes the value of <code>IPAddress</code> in the <code>Host</code> header in each of the above cases.</p> <p> <b>If you don't specify a value for</b> <code>IPAddress</code>:</p> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to the domain that you specify in <code>FullyQualifiedDomainName</code> at the interval you specify in <code>RequestInterval</code>. Using an IPv4 address that is returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <note> <p>If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 uses only IPv4 to send health checks to the endpoint. If there's no resource record set with a type of A for the name that you specify for <code>FullyQualifiedDomainName</code>, the health check fails with a \"DNS resolution failed\" error.</p> </note> <p>If you want to check the health of weighted, latency, or failover resource record sets and you choose to specify the endpoint only by <code>FullyQualifiedDomainName</code>, we recommend that you create a separate health check for each endpoint. For example, create a health check for each HTTP server that is serving content for www.example.com. For the value of <code>FullyQualifiedDomainName</code>, specify the domain name of the server (such as <code>us-east-1-www.example.com</code>), not the name of the resource record sets (www.example.com).</p> <important> <p>In this configuration, if the value of <code>FullyQualifiedDomainName</code> matches the name of the resource record sets and you then associate the health check with those resource record sets, health check results will be unpredictable.</p> </important> <p>In addition, if the value of <code>Type</code> is <code>HTTP</code>, <code>HTTPS</code>, <code>HTTP_STR_MATCH</code>, or <code>HTTPS_STR_MATCH</code>, Amazon Route 53 passes the value of <code>FullyQualifiedDomainName</code> in the <code>Host</code> header, as it does when you specify a value for <code>IPAddress</code>. If the value of <code>Type</code> is <code>TCP</code>, Amazon Route 53 doesn't pass a <code>Host</code> header.</p>"]
pub fully_qualified_domain_name: Option<FullyQualifiedDomainName>,
#[doc="<p>The ID for the health check for which you want detailed information. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p>"]
pub health_check_id: HealthCheckId,
#[doc="<p>A sequential counter that Amazon Route 53 sets to <code>1</code> when you create a health check and increments by <code>1</code> each time you update settings for the health check.</p> <p>We recommend that you use <code>GetHealthCheck</code> or <code>ListHealthChecks</code> to get the current value of <code>HealthCheckVersion</code> for the health check that you want to update, and that you include that value in your <code>UpdateHealthCheck</code> request. This prevents Amazon Route 53 from overwriting an intervening update:</p> <ul> <li> <p>f the value in the <code>UpdateHealthCheck</code> request matches the value of <code>HealthCheckVersion</code> in the health check, Amazon Route 53 updates the health check with the new settings.</p> </li> <li> <p>If the value of <code>HealthCheckVersion</code> in the health check is greater, the health check was changed after you got the version number. Amazon Route 53 does not update the health check, and it returns a <code>HealthCheckVersionMismatch</code> error.</p> </li> </ul>"]
pub health_check_version: Option<HealthCheckVersion>,
#[doc="<p>The number of child health checks that are associated with a <code>CALCULATED</code> health that Amazon Route 53 must consider healthy for the <code>CALCULATED</code> health check to be considered healthy. To specify the child health checks that you want to associate with a <code>CALCULATED</code> health check, use the <code>ChildHealthChecks</code> and <code>ChildHealthCheck</code> elements.</p> <p>Note the following:</p> <ul> <li> <p>If you specify a number greater than the number of child health checks, Amazon Route 53 always considers this health check to be unhealthy.</p> </li> <li> <p>If you specify <code>0</code>, Amazon Route 53 always considers this health check to be healthy.</p> </li> </ul>"]
pub health_threshold: Option<HealthThreshold>,
#[doc="<p>The IPv4 or IPv6 IP address for the endpoint that you want Amazon Route 53 to perform health checks on. If you don't specify a value for <code>IPAddress</code>, Amazon Route 53 sends a DNS request to resolve the domain name that you specify in <code>FullyQualifiedDomainName</code> at the interval that you specify in <code>RequestInterval</code>. Using an IP address that is returned by DNS, Amazon Route 53 then checks the health of the endpoint.</p> <p>If the endpoint is an EC2 instance, we recommend that you create an Elastic IP address, associate it with your EC2 instance, and specify the Elastic IP address for <code>IPAddress</code>. This ensures that the IP address of your instance never changes. For more information, see <a href=\"http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html\">Elastic IP Addresses (EIP)</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <note> <p>If a health check already has a value for <code>IPAddress</code>, you can change the value. However, you can't update an existing health check to add or remove the value of <code>IPAddress</code>. </p> </note> <p>For more information, see <a>UpdateHealthCheckRequest$FullyQualifiedDomainName</a>.</p> <p>Constraints: Amazon Route 53 can't check the health of endpoints for which the IP address is in local, private, non-routable, or multicast ranges. For more information about IP addresses for which you can't create health checks, see the following documents:</p> <ul> <li> <p> <a href=\"https://tools.ietf.org/html/rfc5735\">RFC 5735, Special Use IPv4 Addresses</a> </p> </li> <li> <p> <a href=\"https://tools.ietf.org/html/rfc6598\">RFC 6598, IANA-Reserved IPv4 Prefix for Shared Address Space</a> </p> </li> <li> <p> <a href=\"https://tools.ietf.org/html/rfc5156\">RFC 5156, Special-Use IPv6 Addresses</a> </p> </li> </ul>"]
pub ip_address: Option<IPAddress>,
#[doc="<p>When CloudWatch has insufficient data about the metric to determine the alarm state, the status that you want Amazon Route 53 to assign to the health check:</p> <ul> <li> <p> <code>Healthy</code>: Amazon Route 53 considers the health check to be healthy.</p> </li> <li> <p> <code>Unhealthy</code>: Amazon Route 53 considers the health check to be unhealthy.</p> </li> <li> <p> <code>LastKnownStatus</code>: Amazon Route 53 uses the status of the health check from the last time CloudWatch had sufficient data to determine the alarm state. For new health checks that have no last known status, the default status for the health check is healthy.</p> </li> </ul>"]
pub insufficient_data_health_status: Option<InsufficientDataHealthStatus>,
#[doc="<p>Specify whether you want Amazon Route 53 to invert the status of a health check, for example, to consider a health check unhealthy when it otherwise would be considered healthy.</p>"]
pub inverted: Option<Inverted>,
#[doc="<p>The port on the endpoint on which you want Amazon Route 53 to perform health checks.</p>"]
pub port: Option<Port>,
#[doc="<p>A complex type that contains one Region element for each region from which you want Amazon Route 53 health checkers to check the specified endpoint.</p>"]
pub regions: Option<HealthCheckRegionList>,
#[doc="<p>The path that you want Amazon Route 53 to request when performing health checks. The path can be any value for which your endpoint will return an HTTP status code of 2xx or 3xx when the endpoint is healthy, for example the file /docs/route53-health-check.html. </p> <p>Specify this value only if you want to change it.</p>"]
pub resource_path: Option<ResourcePath>,
#[doc="<p>If the value of <code>Type</code> is <code>HTTP_STR_MATCH</code> or <code>HTTP_STR_MATCH</code>, the string that you want Amazon Route 53 to search for in the response body from the specified resource. If the string appears in the response body, Amazon Route 53 considers the resource healthy. (You can't change the value of <code>Type</code> when you update a health check.)</p>"]
pub search_string: Option<SearchString>,
            }
            
#[derive(Default,Clone,Debug)]
            pub struct UpdateHealthCheckResponse {
                pub health_check: HealthCheck,
            }
            
struct UpdateHealthCheckResponseDeserializer;
            impl UpdateHealthCheckResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<UpdateHealthCheckResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = UpdateHealthCheckResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HealthCheck" => {
                obj.health_check = try!(HealthCheckDeserializer::deserialize("HealthCheck", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains the hosted zone request information.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateHostedZoneCommentRequest {
                #[doc="<p>The new comment for the hosted zone. If you don't specify a value for <code>Comment</code>, Amazon Route 53 deletes the existing value of the <code>Comment</code> element, if any.</p>"]
pub comment: Option<ResourceDescription>,
#[doc="<p>The ID for the hosted zone for which you want to update the comment.</p>"]
pub id: ResourceId,
            }
            
#[doc="<p>A complex type that contains the response to the UpdateHostedZoneCommentRequest.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateHostedZoneCommentResponse {
                pub hosted_zone: HostedZone,
            }
            
struct UpdateHostedZoneCommentResponseDeserializer;
            impl UpdateHostedZoneCommentResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<UpdateHostedZoneCommentResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = UpdateHostedZoneCommentResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "HostedZone" => {
                obj.hosted_zone = try!(HostedZoneDeserializer::deserialize("HostedZone", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the traffic policy for which you want to update the comment.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateTrafficPolicyCommentRequest {
                #[doc="<p>The new comment for the specified traffic policy and version.</p>"]
pub comment: TrafficPolicyComment,
#[doc="<p>The value of <code>Id</code> for the traffic policy for which you want to update the comment.</p>"]
pub id: TrafficPolicyId,
#[doc="<p>The value of <code>Version</code> for the traffic policy for which you want to update the comment.</p>"]
pub version: TrafficPolicyVersion,
            }
            
#[doc="<p>A complex type that contains the response information for the traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateTrafficPolicyCommentResponse {
                #[doc="<p>A complex type that contains settings for the specified traffic policy.</p>"]
pub traffic_policy: TrafficPolicy,
            }
            
struct UpdateTrafficPolicyCommentResponseDeserializer;
            impl UpdateTrafficPolicyCommentResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<UpdateTrafficPolicyCommentResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = UpdateTrafficPolicyCommentResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicy" => {
                obj.traffic_policy = try!(TrafficPolicyDeserializer::deserialize("TrafficPolicy", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about the resource record sets that you want to update based on a specified traffic policy instance.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateTrafficPolicyInstanceRequest {
                #[doc="<p>The ID of the traffic policy instance that you want to update.</p>"]
pub id: TrafficPolicyInstanceId,
#[doc="<p>The TTL that you want Amazon Route 53 to assign to all of the updated resource record sets.</p>"]
pub ttl: TTL,
#[doc="<p>The ID of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>"]
pub traffic_policy_id: TrafficPolicyId,
#[doc="<p>The version of the traffic policy that you want Amazon Route 53 to use to update resource record sets for the specified traffic policy instance.</p>"]
pub traffic_policy_version: TrafficPolicyVersion,
            }
            
#[doc="<p>A complex type that contains information about the resource record sets that Amazon Route 53 created based on a specified traffic policy.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct UpdateTrafficPolicyInstanceResponse {
                #[doc="<p>A complex type that contains settings for the updated traffic policy instance.</p>"]
pub traffic_policy_instance: TrafficPolicyInstance,
            }
            
struct UpdateTrafficPolicyInstanceResponseDeserializer;
            impl UpdateTrafficPolicyInstanceResponseDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<UpdateTrafficPolicyInstanceResponse, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = UpdateTrafficPolicyInstanceResponse::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TrafficPolicyInstance" => {
                obj.traffic_policy_instance = try!(TrafficPolicyInstanceDeserializer::deserialize("TrafficPolicyInstance", stack));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }
#[doc="<p>A complex type that contains information about an Amazon VPC that is associated with a private hosted zone.</p>"]
#[derive(Default,Clone,Debug)]
            pub struct VPC {
                pub vpc_id: Option<VPCId>,
#[doc="<p>The region in which you created the VPC that you want to associate with the specified Amazon Route 53 hosted zone.</p>"]
pub vpc_region: Option<VPCRegion>,
            }
            
struct VPCDeserializer;
            impl VPCDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<VPC, XmlParseError> {
                    try!(start_element(tag_name, stack));

        let mut obj = VPC::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "VPCId" => {
                obj.vpc_id = Some(try!(VPCIdDeserializer::deserialize("VPCId", stack)));
            }
"VPCRegion" => {
                obj.vpc_region = Some(try!(VPCRegionDeserializer::deserialize("VPCRegion", stack)));
            }
                        _ => skip_tree(stack),
                    }
                },
                DeserializerNext::Close => break,
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct VPCSerializer;
                impl VPCSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &VPC) -> String {
                        let mut serialized = format!("<{name}>", name=name);if let Some(ref value) = obj.vpc_id {
                serialized += &format!("<VPCId>{value}</VPCId>", value=value);
            }if let Some(ref value) = obj.vpc_region {
                serialized += &format!("<VPCRegion>{value}</VPCRegion>", value=value);
            }serialized += &format!("</{name}>", name=name);serialized
                    }
                }
                
#[doc="<p>The ID of an Amazon VPC. </p>"]
pub type VPCId = String;
struct VPCIdDeserializer;
            impl VPCIdDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<VPCId, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct VPCIdSerializer;
                impl VPCIdSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &VPCId) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
pub type VPCRegion = String;
struct VPCRegionDeserializer;
            impl VPCRegionDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<VPCRegion, XmlParseError> {
                    try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)
        
                }
            }

                pub struct VPCRegionSerializer;
                impl VPCRegionSerializer {
                    
        #[allow(unused_variables, warnings)]
        pub fn serialize(name: &str, obj: &VPCRegion) -> String {
                        format!("<{name}>{value}</{name}>",
                name = name,
                value = obj.to_string())
                    }
                }
                
#[doc="<p>A list of <code>VPC</code> elements.</p>"]
pub type VPCs = Vec<VPC>;
struct VPCsDeserializer;
            impl VPCsDeserializer {
                #[allow(unused_variables)]
                fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T)
                -> Result<VPCs, XmlParseError> {
                    
        let mut obj = vec![];
        try!(start_element(tag_name, stack));

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => DeserializerNext::Element(name.local_name.to_owned()),
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    if name == "VPC" {
                        obj.push(try!(VPCDeserializer::deserialize("VPC", stack)));
                    } else {
                        skip_tree(stack);
                    }
                },
                DeserializerNext::Close => {
                    try!(end_element(tag_name, stack));
                    break;
                }
                DeserializerNext::Skip => { stack.next(); },
            }
        }

        Ok(obj)
        
                }
            }
/// Errors returned by AssociateVPCWithHostedZone
                #[derive(Debug, PartialEq)]
                pub enum AssociateVPCWithHostedZoneError {
                    
///<p>You specified an Amazon VPC that you're already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can't use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p>
ConflictingDomainExists(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
InvalidVPCId(String),
///<p>The limits specified for a resource have been exceeded.</p>
LimitsExceeded(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>Associating the specified VPC with the specified hosted zone has not been authorized.</p>
NotAuthorized(String),
///<p>You're trying to associate a VPC with a public hosted zone. Amazon Route 53 doesn't support associating a VPC with a public hosted zone.</p>
PublicZoneVPCAssociation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl AssociateVPCWithHostedZoneError {
                    pub fn from_body(body: &str) -> AssociateVPCWithHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConflictingDomainExists" => AssociateVPCWithHostedZoneError::ConflictingDomainExists(String::from(parsed_error.message)),"InvalidInput" => AssociateVPCWithHostedZoneError::InvalidInput(String::from(parsed_error.message)),"InvalidVPCId" => AssociateVPCWithHostedZoneError::InvalidVPCId(String::from(parsed_error.message)),"LimitsExceeded" => AssociateVPCWithHostedZoneError::LimitsExceeded(String::from(parsed_error.message)),"NoSuchHostedZone" => AssociateVPCWithHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message)),"NotAuthorizedException" => AssociateVPCWithHostedZoneError::NotAuthorized(String::from(parsed_error.message)),"PublicZoneVPCAssociation" => AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(String::from(parsed_error.message)),_ => AssociateVPCWithHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => AssociateVPCWithHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for AssociateVPCWithHostedZoneError {
                    fn from(err: XmlParseError) -> AssociateVPCWithHostedZoneError {
                        let XmlParseError(message) = err;
                        AssociateVPCWithHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for AssociateVPCWithHostedZoneError {
                    fn from(err: CredentialsError) -> AssociateVPCWithHostedZoneError {
                        AssociateVPCWithHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for AssociateVPCWithHostedZoneError {
                    fn from(err: HttpDispatchError) -> AssociateVPCWithHostedZoneError {
                        AssociateVPCWithHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for AssociateVPCWithHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for AssociateVPCWithHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            AssociateVPCWithHostedZoneError::ConflictingDomainExists(ref cause) => cause,AssociateVPCWithHostedZoneError::InvalidInput(ref cause) => cause,AssociateVPCWithHostedZoneError::InvalidVPCId(ref cause) => cause,AssociateVPCWithHostedZoneError::LimitsExceeded(ref cause) => cause,AssociateVPCWithHostedZoneError::NoSuchHostedZone(ref cause) => cause,AssociateVPCWithHostedZoneError::NotAuthorized(ref cause) => cause,AssociateVPCWithHostedZoneError::PublicZoneVPCAssociation(ref cause) => cause,AssociateVPCWithHostedZoneError::Validation(ref cause) => cause,AssociateVPCWithHostedZoneError::Credentials(ref err) => err.description(),AssociateVPCWithHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),AssociateVPCWithHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ChangeResourceRecordSets
                #[derive(Debug, PartialEq)]
                pub enum ChangeResourceRecordSetsError {
                    
///<p>This exception contains a list of messages that might contain one or more error messages. Each error message indicates one error in the change batch.</p>
InvalidChangeBatch(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ChangeResourceRecordSetsError {
                    pub fn from_body(body: &str) -> ChangeResourceRecordSetsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidChangeBatch" => ChangeResourceRecordSetsError::InvalidChangeBatch(String::from(parsed_error.message)),"InvalidInput" => ChangeResourceRecordSetsError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => ChangeResourceRecordSetsError::NoSuchHealthCheck(String::from(parsed_error.message)),"NoSuchHostedZone" => ChangeResourceRecordSetsError::NoSuchHostedZone(String::from(parsed_error.message)),"PriorRequestNotComplete" => ChangeResourceRecordSetsError::PriorRequestNotComplete(String::from(parsed_error.message)),_ => ChangeResourceRecordSetsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ChangeResourceRecordSetsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ChangeResourceRecordSetsError {
                    fn from(err: XmlParseError) -> ChangeResourceRecordSetsError {
                        let XmlParseError(message) = err;
                        ChangeResourceRecordSetsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ChangeResourceRecordSetsError {
                    fn from(err: CredentialsError) -> ChangeResourceRecordSetsError {
                        ChangeResourceRecordSetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ChangeResourceRecordSetsError {
                    fn from(err: HttpDispatchError) -> ChangeResourceRecordSetsError {
                        ChangeResourceRecordSetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ChangeResourceRecordSetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ChangeResourceRecordSetsError {
                    fn description(&self) -> &str {
                        match *self {
                            ChangeResourceRecordSetsError::InvalidChangeBatch(ref cause) => cause,ChangeResourceRecordSetsError::InvalidInput(ref cause) => cause,ChangeResourceRecordSetsError::NoSuchHealthCheck(ref cause) => cause,ChangeResourceRecordSetsError::NoSuchHostedZone(ref cause) => cause,ChangeResourceRecordSetsError::PriorRequestNotComplete(ref cause) => cause,ChangeResourceRecordSetsError::Validation(ref cause) => cause,ChangeResourceRecordSetsError::Credentials(ref err) => err.description(),ChangeResourceRecordSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ChangeResourceRecordSetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ChangeTagsForResource
                #[derive(Debug, PartialEq)]
                pub enum ChangeTagsForResourceError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),
///<p></p>
Throttling(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ChangeTagsForResourceError {
                    pub fn from_body(body: &str) -> ChangeTagsForResourceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ChangeTagsForResourceError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => ChangeTagsForResourceError::NoSuchHealthCheck(String::from(parsed_error.message)),"NoSuchHostedZone" => ChangeTagsForResourceError::NoSuchHostedZone(String::from(parsed_error.message)),"PriorRequestNotComplete" => ChangeTagsForResourceError::PriorRequestNotComplete(String::from(parsed_error.message)),"ThrottlingException" => ChangeTagsForResourceError::Throttling(String::from(parsed_error.message)),_ => ChangeTagsForResourceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ChangeTagsForResourceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ChangeTagsForResourceError {
                    fn from(err: XmlParseError) -> ChangeTagsForResourceError {
                        let XmlParseError(message) = err;
                        ChangeTagsForResourceError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ChangeTagsForResourceError {
                    fn from(err: CredentialsError) -> ChangeTagsForResourceError {
                        ChangeTagsForResourceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ChangeTagsForResourceError {
                    fn from(err: HttpDispatchError) -> ChangeTagsForResourceError {
                        ChangeTagsForResourceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ChangeTagsForResourceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ChangeTagsForResourceError {
                    fn description(&self) -> &str {
                        match *self {
                            ChangeTagsForResourceError::InvalidInput(ref cause) => cause,ChangeTagsForResourceError::NoSuchHealthCheck(ref cause) => cause,ChangeTagsForResourceError::NoSuchHostedZone(ref cause) => cause,ChangeTagsForResourceError::PriorRequestNotComplete(ref cause) => cause,ChangeTagsForResourceError::Throttling(ref cause) => cause,ChangeTagsForResourceError::Validation(ref cause) => cause,ChangeTagsForResourceError::Credentials(ref err) => err.description(),ChangeTagsForResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ChangeTagsForResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateHealthCheck
                #[derive(Debug, PartialEq)]
                pub enum CreateHealthCheckError {
                    
///<p> The health check you're attempting to create already exists.</p> <p>Amazon Route 53 returns this error when a health check has already been created with the specified value for <code>CallerReference</code>.</p>
HealthCheckAlreadyExists(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>You have reached the maximum number of active health checks for an AWS account. The default limit is 100. To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the AWS Support Center.</p>
TooManyHealthChecks(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateHealthCheckError {
                    pub fn from_body(body: &str) -> CreateHealthCheckError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "HealthCheckAlreadyExists" => CreateHealthCheckError::HealthCheckAlreadyExists(String::from(parsed_error.message)),"InvalidInput" => CreateHealthCheckError::InvalidInput(String::from(parsed_error.message)),"TooManyHealthChecks" => CreateHealthCheckError::TooManyHealthChecks(String::from(parsed_error.message)),_ => CreateHealthCheckError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateHealthCheckError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateHealthCheckError {
                    fn from(err: XmlParseError) -> CreateHealthCheckError {
                        let XmlParseError(message) = err;
                        CreateHealthCheckError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateHealthCheckError {
                    fn from(err: CredentialsError) -> CreateHealthCheckError {
                        CreateHealthCheckError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateHealthCheckError {
                    fn from(err: HttpDispatchError) -> CreateHealthCheckError {
                        CreateHealthCheckError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateHealthCheckError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateHealthCheckError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateHealthCheckError::HealthCheckAlreadyExists(ref cause) => cause,CreateHealthCheckError::InvalidInput(ref cause) => cause,CreateHealthCheckError::TooManyHealthChecks(ref cause) => cause,CreateHealthCheckError::Validation(ref cause) => cause,CreateHealthCheckError::Credentials(ref err) => err.description(),CreateHealthCheckError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateHealthCheckError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateHostedZone
                #[derive(Debug, PartialEq)]
                pub enum CreateHostedZoneError {
                    
///<p>You specified an Amazon VPC that you're already using for another hosted zone, and the domain that you specified for one of the hosted zones is a subdomain of the domain that you specified for the other hosted zone. For example, you can't use the same Amazon VPC for the hosted zones for example.com and test.example.com.</p>
ConflictingDomainExists(String),
///<p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Amazon Route 53 generates this error, contact Customer Support.</p>
DelegationSetNotAvailable(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
DelegationSetNotReusable(String),
///<p>The hosted zone you are trying to create already exists. Amazon Route 53 returns this error when a hosted zone has already been created with the specified <code>CallerReference</code>.</p>
HostedZoneAlreadyExists(String),
///<p>The specified domain name is not valid.</p>
InvalidDomainName(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
InvalidVPCId(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
NoSuchDelegationSet(String),
///<p>This hosted zone can't be created because the hosted zone limit is exceeded. To request a limit increase, go to the Amazon Route 53 <a href="http://aws.amazon.com/route53-request/">Contact Us</a> page.</p>
TooManyHostedZones(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateHostedZoneError {
                    pub fn from_body(body: &str) -> CreateHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConflictingDomainExists" => CreateHostedZoneError::ConflictingDomainExists(String::from(parsed_error.message)),"DelegationSetNotAvailable" => CreateHostedZoneError::DelegationSetNotAvailable(String::from(parsed_error.message)),"DelegationSetNotReusable" => CreateHostedZoneError::DelegationSetNotReusable(String::from(parsed_error.message)),"HostedZoneAlreadyExists" => CreateHostedZoneError::HostedZoneAlreadyExists(String::from(parsed_error.message)),"InvalidDomainName" => CreateHostedZoneError::InvalidDomainName(String::from(parsed_error.message)),"InvalidInput" => CreateHostedZoneError::InvalidInput(String::from(parsed_error.message)),"InvalidVPCId" => CreateHostedZoneError::InvalidVPCId(String::from(parsed_error.message)),"NoSuchDelegationSet" => CreateHostedZoneError::NoSuchDelegationSet(String::from(parsed_error.message)),"TooManyHostedZones" => CreateHostedZoneError::TooManyHostedZones(String::from(parsed_error.message)),_ => CreateHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateHostedZoneError {
                    fn from(err: XmlParseError) -> CreateHostedZoneError {
                        let XmlParseError(message) = err;
                        CreateHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateHostedZoneError {
                    fn from(err: CredentialsError) -> CreateHostedZoneError {
                        CreateHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateHostedZoneError {
                    fn from(err: HttpDispatchError) -> CreateHostedZoneError {
                        CreateHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateHostedZoneError::ConflictingDomainExists(ref cause) => cause,CreateHostedZoneError::DelegationSetNotAvailable(ref cause) => cause,CreateHostedZoneError::DelegationSetNotReusable(ref cause) => cause,CreateHostedZoneError::HostedZoneAlreadyExists(ref cause) => cause,CreateHostedZoneError::InvalidDomainName(ref cause) => cause,CreateHostedZoneError::InvalidInput(ref cause) => cause,CreateHostedZoneError::InvalidVPCId(ref cause) => cause,CreateHostedZoneError::NoSuchDelegationSet(ref cause) => cause,CreateHostedZoneError::TooManyHostedZones(ref cause) => cause,CreateHostedZoneError::Validation(ref cause) => cause,CreateHostedZoneError::Credentials(ref err) => err.description(),CreateHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateReusableDelegationSet
                #[derive(Debug, PartialEq)]
                pub enum CreateReusableDelegationSetError {
                    
///<p>A delegation set with the same owner and caller reference combination has already been created.</p>
DelegationSetAlreadyCreated(String),
///<p>The specified delegation set has already been marked as reusable.</p>
DelegationSetAlreadyReusable(String),
///<p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Amazon Route 53 generates this error, contact Customer Support.</p>
DelegationSetNotAvailable(String),
///<p>The specified HostedZone can't be found.</p>
HostedZoneNotFound(String),
///<p>Parameter name and problem.</p>
InvalidArgument(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The limits specified for a resource have been exceeded.</p>
LimitsExceeded(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateReusableDelegationSetError {
                    pub fn from_body(body: &str) -> CreateReusableDelegationSetError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "DelegationSetAlreadyCreated" => CreateReusableDelegationSetError::DelegationSetAlreadyCreated(String::from(parsed_error.message)),"DelegationSetAlreadyReusable" => CreateReusableDelegationSetError::DelegationSetAlreadyReusable(String::from(parsed_error.message)),"DelegationSetNotAvailable" => CreateReusableDelegationSetError::DelegationSetNotAvailable(String::from(parsed_error.message)),"HostedZoneNotFound" => CreateReusableDelegationSetError::HostedZoneNotFound(String::from(parsed_error.message)),"InvalidArgument" => CreateReusableDelegationSetError::InvalidArgument(String::from(parsed_error.message)),"InvalidInput" => CreateReusableDelegationSetError::InvalidInput(String::from(parsed_error.message)),"LimitsExceeded" => CreateReusableDelegationSetError::LimitsExceeded(String::from(parsed_error.message)),_ => CreateReusableDelegationSetError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateReusableDelegationSetError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateReusableDelegationSetError {
                    fn from(err: XmlParseError) -> CreateReusableDelegationSetError {
                        let XmlParseError(message) = err;
                        CreateReusableDelegationSetError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateReusableDelegationSetError {
                    fn from(err: CredentialsError) -> CreateReusableDelegationSetError {
                        CreateReusableDelegationSetError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateReusableDelegationSetError {
                    fn from(err: HttpDispatchError) -> CreateReusableDelegationSetError {
                        CreateReusableDelegationSetError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateReusableDelegationSetError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateReusableDelegationSetError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateReusableDelegationSetError::DelegationSetAlreadyCreated(ref cause) => cause,CreateReusableDelegationSetError::DelegationSetAlreadyReusable(ref cause) => cause,CreateReusableDelegationSetError::DelegationSetNotAvailable(ref cause) => cause,CreateReusableDelegationSetError::HostedZoneNotFound(ref cause) => cause,CreateReusableDelegationSetError::InvalidArgument(ref cause) => cause,CreateReusableDelegationSetError::InvalidInput(ref cause) => cause,CreateReusableDelegationSetError::LimitsExceeded(ref cause) => cause,CreateReusableDelegationSetError::Validation(ref cause) => cause,CreateReusableDelegationSetError::Credentials(ref err) => err.description(),CreateReusableDelegationSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateReusableDelegationSetError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateTrafficPolicy
                #[derive(Debug, PartialEq)]
                pub enum CreateTrafficPolicyError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The format of the traffic policy document that you specified in the <code>Document</code> element is invalid.</p>
InvalidTrafficPolicyDocument(String),
///<p>You've created the maximum number of traffic policies that can be created for the current AWS account. You can request an increase to the limit on the <a href="http://aws.amazon.com/route53-request/">Contact Us</a> page.</p>
TooManyTrafficPolicies(String),
///<p>A traffic policy that has the same value for <code>Name</code> already exists.</p>
TrafficPolicyAlreadyExists(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateTrafficPolicyError {
                    pub fn from_body(body: &str) -> CreateTrafficPolicyError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => CreateTrafficPolicyError::InvalidInput(String::from(parsed_error.message)),"InvalidTrafficPolicyDocument" => CreateTrafficPolicyError::InvalidTrafficPolicyDocument(String::from(parsed_error.message)),"TooManyTrafficPolicies" => CreateTrafficPolicyError::TooManyTrafficPolicies(String::from(parsed_error.message)),"TrafficPolicyAlreadyExists" => CreateTrafficPolicyError::TrafficPolicyAlreadyExists(String::from(parsed_error.message)),_ => CreateTrafficPolicyError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateTrafficPolicyError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateTrafficPolicyError {
                    fn from(err: XmlParseError) -> CreateTrafficPolicyError {
                        let XmlParseError(message) = err;
                        CreateTrafficPolicyError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateTrafficPolicyError {
                    fn from(err: CredentialsError) -> CreateTrafficPolicyError {
                        CreateTrafficPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateTrafficPolicyError {
                    fn from(err: HttpDispatchError) -> CreateTrafficPolicyError {
                        CreateTrafficPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateTrafficPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateTrafficPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateTrafficPolicyError::InvalidInput(ref cause) => cause,CreateTrafficPolicyError::InvalidTrafficPolicyDocument(ref cause) => cause,CreateTrafficPolicyError::TooManyTrafficPolicies(ref cause) => cause,CreateTrafficPolicyError::TrafficPolicyAlreadyExists(ref cause) => cause,CreateTrafficPolicyError::Validation(ref cause) => cause,CreateTrafficPolicyError::Credentials(ref err) => err.description(),CreateTrafficPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateTrafficPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateTrafficPolicyInstance
                #[derive(Debug, PartialEq)]
                pub enum CreateTrafficPolicyInstanceError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),
///<p>You've created the maximum number of traffic policy instances that can be created for the current AWS account. You can request an increase to the limit on the <a href="http://aws.amazon.com/route53-request/">Contact Us</a> page.</p>
TooManyTrafficPolicyInstances(String),
///<p>Traffic policy instance with given Id already exists.</p>
TrafficPolicyInstanceAlreadyExists(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateTrafficPolicyInstanceError {
                    pub fn from_body(body: &str) -> CreateTrafficPolicyInstanceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => CreateTrafficPolicyInstanceError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => CreateTrafficPolicyInstanceError::NoSuchHostedZone(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(String::from(parsed_error.message)),"TooManyTrafficPolicyInstances" => CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(String::from(parsed_error.message)),"TrafficPolicyInstanceAlreadyExists" => CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(String::from(parsed_error.message)),_ => CreateTrafficPolicyInstanceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateTrafficPolicyInstanceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateTrafficPolicyInstanceError {
                    fn from(err: XmlParseError) -> CreateTrafficPolicyInstanceError {
                        let XmlParseError(message) = err;
                        CreateTrafficPolicyInstanceError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateTrafficPolicyInstanceError {
                    fn from(err: CredentialsError) -> CreateTrafficPolicyInstanceError {
                        CreateTrafficPolicyInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateTrafficPolicyInstanceError {
                    fn from(err: HttpDispatchError) -> CreateTrafficPolicyInstanceError {
                        CreateTrafficPolicyInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateTrafficPolicyInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateTrafficPolicyInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,CreateTrafficPolicyInstanceError::NoSuchHostedZone(ref cause) => cause,CreateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => cause,CreateTrafficPolicyInstanceError::TooManyTrafficPolicyInstances(ref cause) => cause,CreateTrafficPolicyInstanceError::TrafficPolicyInstanceAlreadyExists(ref cause) => cause,CreateTrafficPolicyInstanceError::Validation(ref cause) => cause,CreateTrafficPolicyInstanceError::Credentials(ref err) => err.description(),CreateTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateTrafficPolicyInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateTrafficPolicyVersion
                #[derive(Debug, PartialEq)]
                pub enum CreateTrafficPolicyVersionError {
                    
///<p>Another user submitted a request to update the object at the same time that you did. Retry the request. </p>
ConcurrentModification(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The format of the traffic policy document that you specified in the <code>Document</code> element is invalid.</p>
InvalidTrafficPolicyDocument(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateTrafficPolicyVersionError {
                    pub fn from_body(body: &str) -> CreateTrafficPolicyVersionError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConcurrentModification" => CreateTrafficPolicyVersionError::ConcurrentModification(String::from(parsed_error.message)),"InvalidInput" => CreateTrafficPolicyVersionError::InvalidInput(String::from(parsed_error.message)),"InvalidTrafficPolicyDocument" => CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(String::from(parsed_error.message)),_ => CreateTrafficPolicyVersionError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateTrafficPolicyVersionError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateTrafficPolicyVersionError {
                    fn from(err: XmlParseError) -> CreateTrafficPolicyVersionError {
                        let XmlParseError(message) = err;
                        CreateTrafficPolicyVersionError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateTrafficPolicyVersionError {
                    fn from(err: CredentialsError) -> CreateTrafficPolicyVersionError {
                        CreateTrafficPolicyVersionError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateTrafficPolicyVersionError {
                    fn from(err: HttpDispatchError) -> CreateTrafficPolicyVersionError {
                        CreateTrafficPolicyVersionError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateTrafficPolicyVersionError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateTrafficPolicyVersionError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateTrafficPolicyVersionError::ConcurrentModification(ref cause) => cause,CreateTrafficPolicyVersionError::InvalidInput(ref cause) => cause,CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument(ref cause) => cause,CreateTrafficPolicyVersionError::NoSuchTrafficPolicy(ref cause) => cause,CreateTrafficPolicyVersionError::Validation(ref cause) => cause,CreateTrafficPolicyVersionError::Credentials(ref err) => err.description(),CreateTrafficPolicyVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateTrafficPolicyVersionError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by CreateVPCAssociationAuthorization
                #[derive(Debug, PartialEq)]
                pub enum CreateVPCAssociationAuthorizationError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
InvalidVPCId(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>You've created the maximum number of authorizations that can be created for the specified hosted zone. To authorize another VPC to be associated with the hosted zone, submit a <code>DeleteVPCAssociationAuthorization</code> request to remove an existing authorization. To get a list of existing authorizations, submit a <code>ListVPCAssociationAuthorizations</code> request.</p>
TooManyVPCAssociationAuthorizations(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl CreateVPCAssociationAuthorizationError {
                    pub fn from_body(body: &str) -> CreateVPCAssociationAuthorizationError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => CreateVPCAssociationAuthorizationError::InvalidInput(String::from(parsed_error.message)),"InvalidVPCId" => CreateVPCAssociationAuthorizationError::InvalidVPCId(String::from(parsed_error.message)),"NoSuchHostedZone" => CreateVPCAssociationAuthorizationError::NoSuchHostedZone(String::from(parsed_error.message)),"TooManyVPCAssociationAuthorizations" => CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(String::from(parsed_error.message)),_ => CreateVPCAssociationAuthorizationError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => CreateVPCAssociationAuthorizationError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for CreateVPCAssociationAuthorizationError {
                    fn from(err: XmlParseError) -> CreateVPCAssociationAuthorizationError {
                        let XmlParseError(message) = err;
                        CreateVPCAssociationAuthorizationError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for CreateVPCAssociationAuthorizationError {
                    fn from(err: CredentialsError) -> CreateVPCAssociationAuthorizationError {
                        CreateVPCAssociationAuthorizationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for CreateVPCAssociationAuthorizationError {
                    fn from(err: HttpDispatchError) -> CreateVPCAssociationAuthorizationError {
                        CreateVPCAssociationAuthorizationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for CreateVPCAssociationAuthorizationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for CreateVPCAssociationAuthorizationError {
                    fn description(&self) -> &str {
                        match *self {
                            CreateVPCAssociationAuthorizationError::InvalidInput(ref cause) => cause,CreateVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => cause,CreateVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => cause,CreateVPCAssociationAuthorizationError::TooManyVPCAssociationAuthorizations(ref cause) => cause,CreateVPCAssociationAuthorizationError::Validation(ref cause) => cause,CreateVPCAssociationAuthorizationError::Credentials(ref err) => err.description(),CreateVPCAssociationAuthorizationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),CreateVPCAssociationAuthorizationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteHealthCheck
                #[derive(Debug, PartialEq)]
                pub enum DeleteHealthCheckError {
                    
///<p>The health check ID for this health check is referenced in the <code>HealthCheckId</code> element in one of the resource record sets in one of the hosted zones that are owned by the current AWS account.</p>
HealthCheckInUse(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteHealthCheckError {
                    pub fn from_body(body: &str) -> DeleteHealthCheckError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "HealthCheckInUse" => DeleteHealthCheckError::HealthCheckInUse(String::from(parsed_error.message)),"InvalidInput" => DeleteHealthCheckError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => DeleteHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message)),_ => DeleteHealthCheckError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteHealthCheckError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteHealthCheckError {
                    fn from(err: XmlParseError) -> DeleteHealthCheckError {
                        let XmlParseError(message) = err;
                        DeleteHealthCheckError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteHealthCheckError {
                    fn from(err: CredentialsError) -> DeleteHealthCheckError {
                        DeleteHealthCheckError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteHealthCheckError {
                    fn from(err: HttpDispatchError) -> DeleteHealthCheckError {
                        DeleteHealthCheckError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteHealthCheckError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteHealthCheckError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteHealthCheckError::HealthCheckInUse(ref cause) => cause,DeleteHealthCheckError::InvalidInput(ref cause) => cause,DeleteHealthCheckError::NoSuchHealthCheck(ref cause) => cause,DeleteHealthCheckError::Validation(ref cause) => cause,DeleteHealthCheckError::Credentials(ref err) => err.description(),DeleteHealthCheckError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteHealthCheckError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteHostedZone
                #[derive(Debug, PartialEq)]
                pub enum DeleteHostedZoneError {
                    
///<p>The hosted zone contains resource records that are not SOA or NS records.</p>
HostedZoneNotEmpty(String),
///<p>The specified domain name is not valid.</p>
InvalidDomainName(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteHostedZoneError {
                    pub fn from_body(body: &str) -> DeleteHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "HostedZoneNotEmpty" => DeleteHostedZoneError::HostedZoneNotEmpty(String::from(parsed_error.message)),"InvalidDomainName" => DeleteHostedZoneError::InvalidDomainName(String::from(parsed_error.message)),"InvalidInput" => DeleteHostedZoneError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => DeleteHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message)),"PriorRequestNotComplete" => DeleteHostedZoneError::PriorRequestNotComplete(String::from(parsed_error.message)),_ => DeleteHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteHostedZoneError {
                    fn from(err: XmlParseError) -> DeleteHostedZoneError {
                        let XmlParseError(message) = err;
                        DeleteHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteHostedZoneError {
                    fn from(err: CredentialsError) -> DeleteHostedZoneError {
                        DeleteHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteHostedZoneError {
                    fn from(err: HttpDispatchError) -> DeleteHostedZoneError {
                        DeleteHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteHostedZoneError::HostedZoneNotEmpty(ref cause) => cause,DeleteHostedZoneError::InvalidDomainName(ref cause) => cause,DeleteHostedZoneError::InvalidInput(ref cause) => cause,DeleteHostedZoneError::NoSuchHostedZone(ref cause) => cause,DeleteHostedZoneError::PriorRequestNotComplete(ref cause) => cause,DeleteHostedZoneError::Validation(ref cause) => cause,DeleteHostedZoneError::Credentials(ref err) => err.description(),DeleteHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteReusableDelegationSet
                #[derive(Debug, PartialEq)]
                pub enum DeleteReusableDelegationSetError {
                    
///<p>The specified delegation contains associated hosted zones which must be deleted before the reusable delegation set can be deleted.</p>
DelegationSetInUse(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
DelegationSetNotReusable(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
NoSuchDelegationSet(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteReusableDelegationSetError {
                    pub fn from_body(body: &str) -> DeleteReusableDelegationSetError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "DelegationSetInUse" => DeleteReusableDelegationSetError::DelegationSetInUse(String::from(parsed_error.message)),"DelegationSetNotReusable" => DeleteReusableDelegationSetError::DelegationSetNotReusable(String::from(parsed_error.message)),"InvalidInput" => DeleteReusableDelegationSetError::InvalidInput(String::from(parsed_error.message)),"NoSuchDelegationSet" => DeleteReusableDelegationSetError::NoSuchDelegationSet(String::from(parsed_error.message)),_ => DeleteReusableDelegationSetError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteReusableDelegationSetError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteReusableDelegationSetError {
                    fn from(err: XmlParseError) -> DeleteReusableDelegationSetError {
                        let XmlParseError(message) = err;
                        DeleteReusableDelegationSetError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteReusableDelegationSetError {
                    fn from(err: CredentialsError) -> DeleteReusableDelegationSetError {
                        DeleteReusableDelegationSetError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteReusableDelegationSetError {
                    fn from(err: HttpDispatchError) -> DeleteReusableDelegationSetError {
                        DeleteReusableDelegationSetError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteReusableDelegationSetError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteReusableDelegationSetError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteReusableDelegationSetError::DelegationSetInUse(ref cause) => cause,DeleteReusableDelegationSetError::DelegationSetNotReusable(ref cause) => cause,DeleteReusableDelegationSetError::InvalidInput(ref cause) => cause,DeleteReusableDelegationSetError::NoSuchDelegationSet(ref cause) => cause,DeleteReusableDelegationSetError::Validation(ref cause) => cause,DeleteReusableDelegationSetError::Credentials(ref err) => err.description(),DeleteReusableDelegationSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteReusableDelegationSetError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteTrafficPolicy
                #[derive(Debug, PartialEq)]
                pub enum DeleteTrafficPolicyError {
                    
///<p>Another user submitted a request to update the object at the same time that you did. Retry the request. </p>
ConcurrentModification(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),
///<p>One or more traffic policy instances were created by using the specified traffic policy.</p>
TrafficPolicyInUse(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteTrafficPolicyError {
                    pub fn from_body(body: &str) -> DeleteTrafficPolicyError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConcurrentModification" => DeleteTrafficPolicyError::ConcurrentModification(String::from(parsed_error.message)),"InvalidInput" => DeleteTrafficPolicyError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => DeleteTrafficPolicyError::NoSuchTrafficPolicy(String::from(parsed_error.message)),"TrafficPolicyInUse" => DeleteTrafficPolicyError::TrafficPolicyInUse(String::from(parsed_error.message)),_ => DeleteTrafficPolicyError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteTrafficPolicyError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteTrafficPolicyError {
                    fn from(err: XmlParseError) -> DeleteTrafficPolicyError {
                        let XmlParseError(message) = err;
                        DeleteTrafficPolicyError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteTrafficPolicyError {
                    fn from(err: CredentialsError) -> DeleteTrafficPolicyError {
                        DeleteTrafficPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteTrafficPolicyError {
                    fn from(err: HttpDispatchError) -> DeleteTrafficPolicyError {
                        DeleteTrafficPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteTrafficPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteTrafficPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteTrafficPolicyError::ConcurrentModification(ref cause) => cause,DeleteTrafficPolicyError::InvalidInput(ref cause) => cause,DeleteTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => cause,DeleteTrafficPolicyError::TrafficPolicyInUse(ref cause) => cause,DeleteTrafficPolicyError::Validation(ref cause) => cause,DeleteTrafficPolicyError::Credentials(ref err) => err.description(),DeleteTrafficPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteTrafficPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteTrafficPolicyInstance
                #[derive(Debug, PartialEq)]
                pub enum DeleteTrafficPolicyInstanceError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteTrafficPolicyInstanceError {
                    pub fn from_body(body: &str) -> DeleteTrafficPolicyInstanceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => DeleteTrafficPolicyInstanceError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),"PriorRequestNotComplete" => DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(String::from(parsed_error.message)),_ => DeleteTrafficPolicyInstanceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteTrafficPolicyInstanceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteTrafficPolicyInstanceError {
                    fn from(err: XmlParseError) -> DeleteTrafficPolicyInstanceError {
                        let XmlParseError(message) = err;
                        DeleteTrafficPolicyInstanceError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteTrafficPolicyInstanceError {
                    fn from(err: CredentialsError) -> DeleteTrafficPolicyInstanceError {
                        DeleteTrafficPolicyInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteTrafficPolicyInstanceError {
                    fn from(err: HttpDispatchError) -> DeleteTrafficPolicyInstanceError {
                        DeleteTrafficPolicyInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteTrafficPolicyInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteTrafficPolicyInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,DeleteTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,DeleteTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => cause,DeleteTrafficPolicyInstanceError::Validation(ref cause) => cause,DeleteTrafficPolicyInstanceError::Credentials(ref err) => err.description(),DeleteTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteTrafficPolicyInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DeleteVPCAssociationAuthorization
                #[derive(Debug, PartialEq)]
                pub enum DeleteVPCAssociationAuthorizationError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
InvalidVPCId(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>The VPC that you specified is not authorized to be associated with the hosted zone.</p>
VPCAssociationAuthorizationNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DeleteVPCAssociationAuthorizationError {
                    pub fn from_body(body: &str) -> DeleteVPCAssociationAuthorizationError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => DeleteVPCAssociationAuthorizationError::InvalidInput(String::from(parsed_error.message)),"InvalidVPCId" => DeleteVPCAssociationAuthorizationError::InvalidVPCId(String::from(parsed_error.message)),"NoSuchHostedZone" => DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(String::from(parsed_error.message)),"VPCAssociationAuthorizationNotFound" => DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(String::from(parsed_error.message)),_ => DeleteVPCAssociationAuthorizationError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DeleteVPCAssociationAuthorizationError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DeleteVPCAssociationAuthorizationError {
                    fn from(err: XmlParseError) -> DeleteVPCAssociationAuthorizationError {
                        let XmlParseError(message) = err;
                        DeleteVPCAssociationAuthorizationError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DeleteVPCAssociationAuthorizationError {
                    fn from(err: CredentialsError) -> DeleteVPCAssociationAuthorizationError {
                        DeleteVPCAssociationAuthorizationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DeleteVPCAssociationAuthorizationError {
                    fn from(err: HttpDispatchError) -> DeleteVPCAssociationAuthorizationError {
                        DeleteVPCAssociationAuthorizationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DeleteVPCAssociationAuthorizationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DeleteVPCAssociationAuthorizationError {
                    fn description(&self) -> &str {
                        match *self {
                            DeleteVPCAssociationAuthorizationError::InvalidInput(ref cause) => cause,DeleteVPCAssociationAuthorizationError::InvalidVPCId(ref cause) => cause,DeleteVPCAssociationAuthorizationError::NoSuchHostedZone(ref cause) => cause,DeleteVPCAssociationAuthorizationError::VPCAssociationAuthorizationNotFound(ref cause) => cause,DeleteVPCAssociationAuthorizationError::Validation(ref cause) => cause,DeleteVPCAssociationAuthorizationError::Credentials(ref err) => err.description(),DeleteVPCAssociationAuthorizationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DeleteVPCAssociationAuthorizationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by DisassociateVPCFromHostedZone
                #[derive(Debug, PartialEq)]
                pub enum DisassociateVPCFromHostedZoneError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
InvalidVPCId(String),
///<p>The VPC that you're trying to disassociate from the private hosted zone is the last VPC that is associated with the hosted zone. Amazon Route 53 doesn't support disassociating the last VPC from a hosted zone.</p>
LastVPCAssociation(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>The specified VPC and hosted zone are not currently associated.</p>
VPCAssociationNotFound(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl DisassociateVPCFromHostedZoneError {
                    pub fn from_body(body: &str) -> DisassociateVPCFromHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => DisassociateVPCFromHostedZoneError::InvalidInput(String::from(parsed_error.message)),"InvalidVPCId" => DisassociateVPCFromHostedZoneError::InvalidVPCId(String::from(parsed_error.message)),"LastVPCAssociation" => DisassociateVPCFromHostedZoneError::LastVPCAssociation(String::from(parsed_error.message)),"NoSuchHostedZone" => DisassociateVPCFromHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message)),"VPCAssociationNotFound" => DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(String::from(parsed_error.message)),_ => DisassociateVPCFromHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => DisassociateVPCFromHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for DisassociateVPCFromHostedZoneError {
                    fn from(err: XmlParseError) -> DisassociateVPCFromHostedZoneError {
                        let XmlParseError(message) = err;
                        DisassociateVPCFromHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for DisassociateVPCFromHostedZoneError {
                    fn from(err: CredentialsError) -> DisassociateVPCFromHostedZoneError {
                        DisassociateVPCFromHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for DisassociateVPCFromHostedZoneError {
                    fn from(err: HttpDispatchError) -> DisassociateVPCFromHostedZoneError {
                        DisassociateVPCFromHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for DisassociateVPCFromHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for DisassociateVPCFromHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            DisassociateVPCFromHostedZoneError::InvalidInput(ref cause) => cause,DisassociateVPCFromHostedZoneError::InvalidVPCId(ref cause) => cause,DisassociateVPCFromHostedZoneError::LastVPCAssociation(ref cause) => cause,DisassociateVPCFromHostedZoneError::NoSuchHostedZone(ref cause) => cause,DisassociateVPCFromHostedZoneError::VPCAssociationNotFound(ref cause) => cause,DisassociateVPCFromHostedZoneError::Validation(ref cause) => cause,DisassociateVPCFromHostedZoneError::Credentials(ref err) => err.description(),DisassociateVPCFromHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),DisassociateVPCFromHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetChange
                #[derive(Debug, PartialEq)]
                pub enum GetChangeError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>A change with the specified change ID does not exist.</p>
NoSuchChange(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetChangeError {
                    pub fn from_body(body: &str) -> GetChangeError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetChangeError::InvalidInput(String::from(parsed_error.message)),"NoSuchChange" => GetChangeError::NoSuchChange(String::from(parsed_error.message)),_ => GetChangeError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetChangeError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetChangeError {
                    fn from(err: XmlParseError) -> GetChangeError {
                        let XmlParseError(message) = err;
                        GetChangeError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetChangeError {
                    fn from(err: CredentialsError) -> GetChangeError {
                        GetChangeError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetChangeError {
                    fn from(err: HttpDispatchError) -> GetChangeError {
                        GetChangeError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetChangeError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetChangeError {
                    fn description(&self) -> &str {
                        match *self {
                            GetChangeError::InvalidInput(ref cause) => cause,GetChangeError::NoSuchChange(ref cause) => cause,GetChangeError::Validation(ref cause) => cause,GetChangeError::Credentials(ref err) => err.description(),GetChangeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetChangeError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetCheckerIpRanges
                #[derive(Debug, PartialEq)]
                pub enum GetCheckerIpRangesError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetCheckerIpRangesError {
                    pub fn from_body(body: &str) -> GetCheckerIpRangesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    _ => GetCheckerIpRangesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetCheckerIpRangesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetCheckerIpRangesError {
                    fn from(err: XmlParseError) -> GetCheckerIpRangesError {
                        let XmlParseError(message) = err;
                        GetCheckerIpRangesError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetCheckerIpRangesError {
                    fn from(err: CredentialsError) -> GetCheckerIpRangesError {
                        GetCheckerIpRangesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetCheckerIpRangesError {
                    fn from(err: HttpDispatchError) -> GetCheckerIpRangesError {
                        GetCheckerIpRangesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetCheckerIpRangesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetCheckerIpRangesError {
                    fn description(&self) -> &str {
                        match *self {
                            GetCheckerIpRangesError::Validation(ref cause) => cause,GetCheckerIpRangesError::Credentials(ref err) => err.description(),GetCheckerIpRangesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetCheckerIpRangesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetGeoLocation
                #[derive(Debug, PartialEq)]
                pub enum GetGeoLocationError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>Amazon Route 53 doesn't support the specified geolocation.</p>
NoSuchGeoLocation(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetGeoLocationError {
                    pub fn from_body(body: &str) -> GetGeoLocationError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetGeoLocationError::InvalidInput(String::from(parsed_error.message)),"NoSuchGeoLocation" => GetGeoLocationError::NoSuchGeoLocation(String::from(parsed_error.message)),_ => GetGeoLocationError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetGeoLocationError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetGeoLocationError {
                    fn from(err: XmlParseError) -> GetGeoLocationError {
                        let XmlParseError(message) = err;
                        GetGeoLocationError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetGeoLocationError {
                    fn from(err: CredentialsError) -> GetGeoLocationError {
                        GetGeoLocationError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetGeoLocationError {
                    fn from(err: HttpDispatchError) -> GetGeoLocationError {
                        GetGeoLocationError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetGeoLocationError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetGeoLocationError {
                    fn description(&self) -> &str {
                        match *self {
                            GetGeoLocationError::InvalidInput(ref cause) => cause,GetGeoLocationError::NoSuchGeoLocation(ref cause) => cause,GetGeoLocationError::Validation(ref cause) => cause,GetGeoLocationError::Credentials(ref err) => err.description(),GetGeoLocationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetGeoLocationError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHealthCheck
                #[derive(Debug, PartialEq)]
                pub enum GetHealthCheckError {
                    
///<p>The resource you are trying to access is unsupported on this Amazon Route 53 endpoint. Please consider using a newer endpoint or a tool that does so.</p>
IncompatibleVersion(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHealthCheckError {
                    pub fn from_body(body: &str) -> GetHealthCheckError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "IncompatibleVersion" => GetHealthCheckError::IncompatibleVersion(String::from(parsed_error.message)),"InvalidInput" => GetHealthCheckError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => GetHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message)),_ => GetHealthCheckError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHealthCheckError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHealthCheckError {
                    fn from(err: XmlParseError) -> GetHealthCheckError {
                        let XmlParseError(message) = err;
                        GetHealthCheckError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHealthCheckError {
                    fn from(err: CredentialsError) -> GetHealthCheckError {
                        GetHealthCheckError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHealthCheckError {
                    fn from(err: HttpDispatchError) -> GetHealthCheckError {
                        GetHealthCheckError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHealthCheckError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHealthCheckError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHealthCheckError::IncompatibleVersion(ref cause) => cause,GetHealthCheckError::InvalidInput(ref cause) => cause,GetHealthCheckError::NoSuchHealthCheck(ref cause) => cause,GetHealthCheckError::Validation(ref cause) => cause,GetHealthCheckError::Credentials(ref err) => err.description(),GetHealthCheckError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHealthCheckError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHealthCheckCount
                #[derive(Debug, PartialEq)]
                pub enum GetHealthCheckCountError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHealthCheckCountError {
                    pub fn from_body(body: &str) -> GetHealthCheckCountError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    _ => GetHealthCheckCountError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHealthCheckCountError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHealthCheckCountError {
                    fn from(err: XmlParseError) -> GetHealthCheckCountError {
                        let XmlParseError(message) = err;
                        GetHealthCheckCountError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHealthCheckCountError {
                    fn from(err: CredentialsError) -> GetHealthCheckCountError {
                        GetHealthCheckCountError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHealthCheckCountError {
                    fn from(err: HttpDispatchError) -> GetHealthCheckCountError {
                        GetHealthCheckCountError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHealthCheckCountError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHealthCheckCountError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHealthCheckCountError::Validation(ref cause) => cause,GetHealthCheckCountError::Credentials(ref err) => err.description(),GetHealthCheckCountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHealthCheckCountError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHealthCheckLastFailureReason
                #[derive(Debug, PartialEq)]
                pub enum GetHealthCheckLastFailureReasonError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHealthCheckLastFailureReasonError {
                    pub fn from_body(body: &str) -> GetHealthCheckLastFailureReasonError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetHealthCheckLastFailureReasonError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(String::from(parsed_error.message)),_ => GetHealthCheckLastFailureReasonError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHealthCheckLastFailureReasonError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHealthCheckLastFailureReasonError {
                    fn from(err: XmlParseError) -> GetHealthCheckLastFailureReasonError {
                        let XmlParseError(message) = err;
                        GetHealthCheckLastFailureReasonError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHealthCheckLastFailureReasonError {
                    fn from(err: CredentialsError) -> GetHealthCheckLastFailureReasonError {
                        GetHealthCheckLastFailureReasonError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHealthCheckLastFailureReasonError {
                    fn from(err: HttpDispatchError) -> GetHealthCheckLastFailureReasonError {
                        GetHealthCheckLastFailureReasonError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHealthCheckLastFailureReasonError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHealthCheckLastFailureReasonError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHealthCheckLastFailureReasonError::InvalidInput(ref cause) => cause,GetHealthCheckLastFailureReasonError::NoSuchHealthCheck(ref cause) => cause,GetHealthCheckLastFailureReasonError::Validation(ref cause) => cause,GetHealthCheckLastFailureReasonError::Credentials(ref err) => err.description(),GetHealthCheckLastFailureReasonError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHealthCheckLastFailureReasonError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHealthCheckStatus
                #[derive(Debug, PartialEq)]
                pub enum GetHealthCheckStatusError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHealthCheckStatusError {
                    pub fn from_body(body: &str) -> GetHealthCheckStatusError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetHealthCheckStatusError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => GetHealthCheckStatusError::NoSuchHealthCheck(String::from(parsed_error.message)),_ => GetHealthCheckStatusError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHealthCheckStatusError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHealthCheckStatusError {
                    fn from(err: XmlParseError) -> GetHealthCheckStatusError {
                        let XmlParseError(message) = err;
                        GetHealthCheckStatusError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHealthCheckStatusError {
                    fn from(err: CredentialsError) -> GetHealthCheckStatusError {
                        GetHealthCheckStatusError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHealthCheckStatusError {
                    fn from(err: HttpDispatchError) -> GetHealthCheckStatusError {
                        GetHealthCheckStatusError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHealthCheckStatusError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHealthCheckStatusError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHealthCheckStatusError::InvalidInput(ref cause) => cause,GetHealthCheckStatusError::NoSuchHealthCheck(ref cause) => cause,GetHealthCheckStatusError::Validation(ref cause) => cause,GetHealthCheckStatusError::Credentials(ref err) => err.description(),GetHealthCheckStatusError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHealthCheckStatusError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHostedZone
                #[derive(Debug, PartialEq)]
                pub enum GetHostedZoneError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHostedZoneError {
                    pub fn from_body(body: &str) -> GetHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetHostedZoneError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => GetHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message)),_ => GetHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHostedZoneError {
                    fn from(err: XmlParseError) -> GetHostedZoneError {
                        let XmlParseError(message) = err;
                        GetHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHostedZoneError {
                    fn from(err: CredentialsError) -> GetHostedZoneError {
                        GetHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHostedZoneError {
                    fn from(err: HttpDispatchError) -> GetHostedZoneError {
                        GetHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHostedZoneError::InvalidInput(ref cause) => cause,GetHostedZoneError::NoSuchHostedZone(ref cause) => cause,GetHostedZoneError::Validation(ref cause) => cause,GetHostedZoneError::Credentials(ref err) => err.description(),GetHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetHostedZoneCount
                #[derive(Debug, PartialEq)]
                pub enum GetHostedZoneCountError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetHostedZoneCountError {
                    pub fn from_body(body: &str) -> GetHostedZoneCountError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetHostedZoneCountError::InvalidInput(String::from(parsed_error.message)),_ => GetHostedZoneCountError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetHostedZoneCountError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetHostedZoneCountError {
                    fn from(err: XmlParseError) -> GetHostedZoneCountError {
                        let XmlParseError(message) = err;
                        GetHostedZoneCountError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetHostedZoneCountError {
                    fn from(err: CredentialsError) -> GetHostedZoneCountError {
                        GetHostedZoneCountError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetHostedZoneCountError {
                    fn from(err: HttpDispatchError) -> GetHostedZoneCountError {
                        GetHostedZoneCountError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetHostedZoneCountError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetHostedZoneCountError {
                    fn description(&self) -> &str {
                        match *self {
                            GetHostedZoneCountError::InvalidInput(ref cause) => cause,GetHostedZoneCountError::Validation(ref cause) => cause,GetHostedZoneCountError::Credentials(ref err) => err.description(),GetHostedZoneCountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetHostedZoneCountError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetReusableDelegationSet
                #[derive(Debug, PartialEq)]
                pub enum GetReusableDelegationSetError {
                    
///<p>A reusable delegation set with the specified ID does not exist.</p>
DelegationSetNotReusable(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
NoSuchDelegationSet(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetReusableDelegationSetError {
                    pub fn from_body(body: &str) -> GetReusableDelegationSetError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "DelegationSetNotReusable" => GetReusableDelegationSetError::DelegationSetNotReusable(String::from(parsed_error.message)),"InvalidInput" => GetReusableDelegationSetError::InvalidInput(String::from(parsed_error.message)),"NoSuchDelegationSet" => GetReusableDelegationSetError::NoSuchDelegationSet(String::from(parsed_error.message)),_ => GetReusableDelegationSetError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetReusableDelegationSetError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetReusableDelegationSetError {
                    fn from(err: XmlParseError) -> GetReusableDelegationSetError {
                        let XmlParseError(message) = err;
                        GetReusableDelegationSetError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetReusableDelegationSetError {
                    fn from(err: CredentialsError) -> GetReusableDelegationSetError {
                        GetReusableDelegationSetError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetReusableDelegationSetError {
                    fn from(err: HttpDispatchError) -> GetReusableDelegationSetError {
                        GetReusableDelegationSetError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetReusableDelegationSetError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetReusableDelegationSetError {
                    fn description(&self) -> &str {
                        match *self {
                            GetReusableDelegationSetError::DelegationSetNotReusable(ref cause) => cause,GetReusableDelegationSetError::InvalidInput(ref cause) => cause,GetReusableDelegationSetError::NoSuchDelegationSet(ref cause) => cause,GetReusableDelegationSetError::Validation(ref cause) => cause,GetReusableDelegationSetError::Credentials(ref err) => err.description(),GetReusableDelegationSetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetReusableDelegationSetError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetTrafficPolicy
                #[derive(Debug, PartialEq)]
                pub enum GetTrafficPolicyError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetTrafficPolicyError {
                    pub fn from_body(body: &str) -> GetTrafficPolicyError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetTrafficPolicyError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => GetTrafficPolicyError::NoSuchTrafficPolicy(String::from(parsed_error.message)),_ => GetTrafficPolicyError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetTrafficPolicyError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetTrafficPolicyError {
                    fn from(err: XmlParseError) -> GetTrafficPolicyError {
                        let XmlParseError(message) = err;
                        GetTrafficPolicyError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetTrafficPolicyError {
                    fn from(err: CredentialsError) -> GetTrafficPolicyError {
                        GetTrafficPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetTrafficPolicyError {
                    fn from(err: HttpDispatchError) -> GetTrafficPolicyError {
                        GetTrafficPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetTrafficPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetTrafficPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            GetTrafficPolicyError::InvalidInput(ref cause) => cause,GetTrafficPolicyError::NoSuchTrafficPolicy(ref cause) => cause,GetTrafficPolicyError::Validation(ref cause) => cause,GetTrafficPolicyError::Credentials(ref err) => err.description(),GetTrafficPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetTrafficPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetTrafficPolicyInstance
                #[derive(Debug, PartialEq)]
                pub enum GetTrafficPolicyInstanceError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetTrafficPolicyInstanceError {
                    pub fn from_body(body: &str) -> GetTrafficPolicyInstanceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => GetTrafficPolicyInstanceError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),_ => GetTrafficPolicyInstanceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetTrafficPolicyInstanceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetTrafficPolicyInstanceError {
                    fn from(err: XmlParseError) -> GetTrafficPolicyInstanceError {
                        let XmlParseError(message) = err;
                        GetTrafficPolicyInstanceError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetTrafficPolicyInstanceError {
                    fn from(err: CredentialsError) -> GetTrafficPolicyInstanceError {
                        GetTrafficPolicyInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetTrafficPolicyInstanceError {
                    fn from(err: HttpDispatchError) -> GetTrafficPolicyInstanceError {
                        GetTrafficPolicyInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetTrafficPolicyInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetTrafficPolicyInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            GetTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,GetTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,GetTrafficPolicyInstanceError::Validation(ref cause) => cause,GetTrafficPolicyInstanceError::Credentials(ref err) => err.description(),GetTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetTrafficPolicyInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by GetTrafficPolicyInstanceCount
                #[derive(Debug, PartialEq)]
                pub enum GetTrafficPolicyInstanceCountError {
                    /// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl GetTrafficPolicyInstanceCountError {
                    pub fn from_body(body: &str) -> GetTrafficPolicyInstanceCountError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    _ => GetTrafficPolicyInstanceCountError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => GetTrafficPolicyInstanceCountError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for GetTrafficPolicyInstanceCountError {
                    fn from(err: XmlParseError) -> GetTrafficPolicyInstanceCountError {
                        let XmlParseError(message) = err;
                        GetTrafficPolicyInstanceCountError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for GetTrafficPolicyInstanceCountError {
                    fn from(err: CredentialsError) -> GetTrafficPolicyInstanceCountError {
                        GetTrafficPolicyInstanceCountError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for GetTrafficPolicyInstanceCountError {
                    fn from(err: HttpDispatchError) -> GetTrafficPolicyInstanceCountError {
                        GetTrafficPolicyInstanceCountError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for GetTrafficPolicyInstanceCountError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for GetTrafficPolicyInstanceCountError {
                    fn description(&self) -> &str {
                        match *self {
                            GetTrafficPolicyInstanceCountError::Validation(ref cause) => cause,GetTrafficPolicyInstanceCountError::Credentials(ref err) => err.description(),GetTrafficPolicyInstanceCountError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),GetTrafficPolicyInstanceCountError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListGeoLocations
                #[derive(Debug, PartialEq)]
                pub enum ListGeoLocationsError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListGeoLocationsError {
                    pub fn from_body(body: &str) -> ListGeoLocationsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListGeoLocationsError::InvalidInput(String::from(parsed_error.message)),_ => ListGeoLocationsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListGeoLocationsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListGeoLocationsError {
                    fn from(err: XmlParseError) -> ListGeoLocationsError {
                        let XmlParseError(message) = err;
                        ListGeoLocationsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListGeoLocationsError {
                    fn from(err: CredentialsError) -> ListGeoLocationsError {
                        ListGeoLocationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListGeoLocationsError {
                    fn from(err: HttpDispatchError) -> ListGeoLocationsError {
                        ListGeoLocationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListGeoLocationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListGeoLocationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListGeoLocationsError::InvalidInput(ref cause) => cause,ListGeoLocationsError::Validation(ref cause) => cause,ListGeoLocationsError::Credentials(ref err) => err.description(),ListGeoLocationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListGeoLocationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListHealthChecks
                #[derive(Debug, PartialEq)]
                pub enum ListHealthChecksError {
                    
///<p>The resource you are trying to access is unsupported on this Amazon Route 53 endpoint. Please consider using a newer endpoint or a tool that does so.</p>
IncompatibleVersion(String),
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListHealthChecksError {
                    pub fn from_body(body: &str) -> ListHealthChecksError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "IncompatibleVersion" => ListHealthChecksError::IncompatibleVersion(String::from(parsed_error.message)),"InvalidInput" => ListHealthChecksError::InvalidInput(String::from(parsed_error.message)),_ => ListHealthChecksError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListHealthChecksError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListHealthChecksError {
                    fn from(err: XmlParseError) -> ListHealthChecksError {
                        let XmlParseError(message) = err;
                        ListHealthChecksError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListHealthChecksError {
                    fn from(err: CredentialsError) -> ListHealthChecksError {
                        ListHealthChecksError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListHealthChecksError {
                    fn from(err: HttpDispatchError) -> ListHealthChecksError {
                        ListHealthChecksError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListHealthChecksError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListHealthChecksError {
                    fn description(&self) -> &str {
                        match *self {
                            ListHealthChecksError::IncompatibleVersion(ref cause) => cause,ListHealthChecksError::InvalidInput(ref cause) => cause,ListHealthChecksError::Validation(ref cause) => cause,ListHealthChecksError::Credentials(ref err) => err.description(),ListHealthChecksError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListHealthChecksError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListHostedZones
                #[derive(Debug, PartialEq)]
                pub enum ListHostedZonesError {
                    
///<p>A reusable delegation set with the specified ID does not exist.</p>
DelegationSetNotReusable(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>A reusable delegation set with the specified ID does not exist.</p>
NoSuchDelegationSet(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListHostedZonesError {
                    pub fn from_body(body: &str) -> ListHostedZonesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "DelegationSetNotReusable" => ListHostedZonesError::DelegationSetNotReusable(String::from(parsed_error.message)),"InvalidInput" => ListHostedZonesError::InvalidInput(String::from(parsed_error.message)),"NoSuchDelegationSet" => ListHostedZonesError::NoSuchDelegationSet(String::from(parsed_error.message)),_ => ListHostedZonesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListHostedZonesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListHostedZonesError {
                    fn from(err: XmlParseError) -> ListHostedZonesError {
                        let XmlParseError(message) = err;
                        ListHostedZonesError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListHostedZonesError {
                    fn from(err: CredentialsError) -> ListHostedZonesError {
                        ListHostedZonesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListHostedZonesError {
                    fn from(err: HttpDispatchError) -> ListHostedZonesError {
                        ListHostedZonesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListHostedZonesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListHostedZonesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListHostedZonesError::DelegationSetNotReusable(ref cause) => cause,ListHostedZonesError::InvalidInput(ref cause) => cause,ListHostedZonesError::NoSuchDelegationSet(ref cause) => cause,ListHostedZonesError::Validation(ref cause) => cause,ListHostedZonesError::Credentials(ref err) => err.description(),ListHostedZonesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListHostedZonesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListHostedZonesByName
                #[derive(Debug, PartialEq)]
                pub enum ListHostedZonesByNameError {
                    
///<p>The specified domain name is not valid.</p>
InvalidDomainName(String),
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListHostedZonesByNameError {
                    pub fn from_body(body: &str) -> ListHostedZonesByNameError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidDomainName" => ListHostedZonesByNameError::InvalidDomainName(String::from(parsed_error.message)),"InvalidInput" => ListHostedZonesByNameError::InvalidInput(String::from(parsed_error.message)),_ => ListHostedZonesByNameError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListHostedZonesByNameError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListHostedZonesByNameError {
                    fn from(err: XmlParseError) -> ListHostedZonesByNameError {
                        let XmlParseError(message) = err;
                        ListHostedZonesByNameError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListHostedZonesByNameError {
                    fn from(err: CredentialsError) -> ListHostedZonesByNameError {
                        ListHostedZonesByNameError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListHostedZonesByNameError {
                    fn from(err: HttpDispatchError) -> ListHostedZonesByNameError {
                        ListHostedZonesByNameError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListHostedZonesByNameError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListHostedZonesByNameError {
                    fn description(&self) -> &str {
                        match *self {
                            ListHostedZonesByNameError::InvalidDomainName(ref cause) => cause,ListHostedZonesByNameError::InvalidInput(ref cause) => cause,ListHostedZonesByNameError::Validation(ref cause) => cause,ListHostedZonesByNameError::Credentials(ref err) => err.description(),ListHostedZonesByNameError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListHostedZonesByNameError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListResourceRecordSets
                #[derive(Debug, PartialEq)]
                pub enum ListResourceRecordSetsError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListResourceRecordSetsError {
                    pub fn from_body(body: &str) -> ListResourceRecordSetsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListResourceRecordSetsError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => ListResourceRecordSetsError::NoSuchHostedZone(String::from(parsed_error.message)),_ => ListResourceRecordSetsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListResourceRecordSetsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListResourceRecordSetsError {
                    fn from(err: XmlParseError) -> ListResourceRecordSetsError {
                        let XmlParseError(message) = err;
                        ListResourceRecordSetsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListResourceRecordSetsError {
                    fn from(err: CredentialsError) -> ListResourceRecordSetsError {
                        ListResourceRecordSetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListResourceRecordSetsError {
                    fn from(err: HttpDispatchError) -> ListResourceRecordSetsError {
                        ListResourceRecordSetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListResourceRecordSetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListResourceRecordSetsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListResourceRecordSetsError::InvalidInput(ref cause) => cause,ListResourceRecordSetsError::NoSuchHostedZone(ref cause) => cause,ListResourceRecordSetsError::Validation(ref cause) => cause,ListResourceRecordSetsError::Credentials(ref err) => err.description(),ListResourceRecordSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListResourceRecordSetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListReusableDelegationSets
                #[derive(Debug, PartialEq)]
                pub enum ListReusableDelegationSetsError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListReusableDelegationSetsError {
                    pub fn from_body(body: &str) -> ListReusableDelegationSetsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListReusableDelegationSetsError::InvalidInput(String::from(parsed_error.message)),_ => ListReusableDelegationSetsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListReusableDelegationSetsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListReusableDelegationSetsError {
                    fn from(err: XmlParseError) -> ListReusableDelegationSetsError {
                        let XmlParseError(message) = err;
                        ListReusableDelegationSetsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListReusableDelegationSetsError {
                    fn from(err: CredentialsError) -> ListReusableDelegationSetsError {
                        ListReusableDelegationSetsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListReusableDelegationSetsError {
                    fn from(err: HttpDispatchError) -> ListReusableDelegationSetsError {
                        ListReusableDelegationSetsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListReusableDelegationSetsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListReusableDelegationSetsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListReusableDelegationSetsError::InvalidInput(ref cause) => cause,ListReusableDelegationSetsError::Validation(ref cause) => cause,ListReusableDelegationSetsError::Credentials(ref err) => err.description(),ListReusableDelegationSetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListReusableDelegationSetsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTagsForResource
                #[derive(Debug, PartialEq)]
                pub enum ListTagsForResourceError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),
///<p></p>
Throttling(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTagsForResourceError {
                    pub fn from_body(body: &str) -> ListTagsForResourceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTagsForResourceError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => ListTagsForResourceError::NoSuchHealthCheck(String::from(parsed_error.message)),"NoSuchHostedZone" => ListTagsForResourceError::NoSuchHostedZone(String::from(parsed_error.message)),"PriorRequestNotComplete" => ListTagsForResourceError::PriorRequestNotComplete(String::from(parsed_error.message)),"ThrottlingException" => ListTagsForResourceError::Throttling(String::from(parsed_error.message)),_ => ListTagsForResourceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTagsForResourceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTagsForResourceError {
                    fn from(err: XmlParseError) -> ListTagsForResourceError {
                        let XmlParseError(message) = err;
                        ListTagsForResourceError::Unknown(message.to_string())
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
                            ListTagsForResourceError::InvalidInput(ref cause) => cause,ListTagsForResourceError::NoSuchHealthCheck(ref cause) => cause,ListTagsForResourceError::NoSuchHostedZone(ref cause) => cause,ListTagsForResourceError::PriorRequestNotComplete(ref cause) => cause,ListTagsForResourceError::Throttling(ref cause) => cause,ListTagsForResourceError::Validation(ref cause) => cause,ListTagsForResourceError::Credentials(ref err) => err.description(),ListTagsForResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTagsForResourceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTagsForResources
                #[derive(Debug, PartialEq)]
                pub enum ListTagsForResourcesError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),
///<p></p>
Throttling(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTagsForResourcesError {
                    pub fn from_body(body: &str) -> ListTagsForResourcesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTagsForResourcesError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => ListTagsForResourcesError::NoSuchHealthCheck(String::from(parsed_error.message)),"NoSuchHostedZone" => ListTagsForResourcesError::NoSuchHostedZone(String::from(parsed_error.message)),"PriorRequestNotComplete" => ListTagsForResourcesError::PriorRequestNotComplete(String::from(parsed_error.message)),"ThrottlingException" => ListTagsForResourcesError::Throttling(String::from(parsed_error.message)),_ => ListTagsForResourcesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTagsForResourcesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTagsForResourcesError {
                    fn from(err: XmlParseError) -> ListTagsForResourcesError {
                        let XmlParseError(message) = err;
                        ListTagsForResourcesError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTagsForResourcesError {
                    fn from(err: CredentialsError) -> ListTagsForResourcesError {
                        ListTagsForResourcesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTagsForResourcesError {
                    fn from(err: HttpDispatchError) -> ListTagsForResourcesError {
                        ListTagsForResourcesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTagsForResourcesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTagsForResourcesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTagsForResourcesError::InvalidInput(ref cause) => cause,ListTagsForResourcesError::NoSuchHealthCheck(ref cause) => cause,ListTagsForResourcesError::NoSuchHostedZone(ref cause) => cause,ListTagsForResourcesError::PriorRequestNotComplete(ref cause) => cause,ListTagsForResourcesError::Throttling(ref cause) => cause,ListTagsForResourcesError::Validation(ref cause) => cause,ListTagsForResourcesError::Credentials(ref err) => err.description(),ListTagsForResourcesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTagsForResourcesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTrafficPolicies
                #[derive(Debug, PartialEq)]
                pub enum ListTrafficPoliciesError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTrafficPoliciesError {
                    pub fn from_body(body: &str) -> ListTrafficPoliciesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTrafficPoliciesError::InvalidInput(String::from(parsed_error.message)),_ => ListTrafficPoliciesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTrafficPoliciesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTrafficPoliciesError {
                    fn from(err: XmlParseError) -> ListTrafficPoliciesError {
                        let XmlParseError(message) = err;
                        ListTrafficPoliciesError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTrafficPoliciesError {
                    fn from(err: CredentialsError) -> ListTrafficPoliciesError {
                        ListTrafficPoliciesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTrafficPoliciesError {
                    fn from(err: HttpDispatchError) -> ListTrafficPoliciesError {
                        ListTrafficPoliciesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTrafficPoliciesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTrafficPoliciesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTrafficPoliciesError::InvalidInput(ref cause) => cause,ListTrafficPoliciesError::Validation(ref cause) => cause,ListTrafficPoliciesError::Credentials(ref err) => err.description(),ListTrafficPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTrafficPoliciesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTrafficPolicyInstances
                #[derive(Debug, PartialEq)]
                pub enum ListTrafficPolicyInstancesError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTrafficPolicyInstancesError {
                    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTrafficPolicyInstancesError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),_ => ListTrafficPolicyInstancesError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTrafficPolicyInstancesError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTrafficPolicyInstancesError {
                    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesError {
                        let XmlParseError(message) = err;
                        ListTrafficPolicyInstancesError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTrafficPolicyInstancesError {
                    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesError {
                        ListTrafficPolicyInstancesError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTrafficPolicyInstancesError {
                    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesError {
                        ListTrafficPolicyInstancesError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTrafficPolicyInstancesError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTrafficPolicyInstancesError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTrafficPolicyInstancesError::InvalidInput(ref cause) => cause,ListTrafficPolicyInstancesError::NoSuchTrafficPolicyInstance(ref cause) => cause,ListTrafficPolicyInstancesError::Validation(ref cause) => cause,ListTrafficPolicyInstancesError::Credentials(ref err) => err.description(),ListTrafficPolicyInstancesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTrafficPolicyInstancesError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTrafficPolicyInstancesByHostedZone
                #[derive(Debug, PartialEq)]
                pub enum ListTrafficPolicyInstancesByHostedZoneError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTrafficPolicyInstancesByHostedZoneError {
                    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesByHostedZoneError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),_ => ListTrafficPolicyInstancesByHostedZoneError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTrafficPolicyInstancesByHostedZoneError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTrafficPolicyInstancesByHostedZoneError {
                    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesByHostedZoneError {
                        let XmlParseError(message) = err;
                        ListTrafficPolicyInstancesByHostedZoneError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTrafficPolicyInstancesByHostedZoneError {
                    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesByHostedZoneError {
                        ListTrafficPolicyInstancesByHostedZoneError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTrafficPolicyInstancesByHostedZoneError {
                    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesByHostedZoneError {
                        ListTrafficPolicyInstancesByHostedZoneError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTrafficPolicyInstancesByHostedZoneError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTrafficPolicyInstancesByHostedZoneError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTrafficPolicyInstancesByHostedZoneError::InvalidInput(ref cause) => cause,ListTrafficPolicyInstancesByHostedZoneError::NoSuchHostedZone(ref cause) => cause,ListTrafficPolicyInstancesByHostedZoneError::NoSuchTrafficPolicyInstance(ref cause) => cause,ListTrafficPolicyInstancesByHostedZoneError::Validation(ref cause) => cause,ListTrafficPolicyInstancesByHostedZoneError::Credentials(ref err) => err.description(),ListTrafficPolicyInstancesByHostedZoneError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTrafficPolicyInstancesByHostedZoneError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTrafficPolicyInstancesByPolicy
                #[derive(Debug, PartialEq)]
                pub enum ListTrafficPolicyInstancesByPolicyError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTrafficPolicyInstancesByPolicyError {
                    pub fn from_body(body: &str) -> ListTrafficPolicyInstancesByPolicyError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTrafficPolicyInstancesByPolicyError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),_ => ListTrafficPolicyInstancesByPolicyError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTrafficPolicyInstancesByPolicyError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTrafficPolicyInstancesByPolicyError {
                    fn from(err: XmlParseError) -> ListTrafficPolicyInstancesByPolicyError {
                        let XmlParseError(message) = err;
                        ListTrafficPolicyInstancesByPolicyError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTrafficPolicyInstancesByPolicyError {
                    fn from(err: CredentialsError) -> ListTrafficPolicyInstancesByPolicyError {
                        ListTrafficPolicyInstancesByPolicyError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTrafficPolicyInstancesByPolicyError {
                    fn from(err: HttpDispatchError) -> ListTrafficPolicyInstancesByPolicyError {
                        ListTrafficPolicyInstancesByPolicyError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTrafficPolicyInstancesByPolicyError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTrafficPolicyInstancesByPolicyError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTrafficPolicyInstancesByPolicyError::InvalidInput(ref cause) => cause,ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy(ref cause) => cause,ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance(ref cause) => cause,ListTrafficPolicyInstancesByPolicyError::Validation(ref cause) => cause,ListTrafficPolicyInstancesByPolicyError::Credentials(ref err) => err.description(),ListTrafficPolicyInstancesByPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTrafficPolicyInstancesByPolicyError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListTrafficPolicyVersions
                #[derive(Debug, PartialEq)]
                pub enum ListTrafficPolicyVersionsError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListTrafficPolicyVersionsError {
                    pub fn from_body(body: &str) -> ListTrafficPolicyVersionsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListTrafficPolicyVersionsError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(String::from(parsed_error.message)),_ => ListTrafficPolicyVersionsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListTrafficPolicyVersionsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListTrafficPolicyVersionsError {
                    fn from(err: XmlParseError) -> ListTrafficPolicyVersionsError {
                        let XmlParseError(message) = err;
                        ListTrafficPolicyVersionsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListTrafficPolicyVersionsError {
                    fn from(err: CredentialsError) -> ListTrafficPolicyVersionsError {
                        ListTrafficPolicyVersionsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListTrafficPolicyVersionsError {
                    fn from(err: HttpDispatchError) -> ListTrafficPolicyVersionsError {
                        ListTrafficPolicyVersionsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListTrafficPolicyVersionsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListTrafficPolicyVersionsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListTrafficPolicyVersionsError::InvalidInput(ref cause) => cause,ListTrafficPolicyVersionsError::NoSuchTrafficPolicy(ref cause) => cause,ListTrafficPolicyVersionsError::Validation(ref cause) => cause,ListTrafficPolicyVersionsError::Credentials(ref err) => err.description(),ListTrafficPolicyVersionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListTrafficPolicyVersionsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by ListVPCAssociationAuthorizations
                #[derive(Debug, PartialEq)]
                pub enum ListVPCAssociationAuthorizationsError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///
InvalidPaginationToken(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl ListVPCAssociationAuthorizationsError {
                    pub fn from_body(body: &str) -> ListVPCAssociationAuthorizationsError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => ListVPCAssociationAuthorizationsError::InvalidInput(String::from(parsed_error.message)),"InvalidPaginationToken" => ListVPCAssociationAuthorizationsError::InvalidPaginationToken(String::from(parsed_error.message)),"NoSuchHostedZone" => ListVPCAssociationAuthorizationsError::NoSuchHostedZone(String::from(parsed_error.message)),_ => ListVPCAssociationAuthorizationsError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => ListVPCAssociationAuthorizationsError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for ListVPCAssociationAuthorizationsError {
                    fn from(err: XmlParseError) -> ListVPCAssociationAuthorizationsError {
                        let XmlParseError(message) = err;
                        ListVPCAssociationAuthorizationsError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for ListVPCAssociationAuthorizationsError {
                    fn from(err: CredentialsError) -> ListVPCAssociationAuthorizationsError {
                        ListVPCAssociationAuthorizationsError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for ListVPCAssociationAuthorizationsError {
                    fn from(err: HttpDispatchError) -> ListVPCAssociationAuthorizationsError {
                        ListVPCAssociationAuthorizationsError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for ListVPCAssociationAuthorizationsError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for ListVPCAssociationAuthorizationsError {
                    fn description(&self) -> &str {
                        match *self {
                            ListVPCAssociationAuthorizationsError::InvalidInput(ref cause) => cause,ListVPCAssociationAuthorizationsError::InvalidPaginationToken(ref cause) => cause,ListVPCAssociationAuthorizationsError::NoSuchHostedZone(ref cause) => cause,ListVPCAssociationAuthorizationsError::Validation(ref cause) => cause,ListVPCAssociationAuthorizationsError::Credentials(ref err) => err.description(),ListVPCAssociationAuthorizationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),ListVPCAssociationAuthorizationsError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by TestDNSAnswer
                #[derive(Debug, PartialEq)]
                pub enum TestDNSAnswerError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl TestDNSAnswerError {
                    pub fn from_body(body: &str) -> TestDNSAnswerError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => TestDNSAnswerError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => TestDNSAnswerError::NoSuchHostedZone(String::from(parsed_error.message)),_ => TestDNSAnswerError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => TestDNSAnswerError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for TestDNSAnswerError {
                    fn from(err: XmlParseError) -> TestDNSAnswerError {
                        let XmlParseError(message) = err;
                        TestDNSAnswerError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for TestDNSAnswerError {
                    fn from(err: CredentialsError) -> TestDNSAnswerError {
                        TestDNSAnswerError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for TestDNSAnswerError {
                    fn from(err: HttpDispatchError) -> TestDNSAnswerError {
                        TestDNSAnswerError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for TestDNSAnswerError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for TestDNSAnswerError {
                    fn description(&self) -> &str {
                        match *self {
                            TestDNSAnswerError::InvalidInput(ref cause) => cause,TestDNSAnswerError::NoSuchHostedZone(ref cause) => cause,TestDNSAnswerError::Validation(ref cause) => cause,TestDNSAnswerError::Credentials(ref err) => err.description(),TestDNSAnswerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),TestDNSAnswerError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateHealthCheck
                #[derive(Debug, PartialEq)]
                pub enum UpdateHealthCheckError {
                    
///<p>The value of <code>HealthCheckVersion</code> in the request doesn't match the value of <code>HealthCheckVersion</code> in the health check.</p>
HealthCheckVersionMismatch(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No health check exists with the ID that you specified in the <code>DeleteHealthCheck</code> request.</p>
NoSuchHealthCheck(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateHealthCheckError {
                    pub fn from_body(body: &str) -> UpdateHealthCheckError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "HealthCheckVersionMismatch" => UpdateHealthCheckError::HealthCheckVersionMismatch(String::from(parsed_error.message)),"InvalidInput" => UpdateHealthCheckError::InvalidInput(String::from(parsed_error.message)),"NoSuchHealthCheck" => UpdateHealthCheckError::NoSuchHealthCheck(String::from(parsed_error.message)),_ => UpdateHealthCheckError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateHealthCheckError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for UpdateHealthCheckError {
                    fn from(err: XmlParseError) -> UpdateHealthCheckError {
                        let XmlParseError(message) = err;
                        UpdateHealthCheckError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for UpdateHealthCheckError {
                    fn from(err: CredentialsError) -> UpdateHealthCheckError {
                        UpdateHealthCheckError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateHealthCheckError {
                    fn from(err: HttpDispatchError) -> UpdateHealthCheckError {
                        UpdateHealthCheckError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateHealthCheckError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateHealthCheckError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateHealthCheckError::HealthCheckVersionMismatch(ref cause) => cause,UpdateHealthCheckError::InvalidInput(ref cause) => cause,UpdateHealthCheckError::NoSuchHealthCheck(ref cause) => cause,UpdateHealthCheckError::Validation(ref cause) => cause,UpdateHealthCheckError::Credentials(ref err) => err.description(),UpdateHealthCheckError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateHealthCheckError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateHostedZoneComment
                #[derive(Debug, PartialEq)]
                pub enum UpdateHostedZoneCommentError {
                    
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No hosted zone exists with the ID that you specified.</p>
NoSuchHostedZone(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateHostedZoneCommentError {
                    pub fn from_body(body: &str) -> UpdateHostedZoneCommentError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "InvalidInput" => UpdateHostedZoneCommentError::InvalidInput(String::from(parsed_error.message)),"NoSuchHostedZone" => UpdateHostedZoneCommentError::NoSuchHostedZone(String::from(parsed_error.message)),_ => UpdateHostedZoneCommentError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateHostedZoneCommentError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for UpdateHostedZoneCommentError {
                    fn from(err: XmlParseError) -> UpdateHostedZoneCommentError {
                        let XmlParseError(message) = err;
                        UpdateHostedZoneCommentError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for UpdateHostedZoneCommentError {
                    fn from(err: CredentialsError) -> UpdateHostedZoneCommentError {
                        UpdateHostedZoneCommentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateHostedZoneCommentError {
                    fn from(err: HttpDispatchError) -> UpdateHostedZoneCommentError {
                        UpdateHostedZoneCommentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateHostedZoneCommentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateHostedZoneCommentError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateHostedZoneCommentError::InvalidInput(ref cause) => cause,UpdateHostedZoneCommentError::NoSuchHostedZone(ref cause) => cause,UpdateHostedZoneCommentError::Validation(ref cause) => cause,UpdateHostedZoneCommentError::Credentials(ref err) => err.description(),UpdateHostedZoneCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateHostedZoneCommentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateTrafficPolicyComment
                #[derive(Debug, PartialEq)]
                pub enum UpdateTrafficPolicyCommentError {
                    
///<p>Another user submitted a request to update the object at the same time that you did. Retry the request. </p>
ConcurrentModification(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateTrafficPolicyCommentError {
                    pub fn from_body(body: &str) -> UpdateTrafficPolicyCommentError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConcurrentModification" => UpdateTrafficPolicyCommentError::ConcurrentModification(String::from(parsed_error.message)),"InvalidInput" => UpdateTrafficPolicyCommentError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(String::from(parsed_error.message)),_ => UpdateTrafficPolicyCommentError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateTrafficPolicyCommentError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for UpdateTrafficPolicyCommentError {
                    fn from(err: XmlParseError) -> UpdateTrafficPolicyCommentError {
                        let XmlParseError(message) = err;
                        UpdateTrafficPolicyCommentError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for UpdateTrafficPolicyCommentError {
                    fn from(err: CredentialsError) -> UpdateTrafficPolicyCommentError {
                        UpdateTrafficPolicyCommentError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateTrafficPolicyCommentError {
                    fn from(err: HttpDispatchError) -> UpdateTrafficPolicyCommentError {
                        UpdateTrafficPolicyCommentError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateTrafficPolicyCommentError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateTrafficPolicyCommentError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateTrafficPolicyCommentError::ConcurrentModification(ref cause) => cause,UpdateTrafficPolicyCommentError::InvalidInput(ref cause) => cause,UpdateTrafficPolicyCommentError::NoSuchTrafficPolicy(ref cause) => cause,UpdateTrafficPolicyCommentError::Validation(ref cause) => cause,UpdateTrafficPolicyCommentError::Credentials(ref err) => err.description(),UpdateTrafficPolicyCommentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateTrafficPolicyCommentError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// Errors returned by UpdateTrafficPolicyInstance
                #[derive(Debug, PartialEq)]
                pub enum UpdateTrafficPolicyInstanceError {
                    
///<p>You tried to update a traffic policy instance by using a traffic policy version that has a different DNS type than the current type for the instance. You specified the type in the JSON document in the <code>CreateTrafficPolicy</code> or <code>CreateTrafficPolicyVersion</code>request. </p>
ConflictingTypes(String),
///<p>The input is not valid.</p>
InvalidInput(String),
///<p>No traffic policy exists with the specified ID.</p>
NoSuchTrafficPolicy(String),
///<p>No traffic policy instance exists with the specified ID.</p>
NoSuchTrafficPolicyInstance(String),
///<p>If Amazon Route 53 can't process a request before the next request arrives, it will reject subsequent requests for the same hosted zone and return an <code>HTTP 400 error</code> (<code>Bad request</code>). If Amazon Route 53 returns this error repeatedly for the same request, we recommend that you wait, in intervals of increasing duration, before you try the request again.</p>
PriorRequestNotComplete(String),/// An error occurred dispatching the HTTP request
HttpDispatch(HttpDispatchError),/// An error was encountered with AWS credentials.
Credentials(CredentialsError),/// A validation error occurred.  Details from AWS are provided.
Validation(String),/// An unknown error occurred.  The raw HTTP response is provided.
Unknown(String)
                }

                
                impl UpdateTrafficPolicyInstanceError {
                    pub fn from_body(body: &str) -> UpdateTrafficPolicyInstanceError {
                        let reader = EventReader::new(body.as_bytes());
                        let mut stack = XmlResponse::new(reader.into_iter().peekable());
                        let _start_document = stack.next();
                        let _response_envelope = stack.next();
                        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
                            Ok(parsed_error) => {
                                match &parsed_error.code[..] {
                                    "ConflictingTypes" => UpdateTrafficPolicyInstanceError::ConflictingTypes(String::from(parsed_error.message)),"InvalidInput" => UpdateTrafficPolicyInstanceError::InvalidInput(String::from(parsed_error.message)),"NoSuchTrafficPolicy" => UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(String::from(parsed_error.message)),"NoSuchTrafficPolicyInstance" => UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(String::from(parsed_error.message)),"PriorRequestNotComplete" => UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(String::from(parsed_error.message)),_ => UpdateTrafficPolicyInstanceError::Unknown(String::from(body))
                                }
                           },
                           Err(_) => UpdateTrafficPolicyInstanceError::Unknown(body.to_string())
                       }
                    }
                }
                
                impl From<XmlParseError> for UpdateTrafficPolicyInstanceError {
                    fn from(err: XmlParseError) -> UpdateTrafficPolicyInstanceError {
                        let XmlParseError(message) = err;
                        UpdateTrafficPolicyInstanceError::Unknown(message.to_string())
                    }
                }
                impl From<CredentialsError> for UpdateTrafficPolicyInstanceError {
                    fn from(err: CredentialsError) -> UpdateTrafficPolicyInstanceError {
                        UpdateTrafficPolicyInstanceError::Credentials(err)
                    }
                }
                impl From<HttpDispatchError> for UpdateTrafficPolicyInstanceError {
                    fn from(err: HttpDispatchError) -> UpdateTrafficPolicyInstanceError {
                        UpdateTrafficPolicyInstanceError::HttpDispatch(err)
                    }
                }
                impl fmt::Display for UpdateTrafficPolicyInstanceError {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}", self.description())
                    }
                }
                impl Error for UpdateTrafficPolicyInstanceError {
                    fn description(&self) -> &str {
                        match *self {
                            UpdateTrafficPolicyInstanceError::ConflictingTypes(ref cause) => cause,UpdateTrafficPolicyInstanceError::InvalidInput(ref cause) => cause,UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicy(ref cause) => cause,UpdateTrafficPolicyInstanceError::NoSuchTrafficPolicyInstance(ref cause) => cause,UpdateTrafficPolicyInstanceError::PriorRequestNotComplete(ref cause) => cause,UpdateTrafficPolicyInstanceError::Validation(ref cause) => cause,UpdateTrafficPolicyInstanceError::Credentials(ref err) => err.description(),UpdateTrafficPolicyInstanceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),UpdateTrafficPolicyInstanceError::Unknown(ref cause) => cause
                        }
                    }
                 }
/// A client for the Route 53 API.
        pub struct Route53Client<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            credentials_provider: P,
            region: region::Region,
            dispatcher: D,
        }

        impl<P, D> Route53Client<P, D> where P: ProvideAwsCredentials, D: DispatchSignedRequest {
            pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
                  Route53Client {
                    credentials_provider: credentials_provider,
                    region: region,
                    dispatcher: request_dispatcher
                }
            }

        
#[doc="<p>Associates an Amazon VPC with a private hosted zone. </p> <important> <p>To perform the association, the VPC and the private hosted zone must already exist. You can't convert a public hosted zone into a private hosted zone.</p> </important> <p>Send a <code>POST</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/associatevpc</code> resource. The request body must include a document with an <code>AssociateVPCWithHostedZoneRequest</code> element. The response contains a <code>ChangeInfo</code> data type that you can use to track the progress of the request. </p> <note> <p>If you want to associate a VPC that was created by using one AWS account with a private hosted zone that was created by using a different account, the AWS account that created the private hosted zone must first submit a <code>CreateVPCAssociationAuthorization</code> request. Then the account that created the VPC must submit an <code>AssociateVPCWithHostedZone</code> request.</p> </note>"]
                #[allow(unused_variables, warnings)]
                pub fn associate_vpc_with_hosted_zone(&self, input: &AssociateVPCWithHostedZoneRequest) -> Result<AssociateVPCWithHostedZoneResponse, AssociateVPCWithHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/associatevpc".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = AssociateVPCWithHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(AssociateVPCWithHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(AssociateVPCWithHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Create, change, update, or delete authoritative DNS information on all Amazon Route 53 servers. Send a <code>POST</code> request to: </p> <p> <code>/2013-04-01/hostedzone/<i>Amazon Route 53 hosted Zone ID</i>/rrset</code> resource. </p> <p>The request body must include a document with a <code>ChangeResourceRecordSetsRequest</code> element. The request body contains a list of change items, known as a change batch. Change batches are considered transactional changes. When using the Amazon Route 53 API to change resource record sets, Amazon Route 53 either makes all or none of the changes in a change batch request. This ensures that Amazon Route 53 never partially implements the intended changes to the resource record sets in a hosted zone. </p> <p>For example, a change batch request that deletes the <code>CNAME</code> record for www.example.com and creates an alias resource record set for www.example.com. Amazon Route 53 deletes the first resource record set and creates the second resource record set in a single operation. If either the <code>DELETE</code> or the <code>CREATE</code> action fails, then both changes (plus any other changes in the batch) fail, and the original <code>CNAME</code> record continues to exist.</p> <important> <p>Due to the nature of transactional changes, you can't delete the same resource record set more than once in a single change batch. If you attempt to delete the same change batch more than once, Amazon Route 53 returns an <code>InvalidChangeBatch</code> error.</p> </important> <note> <p>To create resource record sets for complex routing configurations, use either the traffic flow visual editor in the Amazon Route 53 console or the API actions for traffic policies and traffic policy instances. Save the configuration as a traffic policy, then associate the traffic policy with one or more domain names (such as example.com) or subdomain names (such as www.example.com), in the same hosted zone or in multiple hosted zones. You can roll back the updates if the new configuration isn't performing as expected. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/traffic-flow.html\">Using Traffic Flow to Route DNS Traffic</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </note> <p>Use <code>ChangeResourceRecordsSetsRequest</code> to perform the following actions:</p> <ul> <li> <p> <code>CREATE</code>: Creates a resource record set that has the specified values.</p> </li> <li> <p> <code>DELETE</code>: Deletes an existing resource record set that has the specified values.</p> </li> <li> <p> <code>UPSERT</code>: If a resource record set does not already exist, AWS creates it. If a resource set does exist, Amazon Route 53 updates it with the values in the request. </p> </li> </ul> <p>The values that you need to include in the request depend on the type of resource record set that you're creating, deleting, or updating:</p> <p> <b>Basic resource record sets (excluding alias, failover, geolocation, latency, and weighted resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>TTL</code> </p> </li> </ul> <p> <b>Failover, geolocation, latency, or weighted resource record sets (excluding alias resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>TTL</code> </p> </li> <li> <p> <code>SetIdentifier</code> </p> </li> </ul> <p> <b>Alias resource record sets (including failover alias, geolocation alias, latency alias, and weighted alias resource record sets)</b> </p> <ul> <li> <p> <code>Name</code> </p> </li> <li> <p> <code>Type</code> </p> </li> <li> <p> <code>AliasTarget</code> (includes <code>DNSName</code>, <code>EvaluateTargetHealth</code>, and <code>HostedZoneId</code>)</p> </li> <li> <p> <code>SetIdentifier</code> (for failover, geolocation, latency, and weighted resource record sets)</p> </li> </ul> <p>When you submit a <code>ChangeResourceRecordSets</code> request, Amazon Route 53 propagates your changes to all of the Amazon Route 53 authoritative DNS servers. While your changes are propagating, <code>GetChange</code> returns a status of <code>PENDING</code>. When propagation is complete, <code>GetChange</code> returns a status of <code>INSYNC</code>. Changes generally propagate to all Amazon Route 53 name servers in a few minutes. In rare circumstances, propagation can take up to 30 minutes. For more information, see <a>GetChange</a> </p> <p>For information about the limits on a <code>ChangeResourceRecordSets</code> request, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html\">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn change_resource_record_sets(&self, input: &ChangeResourceRecordSetsRequest) -> Result<ChangeResourceRecordSetsResponse, ChangeResourceRecordSetsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/rrset/".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ChangeResourceRecordSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ChangeResourceRecordSetsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ChangeResourceRecordSetsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Adds, edits, or deletes tags for a health check or a hosted zone.</p> <p>For information about using tags for cost allocation, see <a href=\"http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html\">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn change_tags_for_resource(&self, input: &ChangeTagsForResourceRequest) -> Result<ChangeTagsForResourceResponse, ChangeTagsForResourceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/tags/{ResourceType}/{ResourceId}".to_string();

                    
                    request_uri = request_uri.replace("{ResourceId}", &input.resource_id.to_string());
request_uri = request_uri.replace("{ResourceType}", &input.resource_type.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ChangeTagsForResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ChangeTagsForResourceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ChangeTagsForResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates a new health check.</p> <p>To create a new health check, send a <code>POST</code> request to the <code>/2013-04-01/healthcheck</code> resource. The request body must include a document with a <code>CreateHealthCheckRequest</code> element. The response returns the <code>CreateHealthCheckResponse</code> element, containing the health check ID specified when adding health check to a resource record set. For information about adding health checks to resource record sets, see <a>ResourceRecordSet$HealthCheckId</a> in <a>ChangeResourceRecordSets</a>. </p> <p>If you are registering EC2 instances with an Elastic Load Balancing (ELB) load balancer, do not create Amazon Route 53 health checks for the EC2 instances. When you register an EC2 instance with a load balancer, you configure settings for an ELB health check, which performs a similar function to an Amazon Route 53 health check.</p> <p>You can associate health checks with failover resource record sets in a private hosted zone. Note the following:</p> <ul> <li> <p>Amazon Route 53 health checkers are outside the VPC. To check the health of an endpoint within a VPC by IP address, you must assign a public IP address to the instance in the VPC.</p> </li> <li> <p>You can configure a health checker to check the health of an external resource that the instance relies on, such as a database server.</p> </li> <li> <p>You can create a CloudWatch metric, associate an alarm with the metric, and then create a health check that is based on the state of the alarm. For example, you might create a CloudWatch metric that checks the status of the Amazon EC2 <code>StatusCheckFailed</code> metric, add an alarm to the metric, and then create a health check that is based on the state of the alarm. For information about creating CloudWatch metrics and alarms by using the CloudWatch console, see the <a href=\"http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/WhatIsCloudWatch.html\">Amazon CloudWatch User Guide</a>.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn create_health_check(&self, input: &CreateHealthCheckRequest) -> Result<CreateHealthCheckResponse, CreateHealthCheckError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck".to_string();

                    
                    

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateHealthCheckError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates a new public hosted zone, used to specify how the Domain Name System (DNS) routes traffic on the Internet for a domain, such as example.com, and its subdomains. </p> <important> <p>Public hosted zones can't be converted to a private hosted zone or vice versa. Instead, create a new hosted zone with the same name and create new resource record sets.</p> </important> <p>Send a <code>POST</code> request to the <code>/2013-04-01/hostedzone</code> resource. The request body must include a document with a <code>CreateHostedZoneRequest</code> element. The response returns the <code>CreateHostedZoneResponse</code> element containing metadata about the hosted zone.</p> <p>Fore more information about charges for hosted zones, see <a href=\"http://aws.amazon.com/route53/pricing/\">Amazon Route 53 Pricing</a>.</p> <p>Note the following:</p> <ul> <li> <p>You can't create a hosted zone for a top-level domain (TLD).</p> </li> <li> <p>Amazon Route 53 automatically creates a default SOA record and four NS records for the zone. For more information about SOA and NS records, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/SOA-NSrecords.html\">NS and SOA Records that Amazon Route 53 Creates for a Hosted Zone</a> in the <i>Amazon Route 53 Developer Guide</i>.</p> </li> <li> <p>If your domain is registered with a registrar other than Amazon Route 53, you must update the name servers with your registrar to make Amazon Route 53 your DNS service. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/creating-migrating.html\">Configuring Amazon Route 53 as your DNS Service</a> in the <i>Amazon Route 53 Developer's Guide</i>.</p> </li> </ul> <p>After creating a zone, its initial status is <code>PENDING</code>. This means that it is not yet available on all DNS servers. The status of the zone changes to <code>INSYNC</code> when the NS and SOA records are available on all Amazon Route 53 DNS servers. </p> <p>When trying to create a hosted zone using a reusable delegation set, specify an optional DelegationSetId, and Amazon Route 53 would assign those 4 NS records for the zone, instead of allotting a new one.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_hosted_zone(&self, input: &CreateHostedZoneRequest) -> Result<CreateHostedZoneResponse, CreateHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone".to_string();

                    
                    

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates a delegation set (a group of four name servers) that can be reused by multiple hosted zones. If a hosted zoned ID is specified, <code>CreateReusableDelegationSet</code> marks the delegation set associated with that zone as reusable</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/delegationset</code> resource. The request body must include a document with a <code>CreateReusableDelegationSetRequest</code> element.</p> <note> <p>A reusable delegation set can't be associated with a private hosted zone/</p> </note> <p>For more information, including a procedure on how to create and configure a reusable delegation set (also known as white label name servers), see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/white-label-name-servers.html\">Configuring White Label Name Servers</a>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_reusable_delegation_set(&self, input: &CreateReusableDelegationSetRequest) -> Result<CreateReusableDelegationSetResponse, CreateReusableDelegationSetError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/delegationset".to_string();

                    
                    

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateReusableDelegationSetResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateReusableDelegationSetError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates a traffic policy, which you use to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com).</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/trafficpolicy</code> resource. The request body must include a document with a <code>CreateTrafficPolicyRequest</code> element. The response includes the <code>CreateTrafficPolicyResponse</code> element, which contains information about the new traffic policy.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_traffic_policy(&self, input: &CreateTrafficPolicyRequest) -> Result<CreateTrafficPolicyResponse, CreateTrafficPolicyError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicy".to_string();

                    
                    

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateTrafficPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates resource record sets in a specified hosted zone based on the settings in a specified traffic policy version. In addition, <code>CreateTrafficPolicyInstance</code> associates the resource record sets with a specified domain name (such as example.com) or subdomain name (such as www.example.com). Amazon Route 53 responds to DNS queries for the domain or subdomain name by using the resource record sets that <code>CreateTrafficPolicyInstance</code> created.</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/trafficpolicyinstance</code> resource. The request body must include a document with a <code>CreateTrafficPolicyRequest</code> element. The response returns the <code>CreateTrafficPolicyInstanceResponse</code> element, which contains information about the traffic policy instance.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_traffic_policy_instance(&self, input: &CreateTrafficPolicyInstanceRequest) -> Result<CreateTrafficPolicyInstanceResponse, CreateTrafficPolicyInstanceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstance".to_string();

                    
                    

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateTrafficPolicyInstanceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateTrafficPolicyInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Creates a new version of an existing traffic policy. When you create a new version of a traffic policy, you specify the ID of the traffic policy that you want to update and a JSON-formatted document that describes the new version. You use traffic policies to create multiple DNS resource record sets for one domain name (such as example.com) or one subdomain name (such as www.example.com). You can create a maximum of 1000 versions of a traffic policy. If you reach the limit and need to create another version, you'll need to start a new traffic policy.</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/trafficpolicy/</code> resource. The request body includes a document with a <code>CreateTrafficPolicyVersionRequest</code> element. The response returns the <code>CreateTrafficPolicyVersionResponse</code> element, which contains information about the new version of the traffic policy.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_traffic_policy_version(&self, input: &CreateTrafficPolicyVersionRequest) -> Result<CreateTrafficPolicyVersionResponse, CreateTrafficPolicyVersionError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicy/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateTrafficPolicyVersionResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateTrafficPolicyVersionResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            let value = response.headers.get("Location").unwrap().to_owned();
                 result.location = value;
                            Ok(result)
                        },
                        _ => Err(CreateTrafficPolicyVersionError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Authorizes the AWS account that created a specified VPC to submit an <code>AssociateVPCWithHostedZone</code> request to associate the VPC with a specified hosted zone that was created by a different account. To submit a <code>CreateVPCAssociationAuthorization</code> request, you must use the account that created the hosted zone. After you authorize the association, use the account that created the VPC to submit an <code>AssociateVPCWithHostedZone</code> request.</p> <note> <p>If you want to associate multiple VPCs that you created by using one account with a hosted zone that you created by using a different account, you must submit one authorization request for each VPC.</p> </note> <p>Send a <code>POST</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/authorizevpcassociation</code> resource. The request body must include a document with a <code>CreateVPCAssociationAuthorizationRequest</code> element. The response contains information about the authorization.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn create_vpc_association_authorization(&self, input: &CreateVPCAssociationAuthorizationRequest) -> Result<CreateVPCAssociationAuthorizationResponse, CreateVPCAssociationAuthorizationError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/authorizevpcassociation".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = CreateVPCAssociationAuthorizationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(CreateVPCAssociationAuthorizationResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(CreateVPCAssociationAuthorizationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Deletes a health check. Send a <code>DELETE</code> request to the <code>/2013-04-01/healthcheck/<i>health check ID</i> </code> resource.</p> <important> <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don't update the associated resource record sets, the future status of the health check can't be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html\">Replacing and Deleting Health Checks</a> in the Amazon Route 53 Developer Guide.</p> </important>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_health_check(&self, input: &DeleteHealthCheckRequest) -> Result<DeleteHealthCheckResponse, DeleteHealthCheckError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck/{HealthCheckId}".to_string();

                    
                    request_uri = request_uri.replace("{HealthCheckId}", &input.health_check_id.to_string());

                    let mut request = SignedRequest::new("DELETE", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteHealthCheckError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Deletes a hosted zone. Send a <code>DELETE</code> request to the <code>/<i>Amazon Route 53 API version</i>/hostedzone/<i>hosted zone ID</i> </code> resource.</p> <important> <p>Delete a hosted zone only if there are no resource record sets other than the default SOA record and NS resource record sets. If the hosted zone contains other resource record sets, delete them before deleting the hosted zone. If you try to delete a hosted zone that contains other resource record sets, Amazon Route 53 denies your request with a <code>HostedZoneNotEmpty</code> error. For information about deleting records from your hosted zone, see <a>ChangeResourceRecordSets</a>.</p> </important>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_hosted_zone(&self, input: &DeleteHostedZoneRequest) -> Result<DeleteHostedZoneResponse, DeleteHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("DELETE", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Deletes a reusable delegation set. Send a <code>DELETE</code> request to the <code>/2013-04-01/delegationset/<i>delegation set ID</i> </code> resource.</p> <important> <p> You can delete a reusable delegation set only if there are no associated hosted zones.</p> </important> <p>To verify that the reusable delegation set is not associated with any hosted zones, run the <a>GetReusableDelegationSet</a> action and specify the ID of the reusable delegation set that you want to delete.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_reusable_delegation_set(&self, input: &DeleteReusableDelegationSetRequest) -> Result<DeleteReusableDelegationSetResponse, DeleteReusableDelegationSetError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/delegationset/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("DELETE", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteReusableDelegationSetResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteReusableDelegationSetError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Deletes a traffic policy.</p> <p>Send a <code>DELETE</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicy</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_traffic_policy(&self, input: &DeleteTrafficPolicyRequest) -> Result<DeleteTrafficPolicyResponse, DeleteTrafficPolicyError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicy/{Id}/{Version}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());
request_uri = request_uri.replace("{Version}", &input.version.to_string());

                    let mut request = SignedRequest::new("DELETE", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteTrafficPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Deletes a traffic policy instance and all of the resource record sets that Amazon Route 53 created when you created the instance.</p> <p>Send a <code>DELETE</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicy/<i>traffic policy instance ID</i> </code> resource.</p> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_traffic_policy_instance(&self, input: &DeleteTrafficPolicyInstanceRequest) -> Result<DeleteTrafficPolicyInstanceResponse, DeleteTrafficPolicyInstanceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstance/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("DELETE", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteTrafficPolicyInstanceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteTrafficPolicyInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Removes authorization to submit an <code>AssociateVPCWithHostedZone</code> request to associate a specified VPC with a hosted zone that was created by a different account. You must use the account that created the hosted zone to submit a <code>DeleteVPCAssociationAuthorization</code> request.</p> <important> <p>Sending this request only prevents the AWS account that created the VPC from associating the VPC with the Amazon Route 53 hosted zone in the future. If the VPC is already associated with the hosted zone, <code>DeleteVPCAssociationAuthorization</code> won't disassociate the VPC from the hosted zone. If you want to delete an existing association, use <code>DisassociateVPCFromHostedZone</code>.</p> </important> <p>Send a <code>DELETE</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/deauthorizevpcassociation</code> resource. The request body must include a document with a <code>DeleteVPCAssociationAuthorizationRequest</code> element.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn delete_vpc_association_authorization(&self, input: &DeleteVPCAssociationAuthorizationRequest) -> Result<DeleteVPCAssociationAuthorizationResponse, DeleteVPCAssociationAuthorizationError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/deauthorizevpcassociation".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DeleteVPCAssociationAuthorizationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DeleteVPCAssociationAuthorizationResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DeleteVPCAssociationAuthorizationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Disassociates a VPC from a Amazon Route 53 private hosted zone. </p> <note> <p>You can't disassociate the last VPC from a private hosted zone.</p> </note> <p>Send a <code>POST</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/disassociatevpc</code> resource. The request body must include a document with a <code>DisassociateVPCFromHostedZoneRequest</code> element. The response includes a <code>DisassociateVPCFromHostedZoneResponse</code> element.</p> <important> <p>You can't disassociate a VPC from a private hosted zone when only one VPC is associated with the hosted zone. You also can't convert a private hosted zone into a public hosted zone.</p> </important>"]
                #[allow(unused_variables, warnings)]
                pub fn disassociate_vpc_from_hosted_zone(&self, input: &DisassociateVPCFromHostedZoneRequest) -> Result<DisassociateVPCFromHostedZoneResponse, DisassociateVPCFromHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/disassociatevpc".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = DisassociateVPCFromHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(DisassociateVPCFromHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(DisassociateVPCFromHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Returns the current status of a change batch request. The status is one of the following values:</p> <ul> <li> <p> <code>PENDING</code> indicates that the changes in this request have not replicated to all Amazon Route 53 DNS servers. This is the initial status of all change batch requests.</p> </li> <li> <p> <code>INSYNC</code> indicates that the changes have replicated to all Amazon Route 53 DNS servers. </p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn get_change(&self, input: &GetChangeRequest) -> Result<GetChangeResponse, GetChangeError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/change/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetChangeResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetChangeResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetChangeError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves a list of the IP ranges used by Amazon Route 53 health checkers to check the health of your resources. Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/checkeripranges</code> resource. Use these IP addresses to configure router and firewall rules to allow health checkers to check the health of your resources.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_checker_ip_ranges(&self, input: &GetCheckerIpRangesRequest) -> Result<GetCheckerIpRangesResponse, GetCheckerIpRangesError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/checkeripranges".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetCheckerIpRangesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetCheckerIpRangesResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetCheckerIpRangesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves a single geo location. Send a <code>GET</code> request to the <code>/2013-04-01/geolocation</code> resource with one of these options: continentcode | countrycode | countrycode and subdivisioncode.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_geo_location(&self, input: &GetGeoLocationRequest) -> Result<GetGeoLocationResponse, GetGeoLocationError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/geolocation".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref continent_code) = input.continent_code {
                            params.put("continentcode", &continent_code.to_string());
                        }

                        if let Some(ref country_code) = input.country_code {
                            params.put("countrycode", &country_code.to_string());
                        }

                        if let Some(ref subdivision_code) = input.subdivision_code {
                            params.put("subdivisioncode", &subdivision_code.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetGeoLocationResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetGeoLocationResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetGeoLocationError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about a specified health check. Send a <code>GET</code> request to the <code>/2013-04-01/healthcheck/<i>health check ID</i> </code> resource. For more information about using the console to perform this operation, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a> in the Amazon Route 53 Developer Guide.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_health_check(&self, input: &GetHealthCheckRequest) -> Result<GetHealthCheckResponse, GetHealthCheckError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck/{HealthCheckId}".to_string();

                    
                    request_uri = request_uri.replace("{HealthCheckId}", &input.health_check_id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHealthCheckError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>To retrieve a count of all your health checks, send a <code>GET</code> request to the <code>/2013-04-01/healthcheckcount</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_health_check_count(&self, input: &GetHealthCheckCountRequest) -> Result<GetHealthCheckCountResponse, GetHealthCheckCountError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheckcount".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHealthCheckCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHealthCheckCountResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHealthCheckCountError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>If you want to learn why a health check is currently failing or why it failed most recently (if at all), you can get the failure reason for the most recent failure. Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/healthcheck/<i>health check ID</i>/lastfailurereason</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_health_check_last_failure_reason(&self, input: &GetHealthCheckLastFailureReasonRequest) -> Result<GetHealthCheckLastFailureReasonResponse, GetHealthCheckLastFailureReasonError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck/{HealthCheckId}/lastfailurereason".to_string();

                    
                    request_uri = request_uri.replace("{HealthCheckId}", &input.health_check_id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHealthCheckLastFailureReasonResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHealthCheckLastFailureReasonResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHealthCheckLastFailureReasonError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets status of a specified health check. Send a <code>GET</code> request to the <code>/2013-04-01/healthcheck/<i>health check ID</i>/status</code> resource. You can use this call to get a health check's current status. </p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_health_check_status(&self, input: &GetHealthCheckStatusRequest) -> Result<GetHealthCheckStatusResponse, GetHealthCheckStatusError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck/{HealthCheckId}/status".to_string();

                    
                    request_uri = request_uri.replace("{HealthCheckId}", &input.health_check_id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHealthCheckStatusResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHealthCheckStatusResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHealthCheckStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves the delegation set for a hosted zone, including the four name servers assigned to the hosted zone. Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/hostedzone/<i>hosted zone ID</i> </code> resource. </p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_hosted_zone(&self, input: &GetHostedZoneRequest) -> Result<GetHostedZoneResponse, GetHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves a count of all your hosted zones. Send a <code>GET</code> request to the <code>/2013-04-01/hostedzonecount</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_hosted_zone_count(&self, input: &GetHostedZoneCountRequest) -> Result<GetHostedZoneCountResponse, GetHostedZoneCountError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzonecount".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetHostedZoneCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetHostedZoneCountResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetHostedZoneCountError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves the reusable delegation set. Send a <code>GET</code> request to the <code>/2013-04-01/delegationset/<i>delegation set ID</i> </code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_reusable_delegation_set(&self, input: &GetReusableDelegationSetRequest) -> Result<GetReusableDelegationSetResponse, GetReusableDelegationSetError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/delegationset/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetReusableDelegationSetResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetReusableDelegationSetResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetReusableDelegationSetError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about a specific traffic policy version.</p> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicy</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_traffic_policy(&self, input: &GetTrafficPolicyRequest) -> Result<GetTrafficPolicyResponse, GetTrafficPolicyError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicy/{Id}/{Version}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());
request_uri = request_uri.replace("{Version}", &input.version.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetTrafficPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetTrafficPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetTrafficPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about a specified traffic policy instance.</p> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicyinstance</code> resource.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <note> <p>In the Amazon Route 53 console, traffic policy instances are known as policy records.</p> </note>"]
                #[allow(unused_variables, warnings)]
                pub fn get_traffic_policy_instance(&self, input: &GetTrafficPolicyInstanceRequest) -> Result<GetTrafficPolicyInstanceResponse, GetTrafficPolicyInstanceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstance/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetTrafficPolicyInstanceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetTrafficPolicyInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets the number of traffic policy instances that are associated with the current AWS account.</p> <p>To get the number of traffic policy instances, send a <code>GET</code> request to the <code>/2013-04-01/trafficpolicyinstancecount</code> resource.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn get_traffic_policy_instance_count(&self, input: &GetTrafficPolicyInstanceCountRequest) -> Result<GetTrafficPolicyInstanceCountResponse, GetTrafficPolicyInstanceCountError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstancecount".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = GetTrafficPolicyInstanceCountResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(GetTrafficPolicyInstanceCountResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(GetTrafficPolicyInstanceCountError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves a list of supported geo locations. Send a <code>GET</code> request to the <code>/2013-04-01/geolocations</code> resource. The response to this request includes a <code>GeoLocationDetailsList</code> element for each location that Amazon Route 53 supports.</p> <p>Countries are listed first, and continents are listed last. If Amazon Route 53 supports subdivisions for a country (for example, states or provinces), the subdivisions for that country are listed in alphabetical order immediately after the corresponding country. </p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_geo_locations(&self, input: &ListGeoLocationsRequest) -> Result<ListGeoLocationsResponse, ListGeoLocationsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/geolocations".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref start_continent_code) = input.start_continent_code {
                            params.put("startcontinentcode", &start_continent_code.to_string());
                        }

                        if let Some(ref start_country_code) = input.start_country_code {
                            params.put("startcountrycode", &start_country_code.to_string());
                        }

                        if let Some(ref start_subdivision_code) = input.start_subdivision_code {
                            params.put("startsubdivisioncode", &start_subdivision_code.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListGeoLocationsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListGeoLocationsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListGeoLocationsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieve a list of your health checks. Send a <code>GET</code> request to the <code>/2013-04-01/healthcheck</code> resource. The response to this request includes a <code>HealthChecks</code> element with zero or more <code>HealthCheck</code> child elements. By default, the list of health checks is displayed on a single page. You can control the length of the page that is displayed by using the <code>MaxItems</code> parameter. You can use the <code>Marker</code> parameter to control the health check that the list begins with.</p> <p>For information about listing health checks using the Amazon Route 53 console, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover.html\">Amazon Route 53 Health Checks and DNS Failover</a>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_health_checks(&self, input: &ListHealthChecksRequest) -> Result<ListHealthChecksResponse, ListHealthChecksError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref marker) = input.marker {
                            params.put("marker", &marker.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListHealthChecksResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListHealthChecksResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListHealthChecksError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>To retrieve a list of your public and private hosted zones, send a <code>GET</code> request to the <code>/2013-04-01/hostedzone</code> resource. The response to this request includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of hosted zones, you can use the <code>maxitems</code> parameter to list them in groups of up to 100. The response includes four values that help navigate from one group of <code>maxitems</code> hosted zones to the next:</p> <ul> <li> <p> <code>MaxItems</code> is the value specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> </li> <li> <p> <code>NextMarker</code> is the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZones</code>, and specify the value of the <code>NextMarker</code> element in the marker parameter. </p> <p>If <code>IsTruncated</code> is false, the <code>NextMarker</code> element is omitted from the response.</p> </li> <li> <p>If you're making the second or subsequent call to <code>ListHostedZones</code>, the <code>Marker</code> element matches the value that you specified in the <code>marker</code> parameter in the previous request.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_hosted_zones(&self, input: &ListHostedZonesRequest) -> Result<ListHostedZonesResponse, ListHostedZonesError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref delegation_set_id) = input.delegation_set_id {
                            params.put("delegationsetid", &delegation_set_id.to_string());
                        }

                        if let Some(ref marker) = input.marker {
                            params.put("marker", &marker.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListHostedZonesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListHostedZonesResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListHostedZonesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Retrieves a list of your hosted zones in lexicographic order. Send a <code>GET</code> request to the <code>/2013-04-01/hostedzonesbyname</code> resource. The response includes a <code>HostedZones</code> child element for each hosted zone created by the current AWS account. </p> <p> <code>ListHostedZonesByName</code> sorts hosted zones by name with the labels reversed. For example:</p> <ul> <li> <p> <code>com.example.www.</code> </p> </li> </ul> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>If the domain name includes escape characters or Punycode, <code>ListHostedZonesByName</code> alphabetizes the domain name using the escaped or Punycoded value, which is the format that Amazon Route 53 saves in its database. For example, to create a hosted zone for example.com, specify ex\\344mple.com for the domain name. <code>ListHostedZonesByName</code> alphabetizes it as:</p> <ul> <li> <p> <code>com.ex\\344mple.</code> </p> </li> </ul> <p>The labels are reversed and alphabetized using the escaped value. For more information about valid domain name formats, including internationalized domain names, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DomainNameFormat.html\">DNS Domain Name Format</a> in the Amazon Route 53 Developer Guide.</p> <p>Amazon Route 53 returns up to 100 items in each response. If you have a lot of hosted zones, use the <code>MaxItems</code> parameter to list them in groups of up to 100. The response includes values that help navigate from one group of <code>MaxItems</code> hosted zones to the next:</p> <ul> <li> <p>The <code>DNSName</code> and <code>HostedZoneId</code> elements in the response contain the values, if any, specified for the <code>dnsname</code> and <code>hostedzoneid</code> parameters in the request that produced the current response.</p> </li> <li> <p>The <code>MaxItems</code> element in the response contains the value, if any, that you specified for the <code>maxitems</code> parameter in the request that produced the current response.</p> </li> <li> <p>If the value of <code>IsTruncated</code> in the response is true, there are more hosted zones associated with the current AWS account. </p> <p>If <code>IsTruncated</code> is false, this response includes the last hosted zone that is associated with the current account. The <code>NextDNSName</code> element and <code>NextHostedZoneId</code> elements are omitted from the response.</p> </li> <li> <p>The <code>NextDNSName</code> and <code>NextHostedZoneId</code> elements in the response contain the domain name and the hosted zone ID of the next hosted zone that is associated with the current AWS account. If you want to list more hosted zones, make another call to <code>ListHostedZonesByName</code>, and specify the value of <code>NextDNSName</code> and <code>NextHostedZoneId</code> in the <code>dnsname</code> and <code>hostedzoneid</code> parameters, respectively.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_hosted_zones_by_name(&self, input: &ListHostedZonesByNameRequest) -> Result<ListHostedZonesByNameResponse, ListHostedZonesByNameError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzonesbyname".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref dns_name) = input.dns_name {
                            params.put("dnsname", &dns_name.to_string());
                        }

                        if let Some(ref hosted_zone_id) = input.hosted_zone_id {
                            params.put("hostedzoneid", &hosted_zone_id.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListHostedZonesByNameResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListHostedZonesByNameResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListHostedZonesByNameError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Lists the resource record sets in a specified hosted zone.</p> <p> <code>ListResourceRecordSets</code> returns up to 100 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements. The action sorts results first by DNS name with the labels reversed, for example:</p> <p> <code>com.example.www.</code> </p> <p>Note the trailing dot, which can change the sort order in some circumstances.</p> <p>When multiple records have the same DNS name, the action sorts results by the record type.</p> <p>You can use the name and type elements to adjust the beginning position of the list of resource record sets returned:</p> <dl> <dt>If you do not specify Name or Type</dt> <dd> <p>The results begin with the first resource record set that the hosted zone contains.</p> </dd> <dt>If you specify Name but not Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p> </dd> <dt>If you specify Type but not Name</dt> <dd> <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p> </dd> <dt>If you specify both Name and Type</dt> <dd> <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p> </dd> </dl> <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Amazon Route 53 DNS servers.</p> <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_resource_record_sets(&self, input: &ListResourceRecordSetsRequest) -> Result<ListResourceRecordSetsResponse, ListResourceRecordSetsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/rrset".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref start_record_identifier) = input.start_record_identifier {
                            params.put("identifier", &start_record_identifier.to_string());
                        }

                        if let Some(ref start_record_name) = input.start_record_name {
                            params.put("name", &start_record_name.to_string());
                        }

                        if let Some(ref start_record_type) = input.start_record_type {
                            params.put("type", &start_record_type.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListResourceRecordSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListResourceRecordSetsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListResourceRecordSetsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>To retrieve a list of your reusable delegation sets, send a <code>GET</code> request to the <code>/2013-04-01/delegationset</code> resource. The response to this request includes a <code>DelegationSets</code> element with zero, one, or multiple <code>DelegationSet</code> child elements. By default, the list of delegation sets is displayed on a single page. You can control the length of the page that is displayed by using the <code>MaxItems</code> parameter. You can use the <code>Marker</code> parameter to control the delegation set that the list begins with. </p> <note> <p> Amazon Route 53 returns a maximum of 100 items. If you set MaxItems to a value greater than 100, Amazon Route 53 returns only the first 100.</p> </note>"]
                #[allow(unused_variables, warnings)]
                pub fn list_reusable_delegation_sets(&self, input: &ListReusableDelegationSetsRequest) -> Result<ListReusableDelegationSetsResponse, ListReusableDelegationSetsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/delegationset".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref marker) = input.marker {
                            params.put("marker", &marker.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListReusableDelegationSetsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListReusableDelegationSetsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListReusableDelegationSetsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Lists tags for one health check or hosted zone. </p> <p>For information about using tags for cost allocation, see <a href=\"http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html\">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_tags_for_resource(&self, input: &ListTagsForResourceRequest) -> Result<ListTagsForResourceResponse, ListTagsForResourceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/tags/{ResourceType}/{ResourceId}".to_string();

                    
                    request_uri = request_uri.replace("{ResourceId}", &input.resource_id.to_string());
request_uri = request_uri.replace("{ResourceType}", &input.resource_type.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTagsForResourceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTagsForResourceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Lists tags for up to 10 health checks or hosted zones.</p> <p>For information about using tags for cost allocation, see <a href=\"http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html\">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_tags_for_resources(&self, input: &ListTagsForResourcesRequest) -> Result<ListTagsForResourcesResponse, ListTagsForResourcesError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/tags/{ResourceType}".to_string();

                    
                    request_uri = request_uri.replace("{ResourceType}", &input.resource_type.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTagsForResourcesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTagsForResourcesResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTagsForResourcesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about the latest version for every traffic policy that is associated with the current AWS account. Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicy</code> resource.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policies, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p> <p>The response includes three values that help you navigate from one group of <code>maxitems</code> traffic policies to the next:</p> <ul> <li> <p> <b>IsTruncated</b> </p> <p>If the value of <code>IsTruncated</code> in the response is <code>true</code>, there are more traffic policies associated with the current AWS account.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last traffic policy that is associated with the current account.</p> </li> <li> <p> <b>TrafficPolicyIdMarker</b> </p> <p>If <code>IsTruncated</code> is <code>true</code>, <code>TrafficPolicyIdMarker</code> is the ID of the first traffic policy in the next group of <code>MaxItems</code> traffic policies. If you want to list more traffic policies, make another call to <code>ListTrafficPolicies</code>, and specify the value of the <code>TrafficPolicyIdMarker</code> element from the response in the <code>TrafficPolicyIdMarker</code> request parameter.</p> <p>If <code>IsTruncated</code> is <code>false</code>, the <code>TrafficPolicyIdMarker</code> element is omitted from the response.</p> </li> <li> <p> <b>MaxItems</b> </p> <p>The value that you specified for the <code>MaxItems</code> parameter in the request that produced the current response.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_traffic_policies(&self, input: &ListTrafficPoliciesRequest) -> Result<ListTrafficPoliciesResponse, ListTrafficPoliciesError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicies".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref traffic_policy_id_marker) = input.traffic_policy_id_marker {
                            params.put("trafficpolicyid", &traffic_policy_id_marker.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTrafficPoliciesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTrafficPoliciesResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTrafficPoliciesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about the traffic policy instances that you created by using the current AWS account.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicyinstance</code> resource.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p> <p>The response includes five values that help you navigate from one group of <code>MaxItems</code> traffic policy instances to the next:</p> <ul> <li> <p> <b>IsTruncated</b> </p> <p>If the value of <code>IsTruncated</code> in the response is <code>true</code>, there are more traffic policy instances associated with the current AWS account.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last traffic policy instance that is associated with the current account.</p> </li> <li> <p> <b>MaxItems</b> </p> <p>The value that you specified for the <code>MaxItems</code> parameter in the request that produced the current response.</p> </li> <li> <p> <b>HostedZoneIdMarker</b>, <b>TrafficPolicyInstanceNameMarker</b>, and <b>TrafficPolicyInstanceTypeMarker</b> </p> <p>If <code>IsTruncated</code> is <code>true</code>, these three values in the response represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances. To list more traffic policy instances, make another call to <code>ListTrafficPolicyInstances</code>, and specify these values in the corresponding request parameters.</p> <p>If <code>IsTruncated</code> is <code>false</code>, all three elements are omitted from the response.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_traffic_policy_instances(&self, input: &ListTrafficPolicyInstancesRequest) -> Result<ListTrafficPolicyInstancesResponse, ListTrafficPolicyInstancesError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstances".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref hosted_zone_id_marker) = input.hosted_zone_id_marker {
                            params.put("hostedzoneid", &hosted_zone_id_marker.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref traffic_policy_instance_name_marker) = input.traffic_policy_instance_name_marker {
                            params.put("trafficpolicyinstancename", &traffic_policy_instance_name_marker.to_string());
                        }

                        if let Some(ref traffic_policy_instance_type_marker) = input.traffic_policy_instance_type_marker {
                            params.put("trafficpolicyinstancetype", &traffic_policy_instance_type_marker.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTrafficPolicyInstancesResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTrafficPolicyInstancesResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTrafficPolicyInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about the traffic policy instances that you created in a specified hosted zone.</p> <note> <p>After you submit an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicyinstance</code> resource and include the ID of the hosted zone.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p> <p>The response includes four values that help you navigate from one group of <code>MaxItems</code> traffic policy instances to the next:</p> <ul> <li> <p> <b>IsTruncated</b> </p> <p>If the value of <code/>IsTruncated in the response is <code>true</code>, there are more traffic policy instances associated with the current AWS account.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last traffic policy instance that is associated with the current account.</p> </li> <li> <p> <b>MaxItems</b> </p> <p>The value that you specified for the <code>MaxItems</code> parameter in the request that produced the current response.</p> </li> <li> <p> <b>TrafficPolicyInstanceNameMarker</b> and <b>TrafficPolicyInstanceTypeMarker</b> </p> <p>If <code>IsTruncated</code> is <code>true</code>, these two values in the response represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances. To list more traffic policy instances, make another call to <code>ListTrafficPolicyInstancesByHostedZone</code>, and specify these values in the corresponding request parameters.</p> <p>If <code>IsTruncated</code> is <code>false</code>, all three elements are omitted from the response.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_traffic_policy_instances_by_hosted_zone(&self, input: &ListTrafficPolicyInstancesByHostedZoneRequest) -> Result<ListTrafficPolicyInstancesByHostedZoneResponse, ListTrafficPolicyInstancesByHostedZoneError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstances/hostedzone".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    params.put("id", &input.hosted_zone_id.to_string());

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref traffic_policy_instance_name_marker) = input.traffic_policy_instance_name_marker {
                            params.put("trafficpolicyinstancename", &traffic_policy_instance_name_marker.to_string());
                        }

                        if let Some(ref traffic_policy_instance_type_marker) = input.traffic_policy_instance_type_marker {
                            params.put("trafficpolicyinstancetype", &traffic_policy_instance_type_marker.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTrafficPolicyInstancesByHostedZoneResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTrafficPolicyInstancesByHostedZoneResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTrafficPolicyInstancesByHostedZoneError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about the traffic policy instances that you created by using a specify traffic policy version.</p> <note> <p>After you submit a <code>CreateTrafficPolicyInstance</code> or an <code>UpdateTrafficPolicyInstance</code> request, there's a brief delay while Amazon Route 53 creates the resource record sets that are specified in the traffic policy definition. For more information, see the <code>State</code> response element.</p> </note> <p>Send a <code>GET</code> request to the <code>/<i>Route 53 API version</i>/trafficpolicyinstance</code> resource and include the ID and version of the traffic policy.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policy instances, you can use the <code>MaxItems</code> parameter to list them in groups of up to 100.</p> <p>The response includes five values that help you navigate from one group of <code>MaxItems</code> traffic policy instances to the next:</p> <ul> <li> <p> <b>IsTruncated</b> </p> <p>If the value of <code>IsTruncated</code> in the response is <code>true</code>, there are more traffic policy instances associated with the specified traffic policy.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last traffic policy instance that is associated with the specified traffic policy.</p> </li> <li> <p> <b>MaxItems</b> </p> <p>The value that you specified for the <code>MaxItems</code> parameter in the request that produced the current response.</p> </li> <li> <p> <b>HostedZoneIdMarker</b>, <b>TrafficPolicyInstanceNameMarker</b>, and <b>TrafficPolicyInstanceTypeMarker</b> </p> <p>If <code>IsTruncated</code> is <code>true</code>, these values in the response represent the first traffic policy instance in the next group of <code>MaxItems</code> traffic policy instances. To list more traffic policy instances, make another call to <code>ListTrafficPolicyInstancesByPolicy</code>, and specify these values in the corresponding request parameters.</p> <p>If <code>IsTruncated</code> is <code>false</code>, all three elements are omitted from the response.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_traffic_policy_instances_by_policy(&self, input: &ListTrafficPolicyInstancesByPolicyRequest) -> Result<ListTrafficPolicyInstancesByPolicyResponse, ListTrafficPolicyInstancesByPolicyError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstances/trafficpolicy".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref hosted_zone_id_marker) = input.hosted_zone_id_marker {
                            params.put("hostedzoneid", &hosted_zone_id_marker.to_string());
                        }

                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }
params.put("id", &input.traffic_policy_id.to_string());

                        if let Some(ref traffic_policy_instance_name_marker) = input.traffic_policy_instance_name_marker {
                            params.put("trafficpolicyinstancename", &traffic_policy_instance_name_marker.to_string());
                        }

                        if let Some(ref traffic_policy_instance_type_marker) = input.traffic_policy_instance_type_marker {
                            params.put("trafficpolicyinstancetype", &traffic_policy_instance_type_marker.to_string());
                        }
params.put("version", &input.traffic_policy_version.to_string());
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTrafficPolicyInstancesByPolicyResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTrafficPolicyInstancesByPolicyResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTrafficPolicyInstancesByPolicyError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets information about all of the versions for a specified traffic policy.</p> <p>Send a <code>GET</code> request to the <code>/<i>Amazon Route 53 API version</i>/trafficpolicy</code> resource and specify the ID of the traffic policy for which you want to list versions.</p> <p>Amazon Route 53 returns a maximum of 100 items in each response. If you have a lot of traffic policies, you can use the <code>maxitems</code> parameter to list them in groups of up to 100.</p> <p>The response includes three values that help you navigate from one group of <code>maxitems</code> traffic policies to the next:</p> <ul> <li> <p> <b>IsTruncated</b> </p> <p>If the value of <code>IsTruncated</code> in the response is <code>true</code>, there are more traffic policy versions associated with the specified traffic policy.</p> <p>If <code>IsTruncated</code> is <code>false</code>, this response includes the last traffic policy version that is associated with the specified traffic policy.</p> </li> <li> <p> <b>TrafficPolicyVersionMarker</b> </p> <p>The ID of the next traffic policy version that is associated with the current AWS account. If you want to list more traffic policies, make another call to <code>ListTrafficPolicyVersions</code>, and specify the value of the <code>TrafficPolicyVersionMarker</code> element in the <code>TrafficPolicyVersionMarker</code> request parameter.</p> <p>If <code>IsTruncated</code> is <code>false</code>, Amazon Route 53 omits the <code>TrafficPolicyVersionMarker</code> element from the response.</p> </li> <li> <p> <b>MaxItems</b> </p> <p>The value that you specified for the <code>MaxItems</code> parameter in the request that produced the current response.</p> </li> </ul>"]
                #[allow(unused_variables, warnings)]
                pub fn list_traffic_policy_versions(&self, input: &ListTrafficPolicyVersionsRequest) -> Result<ListTrafficPolicyVersionsResponse, ListTrafficPolicyVersionsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicies/{Id}/versions".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref max_items) = input.max_items {
                            params.put("maxitems", &max_items.to_string());
                        }

                        if let Some(ref traffic_policy_version_marker) = input.traffic_policy_version_marker {
                            params.put("trafficpolicyversion", &traffic_policy_version_marker.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListTrafficPolicyVersionsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListTrafficPolicyVersionsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListTrafficPolicyVersionsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets a list of the VPCs that were created by other accounts and that can be associated with a specified hosted zone because you've submitted one or more <code>CreateVPCAssociationAuthorization</code> requests. </p> <p>Send a <code>GET</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/authorizevpcassociation</code> resource. The response to this request includes a <code>VPCs</code> element with a <code>VPC</code> child element for each VPC that can be associated with the hosted zone.</p> <p>Amazon Route 53 returns up to 50 VPCs per page. To return fewer VPCs per page, include the <code>MaxResults</code> parameter: </p> <p> <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/authorizevpcassociation?MaxItems=<i>VPCs per page</i> </code> </p> <p>If the response includes a <code>NextToken</code> element, there are more VPCs to list. To get the next page of VPCs, submit another <code>ListVPCAssociationAuthorizations</code> request, and include the value of the <code>NextToken</code> element from the response in the <code>NextToken</code> request parameter:</p> <p> <code>/2013-04-01/hostedzone/<i>hosted zone ID</i>/authorizevpcassociation?MaxItems=<i>VPCs per page</i>&amp;NextToken=<i/> </code> </p>"]
                #[allow(unused_variables, warnings)]
                pub fn list_vpc_association_authorizations(&self, input: &ListVPCAssociationAuthorizationsRequest) -> Result<ListVPCAssociationAuthorizationsResponse, ListVPCAssociationAuthorizationsError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}/authorizevpcassociation".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.hosted_zone_id.to_string());

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref max_results) = input.max_results {
                            params.put("maxresults", &max_results.to_string());
                        }

                        if let Some(ref next_token) = input.next_token {
                            params.put("nexttoken", &next_token.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = ListVPCAssociationAuthorizationsResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(ListVPCAssociationAuthorizationsResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(ListVPCAssociationAuthorizationsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Gets the value that Amazon Route 53 returns in response to a DNS request for a specified record name and type. You can optionally specify the IP address of a DNS resolver, an EDNS0 client subnet IP address, and a subnet mask. </p>"]
                #[allow(unused_variables, warnings)]
                pub fn test_dns_answer(&self, input: &TestDNSAnswerRequest) -> Result<TestDNSAnswerResponse, TestDNSAnswerError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/testdnsanswer".to_string();

                    
                    

                    let mut request = SignedRequest::new("GET", "route53", self.region, &request_uri);

                    
                    
                        if let Some(ref edns0_client_subnet_ip) = input.edns0_client_subnet_ip {
                            params.put("edns0clientsubnetip", &edns0_client_subnet_ip.to_string());
                        }

                        if let Some(ref edns0_client_subnet_mask) = input.edns0_client_subnet_mask {
                            params.put("edns0clientsubnetmask", &edns0_client_subnet_mask.to_string());
                        }
params.put("hostedzoneid", &input.hosted_zone_id.to_string());
params.put("recordname", &input.record_name.to_string());
params.put("recordtype", &input.record_type.to_string());

                        if let Some(ref resolver_ip) = input.resolver_ip {
                            params.put("resolverip", &resolver_ip.to_string());
                        }
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = TestDNSAnswerResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(TestDNSAnswerResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(TestDNSAnswerError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Updates an existing health check.</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/healthcheck/<i>health check ID</i> </code> resource. The request body must include a document with an <code>UpdateHealthCheckRequest</code> element. For more information about updating health checks, see <a href=\"http://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html\">Creating, Updating, and Deleting Health Checks</a> in the Amazon Route 53 Developer Guide.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn update_health_check(&self, input: &UpdateHealthCheckRequest) -> Result<UpdateHealthCheckResponse, UpdateHealthCheckError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/healthcheck/{HealthCheckId}".to_string();

                    
                    request_uri = request_uri.replace("{HealthCheckId}", &input.health_check_id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = UpdateHealthCheckResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(UpdateHealthCheckResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(UpdateHealthCheckError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Updates the hosted zone comment. Send a <code>POST</code> request to the <code>/2013-04-01/hostedzone/<i>hosted zone ID</i> </code> resource. </p>"]
                #[allow(unused_variables, warnings)]
                pub fn update_hosted_zone_comment(&self, input: &UpdateHostedZoneCommentRequest) -> Result<UpdateHostedZoneCommentResponse, UpdateHostedZoneCommentError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/hostedzone/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = UpdateHostedZoneCommentResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(UpdateHostedZoneCommentResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(UpdateHostedZoneCommentError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Updates the comment for a specified traffic policy version.</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/trafficpolicy/</code> resource.</p> <p>The request body must include a document with an <code>UpdateTrafficPolicyCommentRequest</code> element.</p>"]
                #[allow(unused_variables, warnings)]
                pub fn update_traffic_policy_comment(&self, input: &UpdateTrafficPolicyCommentRequest) -> Result<UpdateTrafficPolicyCommentResponse, UpdateTrafficPolicyCommentError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicy/{Id}/{Version}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());
request_uri = request_uri.replace("{Version}", &input.version.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = UpdateTrafficPolicyCommentResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(UpdateTrafficPolicyCommentResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(UpdateTrafficPolicyCommentError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
#[doc="<p>Updates the resource record sets in a specified hosted zone that were created based on the settings in a specified traffic policy version.</p> <p>Send a <code>POST</code> request to the <code>/2013-04-01/trafficpolicyinstance/<i>traffic policy ID</i> </code> resource. The request body must include a document with an <code>UpdateTrafficPolicyInstanceRequest</code> element.</p> <p>When you update a traffic policy instance, Amazon Route 53 continues to respond to DNS queries for the root resource record set name (such as example.com) while it replaces one group of resource record sets with another. Amazon Route 53 performs the following operations:</p> <ol> <li> <p>Amazon Route 53 creates a new group of resource record sets based on the specified traffic policy. This is true regardless of how substantial the differences are between the existing resource record sets and the new resource record sets. </p> </li> <li> <p>When all of the new resource record sets have been created, Amazon Route 53 starts to respond to DNS queries for the root resource record set name (such as example.com) by using the new resource record sets.</p> </li> <li> <p>Amazon Route 53 deletes the old group of resource record sets that are associated with the root resource record set name.</p> </li> </ol>"]
                #[allow(unused_variables, warnings)]
                pub fn update_traffic_policy_instance(&self, input: &UpdateTrafficPolicyInstanceRequest) -> Result<UpdateTrafficPolicyInstanceResponse, UpdateTrafficPolicyInstanceError> {
                    let mut params = Params::new();
                    let mut request_uri = "/2013-04-01/trafficpolicyinstance/{Id}".to_string();

                    
                    request_uri = request_uri.replace("{Id}", &input.id.to_string());

                    let mut request = SignedRequest::new("POST", "route53", self.region, &request_uri);

                    
                    
                    

                    request.set_params(params);
                    request.sign(&try!(self.credentials_provider.credentials()));

                    let response = try!(self.dispatcher.dispatch(&request));

                    match response.status {
                        StatusCode::Ok|StatusCode::NoContent|StatusCode::PartialContent => {
                            
        let mut result;

        if response.body.is_empty() {
            result = UpdateTrafficPolicyInstanceResponse::default();
        } else {
            let reader = EventReader::new_with_config(
                response.body.as_slice(),
                ParserConfig::new().trim_whitespace(true)
            );
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            let _start_document = stack.next();
            let actual_tag_name = try!(peek_at_name(&mut stack));
            result = try!(UpdateTrafficPolicyInstanceResponseDeserializer::deserialize(&actual_tag_name, &mut stack));
        }
                            
                            Ok(result)
                        },
                        _ => Err(UpdateTrafficPolicyInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                    }
                }
                
}
