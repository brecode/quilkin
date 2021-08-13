#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatcher {
    #[prost(string, tag="2")]
    pub regex: ::prost::alloc::string::String,
    #[prost(oneof="regex_matcher::EngineType", tags="1")]
    pub engine_type: ::core::option::Option<regex_matcher::EngineType>,
}
/// Nested message and enum types in `RegexMatcher`.
pub mod regex_matcher {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoogleRe2 {
        #[deprecated]
        #[prost(message, optional, tag="1")]
        pub max_program_size: ::core::option::Option<u32>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EngineType {
        #[prost(message, tag="1")]
        GoogleRe2(GoogleRe2),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatchAndSubstitute {
    #[prost(message, optional, tag="1")]
    pub pattern: ::core::option::Option<RegexMatcher>,
    #[prost(string, tag="2")]
    pub substitution: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringMatcher {
    #[prost(bool, tag="6")]
    pub ignore_case: bool,
    #[prost(oneof="string_matcher::MatchPattern", tags="1, 2, 3, 5, 7")]
    pub match_pattern: ::core::option::Option<string_matcher::MatchPattern>,
}
/// Nested message and enum types in `StringMatcher`.
pub mod string_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        #[prost(string, tag="1")]
        Exact(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Prefix(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        Suffix(::prost::alloc::string::String),
        #[prost(message, tag="5")]
        SafeRegex(super::RegexMatcher),
        #[prost(string, tag="7")]
        Contains(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStringMatcher {
    #[prost(message, repeated, tag="1")]
    pub patterns: ::prost::alloc::vec::Vec<StringMatcher>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleMatcher {
    #[prost(oneof="double_matcher::MatchPattern", tags="1, 2")]
    pub match_pattern: ::core::option::Option<double_matcher::MatchPattern>,
}
/// Nested message and enum types in `DoubleMatcher`.
pub mod double_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        #[prost(message, tag="1")]
        Range(super::super::super::v3::DoubleRange),
        #[prost(double, tag="2")]
        Exact(f64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueMatcher {
    #[prost(oneof="value_matcher::MatchPattern", tags="1, 2, 3, 4, 5, 6")]
    pub match_pattern: ::core::option::Option<value_matcher::MatchPattern>,
}
/// Nested message and enum types in `ValueMatcher`.
pub mod value_matcher {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NullMatch {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        #[prost(message, tag="1")]
        NullMatch(NullMatch),
        #[prost(message, tag="2")]
        DoubleMatch(super::DoubleMatcher),
        #[prost(message, tag="3")]
        StringMatch(super::StringMatcher),
        #[prost(bool, tag="4")]
        BoolMatch(bool),
        #[prost(bool, tag="5")]
        PresentMatch(bool),
        #[prost(message, tag="6")]
        ListMatch(::prost::alloc::boxed::Box<super::ListMatcher>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMatcher {
    #[prost(oneof="list_matcher::MatchPattern", tags="1")]
    pub match_pattern: ::core::option::Option<list_matcher::MatchPattern>,
}
/// Nested message and enum types in `ListMatcher`.
pub mod list_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        #[prost(message, tag="1")]
        OneOf(::prost::alloc::boxed::Box<super::ValueMatcher>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataMatcher {
    #[prost(string, tag="1")]
    pub filter: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub path: ::prost::alloc::vec::Vec<metadata_matcher::PathSegment>,
    #[prost(message, optional, tag="3")]
    pub value: ::core::option::Option<ValueMatcher>,
}
/// Nested message and enum types in `MetadataMatcher`.
pub mod metadata_matcher {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathSegment {
        #[prost(oneof="path_segment::Segment", tags="1")]
        pub segment: ::core::option::Option<path_segment::Segment>,
    }
    /// Nested message and enum types in `PathSegment`.
    pub mod path_segment {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Segment {
            #[prost(string, tag="1")]
            Key(::prost::alloc::string::String),
        }
    }
}
