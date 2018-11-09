#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const DDS_XCDR_REPRESENTATION: u32 = 0;
pub const DDS_XML_REPRESENTATION: u32 = 1;
pub const DDS_OSPL_REPRESENTATION: u32 = 1024;
pub const DDS_GPB_REPRESENTATION: u32 = 1025;
pub const DDS_INVALID_REPRESENTATION: u32 = 32767;
pub const DDS_DOMAIN_ID_DEFAULT: u32 = 2147483647;
pub const DDS_DOMAIN_ID_INVALID: i32 = -1;
pub const DDS_HANDLE_NIL_NATIVE: u32 = 0;
pub const DDS_HANDLE_NIL: u32 = 0;
pub const DDS_LENGTH_UNLIMITED: i32 = -1;
pub const DDS_DURATION_INFINITE_SEC: u32 = 2147483647;
pub const DDS_DURATION_INFINITE_NSEC: u32 = 2147483647;
pub const DDS_DURATION_ZERO_SEC: u32 = 0;
pub const DDS_DURATION_ZERO_NSEC: u32 = 0;
pub const DDS_TIMESTAMP_INVALID_SEC: i32 = -1;
pub const DDS_TIMESTAMP_INVALID_NSEC: u32 = 4294967295;
pub const DDS_TIMESTAMP_CURRENT_SEC: i32 = -1;
pub const DDS_TIMESTAMP_CURRENT_NSEC: u32 = 4294967294;
pub const DDS_RETCODE_OK: u32 = 0;
pub const DDS_RETCODE_ERROR: u32 = 1;
pub const DDS_RETCODE_UNSUPPORTED: u32 = 2;
pub const DDS_RETCODE_BAD_PARAMETER: u32 = 3;
pub const DDS_RETCODE_PRECONDITION_NOT_MET: u32 = 4;
pub const DDS_RETCODE_OUT_OF_RESOURCES: u32 = 5;
pub const DDS_RETCODE_NOT_ENABLED: u32 = 6;
pub const DDS_RETCODE_IMMUTABLE_POLICY: u32 = 7;
pub const DDS_RETCODE_INCONSISTENT_POLICY: u32 = 8;
pub const DDS_RETCODE_ALREADY_DELETED: u32 = 9;
pub const DDS_RETCODE_TIMEOUT: u32 = 10;
pub const DDS_RETCODE_NO_DATA: u32 = 11;
pub const DDS_RETCODE_ILLEGAL_OPERATION: u32 = 12;
pub const DDS_INCONSISTENT_TOPIC_STATUS: u32 = 1;
pub const DDS_OFFERED_DEADLINE_MISSED_STATUS: u32 = 2;
pub const DDS_REQUESTED_DEADLINE_MISSED_STATUS: u32 = 4;
pub const DDS_OFFERED_INCOMPATIBLE_QOS_STATUS: u32 = 32;
pub const DDS_REQUESTED_INCOMPATIBLE_QOS_STATUS: u32 = 64;
pub const DDS_SAMPLE_LOST_STATUS: u32 = 128;
pub const DDS_SAMPLE_REJECTED_STATUS: u32 = 256;
pub const DDS_DATA_ON_READERS_STATUS: u32 = 512;
pub const DDS_DATA_AVAILABLE_STATUS: u32 = 1024;
pub const DDS_LIVELINESS_LOST_STATUS: u32 = 2048;
pub const DDS_LIVELINESS_CHANGED_STATUS: u32 = 4096;
pub const DDS_PUBLICATION_MATCHED_STATUS: u32 = 8192;
pub const DDS_SUBSCRIPTION_MATCHED_STATUS: u32 = 16384;
pub const DDS_ALL_DATA_DISPOSED_TOPIC_STATUS: u32 = 2147483648;
pub const DDS_ANY_STATUS: u32 = 32767;
pub const DDS_STATUS_MASK_ANY_V1_2: u32 = 32767;
pub const DDS_STATUS_MASK_ANY: u32 = 4294967295;
pub const DDS_STATUS_MASK_NONE: u32 = 0;
pub const DDS_READ_SAMPLE_STATE: u32 = 1;
pub const DDS_NOT_READ_SAMPLE_STATE: u32 = 2;
pub const DDS_ANY_SAMPLE_STATE: u32 = 65535;
pub const DDS_NEW_VIEW_STATE: u32 = 1;
pub const DDS_NOT_NEW_VIEW_STATE: u32 = 2;
pub const DDS_ANY_VIEW_STATE: u32 = 65535;
pub const DDS_ALIVE_INSTANCE_STATE: u32 = 1;
pub const DDS_NOT_ALIVE_DISPOSED_INSTANCE_STATE: u32 = 2;
pub const DDS_NOT_ALIVE_NO_WRITERS_INSTANCE_STATE: u32 = 4;
pub const DDS_ANY_INSTANCE_STATE: u32 = 65535;
pub const DDS_NOT_ALIVE_INSTANCE_STATE: u32 = 6;
pub const DDS_USERDATA_QOS_POLICY_NAME: &'static [u8; 9usize] = b"UserData\0";
pub const DDS_DURABILITY_QOS_POLICY_NAME: &'static [u8; 11usize] = b"Durability\0";
pub const DDS_PRESENTATION_QOS_POLICY_NAME: &'static [u8; 13usize] = b"Presentation\0";
pub const DDS_DEADLINE_QOS_POLICY_NAME: &'static [u8; 9usize] = b"Deadline\0";
pub const DDS_LATENCYBUDGET_QOS_POLICY_NAME: &'static [u8; 14usize] = b"LatencyBudget\0";
pub const DDS_OWNERSHIP_QOS_POLICY_NAME: &'static [u8; 10usize] = b"Ownership\0";
pub const DDS_OWNERSHIPSTRENGTH_QOS_POLICY_NAME: &'static [u8; 18usize] = b"OwnershipStrength\0";
pub const DDS_LIVELINESS_QOS_POLICY_NAME: &'static [u8; 11usize] = b"Liveliness\0";
pub const DDS_TIMEBASEDFILTER_QOS_POLICY_NAME: &'static [u8; 16usize] = b"TimeBasedFilter\0";
pub const DDS_PARTITION_QOS_POLICY_NAME: &'static [u8; 10usize] = b"Partition\0";
pub const DDS_RELIABILITY_QOS_POLICY_NAME: &'static [u8; 12usize] = b"Reliability\0";
pub const DDS_DESTINATIONORDER_QOS_POLICY_NAME: &'static [u8; 17usize] = b"DestinationOrder\0";
pub const DDS_HISTORY_QOS_POLICY_NAME: &'static [u8; 8usize] = b"History\0";
pub const DDS_RESOURCELIMITS_QOS_POLICY_NAME: &'static [u8; 15usize] = b"ResourceLimits\0";
pub const DDS_ENTITYFACTORY_QOS_POLICY_NAME: &'static [u8; 14usize] = b"EntityFactory\0";
pub const DDS_WRITERDATALIFECYCLE_QOS_POLICY_NAME: &'static [u8; 20usize] =
    b"WriterDataLifecycle\0";
pub const DDS_READERDATALIFECYCLE_QOS_POLICY_NAME: &'static [u8; 20usize] =
    b"ReaderDataLifecycle\0";
