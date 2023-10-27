// @generated
/// Vector of Prometheus metrics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrometheusOperations {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<PrometheusOperation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrometheusOperation {
    /// Name of the Prometheus metric
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Labels represents a collection of label name -> value mappings. 
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(oneof="prometheus_operation::Operation", tags="3, 4, 5, 6")]
    pub operation: ::core::option::Option<prometheus_operation::Operation>,
}
/// Nested message and enum types in `PrometheusOperation`.
pub mod prometheus_operation {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="3")]
        Gauge(super::GaugeOp),
        #[prost(message, tag="4")]
        Counter(super::CounterOp),
        #[prost(message, tag="5")]
        Histogram(super::HistogramOp),
        #[prost(message, tag="6")]
        Summary(super::SummaryOp),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeOp {
    #[prost(enumeration="gauge_op::Operation", tag="1")]
    pub operation: i32,
    /// Value (Float) to be used in the operation
    #[prost(double, tag="2")]
    pub value: f64,
}
/// Nested message and enum types in `GaugeOp`.
pub mod gauge_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unspecified = 0,
        /// Inc increments the Gauge by 1. Use Add to increment it by arbitrary values.
        Inc = 1,
        /// Add adds the given value to the Gauge. (The value can be negative, resulting in a decrease of the Gauge.)
        ///
        /// float
        Add = 2,
        /// Set sets the Gauge to an arbitrary value. 
        ///
        /// float
        Set = 3,
        /// Dec decrements the Gauge by 1. Use Sub to decrement it by arbitrary values.
        Dec = 4,
        /// Sub subtracts the given value from the Gauge. (The value can be negative, resulting in an increase of the Gauge.)
        ///
        /// float
        Sub = 5,
        /// SetToCurrentTime sets the Gauge to the current Unix time in seconds.
        SetToCurrentTime = 6,
        /// Remove metrics for the given label values
        Remove = 7,
        /// Reset gauge values
        Reset = 8,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Inc => "OPERATION_INC",
                Operation::Add => "OPERATION_ADD",
                Operation::Set => "OPERATION_SET",
                Operation::Dec => "OPERATION_DEC",
                Operation::Sub => "OPERATION_SUB",
                Operation::SetToCurrentTime => "OPERATION_SET_TO_CURRENT_TIME",
                Operation::Remove => "OPERATION_REMOVE",
                Operation::Reset => "OPERATION_RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATION_INC" => Some(Self::Inc),
                "OPERATION_ADD" => Some(Self::Add),
                "OPERATION_SET" => Some(Self::Set),
                "OPERATION_DEC" => Some(Self::Dec),
                "OPERATION_SUB" => Some(Self::Sub),
                "OPERATION_SET_TO_CURRENT_TIME" => Some(Self::SetToCurrentTime),
                "OPERATION_REMOVE" => Some(Self::Remove),
                "OPERATION_RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterOp {
    #[prost(enumeration="counter_op::Operation", tag="1")]
    pub operation: i32,
    /// Value (Float) to be used in the operation
    #[prost(double, tag="2")]
    pub value: f64,
}
/// Nested message and enum types in `CounterOp`.
pub mod counter_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unspecified = 0,
        /// Increments the Counter by 1.
        Inc = 1,
        /// Adds an arbitrary value to a Counter. (Returns an error if the value is < 0.)
        ///
        /// float
        Add = 2,
        /// Remove metrics for the given label values
        Remove = 7,
        /// Reset counter values
        Reset = 8,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Inc => "OPERATION_INC",
                Operation::Add => "OPERATION_ADD",
                Operation::Remove => "OPERATION_REMOVE",
                Operation::Reset => "OPERATION_RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATION_INC" => Some(Self::Inc),
                "OPERATION_ADD" => Some(Self::Add),
                "OPERATION_REMOVE" => Some(Self::Remove),
                "OPERATION_RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummaryOp {
    #[prost(enumeration="summary_op::Operation", tag="1")]
    pub operation: i32,
    /// Value (Float) to be used in the operation
    #[prost(double, tag="2")]
    pub value: f64,
}
/// Nested message and enum types in `SummaryOp`.
pub mod summary_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unspecified = 0,
        /// Observe adds a single observation to the summary.
        /// Observations are usually positive or zero.
        /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations
        Observe = 1,
        /// Start a timer. Calling the returned function will observe the duration in seconds in the summary.
        StartTimer = 2,
        /// Remove metrics for the given label values
        Remove = 7,
        /// Reset counter values
        Reset = 8,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Observe => "OPERATION_OBSERVE",
                Operation::StartTimer => "OPERATION_START_TIMER",
                Operation::Remove => "OPERATION_REMOVE",
                Operation::Reset => "OPERATION_RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATION_OBSERVE" => Some(Self::Observe),
                "OPERATION_START_TIMER" => Some(Self::StartTimer),
                "OPERATION_REMOVE" => Some(Self::Remove),
                "OPERATION_RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramOp {
    #[prost(enumeration="histogram_op::Operation", tag="1")]
    pub operation: i32,
    /// Value (Float) to be used in the operation
    #[prost(double, tag="2")]
    pub value: f64,
}
/// Nested message and enum types in `HistogramOp`.
pub mod histogram_op {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unspecified = 0,
        /// Observe adds a single observation to the histogram.
        /// Observations are usually positive or zero.
        /// Negative observations are accepted but prevent current versions of Prometheus from properly detecting counter resets in the sum of observations. 
        Observe = 1,
        /// Start a timer. Calling the returned function will observe the duration in seconds in the summary.
        StartTimer = 2,
        /// Initialize the metrics for the given combination of labels to zero
        Zero = 3,
        /// Remove metrics for the given label values
        Remove = 7,
        /// Reset counter values
        Reset = 8,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Observe => "OPERATION_OBSERVE",
                Operation::StartTimer => "OPERATION_START_TIMER",
                Operation::Zero => "OPERATION_ZERO",
                Operation::Remove => "OPERATION_REMOVE",
                Operation::Reset => "OPERATION_RESET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATION_OBSERVE" => Some(Self::Observe),
                "OPERATION_START_TIMER" => Some(Self::StartTimer),
                "OPERATION_ZERO" => Some(Self::Zero),
                "OPERATION_REMOVE" => Some(Self::Remove),
                "OPERATION_RESET" => Some(Self::Reset),
                _ => None,
            }
        }
    }
}
// @@protoc_insertion_point(module)