pub const DDS_TOPICDATA_QOS_POLICY_NAME: &'static [u8; 10usize] = b"TopicData\0";
pub const DDS_GROUPDATA_QOS_POLICY_NAME: &'static [u8; 10usize] = b"GroupData\0";
pub const DDS_TRANSPORTPRIORITY_QOS_POLICY_NAME: &'static [u8; 18usize] = b"TransportPriority\0";
pub const DDS_LIFESPAN_QOS_POLICY_NAME: &'static [u8; 9usize] = b"Lifespan\0";
pub const DDS_DURABILITYSERVICE_QOS_POLICY_NAME: &'static [u8; 18usize] = b"DurabilityService\0";
pub const DDS_SUBSCRIPTIONKEY_QOS_POLICY_NAME: &'static [u8; 16usize] = b"SubscriptionKey\0";
pub const DDS_VIEWKEY_QOS_POLICY_NAME: &'static [u8; 8usize] = b"ViewKey\0";
pub const DDS_READERLIFESPAN_QOS_POLICY_NAME: &'static [u8; 15usize] = b"ReaderLifespan\0";
pub const DDS_SHARE_QOS_POLICY_NAME: &'static [u8; 6usize] = b"Share\0";
pub const DDS_SCHEDULING_QOS_POLICY_NAME: &'static [u8; 11usize] = b"Scheduling\0";
pub const DDS_INVALID_QOS_POLICY_ID: u32 = 0;
pub const DDS_USERDATA_QOS_POLICY_ID: u32 = 1;
pub const DDS_DURABILITY_QOS_POLICY_ID: u32 = 2;
pub const DDS_PRESENTATION_QOS_POLICY_ID: u32 = 3;
pub const DDS_DEADLINE_QOS_POLICY_ID: u32 = 4;
pub const DDS_LATENCYBUDGET_QOS_POLICY_ID: u32 = 5;
pub const DDS_OWNERSHIP_QOS_POLICY_ID: u32 = 6;
pub const DDS_OWNERSHIPSTRENGTH_QOS_POLICY_ID: u32 = 7;
pub const DDS_LIVELINESS_QOS_POLICY_ID: u32 = 8;
pub const DDS_TIMEBASEDFILTER_QOS_POLICY_ID: u32 = 9;
pub const DDS_PARTITION_QOS_POLICY_ID: u32 = 10;
pub const DDS_RELIABILITY_QOS_POLICY_ID: u32 = 11;
pub const DDS_DESTINATIONORDER_QOS_POLICY_ID: u32 = 12;
pub const DDS_HISTORY_QOS_POLICY_ID: u32 = 13;
pub const DDS_RESOURCELIMITS_QOS_POLICY_ID: u32 = 14;
pub const DDS_ENTITYFACTORY_QOS_POLICY_ID: u32 = 15;
pub const DDS_WRITERDATALIFECYCLE_QOS_POLICY_ID: u32 = 16;
pub const DDS_READERDATALIFECYCLE_QOS_POLICY_ID: u32 = 17;
pub const DDS_TOPICDATA_QOS_POLICY_ID: u32 = 18;
pub const DDS_GROUPDATA_QOS_POLICY_ID: u32 = 19;
pub const DDS_TRANSPORTPRIORITY_QOS_POLICY_ID: u32 = 20;
pub const DDS_LIFESPAN_QOS_POLICY_ID: u32 = 21;
pub const DDS_DURABILITYSERVICE_QOS_POLICY_ID: u32 = 22;
pub const DDS_SUBSCRIPTIONKEY_QOS_POLICY_ID: u32 = 23;
pub const DDS_VIEWKEY_QOS_POLICY_ID: u32 = 24;
pub const DDS_READERLIFESPAN_QOS_POLICY_ID: u32 = 25;
pub const DDS_SHARE_QOS_POLICY_ID: u32 = 26;
pub const DDS_SCHEDULING_QOS_POLICY_ID: u32 = 27;
pub const DDS_ERRORCODE_UNDEFINED: u32 = 0;
pub const DDS_ERRORCODE_ERROR: u32 = 1;
pub const DDS_ERRORCODE_OUT_OF_RESOURCES: u32 = 2;
pub const DDS_ERRORCODE_CREATION_KERNEL_ENTITY_FAILED: u32 = 3;
pub const DDS_ERRORCODE_INVALID_VALUE: u32 = 4;
pub const DDS_ERRORCODE_INVALID_DURATION: u32 = 5;
pub const DDS_ERRORCODE_INVALID_TIME: u32 = 6;
pub const DDS_ERRORCODE_ENTITY_INUSE: u32 = 7;
pub const DDS_ERRORCODE_CONTAINS_ENTITIES: u32 = 8;
pub const DDS_ERRORCODE_ENTITY_UNKNOWN: u32 = 9;
pub const DDS_ERRORCODE_HANDLE_NOT_REGISTERED: u32 = 10;
pub const DDS_ERRORCODE_HANDLE_NOT_MATCH: u32 = 11;
pub const DDS_ERRORCODE_HANDLE_INVALID: u32 = 12;
pub const DDS_ERRORCODE_INVALID_SEQUENCE: u32 = 13;
pub const DDS_ERRORCODE_UNSUPPORTED_VALUE: u32 = 14;
pub const DDS_ERRORCODE_INCONSISTENT_VALUE: u32 = 15;
pub const DDS_ERRORCODE_IMMUTABLE_QOS_POLICY: u32 = 16;
pub const DDS_ERRORCODE_INCONSISTENT_QOS: u32 = 17;
pub const DDS_ERRORCODE_UNSUPPORTED_QOS_POLICY: u32 = 18;
pub const DDS_ERRORCODE_CONTAINS_CONDITIONS: u32 = 19;
pub const DDS_ERRORCODE_CONTAINS_LOANS: u32 = 20;
pub const DDS_ERRORCODE_INCONSISTENT_TOPIC: u32 = 21;
pub type DDS_short = ::std::os::raw::c_short;
pub type DDS_long = ::std::os::raw::c_int;
pub type DDS_long_long = ::std::os::raw::c_longlong;
pub type DDS_unsigned_short = ::std::os::raw::c_ushort;
pub type DDS_unsigned_long = ::std::os::raw::c_uint;
pub type DDS_unsigned_long_long = ::std::os::raw::c_ulonglong;
pub type DDS_float = f32;
pub type DDS_double = f64;
pub type DDS_long_double = f64;
pub type DDS_char = ::std::os::raw::c_char;
pub type DDS_octet = ::std::os::raw::c_uchar;
pub type DDS_boolean = ::std::os::raw::c_uchar;
pub type DDS_string = *mut DDS_char;
pub type DDS_Object = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_octet {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_octet,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_octet__alloc() -> *mut DDS_sequence_octet;
}
extern "C" {
    pub fn DDS_sequence_octet_allocbuf(len: DDS_unsigned_long) -> *mut DDS_octet;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_string {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_string,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_string__alloc() -> *mut DDS_sequence_string;
}
extern "C" {
    pub fn DDS_sequence_string_allocbuf(len: DDS_unsigned_long) -> *mut DDS_string;
}
pub type DDS_octSeq = DDS_sequence_octet;
extern "C" {
    pub fn DDS_octSeq__alloc() -> *mut DDS_octSeq;
}
extern "C" {
    pub fn DDS_octSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_octet;
}
pub type DDS_BuiltinTopicKey_t = [DDS_long; 3usize];
pub type DDS_BuiltinTopicKey_t_slice = DDS_long;
extern "C" {
    pub fn DDS_BuiltinTopicKey_t__alloc() -> *mut DDS_BuiltinTopicKey_t_slice;
}
pub type DDS_StringSeq = DDS_sequence_string;
extern "C" {
    pub fn DDS_StringSeq__alloc() -> *mut DDS_StringSeq;
}
extern "C" {
    pub fn DDS_StringSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_string;
}
pub type DDS_DataRepresentationId_t = DDS_short;
extern "C" {
    pub fn DDS_DataRepresentationId_t__alloc() -> *mut DDS_DataRepresentationId_t;
}
extern "C" {
    pub fn DDS_Duration_t__alloc() -> *mut DDS_Duration_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_Duration_t {
    pub sec: DDS_long,
    pub nanosec: DDS_unsigned_long,
}
extern "C" {
    pub fn DDS_UserDataQosPolicy__alloc() -> *mut DDS_UserDataQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_UserDataQosPolicy {
    pub value: DDS_octSeq,
}
extern "C" {
    pub fn DDS_TopicDataQosPolicy__alloc() -> *mut DDS_TopicDataQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TopicDataQosPolicy {
    pub value: DDS_octSeq,
}
extern "C" {
    pub fn DDS_GroupDataQosPolicy__alloc() -> *mut DDS_GroupDataQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_GroupDataQosPolicy {
    pub value: DDS_octSeq,
}
extern "C" {
    pub fn DDS_TransportPriorityQosPolicy__alloc() -> *mut DDS_TransportPriorityQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TransportPriorityQosPolicy {
    pub value: DDS_long,
}
extern "C" {
    pub fn DDS_LifespanQosPolicy__alloc() -> *mut DDS_LifespanQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_LifespanQosPolicy {
    pub duration: DDS_Duration_t,
}
pub const DDS_DurabilityQosPolicyKind_e_DDS_VOLATILE_DURABILITY_QOS: DDS_DurabilityQosPolicyKind_e =
    0;
pub const DDS_DurabilityQosPolicyKind_e_DDS_TRANSIENT_LOCAL_DURABILITY_QOS:
    DDS_DurabilityQosPolicyKind_e = 1;
pub const DDS_DurabilityQosPolicyKind_e_DDS_TRANSIENT_DURABILITY_QOS:
    DDS_DurabilityQosPolicyKind_e = 2;
pub const DDS_DurabilityQosPolicyKind_e_DDS_PERSISTENT_DURABILITY_QOS:
    DDS_DurabilityQosPolicyKind_e = 3;
pub type DDS_DurabilityQosPolicyKind_e = u32;
pub use self::DDS_DurabilityQosPolicyKind_e as DDS_DurabilityQosPolicyKind;
extern "C" {
    pub fn DDS_DurabilityQosPolicy__alloc() -> *mut DDS_DurabilityQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DurabilityQosPolicy {
    pub kind: DDS_DurabilityQosPolicyKind,
}
pub const DDS_PresentationQosPolicyAccessScopeKind_e_DDS_INSTANCE_PRESENTATION_QOS:
    DDS_PresentationQosPolicyAccessScopeKind_e = 0;
pub const DDS_PresentationQosPolicyAccessScopeKind_e_DDS_TOPIC_PRESENTATION_QOS:
    DDS_PresentationQosPolicyAccessScopeKind_e = 1;
pub const DDS_PresentationQosPolicyAccessScopeKind_e_DDS_GROUP_PRESENTATION_QOS:
    DDS_PresentationQosPolicyAccessScopeKind_e = 2;
pub type DDS_PresentationQosPolicyAccessScopeKind_e = u32;
pub use self::DDS_PresentationQosPolicyAccessScopeKind_e as DDS_PresentationQosPolicyAccessScopeKind;
extern "C" {
    pub fn DDS_PresentationQosPolicy__alloc() -> *mut DDS_PresentationQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PresentationQosPolicy {
    pub access_scope: DDS_PresentationQosPolicyAccessScopeKind,
    pub coherent_access: DDS_boolean,
    pub ordered_access: DDS_boolean,
}
extern "C" {
    pub fn DDS_DeadlineQosPolicy__alloc() -> *mut DDS_DeadlineQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DeadlineQosPolicy {
    pub period: DDS_Duration_t,
}
extern "C" {
    pub fn DDS_LatencyBudgetQosPolicy__alloc() -> *mut DDS_LatencyBudgetQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_LatencyBudgetQosPolicy {
    pub duration: DDS_Duration_t,
}
pub const DDS_OwnershipQosPolicyKind_e_DDS_SHARED_OWNERSHIP_QOS: DDS_OwnershipQosPolicyKind_e = 0;
pub const DDS_OwnershipQosPolicyKind_e_DDS_EXCLUSIVE_OWNERSHIP_QOS: DDS_OwnershipQosPolicyKind_e =
    1;
pub type DDS_OwnershipQosPolicyKind_e = u32;
pub use self::DDS_OwnershipQosPolicyKind_e as DDS_OwnershipQosPolicyKind;
extern "C" {
    pub fn DDS_OwnershipQosPolicy__alloc() -> *mut DDS_OwnershipQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_OwnershipQosPolicy {
    pub kind: DDS_OwnershipQosPolicyKind,
}
extern "C" {
    pub fn DDS_OwnershipStrengthQosPolicy__alloc() -> *mut DDS_OwnershipStrengthQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_OwnershipStrengthQosPolicy {
    pub value: DDS_long,
}
pub const DDS_LivelinessQosPolicyKind_e_DDS_AUTOMATIC_LIVELINESS_QOS:
    DDS_LivelinessQosPolicyKind_e = 0;
pub const DDS_LivelinessQosPolicyKind_e_DDS_MANUAL_BY_PARTICIPANT_LIVELINESS_QOS:
    DDS_LivelinessQosPolicyKind_e = 1;
pub const DDS_LivelinessQosPolicyKind_e_DDS_MANUAL_BY_TOPIC_LIVELINESS_QOS:
    DDS_LivelinessQosPolicyKind_e = 2;
pub type DDS_LivelinessQosPolicyKind_e = u32;
pub use self::DDS_LivelinessQosPolicyKind_e as DDS_LivelinessQosPolicyKind;
extern "C" {
    pub fn DDS_LivelinessQosPolicy__alloc() -> *mut DDS_LivelinessQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_LivelinessQosPolicy {
    pub kind: DDS_LivelinessQosPolicyKind,
    pub lease_duration: DDS_Duration_t,
}
extern "C" {
    pub fn DDS_TimeBasedFilterQosPolicy__alloc() -> *mut DDS_TimeBasedFilterQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TimeBasedFilterQosPolicy {
    pub minimum_separation: DDS_Duration_t,
}
extern "C" {
    pub fn DDS_PartitionQosPolicy__alloc() -> *mut DDS_PartitionQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PartitionQosPolicy {
    pub name: DDS_StringSeq,
}
pub const DDS_ReliabilityQosPolicyKind_e_DDS_BEST_EFFORT_RELIABILITY_QOS:
    DDS_ReliabilityQosPolicyKind_e = 0;
pub const DDS_ReliabilityQosPolicyKind_e_DDS_RELIABLE_RELIABILITY_QOS:
    DDS_ReliabilityQosPolicyKind_e = 1;
pub type DDS_ReliabilityQosPolicyKind_e = u32;
pub use self::DDS_ReliabilityQosPolicyKind_e as DDS_ReliabilityQosPolicyKind;
extern "C" {
    pub fn DDS_ReliabilityQosPolicy__alloc() -> *mut DDS_ReliabilityQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ReliabilityQosPolicy {
    pub kind: DDS_ReliabilityQosPolicyKind,
    pub max_blocking_time: DDS_Duration_t,
    pub synchronous: DDS_boolean,
}
pub const DDS_DestinationOrderQosPolicyKind_e_DDS_BY_RECEPTION_TIMESTAMP_DESTINATIONORDER_QOS:
    DDS_DestinationOrderQosPolicyKind_e = 0;
pub const DDS_DestinationOrderQosPolicyKind_e_DDS_BY_SOURCE_TIMESTAMP_DESTINATIONORDER_QOS:
    DDS_DestinationOrderQosPolicyKind_e = 1;
pub type DDS_DestinationOrderQosPolicyKind_e = u32;
pub use self::DDS_DestinationOrderQosPolicyKind_e as DDS_DestinationOrderQosPolicyKind;
extern "C" {
    pub fn DDS_DestinationOrderQosPolicy__alloc() -> *mut DDS_DestinationOrderQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DestinationOrderQosPolicy {
    pub kind: DDS_DestinationOrderQosPolicyKind,
}
pub const DDS_HistoryQosPolicyKind_e_DDS_KEEP_LAST_HISTORY_QOS: DDS_HistoryQosPolicyKind_e = 0;
pub const DDS_HistoryQosPolicyKind_e_DDS_KEEP_ALL_HISTORY_QOS: DDS_HistoryQosPolicyKind_e = 1;
pub type DDS_HistoryQosPolicyKind_e = u32;
pub use self::DDS_HistoryQosPolicyKind_e as DDS_HistoryQosPolicyKind;
extern "C" {
    pub fn DDS_HistoryQosPolicy__alloc() -> *mut DDS_HistoryQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_HistoryQosPolicy {
    pub kind: DDS_HistoryQosPolicyKind,
    pub depth: DDS_long,
}
extern "C" {
    pub fn DDS_ResourceLimitsQosPolicy__alloc() -> *mut DDS_ResourceLimitsQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ResourceLimitsQosPolicy {
    pub max_samples: DDS_long,
    pub max_instances: DDS_long,
    pub max_samples_per_instance: DDS_long,
}
extern "C" {
    pub fn DDS_DurabilityServiceQosPolicy__alloc() -> *mut DDS_DurabilityServiceQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DurabilityServiceQosPolicy {
    pub service_cleanup_delay: DDS_Duration_t,
    pub history_kind: DDS_HistoryQosPolicyKind,
    pub history_depth: DDS_long,
    pub max_samples: DDS_long,
    pub max_instances: DDS_long,
    pub max_samples_per_instance: DDS_long,
}
extern "C" {
    pub fn DDS_ProductDataQosPolicy__alloc() -> *mut DDS_ProductDataQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ProductDataQosPolicy {
    pub value: DDS_string,
}
extern "C" {
    pub fn DDS_EntityFactoryQosPolicy__alloc() -> *mut DDS_EntityFactoryQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_EntityFactoryQosPolicy {
    pub autoenable_created_entities: DDS_boolean,
}
extern "C" {
    pub fn DDS_ShareQosPolicy__alloc() -> *mut DDS_ShareQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ShareQosPolicy {
    pub name: DDS_string,
    pub enable: DDS_boolean,
}
extern "C" {
    pub fn DDS_WriterDataLifecycleQosPolicy__alloc() -> *mut DDS_WriterDataLifecycleQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_WriterDataLifecycleQosPolicy {
    pub autodispose_unregistered_instances: DDS_boolean,
    pub autopurge_suspended_samples_delay: DDS_Duration_t,
    pub autounregister_instance_delay: DDS_Duration_t,
}
pub const DDS_InvalidSampleVisibilityQosPolicyKind_e_DDS_NO_INVALID_SAMPLES:
    DDS_InvalidSampleVisibilityQosPolicyKind_e = 0;
pub const DDS_InvalidSampleVisibilityQosPolicyKind_e_DDS_MINIMUM_INVALID_SAMPLES:
    DDS_InvalidSampleVisibilityQosPolicyKind_e = 1;
pub const DDS_InvalidSampleVisibilityQosPolicyKind_e_DDS_ALL_INVALID_SAMPLES:
    DDS_InvalidSampleVisibilityQosPolicyKind_e = 2;
pub type DDS_InvalidSampleVisibilityQosPolicyKind_e = u32;
pub use self::DDS_InvalidSampleVisibilityQosPolicyKind_e as DDS_InvalidSampleVisibilityQosPolicyKind;
extern "C" {
    pub fn DDS_InvalidSampleVisibilityQosPolicy__alloc() -> *mut DDS_InvalidSampleVisibilityQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_InvalidSampleVisibilityQosPolicy {
    pub kind: DDS_InvalidSampleVisibilityQosPolicyKind,
}
extern "C" {
    pub fn DDS_SubscriptionKeyQosPolicy__alloc() -> *mut DDS_SubscriptionKeyQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SubscriptionKeyQosPolicy {
    pub use_key_list: DDS_boolean,
    pub key_list: DDS_StringSeq,
}
extern "C" {
    pub fn DDS_ReaderDataLifecycleQosPolicy__alloc() -> *mut DDS_ReaderDataLifecycleQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ReaderDataLifecycleQosPolicy {
    pub autopurge_nowriter_samples_delay: DDS_Duration_t,
    pub autopurge_disposed_samples_delay: DDS_Duration_t,
    pub autopurge_dispose_all: DDS_boolean,
    pub enable_invalid_samples: DDS_boolean,
    pub invalid_sample_visibility: DDS_InvalidSampleVisibilityQosPolicy,
}
extern "C" {
    pub fn DDS_UserKeyQosPolicy__alloc() -> *mut DDS_UserKeyQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_UserKeyQosPolicy {
    pub enable: DDS_boolean,
    pub expression: DDS_string,
}
extern "C" {
    pub fn DDS_ReaderLifespanQosPolicy__alloc() -> *mut DDS_ReaderLifespanQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ReaderLifespanQosPolicy {
    pub use_lifespan: DDS_boolean,
    pub duration: DDS_Duration_t,
}
extern "C" {
    pub fn DDS_TypeHash__alloc() -> *mut DDS_TypeHash;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TypeHash {
    pub msb: DDS_unsigned_long_long,
    pub lsb: DDS_unsigned_long_long,
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicData__alloc() -> *mut DDS_ParticipantBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ParticipantBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub user_data: DDS_UserDataQosPolicy,
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicData__alloc() -> *mut DDS_TopicBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TopicBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub name: DDS_string,
    pub type_name: DDS_string,
    pub durability: DDS_DurabilityQosPolicy,
    pub durability_service: DDS_DurabilityServiceQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub transport_priority: DDS_TransportPriorityQosPolicy,
    pub lifespan: DDS_LifespanQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
    pub topic_data: DDS_TopicDataQosPolicy,
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicData__alloc() -> *mut DDS_TypeBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TypeBuiltinTopicData {
    pub name: DDS_string,
    pub data_representation_id: DDS_DataRepresentationId_t,
    pub type_hash: DDS_TypeHash,
    pub meta_data: DDS_octSeq,
    pub extentions: DDS_octSeq,
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicData__alloc() -> *mut DDS_PublicationBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PublicationBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub participant_key: DDS_BuiltinTopicKey_t,
    pub topic_name: DDS_string,
    pub type_name: DDS_string,
    pub durability: DDS_DurabilityQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub lifespan: DDS_LifespanQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub user_data: DDS_UserDataQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
    pub ownership_strength: DDS_OwnershipStrengthQosPolicy,
    pub presentation: DDS_PresentationQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
    pub topic_data: DDS_TopicDataQosPolicy,
    pub group_data: DDS_GroupDataQosPolicy,
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicData__alloc() -> *mut DDS_SubscriptionBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SubscriptionBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub participant_key: DDS_BuiltinTopicKey_t,
    pub topic_name: DDS_string,
    pub type_name: DDS_string,
    pub durability: DDS_DurabilityQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub user_data: DDS_UserDataQosPolicy,
    pub time_based_filter: DDS_TimeBasedFilterQosPolicy,
    pub presentation: DDS_PresentationQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
    pub topic_data: DDS_TopicDataQosPolicy,
    pub group_data: DDS_GroupDataQosPolicy,
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicData__alloc() -> *mut DDS_CMParticipantBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_CMParticipantBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub product: DDS_ProductDataQosPolicy,
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicData__alloc() -> *mut DDS_CMPublisherBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_CMPublisherBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub product: DDS_ProductDataQosPolicy,
    pub participant_key: DDS_BuiltinTopicKey_t,
    pub name: DDS_string,
    pub entity_factory: DDS_EntityFactoryQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicData__alloc() -> *mut DDS_CMSubscriberBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_CMSubscriberBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub product: DDS_ProductDataQosPolicy,
    pub participant_key: DDS_BuiltinTopicKey_t,
    pub name: DDS_string,
    pub entity_factory: DDS_EntityFactoryQosPolicy,
    pub share: DDS_ShareQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicData__alloc() -> *mut DDS_CMDataWriterBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_CMDataWriterBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub product: DDS_ProductDataQosPolicy,
    pub publisher_key: DDS_BuiltinTopicKey_t,
    pub name: DDS_string,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub writer_data_lifecycle: DDS_WriterDataLifecycleQosPolicy,
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicData__alloc() -> *mut DDS_CMDataReaderBuiltinTopicData;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_CMDataReaderBuiltinTopicData {
    pub key: DDS_BuiltinTopicKey_t,
    pub product: DDS_ProductDataQosPolicy,
    pub subscriber_key: DDS_BuiltinTopicKey_t,
    pub name: DDS_string,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub reader_data_lifecycle: DDS_ReaderDataLifecycleQosPolicy,
    pub subscription_keys: DDS_UserKeyQosPolicy,
    pub reader_lifespan: DDS_ReaderLifespanQosPolicy,
    pub share: DDS_ShareQosPolicy,
}
extern "C" {
    pub fn DDS_Time_t__alloc() -> *mut DDS_Time_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_Time_t {
    pub sec: DDS_long,
    pub nanosec: DDS_unsigned_long,
}
pub const DDS_SchedulingClassQosPolicyKind_e_DDS_SCHEDULE_DEFAULT:
    DDS_SchedulingClassQosPolicyKind_e = 0;
pub const DDS_SchedulingClassQosPolicyKind_e_DDS_SCHEDULE_TIMESHARING:
    DDS_SchedulingClassQosPolicyKind_e = 1;
pub const DDS_SchedulingClassQosPolicyKind_e_DDS_SCHEDULE_REALTIME:
    DDS_SchedulingClassQosPolicyKind_e = 2;
pub type DDS_SchedulingClassQosPolicyKind_e = u32;
pub use self::DDS_SchedulingClassQosPolicyKind_e as DDS_SchedulingClassQosPolicyKind;
extern "C" {
    pub fn DDS_SchedulingClassQosPolicy__alloc() -> *mut DDS_SchedulingClassQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SchedulingClassQosPolicy {
    pub kind: DDS_SchedulingClassQosPolicyKind,
}
pub const DDS_SchedulingPriorityQosPolicyKind_e_DDS_PRIORITY_RELATIVE:
    DDS_SchedulingPriorityQosPolicyKind_e = 0;
pub const DDS_SchedulingPriorityQosPolicyKind_e_DDS_PRIORITY_ABSOLUTE:
    DDS_SchedulingPriorityQosPolicyKind_e = 1;
pub type DDS_SchedulingPriorityQosPolicyKind_e = u32;
pub use self::DDS_SchedulingPriorityQosPolicyKind_e as DDS_SchedulingPriorityQosPolicyKind;
extern "C" {
    pub fn DDS_SchedulingPriorityQosPolicy__alloc() -> *mut DDS_SchedulingPriorityQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SchedulingPriorityQosPolicy {
    pub kind: DDS_SchedulingPriorityQosPolicyKind,
}
extern "C" {
    pub fn DDS_SchedulingQosPolicy__alloc() -> *mut DDS_SchedulingQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SchedulingQosPolicy {
    pub scheduling_class: DDS_SchedulingClassQosPolicy,
    pub scheduling_priority_kind: DDS_SchedulingPriorityQosPolicy,
    pub scheduling_priority: DDS_long,
}
extern "C" {
    pub fn DDS_ViewKeyQosPolicy__alloc() -> *mut DDS_ViewKeyQosPolicy;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ViewKeyQosPolicy {
    pub use_key_list: DDS_boolean,
    pub key_list: DDS_StringSeq,
}
extern "C" {
    pub fn DDS_DataReaderViewQos__alloc() -> *mut DDS_DataReaderViewQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DataReaderViewQos {
    pub view_keys: DDS_ViewKeyQosPolicy,
}
extern "C" {
    pub fn DDS_DomainParticipantFactoryQos__alloc() -> *mut DDS_DomainParticipantFactoryQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DomainParticipantFactoryQos {
    pub entity_factory: DDS_EntityFactoryQosPolicy,
}
extern "C" {
    pub fn DDS_DomainParticipantQos__alloc() -> *mut DDS_DomainParticipantQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DomainParticipantQos {
    pub user_data: DDS_UserDataQosPolicy,
    pub entity_factory: DDS_EntityFactoryQosPolicy,
    pub watchdog_scheduling: DDS_SchedulingQosPolicy,
    pub listener_scheduling: DDS_SchedulingQosPolicy,
}
extern "C" {
    pub fn DDS_TopicQos__alloc() -> *mut DDS_TopicQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TopicQos {
    pub topic_data: DDS_TopicDataQosPolicy,
    pub durability: DDS_DurabilityQosPolicy,
    pub durability_service: DDS_DurabilityServiceQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub transport_priority: DDS_TransportPriorityQosPolicy,
    pub lifespan: DDS_LifespanQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
}
extern "C" {
    pub fn DDS_DataWriterQos__alloc() -> *mut DDS_DataWriterQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DataWriterQos {
    pub durability: DDS_DurabilityQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub transport_priority: DDS_TransportPriorityQosPolicy,
    pub lifespan: DDS_LifespanQosPolicy,
    pub user_data: DDS_UserDataQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
    pub ownership_strength: DDS_OwnershipStrengthQosPolicy,
    pub writer_data_lifecycle: DDS_WriterDataLifecycleQosPolicy,
}
extern "C" {
    pub fn DDS_PublisherQos__alloc() -> *mut DDS_PublisherQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PublisherQos {
    pub presentation: DDS_PresentationQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
    pub group_data: DDS_GroupDataQosPolicy,
    pub entity_factory: DDS_EntityFactoryQosPolicy,
}
extern "C" {
    pub fn DDS_DataReaderQos__alloc() -> *mut DDS_DataReaderQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DataReaderQos {
    pub durability: DDS_DurabilityQosPolicy,
    pub deadline: DDS_DeadlineQosPolicy,
    pub latency_budget: DDS_LatencyBudgetQosPolicy,
    pub liveliness: DDS_LivelinessQosPolicy,
    pub reliability: DDS_ReliabilityQosPolicy,
    pub destination_order: DDS_DestinationOrderQosPolicy,
    pub history: DDS_HistoryQosPolicy,
    pub resource_limits: DDS_ResourceLimitsQosPolicy,
    pub user_data: DDS_UserDataQosPolicy,
    pub ownership: DDS_OwnershipQosPolicy,
    pub time_based_filter: DDS_TimeBasedFilterQosPolicy,
    pub reader_data_lifecycle: DDS_ReaderDataLifecycleQosPolicy,
    pub subscription_keys: DDS_SubscriptionKeyQosPolicy,
    pub reader_lifespan: DDS_ReaderLifespanQosPolicy,
    pub share: DDS_ShareQosPolicy,
}
extern "C" {
    pub fn DDS_SubscriberQos__alloc() -> *mut DDS_SubscriberQos;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SubscriberQos {
    pub presentation: DDS_PresentationQosPolicy,
    pub partition: DDS_PartitionQosPolicy,
    pub group_data: DDS_GroupDataQosPolicy,
    pub entity_factory: DDS_EntityFactoryQosPolicy,
    pub share: DDS_ShareQosPolicy,
}
extern "C" {
    #[link_name = "\u{1}DDS_PARTICIPANT_QOS_DEFAULT"]
    pub static mut DDS_PARTICIPANT_QOS_DEFAULT: *const DDS_DomainParticipantQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_TOPIC_QOS_DEFAULT"]
    pub static mut DDS_TOPIC_QOS_DEFAULT: *const DDS_TopicQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_PUBLISHER_QOS_DEFAULT"]
    pub static mut DDS_PUBLISHER_QOS_DEFAULT: *const DDS_PublisherQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_SUBSCRIBER_QOS_DEFAULT"]
    pub static mut DDS_SUBSCRIBER_QOS_DEFAULT: *const DDS_SubscriberQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_DATAREADER_QOS_DEFAULT"]
    pub static mut DDS_DATAREADER_QOS_DEFAULT: *const DDS_DataReaderQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_DATAREADER_QOS_USE_TOPIC_QOS"]
    pub static mut DDS_DATAREADER_QOS_USE_TOPIC_QOS: *const DDS_DataReaderQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_DATAREADERVIEW_QOS_DEFAULT"]
    pub static mut DDS_DATAREADERVIEW_QOS_DEFAULT: *const DDS_DataReaderViewQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_DATAWRITER_QOS_DEFAULT"]
    pub static mut DDS_DATAWRITER_QOS_DEFAULT: *const DDS_DataWriterQos;
}
extern "C" {
    #[link_name = "\u{1}DDS_DATAWRITER_QOS_USE_TOPIC_QOS"]
    pub static mut DDS_DATAWRITER_QOS_USE_TOPIC_QOS: *const DDS_DataWriterQos;
}
pub type DDS_ReturnCode_t = DDS_long;
pub type DDS_QosPolicyId_t = DDS_long;
extern "C" {
    pub fn DDS_free(object: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn DDS_string_alloc(len: DDS_unsigned_long) -> *mut DDS_char;
}
extern "C" {
    pub fn DDS_string_dup(src: *const DDS_char) -> *mut DDS_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_s {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut ::std::os::raw::c_void,
    pub _release: DDS_boolean,
}
pub type DDS_sequence = *mut DDS_sequence_s;
extern "C" {
    pub fn DDS_sequence_set_release(sequence: *mut ::std::os::raw::c_void, release: DDS_boolean);
}
extern "C" {
    pub fn DDS_sequence_get_release(sequence: *mut ::std::os::raw::c_void) -> DDS_boolean;
}
pub type DDS_DomainId_t = DDS_long;
pub type DDS_InstanceHandle_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_InstanceHandle_t {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_InstanceHandle_t,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_InstanceHandle_t__alloc() -> *mut DDS_sequence_DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_sequence_DDS_InstanceHandle_t_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_InstanceHandle_t;
}
pub type DDS_InstanceHandleSeq = DDS_sequence_DDS_InstanceHandle_t;
extern "C" {
    pub fn DDS_InstanceHandleSeq__alloc() -> *mut DDS_InstanceHandleSeq;
}
extern "C" {
    pub fn DDS_InstanceHandleSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_InstanceHandle_t;
}
pub type DDS_StatusKind = DDS_unsigned_long;
pub type DDS_StatusMask = DDS_unsigned_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_InconsistentTopicStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SampleLostStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
}
pub const DDS_SampleRejectedStatusKind_DDS_NOT_REJECTED: DDS_SampleRejectedStatusKind = 0;
pub const DDS_SampleRejectedStatusKind_DDS_REJECTED_BY_INSTANCES_LIMIT:
    DDS_SampleRejectedStatusKind = 1;
pub const DDS_SampleRejectedStatusKind_DDS_REJECTED_BY_SAMPLES_LIMIT: DDS_SampleRejectedStatusKind =
    2;
pub const DDS_SampleRejectedStatusKind_DDS_REJECTED_BY_SAMPLES_PER_INSTANCE_LIMIT:
    DDS_SampleRejectedStatusKind = 3;
pub type DDS_SampleRejectedStatusKind = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SampleRejectedStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub last_reason: DDS_SampleRejectedStatusKind,
    pub last_instance_handle: DDS_InstanceHandle_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_LivelinessLostStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_LivelinessChangedStatus {
    pub alive_count: DDS_long,
    pub not_alive_count: DDS_long,
    pub alive_count_change: DDS_long,
    pub not_alive_count_change: DDS_long,
    pub last_publication_handle: DDS_InstanceHandle_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_OfferedDeadlineMissedStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub last_instance_handle: DDS_InstanceHandle_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_RequestedDeadlineMissedStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub last_instance_handle: DDS_InstanceHandle_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_QosPolicyCount {
    pub policy_id: DDS_QosPolicyId_t,
    pub count: DDS_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_QosPolicyCount {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_QosPolicyCount,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_QosPolicyCount__alloc() -> *mut DDS_sequence_DDS_QosPolicyCount;
}
extern "C" {
    pub fn DDS_sequence_DDS_QosPolicyCount_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_QosPolicyCount;
}
pub type DDS_QosPolicyCountSeq = DDS_sequence_DDS_QosPolicyCount;
extern "C" {
    pub fn DDS_QosPolicyCountSeq__alloc() -> *mut DDS_QosPolicyCountSeq;
}
extern "C" {
    pub fn DDS_QosPolicyCountSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_QosPolicyCount;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_OfferedIncompatibleQosStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub last_policy_id: DDS_QosPolicyId_t,
    pub policies: DDS_QosPolicyCountSeq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_RequestedIncompatibleQosStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub last_policy_id: DDS_QosPolicyId_t,
    pub policies: DDS_QosPolicyCountSeq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PublicationMatchedStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub current_count: DDS_long,
    pub current_count_change: DDS_long,
    pub last_subscription_handle: DDS_InstanceHandle_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SubscriptionMatchedStatus {
    pub total_count: DDS_long,
    pub total_count_change: DDS_long,
    pub current_count: DDS_long,
    pub current_count_change: DDS_long,
    pub last_publication_handle: DDS_InstanceHandle_t,
}
pub type DDS_Entity = DDS_Object;
pub type DDS_DomainParticipant = DDS_Object;
pub type DDS_TypeSupport = DDS_Object;
pub type DDS_TopicDescription = DDS_Object;
pub type DDS_Topic = DDS_Object;
pub type DDS_ContentFilteredTopic = DDS_Object;
pub type DDS_MultiTopic = DDS_Object;
pub type DDS_DataWriter = DDS_Object;
pub type DDS_DataReader = DDS_Object;
pub type DDS_DataReaderView = DDS_Object;
pub type DDS_Subscriber = DDS_Object;
pub type DDS_Publisher = DDS_Object;
pub type DDS_Sample = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_Topic {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_Topic,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_Topic__alloc() -> *mut DDS_sequence_DDS_Topic;
}
extern "C" {
    pub fn DDS_sequence_DDS_Topic_allocbuf(len: DDS_unsigned_long) -> *mut DDS_Topic;
}
pub type DDS_TopicSeq = DDS_sequence_DDS_Topic;
extern "C" {
    pub fn DDS_TopicSeq__alloc() -> *mut DDS_TopicSeq;
}
extern "C" {
    pub fn DDS_TopicSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_Topic;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_DataReader {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_DataReader,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_DataReader__alloc() -> *mut DDS_sequence_DDS_DataReader;
}
extern "C" {
    pub fn DDS_sequence_DDS_DataReader_allocbuf(len: DDS_unsigned_long) -> *mut DDS_DataReader;
}
pub type DDS_DataReaderSeq = DDS_sequence_DDS_DataReader;
extern "C" {
    pub fn DDS_DataReaderSeq__alloc() -> *mut DDS_DataReaderSeq;
}
extern "C" {
    pub fn DDS_DataReaderSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_DataReader;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_Listener {
    pub listener_data: *mut ::std::os::raw::c_void,
}
pub type DDS_TopicListener_InconsistentTopicListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        topic: DDS_Topic,
        status: *const DDS_InconsistentTopicStatus,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_TopicListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_inconsistent_topic: DDS_TopicListener_InconsistentTopicListener,
}
extern "C" {
    pub fn DDS_TopicListener__alloc() -> *mut DDS_TopicListener;
}
pub type DDS_ExtTopicListener_AllDataDisposedListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, topic: DDS_Topic),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ExtTopicListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_inconsistent_topic: DDS_TopicListener_InconsistentTopicListener,
    pub on_all_data_disposed: DDS_ExtTopicListener_AllDataDisposedListener,
}
extern "C" {
    pub fn DDS_ExtTopicListener__alloc() -> *mut DDS_ExtTopicListener;
}
pub type DDS_DataWriterListener_OfferedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedDeadlineMissedStatus,
    ),
>;
pub type DDS_DataWriterListener_LivelinessLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_LivelinessLostStatus,
    ),
>;
pub type DDS_DataWriterListener_OfferedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedIncompatibleQosStatus,
    ),
>;
pub type DDS_DataWriterListener_PublicationMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_PublicationMatchedStatus,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DataWriterListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_offered_deadline_missed: DDS_DataWriterListener_OfferedDeadlineMissedListener,
    pub on_offered_incompatible_qos: DDS_DataWriterListener_OfferedIncompatibleQosListener,
    pub on_liveliness_lost: DDS_DataWriterListener_LivelinessLostListener,
    pub on_publication_matched: DDS_DataWriterListener_PublicationMatchedListener,
}
extern "C" {
    pub fn DDS_DataWriterListener__alloc() -> *mut DDS_DataWriterListener;
}
pub type DDS_PublisherListener_OfferedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedDeadlineMissedStatus,
    ),
>;
pub type DDS_PublisherListener_LivelinessLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_LivelinessLostStatus,
    ),
>;
pub type DDS_PublisherListener_OfferedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedIncompatibleQosStatus,
    ),
>;
pub type DDS_PublisherListener_PublicationMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_PublicationMatchedStatus,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_PublisherListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_offered_deadline_missed: DDS_PublisherListener_OfferedDeadlineMissedListener,
    pub on_offered_incompatible_qos: DDS_PublisherListener_OfferedIncompatibleQosListener,
    pub on_liveliness_lost: DDS_PublisherListener_LivelinessLostListener,
    pub on_publication_matched: DDS_PublisherListener_PublicationMatchedListener,
}
extern "C" {
    pub fn DDS_PublisherListener__alloc() -> *mut DDS_PublisherListener;
}
pub type DDS_DataReaderListener_RequestedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedDeadlineMissedStatus,
    ),
>;
pub type DDS_DataReaderListener_LivelinessChangedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_LivelinessChangedStatus,
    ),
>;
pub type DDS_DataReaderListener_RequestedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedIncompatibleQosStatus,
    ),
>;
pub type DDS_DataReaderListener_SampleRejectedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleRejectedStatus,
    ),
>;
pub type DDS_DataReaderListener_DataAvailableListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, reader: DDS_DataReader),
>;
pub type DDS_DataReaderListener_SubscriptionMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SubscriptionMatchedStatus,
    ),
>;
pub type DDS_DataReaderListener_SampleLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleLostStatus,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DataReaderListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_requested_deadline_missed: DDS_DataReaderListener_RequestedDeadlineMissedListener,
    pub on_requested_incompatible_qos: DDS_DataReaderListener_RequestedIncompatibleQosListener,
    pub on_sample_rejected: DDS_DataReaderListener_SampleRejectedListener,
    pub on_liveliness_changed: DDS_DataReaderListener_LivelinessChangedListener,
    pub on_data_available: DDS_DataReaderListener_DataAvailableListener,
    pub on_subscription_matched: DDS_DataReaderListener_SubscriptionMatchedListener,
    pub on_sample_lost: DDS_DataReaderListener_SampleLostListener,
}
extern "C" {
    pub fn DDS_DataReaderListener__alloc() -> *mut DDS_DataReaderListener;
}
pub type DDS_SubscriberListener_RequestedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedDeadlineMissedStatus,
    ),
>;
pub type DDS_SubscriberListener_LivelinessChangedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_LivelinessChangedStatus,
    ),
>;
pub type DDS_SubscriberListener_RequestedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedIncompatibleQosStatus,
    ),
>;
pub type DDS_SubscriberListener_SampleRejectedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleRejectedStatus,
    ),
>;
pub type DDS_SubscriberListener_DataAvailableListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, reader: DDS_DataReader),
>;
pub type DDS_SubscriberListener_SubscriptionMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SubscriptionMatchedStatus,
    ),
>;
pub type DDS_SubscriberListener_SampleLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleLostStatus,
    ),
>;
pub type DDS_SubscriberListener_DataOnReadersListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, subs: DDS_Subscriber),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SubscriberListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_requested_deadline_missed: DDS_SubscriberListener_RequestedDeadlineMissedListener,
    pub on_requested_incompatible_qos: DDS_SubscriberListener_RequestedIncompatibleQosListener,
    pub on_sample_rejected: DDS_SubscriberListener_SampleRejectedListener,
    pub on_liveliness_changed: DDS_SubscriberListener_LivelinessChangedListener,
    pub on_data_available: DDS_SubscriberListener_DataAvailableListener,
    pub on_subscription_matched: DDS_SubscriberListener_SubscriptionMatchedListener,
    pub on_sample_lost: DDS_SubscriberListener_SampleLostListener,
    pub on_data_on_readers: DDS_SubscriberListener_DataOnReadersListener,
}
extern "C" {
    pub fn DDS_SubscriberListener__alloc() -> *mut DDS_SubscriberListener;
}
pub type DDS_DomainParticipantListener_InconsistentTopicListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        topic: DDS_Topic,
        status: *const DDS_InconsistentTopicStatus,
    ),
>;
pub type DDS_DomainParticipantListener_OfferedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedDeadlineMissedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_LivelinessLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_LivelinessLostStatus,
    ),
>;
pub type DDS_DomainParticipantListener_OfferedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_OfferedIncompatibleQosStatus,
    ),
>;
pub type DDS_DomainParticipantListener_PublicationMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        writer: DDS_DataWriter,
        status: *const DDS_PublicationMatchedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_RequestedDeadlineMissedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedDeadlineMissedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_LivelinessChangedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_LivelinessChangedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_RequestedIncompatibleQosListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_RequestedIncompatibleQosStatus,
    ),
>;
pub type DDS_DomainParticipantListener_SampleRejectedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleRejectedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_DataAvailableListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, reader: DDS_DataReader),
>;
pub type DDS_DomainParticipantListener_SubscriptionMatchedListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SubscriptionMatchedStatus,
    ),
>;
pub type DDS_DomainParticipantListener_SampleLostListener = ::std::option::Option<
    unsafe extern "C" fn(
        listener_data: *mut ::std::os::raw::c_void,
        reader: DDS_DataReader,
        status: *const DDS_SampleLostStatus,
    ),
>;
pub type DDS_DomainParticipantListener_DataOnReadersListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, subs: DDS_Subscriber),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_DomainParticipantListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_inconsistent_topic: DDS_DomainParticipantListener_InconsistentTopicListener,
    pub on_offered_deadline_missed: DDS_DomainParticipantListener_OfferedDeadlineMissedListener,
    pub on_offered_incompatible_qos: DDS_DomainParticipantListener_OfferedIncompatibleQosListener,
    pub on_liveliness_lost: DDS_DomainParticipantListener_LivelinessLostListener,
    pub on_publication_matched: DDS_DomainParticipantListener_PublicationMatchedListener,
    pub on_requested_deadline_missed: DDS_DomainParticipantListener_RequestedDeadlineMissedListener,
    pub on_requested_incompatible_qos:
        DDS_DomainParticipantListener_RequestedIncompatibleQosListener,
    pub on_sample_rejected: DDS_DomainParticipantListener_SampleRejectedListener,
    pub on_liveliness_changed: DDS_DomainParticipantListener_LivelinessChangedListener,
    pub on_data_available: DDS_DomainParticipantListener_DataAvailableListener,
    pub on_subscription_matched: DDS_DomainParticipantListener_SubscriptionMatchedListener,
    pub on_sample_lost: DDS_DomainParticipantListener_SampleLostListener,
    pub on_data_on_readers: DDS_DomainParticipantListener_DataOnReadersListener,
}
extern "C" {
    pub fn DDS_DomainParticipantListener__alloc() -> *mut DDS_DomainParticipantListener;
}
pub type DDS_ExtDomainParticipantListener_AllDataDisposedListener = ::std::option::Option<
    unsafe extern "C" fn(listener_data: *mut ::std::os::raw::c_void, topic: DDS_Topic),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_ExtDomainParticipantListener {
    pub listener_data: *mut ::std::os::raw::c_void,
    pub on_inconsistent_topic: DDS_DomainParticipantListener_InconsistentTopicListener,
    pub on_offered_deadline_missed: DDS_DomainParticipantListener_OfferedDeadlineMissedListener,
    pub on_offered_incompatible_qos: DDS_DomainParticipantListener_OfferedIncompatibleQosListener,
    pub on_liveliness_lost: DDS_DomainParticipantListener_LivelinessLostListener,
    pub on_publication_matched: DDS_DomainParticipantListener_PublicationMatchedListener,
    pub on_requested_deadline_missed: DDS_DomainParticipantListener_RequestedDeadlineMissedListener,
    pub on_requested_incompatible_qos:
        DDS_DomainParticipantListener_RequestedIncompatibleQosListener,
    pub on_sample_rejected: DDS_DomainParticipantListener_SampleRejectedListener,
    pub on_liveliness_changed: DDS_DomainParticipantListener_LivelinessChangedListener,
    pub on_data_available: DDS_DomainParticipantListener_DataAvailableListener,
    pub on_subscription_matched: DDS_DomainParticipantListener_SubscriptionMatchedListener,
    pub on_sample_lost: DDS_DomainParticipantListener_SampleLostListener,
    pub on_data_on_readers: DDS_DomainParticipantListener_DataOnReadersListener,
    pub on_all_data_disposed: DDS_ExtDomainParticipantListener_AllDataDisposedListener,
}
extern "C" {
    pub fn DDS_ExtDomainParticipantListener__alloc() -> *mut DDS_ExtDomainParticipantListener;
}
pub type DDS_Condition = DDS_Object;
extern "C" {
    pub fn DDS_Condition_get_trigger_value(_this: DDS_Condition) -> DDS_boolean;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_Condition {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_Condition,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_Condition__alloc() -> *mut DDS_sequence_DDS_Condition;
}
extern "C" {
    pub fn DDS_sequence_DDS_Condition_allocbuf(len: DDS_unsigned_long) -> *mut DDS_Condition;
}
pub type DDS_ConditionSeq = DDS_sequence_DDS_Condition;
extern "C" {
    pub fn DDS_ConditionSeq__alloc() -> *mut DDS_ConditionSeq;
}
extern "C" {
    pub fn DDS_ConditionSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_Condition;
}
pub type DDS_WaitSet = DDS_Object;
extern "C" {
    pub fn DDS_WaitSet_wait(
        _this: DDS_WaitSet,
        active_conditions: *mut DDS_ConditionSeq,
        timeout: *const DDS_Duration_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_WaitSet_attach_condition(
        _this: DDS_WaitSet,
        cond: DDS_Condition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_WaitSet_detach_condition(
        _this: DDS_WaitSet,
        cond: DDS_Condition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_WaitSet_get_conditions(
        _this: DDS_WaitSet,
        attached_conditions: *mut DDS_ConditionSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_WaitSet__alloc() -> DDS_WaitSet;
}
pub type DDS_GuardCondition = DDS_Object;
extern "C" {
    pub fn DDS_GuardCondition_set_trigger_value(
        _this: DDS_GuardCondition,
        value: DDS_boolean,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_GuardCondition__alloc() -> DDS_GuardCondition;
}
pub type DDS_StatusCondition = DDS_Object;
extern "C" {
    pub fn DDS_StatusCondition_get_enabled_statuses(_this: DDS_StatusCondition) -> DDS_StatusMask;
}
extern "C" {
    pub fn DDS_StatusCondition_set_enabled_statuses(
        _this: DDS_StatusCondition,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_StatusCondition_get_entity(_this: DDS_StatusCondition) -> DDS_Entity;
}
pub type DDS_SampleStateKind = DDS_unsigned_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_SampleStateKind {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_SampleStateKind,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_SampleStateKind__alloc() -> *mut DDS_sequence_DDS_SampleStateKind;
}
extern "C" {
    pub fn DDS_sequence_DDS_SampleStateKind_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_SampleStateKind;
}
pub type DDS_SampleStateSeq = DDS_sequence_DDS_SampleStateKind;
extern "C" {
    pub fn DDS_SampleStateSeq__alloc() -> *mut DDS_SampleStateSeq;
}
extern "C" {
    pub fn DDS_SampleStateSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_SampleStateKind;
}
pub type DDS_SampleStateMask = DDS_unsigned_long;
pub type DDS_ViewStateKind = DDS_unsigned_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_ViewStateKind {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_ViewStateKind,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_ViewStateKind__alloc() -> *mut DDS_sequence_DDS_ViewStateKind;
}
extern "C" {
    pub fn DDS_sequence_DDS_ViewStateKind_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_ViewStateKind;
}
pub type DDS_ViewStateSeq = DDS_sequence_DDS_ViewStateKind;
extern "C" {
    pub fn DDS_ViewStateSeq__alloc() -> *mut DDS_ViewStateSeq;
}
extern "C" {
    pub fn DDS_ViewStateSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_ViewStateKind;
}
pub type DDS_ViewStateMask = DDS_unsigned_long;
pub type DDS_InstanceStateKind = DDS_unsigned_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_InstanceStateKind {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_InstanceStateKind,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_InstanceStateKind__alloc() -> *mut DDS_sequence_DDS_InstanceStateKind;
}
extern "C" {
    pub fn DDS_sequence_DDS_InstanceStateKind_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_InstanceStateKind;
}
pub type DDS_InstanceStateSeq = DDS_sequence_DDS_InstanceStateKind;
extern "C" {
    pub fn DDS_InstanceStateSeq__alloc() -> *mut DDS_InstanceStateSeq;
}
extern "C" {
    pub fn DDS_InstanceStateSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_InstanceStateKind;
}
pub type DDS_InstanceStateMask = DDS_unsigned_long;
pub type DDS_ReadCondition = DDS_Object;
extern "C" {
    pub fn DDS_ReadCondition_get_sample_state_mask(_this: DDS_ReadCondition)
        -> DDS_SampleStateMask;
}
extern "C" {
    pub fn DDS_ReadCondition_get_view_state_mask(_this: DDS_ReadCondition) -> DDS_ViewStateMask;
}
extern "C" {
    pub fn DDS_ReadCondition_get_instance_state_mask(
        _this: DDS_ReadCondition,
    ) -> DDS_InstanceStateMask;
}
extern "C" {
    pub fn DDS_ReadCondition_get_datareader(_this: DDS_ReadCondition) -> DDS_DataReader;
}
extern "C" {
    pub fn DDS_ReadCondition_get_datareaderview(_this: DDS_ReadCondition) -> DDS_DataReaderView;
}
pub type DDS_QueryCondition = DDS_Object;
extern "C" {
    pub fn DDS_QueryCondition_get_query_expression(_this: DDS_QueryCondition) -> DDS_string;
}
extern "C" {
    pub fn DDS_QueryCondition_get_query_parameters(
        _this: DDS_QueryCondition,
        query_parameters: *mut DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QueryCondition_set_query_parameters(
        _this: DDS_QueryCondition,
        query_parameters: *const DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Entity_enable(_this: DDS_Entity) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Entity_get_statuscondition(_this: DDS_Entity) -> DDS_StatusCondition;
}
extern "C" {
    pub fn DDS_Entity_get_status_changes(_this: DDS_Entity) -> DDS_StatusMask;
}
extern "C" {
    pub fn DDS_Entity_get_instance_handle(_this: DDS_Entity) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeSupport__alloc(
        name: *const DDS_char,
        keys: *const DDS_char,
        spec: *const DDS_char,
    ) -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_TypeSupport_register_type(
        _this: DDS_TypeSupport,
        participant: DDS_DomainParticipant,
        type_name: *const DDS_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeSupport_get_description(_this: DDS_TypeSupport) -> *mut DDS_char;
}
extern "C" {
    pub fn DDS_TypeSupport_get_key_list(_this: DDS_TypeSupport) -> *mut DDS_char;
}
extern "C" {
    pub fn DDS_TypeSupport_get_type_name(_this: DDS_TypeSupport) -> *mut DDS_char;
}
extern "C" {
    pub fn DDS_TypeSupport_allocbuf(
        _this: DDS_TypeSupport,
        len: DDS_unsigned_long,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn DDS_DomainParticipant_create_publisher(
        _this: DDS_DomainParticipant,
        qos: *const DDS_PublisherQos,
        a_listener: *const DDS_PublisherListener,
        mask: DDS_StatusMask,
    ) -> DDS_Publisher;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_publisher(
        _this: DDS_DomainParticipant,
        p: DDS_Publisher,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_create_subscriber(
        _this: DDS_DomainParticipant,
        qos: *const DDS_SubscriberQos,
        a_listener: *const DDS_SubscriberListener,
        mask: DDS_StatusMask,
    ) -> DDS_Subscriber;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_subscriber(
        _this: DDS_DomainParticipant,
        s: DDS_Subscriber,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_builtin_subscriber(
        _this: DDS_DomainParticipant,
    ) -> DDS_Subscriber;
}
extern "C" {
    pub fn DDS_DomainParticipant_create_topic(
        _this: DDS_DomainParticipant,
        topic_name: *const DDS_char,
        type_name: *const DDS_char,
        qos: *const DDS_TopicQos,
        a_listener: *const DDS_TopicListener,
        mask: DDS_StatusMask,
    ) -> DDS_Topic;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_topic(
        _this: DDS_DomainParticipant,
        a_topic: DDS_Topic,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_find_topic(
        _this: DDS_DomainParticipant,
        topic_name: *const DDS_char,
        timeout: *const DDS_Duration_t,
    ) -> DDS_Topic;
}
extern "C" {
    pub fn DDS_DomainParticipant_lookup_topicdescription(
        _this: DDS_DomainParticipant,
        name: *const DDS_char,
    ) -> DDS_TopicDescription;
}
extern "C" {
    pub fn DDS_DomainParticipant_create_contentfilteredtopic(
        _this: DDS_DomainParticipant,
        name: *const DDS_char,
        related_topic: DDS_Topic,
        filter_expression: *const DDS_char,
        filter_parameters: *const DDS_StringSeq,
    ) -> DDS_ContentFilteredTopic;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_contentfilteredtopic(
        _this: DDS_DomainParticipant,
        a_contentfilteredtopic: DDS_ContentFilteredTopic,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_create_multitopic(
        _this: DDS_DomainParticipant,
        name: *const DDS_char,
        type_name: *const DDS_char,
        subscription_expression: *const DDS_char,
        expression_parameters: *const DDS_StringSeq,
    ) -> DDS_MultiTopic;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_multitopic(
        _this: DDS_DomainParticipant,
        a_multitopic: DDS_MultiTopic,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_contained_entities(
        _this: DDS_DomainParticipant,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_set_qos(
        _this: DDS_DomainParticipant,
        qos: *const DDS_DomainParticipantQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_qos(
        _this: DDS_DomainParticipant,
        qos: *mut DDS_DomainParticipantQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_set_listener(
        _this: DDS_DomainParticipant,
        a_listener: *const DDS_DomainParticipantListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_listener(
        _this: DDS_DomainParticipant,
    ) -> DDS_DomainParticipantListener;
}
extern "C" {
    pub fn DDS_DomainParticipant_ignore_participant(
        _this: DDS_DomainParticipant,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_ignore_topic(
        _this: DDS_DomainParticipant,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_ignore_publication(
        _this: DDS_DomainParticipant,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_ignore_subscription(
        _this: DDS_DomainParticipant,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_domain_id(_this: DDS_DomainParticipant) -> DDS_DomainId_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_assert_liveliness(
        _this: DDS_DomainParticipant,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_set_default_publisher_qos(
        _this: DDS_DomainParticipant,
        qos: *const DDS_PublisherQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_default_publisher_qos(
        _this: DDS_DomainParticipant,
        qos: *mut DDS_PublisherQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_set_default_subscriber_qos(
        _this: DDS_DomainParticipant,
        qos: *const DDS_SubscriberQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_default_subscriber_qos(
        _this: DDS_DomainParticipant,
        qos: *mut DDS_SubscriberQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_set_default_topic_qos(
        _this: DDS_DomainParticipant,
        qos: *const DDS_TopicQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_default_topic_qos(
        _this: DDS_DomainParticipant,
        qos: *mut DDS_TopicQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_discovered_participants(
        _this: DDS_DomainParticipant,
        participant_handles: *mut DDS_InstanceHandleSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_discovered_participant_data(
        _this: DDS_DomainParticipant,
        participant_data: *mut DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_discovered_topics(
        _this: DDS_DomainParticipant,
        topic_handles: *mut DDS_InstanceHandleSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_discovered_topic_data(
        _this: DDS_DomainParticipant,
        topic_data: *mut DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_contains_entity(
        _this: DDS_DomainParticipant,
        a_handle: DDS_InstanceHandle_t,
    ) -> DDS_boolean;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_current_time(
        _this: DDS_DomainParticipant,
        current_time: *mut DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_lookup_typesupport(
        _this: DDS_DomainParticipant,
        type_name: *const DDS_char,
    ) -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_DomainParticipant_delete_historical_data(
        _this: DDS_DomainParticipant,
        partition_expression: DDS_string,
        topic_expression: DDS_string,
    ) -> DDS_ReturnCode_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_Property_s {
    pub name: DDS_string,
    pub value: DDS_string,
}
pub type DDS_DomainParticipantProperty = DDS_Property_s;
extern "C" {
    pub fn DDS_DomainParticipant_set_property(
        _this: DDS_DomainParticipant,
        property: *const DDS_DomainParticipantProperty,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipant_get_property(
        _this: DDS_DomainParticipant,
        property: *mut DDS_DomainParticipantProperty,
    ) -> DDS_ReturnCode_t;
}
pub type DDS_Domain = DDS_Object;
extern "C" {
    pub fn DDS_Domain_get_domain_id(_this: DDS_Domain) -> DDS_DomainId_t;
}
extern "C" {
    pub fn DDS_Domain_create_persistent_snapshot(
        _this: DDS_Domain,
        partition_expression: *const DDS_char,
        topic_expression: *const DDS_char,
        URI: *const DDS_char,
    ) -> DDS_ReturnCode_t;
}
pub type DDS_DomainParticipantFactory = DDS_Object;
extern "C" {
    pub fn DDS_DomainParticipantFactory_get_instance() -> DDS_DomainParticipantFactory;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_create_participant(
        _this: DDS_DomainParticipantFactory,
        domain_id: DDS_DomainId_t,
        qos: *const DDS_DomainParticipantQos,
        a_listener: *const DDS_DomainParticipantListener,
        mask: DDS_StatusMask,
    ) -> DDS_DomainParticipant;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_delete_participant(
        _this: DDS_DomainParticipantFactory,
        a_participant: DDS_DomainParticipant,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_lookup_participant(
        _this: DDS_DomainParticipantFactory,
        domain_id: DDS_DomainId_t,
    ) -> DDS_DomainParticipant;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_set_qos(
        _this: DDS_DomainParticipantFactory,
        qos: *const DDS_DomainParticipantFactoryQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_get_qos(
        _this: DDS_DomainParticipantFactory,
        qos: *mut DDS_DomainParticipantFactoryQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_set_default_participant_qos(
        _this: DDS_DomainParticipantFactory,
        qos: *const DDS_DomainParticipantQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_get_default_participant_qos(
        _this: DDS_DomainParticipantFactory,
        qos: *mut DDS_DomainParticipantQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_lookup_domain(
        _this: DDS_DomainParticipantFactory,
        domain_id: DDS_DomainId_t,
    ) -> DDS_Domain;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_delete_domain(
        _this: DDS_DomainParticipantFactory,
        a_domain: DDS_Domain,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_delete_contained_entities(
        _this: DDS_DomainParticipantFactory,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DomainParticipantFactory_detach_all_domains(
        _this: DDS_DomainParticipantFactory,
        block_operations: DDS_boolean,
        delete_entities: DDS_boolean,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicDescription_get_type_name(_this: DDS_TopicDescription) -> DDS_string;
}
extern "C" {
    pub fn DDS_TopicDescription_get_name(_this: DDS_TopicDescription) -> DDS_string;
}
extern "C" {
    pub fn DDS_TopicDescription_get_participant(
        _this: DDS_TopicDescription,
    ) -> DDS_DomainParticipant;
}
extern "C" {
    pub fn DDS_Topic_get_inconsistent_topic_status(
        _this: DDS_Topic,
        a_status: *mut DDS_InconsistentTopicStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Topic_set_listener(
        _this: DDS_Topic,
        a_listener: *const DDS_TopicListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Topic_get_listener(_this: DDS_Topic) -> DDS_TopicListener;
}
extern "C" {
    pub fn DDS_Topic_set_qos(_this: DDS_Topic, qos: *const DDS_TopicQos) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Topic_get_qos(_this: DDS_Topic, qos: *mut DDS_TopicQos) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Topic_dispose_all_data(_this: DDS_Topic) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Topic_get_metadescription(_this: DDS_Topic) -> DDS_string;
}
extern "C" {
    pub fn DDS_Topic_get_keylist(_this: DDS_Topic) -> DDS_string;
}
extern "C" {
    pub fn DDS_ContentFilteredTopic_get_filter_expression(
        _this: DDS_ContentFilteredTopic,
    ) -> DDS_string;
}
extern "C" {
    pub fn DDS_ContentFilteredTopic_get_expression_parameters(
        _this: DDS_ContentFilteredTopic,
        expression_parameters: *mut DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ContentFilteredTopic_set_expression_parameters(
        _this: DDS_ContentFilteredTopic,
        expression_parameters: *const DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ContentFilteredTopic_get_related_topic(_this: DDS_ContentFilteredTopic)
        -> DDS_Topic;
}
extern "C" {
    pub fn DDS_MultiTopic_get_subscription_expression(_this: DDS_MultiTopic) -> DDS_string;
}
extern "C" {
    pub fn DDS_MultiTopic_get_expression_parameters(
        _this: DDS_MultiTopic,
        expression_parameters: *mut DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_MultiTopic_set_expression_parameters(
        _this: DDS_MultiTopic,
        expression_parameters: *const DDS_StringSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_create_datawriter(
        _this: DDS_Publisher,
        a_topic: DDS_Topic,
        qos: *const DDS_DataWriterQos,
        a_listener: *const DDS_DataWriterListener,
        mask: DDS_StatusMask,
    ) -> DDS_DataWriter;
}
extern "C" {
    pub fn DDS_Publisher_delete_datawriter(
        _this: DDS_Publisher,
        a_datawriter: DDS_DataWriter,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_lookup_datawriter(
        _this: DDS_Publisher,
        topic_name: *const DDS_char,
    ) -> DDS_DataWriter;
}
extern "C" {
    pub fn DDS_Publisher_delete_contained_entities(_this: DDS_Publisher) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_set_qos(
        _this: DDS_Publisher,
        qos: *const DDS_PublisherQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_get_qos(
        _this: DDS_Publisher,
        qos: *mut DDS_PublisherQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_set_listener(
        _this: DDS_Publisher,
        a_listener: *const DDS_PublisherListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_get_listener(_this: DDS_Publisher) -> DDS_PublisherListener;
}
extern "C" {
    pub fn DDS_Publisher_suspend_publications(_this: DDS_Publisher) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_resume_publications(_this: DDS_Publisher) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_begin_coherent_changes(_this: DDS_Publisher) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_end_coherent_changes(_this: DDS_Publisher) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_wait_for_acknowledgments(
        _this: DDS_Publisher,
        max_wait: *const DDS_Duration_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_get_participant(_this: DDS_Publisher) -> DDS_DomainParticipant;
}
extern "C" {
    pub fn DDS_Publisher_set_default_datawriter_qos(
        _this: DDS_Publisher,
        qos: *const DDS_DataWriterQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_get_default_datawriter_qos(
        _this: DDS_Publisher,
        qos: *mut DDS_DataWriterQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Publisher_copy_from_topic_qos(
        _this: DDS_Publisher,
        a_datawriter_qos: *mut DDS_DataWriterQos,
        a_topic_qos: *const DDS_TopicQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_set_qos(
        _this: DDS_DataWriter,
        qos: *const DDS_DataWriterQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_qos(
        _this: DDS_DataWriter,
        qos: *mut DDS_DataWriterQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_set_listener(
        _this: DDS_DataWriter,
        a_listener: *const DDS_DataWriterListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_listener(_this: DDS_DataWriter) -> DDS_DataWriterListener;
}
extern "C" {
    pub fn DDS_DataWriter_get_topic(_this: DDS_DataWriter) -> DDS_Topic;
}
extern "C" {
    pub fn DDS_DataWriter_get_publisher(_this: DDS_DataWriter) -> DDS_Publisher;
}
extern "C" {
    pub fn DDS_DataWriter_wait_for_acknowledgments(
        _this: DDS_DataWriter,
        max_wait: *const DDS_Duration_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_liveliness_lost_status(
        _this: DDS_DataWriter,
        a_status: *mut DDS_LivelinessLostStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_offered_deadline_missed_status(
        _this: DDS_DataWriter,
        a_status: *mut DDS_OfferedDeadlineMissedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_offered_incompatible_qos_status(
        _this: DDS_DataWriter,
        a_status: *mut DDS_OfferedIncompatibleQosStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_publication_matched_status(
        _this: DDS_DataWriter,
        a_status: *mut DDS_PublicationMatchedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_assert_liveliness(_this: DDS_DataWriter) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_matched_subscriptions(
        _this: DDS_DataWriter,
        subscription_handles: *mut DDS_InstanceHandleSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_matched_subscription_data(
        _this: DDS_DataWriter,
        subscription_data: *mut DDS_SubscriptionBuiltinTopicData,
        subscription_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_create_datareader(
        _this: DDS_Subscriber,
        a_topic: DDS_TopicDescription,
        qos: *const DDS_DataReaderQos,
        a_listener: *const DDS_DataReaderListener,
        mask: DDS_StatusMask,
    ) -> DDS_DataReader;
}
extern "C" {
    pub fn DDS_Subscriber_delete_datareader(
        _this: DDS_Subscriber,
        a_datareader: DDS_DataReader,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_delete_contained_entities(_this: DDS_Subscriber) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_lookup_datareader(
        _this: DDS_Subscriber,
        topic_name: *const DDS_char,
    ) -> DDS_DataReader;
}
extern "C" {
    pub fn DDS_Subscriber_get_datareaders(
        _this: DDS_Subscriber,
        readers: *mut DDS_DataReaderSeq,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_notify_datareaders(_this: DDS_Subscriber) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_set_qos(
        _this: DDS_Subscriber,
        qos: *const DDS_SubscriberQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_get_qos(
        _this: DDS_Subscriber,
        qos: *mut DDS_SubscriberQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_set_listener(
        _this: DDS_Subscriber,
        a_listener: *const DDS_SubscriberListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_get_listener(_this: DDS_Subscriber) -> DDS_SubscriberListener;
}
extern "C" {
    pub fn DDS_Subscriber_begin_access(_this: DDS_Subscriber) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_end_access(_this: DDS_Subscriber) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_get_participant(_this: DDS_Subscriber) -> DDS_DomainParticipant;
}
extern "C" {
    pub fn DDS_Subscriber_set_default_datareader_qos(
        _this: DDS_Subscriber,
        qos: *const DDS_DataReaderQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_get_default_datareader_qos(
        _this: DDS_Subscriber,
        qos: *mut DDS_DataReaderQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_Subscriber_copy_from_topic_qos(
        _this: DDS_Subscriber,
        a_datareader_qos: *mut DDS_DataReaderQos,
        a_topic_qos: *const DDS_TopicQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_create_readcondition(
        _this: DDS_DataReader,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReadCondition;
}
extern "C" {
    pub fn DDS_DataReader_create_querycondition(
        _this: DDS_DataReader,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
        query_expression: *const DDS_char,
        query_parameters: *const DDS_StringSeq,
    ) -> DDS_QueryCondition;
}
extern "C" {
    pub fn DDS_DataReader_delete_readcondition(
        _this: DDS_DataReader,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_delete_contained_entities(_this: DDS_DataReader) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_create_view(
        _this: DDS_DataReader,
        qos: *const DDS_DataReaderViewQos,
    ) -> DDS_DataReaderView;
}
extern "C" {
    pub fn DDS_DataReader_delete_view(
        _this: DDS_DataReader,
        a_view: DDS_DataReaderView,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_set_qos(
        _this: DDS_DataReader,
        qos: *const DDS_DataReaderQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_qos(
        _this: DDS_DataReader,
        qos: *mut DDS_DataReaderQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_set_listener(
        _this: DDS_DataReader,
        a_listener: *const DDS_DataReaderListener,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_listener(_this: DDS_DataReader) -> DDS_DataReaderListener;
}
extern "C" {
    pub fn DDS_DataReader_get_topicdescription(_this: DDS_DataReader) -> DDS_TopicDescription;
}
extern "C" {
    pub fn DDS_DataReader_get_subscriber(_this: DDS_DataReader) -> DDS_Subscriber;
}
extern "C" {
    pub fn DDS_DataReader_get_sample_rejected_status(
        _this: DDS_DataReader,
        status: *mut DDS_SampleRejectedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_liveliness_changed_status(
        _this: DDS_DataReader,
        status: *mut DDS_LivelinessChangedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_requested_deadline_missed_status(
        _this: DDS_DataReader,
        status: *mut DDS_RequestedDeadlineMissedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_requested_incompatible_qos_status(
        _this: DDS_DataReader,
        status: *mut DDS_RequestedIncompatibleQosStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_sample_lost_status(
        _this: DDS_DataReader,
        status: *mut DDS_SampleLostStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_subscription_matched_status(
        _this: DDS_DataReader,
        status: *mut DDS_SubscriptionMatchedStatus,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_wait_for_historical_data(
        _this: DDS_DataReader,
        max_wait: *const DDS_Duration_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_wait_for_historical_data_w_condition(
        _this: DDS_DataReader,
        filter_expression: *const DDS_char,
        filter_parameters: *const DDS_StringSeq,
        min_source_timestamp: *const DDS_Time_t,
        max_source_timestamp: *const DDS_Time_t,
        resource_limits: *const DDS_ResourceLimitsQosPolicy,
        max_wait: *const DDS_Duration_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_matched_publications(
        _this: DDS_DataReader,
        publication_handles: *mut DDS_InstanceHandleSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_matched_publication_data(
        _this: DDS_DataReader,
        publication_data: *mut DDS_PublicationBuiltinTopicData,
        publication_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_set_default_datareaderview_qos(
        _this: DDS_DataReader,
        qos: *const DDS_DataReaderViewQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_default_datareaderview_qos(
        _this: DDS_DataReader,
        qos: *mut DDS_DataReaderViewQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_get_statuscondition(_this: DDS_DataReaderView)
        -> DDS_StatusCondition;
}
extern "C" {
    pub fn DDS_DataReaderView_get_status_changes(_this: DDS_DataReaderView) -> DDS_StatusMask;
}
extern "C" {
    pub fn DDS_DataReaderView_set_qos(
        _this: DDS_DataReaderView,
        qos: *const DDS_DataReaderViewQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_get_qos(
        _this: DDS_DataReaderView,
        qos: *mut DDS_DataReaderViewQos,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_get_datareader(_this: DDS_DataReaderView) -> DDS_DataReader;
}
extern "C" {
    pub fn DDS_DataReaderView_create_readcondition(
        _this: DDS_DataReaderView,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReadCondition;
}
extern "C" {
    pub fn DDS_DataReaderView_create_querycondition(
        _this: DDS_DataReaderView,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
        query_expression: *const DDS_char,
        query_parameters: *const DDS_StringSeq,
    ) -> DDS_QueryCondition;
}
extern "C" {
    pub fn DDS_DataReaderView_delete_readcondition(
        _this: DDS_DataReaderView,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_delete_contained_entities(
        _this: DDS_DataReaderView,
    ) -> DDS_ReturnCode_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_SampleInfo {
    pub sample_state: DDS_SampleStateKind,
    pub view_state: DDS_ViewStateKind,
    pub instance_state: DDS_InstanceStateKind,
    pub disposed_generation_count: DDS_long,
    pub no_writers_generation_count: DDS_long,
    pub sample_rank: DDS_long,
    pub generation_rank: DDS_long,
    pub absolute_generation_rank: DDS_long,
    pub source_timestamp: DDS_Time_t,
    pub instance_handle: DDS_InstanceHandle_t,
    pub publication_handle: DDS_InstanceHandle_t,
    pub valid_data: DDS_boolean,
    pub reception_timestamp: DDS_Time_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_SampleInfo {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_SampleInfo,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_SampleInfo__alloc() -> *mut DDS_sequence_DDS_SampleInfo;
}
extern "C" {
    pub fn DDS_sequence_DDS_SampleInfo_allocbuf(len: DDS_unsigned_long) -> *mut DDS_SampleInfo;
}
pub type DDS_SampleInfoSeq = DDS_sequence_DDS_SampleInfo;
extern "C" {
    pub fn DDS_SampleInfoSeq__alloc() -> *mut DDS_SampleInfoSeq;
}
extern "C" {
    pub fn DDS_SampleInfoSeq_allocbuf(len: DDS_unsigned_long) -> *mut DDS_SampleInfo;
}
extern "C" {
    pub fn DDS_DataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_DataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_DataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_write(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: DDS_Sample,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataWriter_lookup_instance(
        _this: DDS_DataWriter,
        instance_data: DDS_Sample,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_DataReader_read(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_read_w_condition(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take_w_condition(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_read_next_sample(
        _this: DDS_DataReader,
        data_values: DDS_Sample,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take_next_sample(
        _this: DDS_DataReader,
        data_values: DDS_Sample,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_read_instance(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take_instance(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_read_next_instance(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take_next_instance(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_return_loan(
        _this: DDS_DataReader,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: DDS_Sample,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReader_lookup_instance(
        _this: DDS_DataReader,
        instance_data: DDS_Sample,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        data_values: DDS_Sample,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        data_values: DDS_Sample,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read_instance(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take_instance(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_return_loan(
        _this: DDS_DataReaderView,
        data_values: DDS_sequence,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: DDS_Sample,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_DataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        instance_data: DDS_Sample,
    ) -> DDS_InstanceHandle_t;
}
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_MODULE: DDS_TypeElementKind = 0;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_STRUCT: DDS_TypeElementKind = 1;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_MEMBER: DDS_TypeElementKind = 2;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_UNION: DDS_TypeElementKind = 3;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_UNIONCASE: DDS_TypeElementKind = 4;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_UNIONSWITCH: DDS_TypeElementKind = 5;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_UNIONLABEL: DDS_TypeElementKind = 6;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_TYPEDEF: DDS_TypeElementKind = 7;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_ENUM: DDS_TypeElementKind = 8;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_ENUMLABEL: DDS_TypeElementKind = 9;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_TYPE: DDS_TypeElementKind = 10;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_ARRAY: DDS_TypeElementKind = 11;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_SEQUENCE: DDS_TypeElementKind = 12;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_STRING: DDS_TypeElementKind = 13;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_CHAR: DDS_TypeElementKind = 14;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_BOOLEAN: DDS_TypeElementKind = 15;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_OCTET: DDS_TypeElementKind = 16;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_SHORT: DDS_TypeElementKind = 17;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_USHORT: DDS_TypeElementKind = 18;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_LONG: DDS_TypeElementKind = 19;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_ULONG: DDS_TypeElementKind = 20;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_LONGLONG: DDS_TypeElementKind = 21;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_ULONGLONG: DDS_TypeElementKind = 22;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_FLOAT: DDS_TypeElementKind = 23;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_DOUBLE: DDS_TypeElementKind = 24;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_TIME: DDS_TypeElementKind = 25;
pub const DDS_TypeElementKind_DDS_TYPE_ELEMENT_KIND_UNIONLABELDEFAULT: DDS_TypeElementKind = 26;
pub type DDS_TypeElementKind = u32;
pub const DDS_TypeAttributeKind_DDS_TYPE_ATTRIBUTE_KIND_NUMBER: DDS_TypeAttributeKind = 0;
pub const DDS_TypeAttributeKind_DDS_TYPE_ATTRIBUTE_KIND_STRING: DDS_TypeAttributeKind = 1;
pub type DDS_TypeAttributeKind = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDS_TypeAttributeValue {
    pub _d: DDS_TypeAttributeKind,
    pub _u: DDS_TypeAttributeValue__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DDS_TypeAttributeValue__bindgen_ty_1 {
    pub nvalue: DDS_long,
    pub svalue: DDS_string,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDS_TypeAttribute {
    pub name: DDS_string,
    pub value: DDS_TypeAttributeValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_TypeAttribute {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_TypeAttribute,
    pub _release: DDS_boolean,
}
pub type DDS_TypeAttributeSeq = DDS_sequence_DDS_TypeAttribute;
pub type DDS_TypeParserHandle = *mut ::std::os::raw::c_void;
pub type DDS_TypeParserCallback = ::std::option::Option<
    unsafe extern "C" fn(
        kind: DDS_TypeElementKind,
        elementName: DDS_string,
        attributes: *const DDS_TypeAttributeSeq,
        handle: DDS_TypeParserHandle,
        argument: *mut ::std::os::raw::c_void,
    ) -> DDS_boolean,
>;
extern "C" {
    pub fn DDS_TypeSupport_parse_type_description(
        description: DDS_string,
        callback: DDS_TypeParserCallback,
        argument: *mut ::std::os::raw::c_void,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeSupport_walk_type_description(
        handle: DDS_TypeParserHandle,
        callback: DDS_TypeParserCallback,
        argument: *mut ::std::os::raw::c_void,
    ) -> DDS_ReturnCode_t;
}
pub type DDS_ErrorCode_t = DDS_long;
pub type DDS_ErrorInfo = DDS_Object;
extern "C" {
    pub fn DDS_ErrorInfo_update(_this: DDS_ErrorInfo) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo_get_code(
        _this: DDS_ErrorInfo,
        code: *mut DDS_ReturnCode_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo_get_location(
        _this: DDS_ErrorInfo,
        location: *mut DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo_get_source_line(
        _this: DDS_ErrorInfo,
        source_line: *mut DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo_get_stack_trace(
        _this: DDS_ErrorInfo,
        stack_trace: *mut DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo_get_message(
        _this: DDS_ErrorInfo,
        message: *mut DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ErrorInfo__alloc() -> DDS_ErrorInfo;
}
pub type DDS_QosProvider = DDS_Object;
extern "C" {
    pub fn DDS_QosProvider_get_participant_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_DomainParticipantQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider_get_topic_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_TopicQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider_get_subscriber_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_SubscriberQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider_get_datareader_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_DataReaderQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider_get_publisher_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_PublisherQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider_get_datawriter_qos(
        _this: DDS_QosProvider,
        qos: *mut DDS_DataWriterQos,
        id: *const ::std::os::raw::c_char,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_QosProvider__alloc(
        uri: *const ::std::os::raw::c_char,
        profile: *const ::std::os::raw::c_char,
    ) -> DDS_QosProvider;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_ParticipantBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_ParticipantBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_ParticipantBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_ParticipantBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_ParticipantBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_ParticipantBuiltinTopicData;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_ParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_ParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_ParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_ParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_ParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_ParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_ParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_ParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_ParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_ParticipantBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_ParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataTypeSupport_get_type_name(_this: DDS_TypeSupport)
        -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_TopicBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_TopicBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_TopicBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_TopicBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_TopicBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_TopicBuiltinTopicData;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TopicBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_TopicBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_TopicBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_TopicBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_TopicBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_TopicBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_TopicBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TopicBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_TopicBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TopicBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_TopicBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataTypeSupport_get_type_name(_this: DDS_TypeSupport) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_TypeBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_TypeBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_TypeBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_TypeBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_TypeBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_TypeBuiltinTopicData;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_TypeBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_TypeBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_TypeBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_TypeBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_TypeBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_TypeBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_TypeBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_TypeBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_TypeBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_TypeBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_TypeBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_PublicationBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_PublicationBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_PublicationBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_PublicationBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_PublicationBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_PublicationBuiltinTopicData;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_PublicationBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_PublicationBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_PublicationBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_PublicationBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_PublicationBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_PublicationBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_PublicationBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_PublicationBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_PublicationBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_PublicationBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_PublicationBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_SubscriptionBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_SubscriptionBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_SubscriptionBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_SubscriptionBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_SubscriptionBuiltinTopicData;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_SubscriptionBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_SubscriptionBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_SubscriptionBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_SubscriptionBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_SubscriptionBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_SubscriptionBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_SubscriptionBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_SubscriptionBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_SubscriptionBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_SubscriptionBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_SubscriptionBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_CMParticipantBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_CMParticipantBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_CMParticipantBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_CMParticipantBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_CMParticipantBuiltinTopicData;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMParticipantBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_CMParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_CMParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMParticipantBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMParticipantBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMParticipantBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMParticipantBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMParticipantBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_CMPublisherBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_CMPublisherBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_CMPublisherBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_CMPublisherBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_CMPublisherBuiltinTopicData;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMPublisherBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_CMPublisherBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMPublisherBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMPublisherBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_CMPublisherBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMPublisherBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMPublisherBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMPublisherBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMPublisherBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMPublisherBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMPublisherBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_CMSubscriberBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_CMSubscriberBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_CMSubscriberBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_CMSubscriberBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_CMSubscriberBuiltinTopicData;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMSubscriberBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_CMSubscriberBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMSubscriberBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMSubscriberBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_CMSubscriberBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMSubscriberBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMSubscriberBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMSubscriberBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMSubscriberBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMSubscriberBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMSubscriberBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_CMDataWriterBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_CMDataWriterBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_CMDataWriterBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_CMDataWriterBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_CMDataWriterBuiltinTopicData;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataWriterBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_CMDataWriterBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMDataWriterBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMDataWriterBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_CMDataWriterBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMDataWriterBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMDataWriterBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataWriterBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMDataWriterBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataWriterBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMDataWriterBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataTypeSupport__alloc() -> DDS_TypeSupport;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataTypeSupport_register_type(
        _this: DDS_TypeSupport,
        domain: DDS_DomainParticipant,
        name: DDS_string,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataTypeSupport_get_type_name(
        _this: DDS_TypeSupport,
    ) -> DDS_string;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DDS_sequence_DDS_CMDataReaderBuiltinTopicData {
    pub _maximum: DDS_unsigned_long,
    pub _length: DDS_unsigned_long,
    pub _buffer: *mut DDS_CMDataReaderBuiltinTopicData,
    pub _release: DDS_boolean,
}
extern "C" {
    pub fn DDS_sequence_DDS_CMDataReaderBuiltinTopicData__alloc(
) -> *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData;
}
extern "C" {
    pub fn DDS_sequence_DDS_CMDataReaderBuiltinTopicData_allocbuf(
        len: DDS_unsigned_long,
    ) -> *mut DDS_CMDataReaderBuiltinTopicData;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_register_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_register_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_unregister_instance(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_unregister_instance_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_write(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_write_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_dispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_dispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_writedispose(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_writedispose_w_timestamp(
        _this: DDS_DataWriter,
        instance_data: *const DDS_CMDataReaderBuiltinTopicData,
        instance_handle: DDS_InstanceHandle_t,
        source_timestamp: *const DDS_Time_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_get_key_value(
        _this: DDS_DataWriter,
        key_holder: *mut DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataWriter_lookup_instance(
        _this: DDS_DataWriter,
        key_holder: *const DDS_CMDataReaderBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMDataReaderBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take_next_sample(
        _this: DDS_DataReader,
        received_data: *mut DDS_CMDataReaderBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take_next_instance(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_read_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_take_next_instance_w_condition(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_return_loan(
        _this: DDS_DataReader,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_get_key_value(
        _this: DDS_DataReader,
        key_holder: *mut DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReader_lookup_instance(
        _this: DDS_DataReader,
        key_holder: *const DDS_CMDataReaderBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMDataReaderBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take_next_sample(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_CMDataReaderBuiltinTopicData,
        sample_info: *mut DDS_SampleInfo,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take_next_instance(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        sample_states: DDS_SampleStateMask,
        view_states: DDS_ViewStateMask,
        instance_states: DDS_InstanceStateMask,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_return_loan(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_read_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_take_next_instance_w_condition(
        _this: DDS_DataReaderView,
        received_data: *mut DDS_sequence_DDS_CMDataReaderBuiltinTopicData,
        info_seq: *mut DDS_SampleInfoSeq,
        max_samples: DDS_long,
        a_handle: DDS_InstanceHandle_t,
        a_condition: DDS_ReadCondition,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_get_key_value(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMDataReaderBuiltinTopicData,
        handle: DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}
extern "C" {
    pub fn DDS_CMDataReaderBuiltinTopicDataDataReaderView_lookup_instance(
        _this: DDS_DataReaderView,
        key_holder: *mut DDS_CMDataReaderBuiltinTopicData,
    ) -> DDS_InstanceHandle_t;
}
