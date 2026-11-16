/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod common {
    #![doc = "Diagnostics shared by compatibility checking and health evaluation.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "A record that was skipped or used despite a problem.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct Diagnostic {
        #[doc = "Whether the results may not reflect the records concerned.\n"]
        pub severity: Severity,
        #[doc = "Kind of the records or directory configurations concerned.\n"]
        pub kind: DiagnosticKind,
        #[doc = "Description of the problem; not specified and differs between implementations.\n"]
        pub message: ::std::string::String,
        #[doc = "Path of the file or directory concerned.\n"]
        pub path: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Diagnostic {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::common::Diagnostic" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("severity" , :: sidex :: shape :: shape_of :: < Severity > ,) . with_docs ("Whether the results may not reflect the records concerned.\n") , :: sidex :: shape :: FieldShape :: new ("kind" , :: sidex :: shape :: shape_of :: < DiagnosticKind > ,) . with_docs ("Kind of the records or directory configurations concerned.\n") , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Description of the problem; not specified and differs between implementations.\n") , :: sidex :: shape :: FieldShape :: new ("path" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Path of the file or directory concerned.\n") . optional ()]))) . with_docs ("A record that was skipped or used despite a problem.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Diagnostic {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Diagnostic {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.severity)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.message)),
                3usize => self
                    .path
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Diagnostic {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<Severity>::None;
            let mut __field_1 = ::std::option::Option::<DiagnosticKind>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <Severity as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <DiagnosticKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                severity: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `severity`"))?,
                kind: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                message: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `message`"))?,
                path: __field_3,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Diagnostic {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Diagnostic {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl Diagnostic {
        #[doc = "Creates a new [`Diagnostic`]."]
        pub fn new(
            severity: Severity,
            kind: DiagnosticKind,
            message: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            Self {
                severity: severity,
                kind: kind,
                message: message.into(),
                path: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `severity`."]
        pub fn set_severity(&mut self, severity: Severity) -> &mut Self {
            self.severity = severity;
            self
        }
        #[doc = "Sets the value of `severity`."]
        pub fn with_severity(mut self, severity: Severity) -> Self {
            self.severity = severity;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: DiagnosticKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: DiagnosticKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn set_message(
            &mut self,
            message: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.message = message.into();
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn with_message(
            mut self,
            message: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.message = message.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: ::std::option::Option<::std::string::String>) -> Self {
            self.path = path;
            self
        }
    }
    #[doc = "Effect of a diagnostic on the results.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum Severity {
        #[doc = "The results may not reflect the records concerned, for example, because they were skipped.\n"]
        Error,
        #[doc = "The results account for the problem.\n"]
        Warning,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Severity {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::common::Severity" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Error" , :: std :: option :: Option :: None) . with_docs ("The results may not reflect the records concerned, for example, because they were skipped.\n") , :: sidex :: shape :: CaseShape :: new ("Warning" , :: std :: option :: Option :: None) . with_docs ("The results account for the problem.\n")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Effect of a diagnostic on the results.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("Copy") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Severity {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Severity {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Error => 0usize,
                Self::Warning => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Error => ::std::option::Option::None,
                Self::Warning => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Severity {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Error,
                    1usize => Self::Warning,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Severity {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Severity {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Kind of the files that a diagnostic concerns, named after their types.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum DiagnosticKind {
        #[doc = "Component declarations.\n"]
        ComponentDeclaration,
        #[doc = "Health nodes.\n"]
        HealthNode,
        #[doc = "Status records.\n"]
        HealthStatus,
        #[doc = "Metric definitions.\n"]
        MetricDefinition,
        #[doc = "Metric samples.\n"]
        MetricSample,
        #[doc = "Directory configurations.\n"]
        DirectoryConfig,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DiagnosticKind {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::common::DiagnosticKind",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "ComponentDeclaration",
                        ::std::option::Option::None,
                    )
                    .with_docs("Component declarations.\n"),
                    ::sidex::shape::CaseShape::new("HealthNode", ::std::option::Option::None)
                        .with_docs("Health nodes.\n"),
                    ::sidex::shape::CaseShape::new("HealthStatus", ::std::option::Option::None)
                        .with_docs("Status records.\n"),
                    ::sidex::shape::CaseShape::new("MetricDefinition", ::std::option::Option::None)
                        .with_docs("Metric definitions.\n"),
                    ::sidex::shape::CaseShape::new("MetricSample", ::std::option::Option::None)
                        .with_docs("Metric samples.\n"),
                    ::sidex::shape::CaseShape::new("DirectoryConfig", ::std::option::Option::None)
                        .with_docs("Directory configurations.\n"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Kind of the files that a diagnostic concerns, named after their types.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("externally"),
                }],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DiagnosticKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for DiagnosticKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::ComponentDeclaration => 0usize,
                Self::HealthNode => 1usize,
                Self::HealthStatus => 2usize,
                Self::MetricDefinition => 3usize,
                Self::MetricSample => 4usize,
                Self::DirectoryConfig => 5usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::ComponentDeclaration => ::std::option::Option::None,
                Self::HealthNode => ::std::option::Option::None,
                Self::HealthStatus => ::std::option::Option::None,
                Self::MetricDefinition => ::std::option::Option::None,
                Self::MetricSample => ::std::option::Option::None,
                Self::DirectoryConfig => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DiagnosticKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::ComponentDeclaration,
                    1usize => Self::HealthNode,
                    2usize => Self::HealthStatus,
                    3usize => Self::MetricDefinition,
                    4usize => Self::MetricSample,
                    5usize => Self::DirectoryConfig,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for DiagnosticKind {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for DiagnosticKind {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod component {
    #![doc = "Component declarations and compatibility reports.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "One part of a system, such as the system image, an app, a runtime, or a hardware module.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ComponentDeclaration {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "Unique ID of the component within the composition.\n"]
        pub id: ::std::string::String,
        #[doc = "Owner of the declaration, which replaces or removes it as a whole.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Version of the component, also provided as its implicit capability.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Capabilities the component provides.\n"]
        pub provides: ::std::option::Option<::std::vec::Vec<CapabilityDeclaration>>,
        #[doc = "Resources the component claims.\n"]
        pub claims: ::std::option::Option<::std::vec::Vec<ClaimDeclaration>>,
        #[doc = "Selectors that each need to match a capability.\n"]
        pub requires: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        #[doc = "Selectors that must not match any capability, including the component's own.\n"]
        pub conflicts: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        #[doc = "Further data that the specification does not define, with namespaced member names.\n"]
        pub annotations: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ComponentDeclaration {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::component::ComponentDeclaration" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Unique ID of the component within the composition.\n") , :: sidex :: shape :: FieldShape :: new ("owner" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Owner of the declaration, which replaces or removes it as a whole.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the component, also provided as its implicit capability.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("provides" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < CapabilityDeclaration , > > ,) . with_docs ("Capabilities the component provides.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("claims" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < ClaimDeclaration , > > ,) . with_docs ("Resources the component claims.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("requires" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < CapabilitySelectorDeclaration , > > ,) . with_docs ("Selectors that each need to match a capability.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("conflicts" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < CapabilitySelectorDeclaration , > > ,) . with_docs ("Selectors that must not match any capability, including the component's own.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("annotations" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further data that the specification does not define, with namespaced member names.\n") . optional ()]))) . with_docs ("One part of a system, such as the system image, an app, a runtime, or a hardware module.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ComponentDeclaration {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ComponentDeclaration {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                2usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .provides
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .claims
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .requires
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .conflicts
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                8usize => self
                    .annotations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ComponentDeclaration {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<CapabilityDeclaration>>::None;
            let mut __field_5 = ::std::option::Option::<::std::vec::Vec<ClaimDeclaration>>::None;
            let mut __field_6 =
                ::std::option::Option::<::std::vec::Vec<CapabilitySelectorDeclaration>>::None;
            let mut __field_7 =
                ::std::option::Option::<::std::vec::Vec<CapabilitySelectorDeclaration>>::None;
            let mut __field_8 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilityDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <::std::vec::Vec<ClaimDeclaration> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilitySelectorDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilitySelectorDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                id: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                owner: __field_2,
                version: __field_3,
                provides: __field_4,
                claims: __field_5,
                requires: __field_6,
                conflicts: __field_7,
                annotations: __field_8,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ComponentDeclaration {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ComponentDeclaration {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ComponentDeclaration {
        #[doc = "Creates a new [`ComponentDeclaration`]."]
        pub fn new(id: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                id: id.into(),
                spec_version: ::std::default::Default::default(),
                owner: ::std::default::Default::default(),
                version: ::std::default::Default::default(),
                provides: ::std::default::Default::default(),
                claims: ::std::default::Default::default(),
                requires: ::std::default::Default::default(),
                conflicts: ::std::default::Default::default(),
                annotations: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn set_version(
            &mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn with_version(
            mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `provides`."]
        pub fn set_provides(
            &mut self,
            provides: ::std::option::Option<::std::vec::Vec<CapabilityDeclaration>>,
        ) -> &mut Self {
            self.provides = provides;
            self
        }
        #[doc = "Sets the value of `provides`."]
        pub fn with_provides(
            mut self,
            provides: ::std::option::Option<::std::vec::Vec<CapabilityDeclaration>>,
        ) -> Self {
            self.provides = provides;
            self
        }
        #[doc = "Sets the value of `claims`."]
        pub fn set_claims(
            &mut self,
            claims: ::std::option::Option<::std::vec::Vec<ClaimDeclaration>>,
        ) -> &mut Self {
            self.claims = claims;
            self
        }
        #[doc = "Sets the value of `claims`."]
        pub fn with_claims(
            mut self,
            claims: ::std::option::Option<::std::vec::Vec<ClaimDeclaration>>,
        ) -> Self {
            self.claims = claims;
            self
        }
        #[doc = "Sets the value of `requires`."]
        pub fn set_requires(
            &mut self,
            requires: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        ) -> &mut Self {
            self.requires = requires;
            self
        }
        #[doc = "Sets the value of `requires`."]
        pub fn with_requires(
            mut self,
            requires: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        ) -> Self {
            self.requires = requires;
            self
        }
        #[doc = "Sets the value of `conflicts`."]
        pub fn set_conflicts(
            &mut self,
            conflicts: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        ) -> &mut Self {
            self.conflicts = conflicts;
            self
        }
        #[doc = "Sets the value of `conflicts`."]
        pub fn with_conflicts(
            mut self,
            conflicts: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        ) -> Self {
            self.conflicts = conflicts;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn set_annotations(
            &mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.annotations = annotations;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn with_annotations(
            mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.annotations = annotations;
            self
        }
    }
    #[doc = "A resource that at most one component may claim.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClaimDeclaration {
        #[doc = "ID of the resource.\n"]
        pub id: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ClaimDeclaration {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::ClaimDeclaration",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the resource.\n"),
            ])),
        )
        .with_docs("A resource that at most one component may claim.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ClaimDeclaration {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ClaimDeclaration {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ClaimDeclaration {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ClaimDeclaration {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ClaimDeclaration {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ClaimDeclaration {
        #[doc = "Creates a new [`ClaimDeclaration`]."]
        pub fn new(id: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self { id: id.into() }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
    }
    #[doc = "A fact that a component provides.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CapabilityDeclaration {
        #[doc = "ID of the capability.\n"]
        pub id: ::std::string::String,
        #[doc = "Version of what is present; mutually exclusive with `value`.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Exact property; mutually exclusive with `version`.\n"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for CapabilityDeclaration {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::CapabilityDeclaration",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the capability.\n"),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Version of what is present; mutually exclusive with `value`.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Exact property; mutually exclusive with `version`.\n")
                .optional(),
            ])),
        )
        .with_docs("A fact that a component provides.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for CapabilityDeclaration {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for CapabilityDeclaration {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .value
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for CapabilityDeclaration {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                version: __field_1,
                value: __field_2,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for CapabilityDeclaration {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for CapabilityDeclaration {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl CapabilityDeclaration {
        #[doc = "Creates a new [`CapabilityDeclaration`]."]
        pub fn new(id: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                id: id.into(),
                version: ::std::default::Default::default(),
                value: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn set_version(
            &mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn with_version(
            mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(
            &mut self,
            value: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: ::std::option::Option<::std::string::String>) -> Self {
            self.value = value;
            self
        }
    }
    #[doc = "A selector over capabilities.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CapabilitySelectorDeclaration {
        #[doc = "ID of the capability.\n"]
        pub id: ::std::string::String,
        #[doc = "Version requirement; mutually exclusive with `value`.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Exact value; mutually exclusive with `version`.\n"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for CapabilitySelectorDeclaration {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::CapabilitySelectorDeclaration",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the capability.\n"),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Version requirement; mutually exclusive with `value`.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Exact value; mutually exclusive with `version`.\n")
                .optional(),
            ])),
        )
        .with_docs("A selector over capabilities.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for CapabilitySelectorDeclaration {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for CapabilitySelectorDeclaration {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .value
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for CapabilitySelectorDeclaration {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                version: __field_1,
                value: __field_2,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for CapabilitySelectorDeclaration {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for CapabilitySelectorDeclaration {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl CapabilitySelectorDeclaration {
        #[doc = "Creates a new [`CapabilitySelectorDeclaration`]."]
        pub fn new(id: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                id: id.into(),
                version: ::std::default::Default::default(),
                value: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn set_version(
            &mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `version`."]
        pub fn with_version(
            mut self,
            version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.version = version;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(
            &mut self,
            value: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: ::std::option::Option<::std::string::String>) -> Self {
            self.value = value;
            self
        }
    }
    #[doc = "Result of checking a composition.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompatibilityReport {
        #[doc = "Whether there are no problems and no diagnostics with the severity `Error`.\n"]
        pub consistent: bool,
        #[doc = "Violated constraints, in the order that the specification defines.\n"]
        pub problems: ::std::vec::Vec<CompatibilityProblem>,
        #[doc = "Diagnostics about component declarations and directory configurations.\n"]
        pub diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for CompatibilityReport {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::component::CompatibilityReport" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("consistent" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Whether there are no problems and no diagnostics with the severity `Error`.\n") , :: sidex :: shape :: FieldShape :: new ("problems" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < CompatibilityProblem , > > ,) . with_docs ("Violated constraints, in the order that the specification defines.\n") , :: sidex :: shape :: FieldShape :: new ("diagnostics" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: common :: Diagnostic , > > ,) . with_docs ("Diagnostics about component declarations and directory configurations.\n")]))) . with_docs ("Result of checking a composition.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for CompatibilityReport {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for CompatibilityReport {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.consistent)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.problems)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.diagnostics)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for CompatibilityReport {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<bool>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<CompatibilityProblem>>::None;
            let mut __field_2 =
                ::std::option::Option::<::std::vec::Vec<super::common::Diagnostic>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            CompatibilityProblem,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::common::Diagnostic,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                consistent: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `consistent`"))?,
                problems: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `problems`"))?,
                diagnostics: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `diagnostics`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for CompatibilityReport {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for CompatibilityReport {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl CompatibilityReport {
        #[doc = "Creates a new [`CompatibilityReport`]."]
        pub fn new(
            consistent: bool,
            problems: ::std::vec::Vec<CompatibilityProblem>,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> Self {
            Self {
                consistent: consistent,
                problems: problems,
                diagnostics: diagnostics,
            }
        }
        #[doc = "Sets the value of `consistent`."]
        pub fn set_consistent(&mut self, consistent: bool) -> &mut Self {
            self.consistent = consistent;
            self
        }
        #[doc = "Sets the value of `consistent`."]
        pub fn with_consistent(mut self, consistent: bool) -> Self {
            self.consistent = consistent;
            self
        }
        #[doc = "Sets the value of `problems`."]
        pub fn set_problems(
            &mut self,
            problems: ::std::vec::Vec<CompatibilityProblem>,
        ) -> &mut Self {
            self.problems = problems;
            self
        }
        #[doc = "Sets the value of `problems`."]
        pub fn with_problems(mut self, problems: ::std::vec::Vec<CompatibilityProblem>) -> Self {
            self.problems = problems;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn set_diagnostics(
            &mut self,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> &mut Self {
            self.diagnostics = diagnostics;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn with_diagnostics(
            mut self,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> Self {
            self.diagnostics = diagnostics;
            self
        }
    }
    #[doc = "A violated constraint.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub enum CompatibilityProblem {
        #[doc = "More than one component uses an ID.\n"]
        DuplicateComponent(ComponentIdProblem),
        #[doc = "More than one component claims a resource.\n"]
        DuplicateClaim(ClaimProblem),
        #[doc = "A requirement matches no capability.\n"]
        UnsatisfiedRequirement(SelectorProblem),
        #[doc = "A conflict matches a capability.\n"]
        Conflict(ConflictProblem),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for CompatibilityProblem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::CompatibilityProblem",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "DuplicateComponent",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ComponentIdProblem>),
                    )
                    .with_docs("More than one component uses an ID.\n"),
                    ::sidex::shape::CaseShape::new(
                        "DuplicateClaim",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ClaimProblem>),
                    )
                    .with_docs("More than one component claims a resource.\n"),
                    ::sidex::shape::CaseShape::new(
                        "UnsatisfiedRequirement",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<SelectorProblem>),
                    )
                    .with_docs("A requirement matches no capability.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Conflict",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ConflictProblem>),
                    )
                    .with_docs("A conflict matches a capability.\n"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "problem",
                    content: "content",
                }),
            ),
        )
        .with_docs("A violated constraint.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tag",
                value: ::sidex::shape::AttrValue::String("problem"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for CompatibilityProblem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for CompatibilityProblem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::DuplicateComponent(_) => 0usize,
                Self::DuplicateClaim(_) => 1usize,
                Self::UnsatisfiedRequirement(_) => 2usize,
                Self::Conflict(_) => 3usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::DuplicateComponent(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::DuplicateClaim(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::UnsatisfiedRequirement(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::Conflict(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for CompatibilityProblem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::DuplicateComponent(
                        <ComponentIdProblem as ::sidex::Decode>::decode(decoder)?,
                    ),
                    1usize => {
                        Self::DuplicateClaim(<ClaimProblem as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => Self::UnsatisfiedRequirement(
                        <SelectorProblem as ::sidex::Decode>::decode(decoder)?,
                    ),
                    3usize => {
                        Self::Conflict(<ConflictProblem as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for CompatibilityProblem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for CompatibilityProblem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A component ID that more than one component uses.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ComponentIdProblem {
        #[doc = "ID of the components.\n"]
        pub id: ::std::string::String,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ComponentIdProblem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::ComponentIdProblem",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the components.\n"),
            ])),
        )
        .with_docs("A component ID that more than one component uses.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ComponentIdProblem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ComponentIdProblem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ComponentIdProblem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ComponentIdProblem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ComponentIdProblem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ComponentIdProblem {
        #[doc = "Creates a new [`ComponentIdProblem`]."]
        pub fn new(id: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self { id: id.into() }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
    }
    #[doc = "A resource that more than one component claims.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClaimProblem {
        #[doc = "ID of the resource.\n"]
        pub id: ::std::string::String,
        #[doc = "IDs of the claiming components, in declaration order.\n"]
        pub components: ::std::vec::Vec<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ClaimProblem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::ClaimProblem",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the resource.\n"),
                ::sidex::shape::FieldShape::new(
                    "components",
                    ::sidex::shape::shape_of::<::std::vec::Vec<::std::string::String>>,
                )
                .with_docs("IDs of the claiming components, in declaration order.\n"),
            ])),
        )
        .with_docs("A resource that more than one component claims.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ClaimProblem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ClaimProblem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.components)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ClaimProblem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                components: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `components`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ClaimProblem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ClaimProblem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ClaimProblem {
        #[doc = "Creates a new [`ClaimProblem`]."]
        pub fn new(
            id: impl ::std::convert::Into<::std::string::String>,
            components: ::std::vec::Vec<::std::string::String>,
        ) -> Self {
            Self {
                id: id.into(),
                components: components,
            }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(
            &mut self,
            id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.id = id.into();
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::vec::Vec<::std::string::String>,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(
            mut self,
            components: ::std::vec::Vec<::std::string::String>,
        ) -> Self {
            self.components = components;
            self
        }
    }
    #[doc = "A requirement that matches no capability.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct SelectorProblem {
        #[doc = "ID of the requiring component.\n"]
        pub component: ::std::string::String,
        #[doc = "The requirement as declared.\n"]
        pub selector: CapabilitySelectorDeclaration,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SelectorProblem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::SelectorProblem",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "component",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the requiring component.\n"),
                ::sidex::shape::FieldShape::new(
                    "selector",
                    ::sidex::shape::shape_of::<CapabilitySelectorDeclaration>,
                )
                .with_docs("The requirement as declared.\n"),
            ])),
        )
        .with_docs("A requirement that matches no capability.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for SelectorProblem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SelectorProblem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.component)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.selector)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SelectorProblem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<CapabilitySelectorDeclaration>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <CapabilitySelectorDeclaration as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                component: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `component`"))?,
                selector: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `selector`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SelectorProblem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SelectorProblem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl SelectorProblem {
        #[doc = "Creates a new [`SelectorProblem`]."]
        pub fn new(
            component: impl ::std::convert::Into<::std::string::String>,
            selector: CapabilitySelectorDeclaration,
        ) -> Self {
            Self {
                component: component.into(),
                selector: selector,
            }
        }
        #[doc = "Sets the value of `component`."]
        pub fn set_component(
            &mut self,
            component: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.component = component.into();
            self
        }
        #[doc = "Sets the value of `component`."]
        pub fn with_component(
            mut self,
            component: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.component = component.into();
            self
        }
        #[doc = "Sets the value of `selector`."]
        pub fn set_selector(&mut self, selector: CapabilitySelectorDeclaration) -> &mut Self {
            self.selector = selector;
            self
        }
        #[doc = "Sets the value of `selector`."]
        pub fn with_selector(mut self, selector: CapabilitySelectorDeclaration) -> Self {
            self.selector = selector;
            self
        }
    }
    #[doc = "A conflict that matches a capability.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConflictProblem {
        #[doc = "ID of the component declaring the conflict.\n"]
        pub component: ::std::string::String,
        #[doc = "The conflict as declared.\n"]
        pub selector: CapabilitySelectorDeclaration,
        #[doc = "ID of the component providing the capability.\n"]
        pub provider: ::std::string::String,
        #[doc = "The matching capability as declared.\n"]
        pub capability: CapabilityDeclaration,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ConflictProblem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::ConflictProblem",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "component",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the component declaring the conflict.\n"),
                ::sidex::shape::FieldShape::new(
                    "selector",
                    ::sidex::shape::shape_of::<CapabilitySelectorDeclaration>,
                )
                .with_docs("The conflict as declared.\n"),
                ::sidex::shape::FieldShape::new(
                    "provider",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("ID of the component providing the capability.\n"),
                ::sidex::shape::FieldShape::new(
                    "capability",
                    ::sidex::shape::shape_of::<CapabilityDeclaration>,
                )
                .with_docs("The matching capability as declared.\n"),
            ])),
        )
        .with_docs("A conflict that matches a capability.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ConflictProblem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ConflictProblem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.component)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.selector)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.provider)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.capability)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ConflictProblem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<CapabilitySelectorDeclaration>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<CapabilityDeclaration>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <CapabilitySelectorDeclaration as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <CapabilityDeclaration as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                component: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `component`"))?,
                selector: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `selector`"))?,
                provider: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `provider`"))?,
                capability: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `capability`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ConflictProblem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ConflictProblem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ConflictProblem {
        #[doc = "Creates a new [`ConflictProblem`]."]
        pub fn new(
            component: impl ::std::convert::Into<::std::string::String>,
            selector: CapabilitySelectorDeclaration,
            provider: impl ::std::convert::Into<::std::string::String>,
            capability: CapabilityDeclaration,
        ) -> Self {
            Self {
                component: component.into(),
                selector: selector,
                provider: provider.into(),
                capability: capability,
            }
        }
        #[doc = "Sets the value of `component`."]
        pub fn set_component(
            &mut self,
            component: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.component = component.into();
            self
        }
        #[doc = "Sets the value of `component`."]
        pub fn with_component(
            mut self,
            component: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.component = component.into();
            self
        }
        #[doc = "Sets the value of `selector`."]
        pub fn set_selector(&mut self, selector: CapabilitySelectorDeclaration) -> &mut Self {
            self.selector = selector;
            self
        }
        #[doc = "Sets the value of `selector`."]
        pub fn with_selector(mut self, selector: CapabilitySelectorDeclaration) -> Self {
            self.selector = selector;
            self
        }
        #[doc = "Sets the value of `provider`."]
        pub fn set_provider(
            &mut self,
            provider: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.provider = provider.into();
            self
        }
        #[doc = "Sets the value of `provider`."]
        pub fn with_provider(
            mut self,
            provider: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.provider = provider.into();
            self
        }
        #[doc = "Sets the value of `capability`."]
        pub fn set_capability(&mut self, capability: CapabilityDeclaration) -> &mut Self {
            self.capability = capability;
            self
        }
        #[doc = "Sets the value of `capability`."]
        pub fn with_capability(mut self, capability: CapabilityDeclaration) -> Self {
            self.capability = capability;
            self
        }
    }
}
pub mod filesystem {
    #![doc = "Directory configurations and snapshots.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Directories that a contributor adds, as absolute paths without `..` segments.\n\nA segment consisting only of `*` matches every directory at that level whose name does not\nstart with `.`, in byte order, but no symlink.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct DirectoryConfig {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "Directories of component declarations.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Directories of health nodes.\n"]
        pub health_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Directories of status records.\n"]
        pub health_statuses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Directories of metric definitions.\n"]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Directories of metric samples.\n"]
        pub metric_samples: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DirectoryConfig {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::filesystem::DirectoryConfig" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Directories of component declarations.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("health_nodes" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Directories of health nodes.\n") . optional () . json_name ("healthNodes") , :: sidex :: shape :: FieldShape :: new ("health_statuses" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Directories of status records.\n") . optional () . json_name ("healthStatuses") , :: sidex :: shape :: FieldShape :: new ("metrics" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Directories of metric definitions.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("metric_samples" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Directories of metric samples.\n") . optional () . json_name ("metricSamples")]))) . with_docs ("Directories that a contributor adds, as absolute paths without `..` segments.\n\nA segment consisting only of `*` matches every directory at that level whose name does not\nstart with `.`, in byte order, but no symlink.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for DirectoryConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for DirectoryConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .health_nodes
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .health_statuses
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .metrics
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .metric_samples
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DirectoryConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_2 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_5 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                components: __field_1,
                health_nodes: __field_2,
                health_statuses: __field_3,
                metrics: __field_4,
                metric_samples: __field_5,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for DirectoryConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for DirectoryConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl DirectoryConfig {
        #[doc = "Creates a new [`DirectoryConfig`]."]
        pub fn new() -> Self {
            Self {
                spec_version: ::std::default::Default::default(),
                components: ::std::default::Default::default(),
                health_nodes: ::std::default::Default::default(),
                health_statuses: ::std::default::Default::default(),
                metrics: ::std::default::Default::default(),
                metric_samples: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(
            mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `health_nodes`."]
        pub fn set_health_nodes(
            &mut self,
            health_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.health_nodes = health_nodes;
            self
        }
        #[doc = "Sets the value of `health_nodes`."]
        pub fn with_health_nodes(
            mut self,
            health_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.health_nodes = health_nodes;
            self
        }
        #[doc = "Sets the value of `health_statuses`."]
        pub fn set_health_statuses(
            &mut self,
            health_statuses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.health_statuses = health_statuses;
            self
        }
        #[doc = "Sets the value of `health_statuses`."]
        pub fn with_health_statuses(
            mut self,
            health_statuses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.health_statuses = health_statuses;
            self
        }
        #[doc = "Sets the value of `metrics`."]
        pub fn set_metrics(
            &mut self,
            metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.metrics = metrics;
            self
        }
        #[doc = "Sets the value of `metrics`."]
        pub fn with_metrics(
            mut self,
            metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.metrics = metrics;
            self
        }
        #[doc = "Sets the value of `metric_samples`."]
        pub fn set_metric_samples(
            &mut self,
            metric_samples: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.metric_samples = metric_samples;
            self
        }
        #[doc = "Sets the value of `metric_samples`."]
        pub fn with_metric_samples(
            mut self,
            metric_samples: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.metric_samples = metric_samples;
            self
        }
    }
    impl ::std::default::Default for DirectoryConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[doc = "Records read from a system, for sending them to another system.\n\nAn omitted list means that the snapshot does not include records of its kind. The lists of the\nhealth part are either all present or all omitted.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct RuntimeSnapshot {
        #[doc = "Version of the specification that the snapshot's own fields follow.\n"]
        pub spec_version: ::std::string::String,
        #[doc = "Correlation of the boot-time clock with wall-clock time, sampled after collecting the\nrecords.\n"]
        pub clock: super::health::HealthClockSample,
        #[doc = "Component declarations.\n"]
        pub components:
            ::std::option::Option<::std::vec::Vec<super::component::ComponentDeclaration>>,
        #[doc = "Health nodes.\n"]
        pub health_nodes: ::std::option::Option<::std::vec::Vec<super::health::HealthNode>>,
        #[doc = "Status records.\n"]
        pub health_statuses: ::std::option::Option<::std::vec::Vec<super::health::HealthStatus>>,
        #[doc = "Metric definitions.\n"]
        pub metrics: ::std::option::Option<::std::vec::Vec<super::metric::MetricDefinition>>,
        #[doc = "Metric samples.\n"]
        pub metric_samples: ::std::option::Option<::std::vec::Vec<super::metric::MetricSample>>,
        #[doc = "Diagnostics about the included records and about directory configurations.\n"]
        pub diagnostics: ::std::option::Option<::std::vec::Vec<super::common::Diagnostic>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuntimeSnapshot {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::filesystem::RuntimeSnapshot" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the snapshot's own fields follow.\n") . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("clock" , :: sidex :: shape :: shape_of :: < super :: health :: HealthClockSample > ,) . with_docs ("Correlation of the boot-time clock with wall-clock time, sampled after collecting the\nrecords.\n") , :: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: component :: ComponentDeclaration , > > ,) . with_docs ("Component declarations.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("health_nodes" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: health :: HealthNode , > > ,) . with_docs ("Health nodes.\n") . optional () . json_name ("healthNodes") , :: sidex :: shape :: FieldShape :: new ("health_statuses" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: health :: HealthStatus , > > ,) . with_docs ("Status records.\n") . optional () . json_name ("healthStatuses") , :: sidex :: shape :: FieldShape :: new ("metrics" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: metric :: MetricDefinition , > > ,) . with_docs ("Metric definitions.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("metric_samples" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: metric :: MetricSample , > > ,) . with_docs ("Metric samples.\n") . optional () . json_name ("metricSamples") , :: sidex :: shape :: FieldShape :: new ("diagnostics" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: common :: Diagnostic , > > ,) . with_docs ("Diagnostics about the included records and about directory configurations.\n") . optional ()]))) . with_docs ("Records read from a system, for sending them to another system.\n\nAn omitted list means that the snapshot does not include records of its kind. The lists of the\nhealth part are either all present or all omitted.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuntimeSnapshot {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuntimeSnapshot {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.spec_version)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.clock)),
                2usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .health_nodes
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .health_statuses
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .metrics
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .metric_samples
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .diagnostics
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuntimeSnapshot {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<super::health::HealthClockSample>::None;
            let mut __field_2 = ::std::option::Option::<
                ::std::vec::Vec<super::component::ComponentDeclaration>,
            >::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<super::health::HealthNode>>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<super::health::HealthStatus>>::None;
            let mut __field_5 =
                ::std::option::Option::<::std::vec::Vec<super::metric::MetricDefinition>>::None;
            let mut __field_6 =
                ::std::option::Option::<::std::vec::Vec<super::metric::MetricSample>>::None;
            let mut __field_7 =
                ::std::option::Option::<::std::vec::Vec<super::common::Diagnostic>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::health::HealthClockSample as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::component::ComponentDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::health::HealthNode,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::health::HealthStatus,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::metric::MetricDefinition,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::metric::MetricSample,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::common::Diagnostic,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `specVersion`"))?,
                clock: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `clock`"))?,
                components: __field_2,
                health_nodes: __field_3,
                health_statuses: __field_4,
                metrics: __field_5,
                metric_samples: __field_6,
                diagnostics: __field_7,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuntimeSnapshot {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuntimeSnapshot {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl RuntimeSnapshot {
        #[doc = "Creates a new [`RuntimeSnapshot`]."]
        pub fn new(
            spec_version: impl ::std::convert::Into<::std::string::String>,
            clock: super::health::HealthClockSample,
        ) -> Self {
            Self {
                spec_version: spec_version.into(),
                clock: clock,
                components: ::std::default::Default::default(),
                health_nodes: ::std::default::Default::default(),
                health_statuses: ::std::default::Default::default(),
                metrics: ::std::default::Default::default(),
                metric_samples: ::std::default::Default::default(),
                diagnostics: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version.into();
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version.into();
            self
        }
        #[doc = "Sets the value of `clock`."]
        pub fn set_clock(&mut self, clock: super::health::HealthClockSample) -> &mut Self {
            self.clock = clock;
            self
        }
        #[doc = "Sets the value of `clock`."]
        pub fn with_clock(mut self, clock: super::health::HealthClockSample) -> Self {
            self.clock = clock;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::option::Option<
                ::std::vec::Vec<super::component::ComponentDeclaration>,
            >,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(
            mut self,
            components: ::std::option::Option<
                ::std::vec::Vec<super::component::ComponentDeclaration>,
            >,
        ) -> Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `health_nodes`."]
        pub fn set_health_nodes(
            &mut self,
            health_nodes: ::std::option::Option<::std::vec::Vec<super::health::HealthNode>>,
        ) -> &mut Self {
            self.health_nodes = health_nodes;
            self
        }
        #[doc = "Sets the value of `health_nodes`."]
        pub fn with_health_nodes(
            mut self,
            health_nodes: ::std::option::Option<::std::vec::Vec<super::health::HealthNode>>,
        ) -> Self {
            self.health_nodes = health_nodes;
            self
        }
        #[doc = "Sets the value of `health_statuses`."]
        pub fn set_health_statuses(
            &mut self,
            health_statuses: ::std::option::Option<::std::vec::Vec<super::health::HealthStatus>>,
        ) -> &mut Self {
            self.health_statuses = health_statuses;
            self
        }
        #[doc = "Sets the value of `health_statuses`."]
        pub fn with_health_statuses(
            mut self,
            health_statuses: ::std::option::Option<::std::vec::Vec<super::health::HealthStatus>>,
        ) -> Self {
            self.health_statuses = health_statuses;
            self
        }
        #[doc = "Sets the value of `metrics`."]
        pub fn set_metrics(
            &mut self,
            metrics: ::std::option::Option<::std::vec::Vec<super::metric::MetricDefinition>>,
        ) -> &mut Self {
            self.metrics = metrics;
            self
        }
        #[doc = "Sets the value of `metrics`."]
        pub fn with_metrics(
            mut self,
            metrics: ::std::option::Option<::std::vec::Vec<super::metric::MetricDefinition>>,
        ) -> Self {
            self.metrics = metrics;
            self
        }
        #[doc = "Sets the value of `metric_samples`."]
        pub fn set_metric_samples(
            &mut self,
            metric_samples: ::std::option::Option<::std::vec::Vec<super::metric::MetricSample>>,
        ) -> &mut Self {
            self.metric_samples = metric_samples;
            self
        }
        #[doc = "Sets the value of `metric_samples`."]
        pub fn with_metric_samples(
            mut self,
            metric_samples: ::std::option::Option<::std::vec::Vec<super::metric::MetricSample>>,
        ) -> Self {
            self.metric_samples = metric_samples;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn set_diagnostics(
            &mut self,
            diagnostics: ::std::option::Option<::std::vec::Vec<super::common::Diagnostic>>,
        ) -> &mut Self {
            self.diagnostics = diagnostics;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn with_diagnostics(
            mut self,
            diagnostics: ::std::option::Option<::std::vec::Vec<super::common::Diagnostic>>,
        ) -> Self {
            self.diagnostics = diagnostics;
            self
        }
    }
}
pub mod health {
    #![doc = "Health nodes, status records, evaluation, and reports.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "ID of a health node.\n"]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HealthNodeId(pub(crate) ::std::string::String);
    impl ::std::convert::From<HealthNodeId> for ::std::string::String {
        fn from(wrapped: HealthNodeId) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthNodeId {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthNodeId",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<::std::string::String>),
        )
        .with_docs("ID of a health node.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("PartialOrd"),
                    ::sidex::shape::Attr::Path("Ord"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthNodeId {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for HealthNodeId {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<::std::string::String as ::sidex::Decode>::decode(
                decoder,
            )?))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthNodeId {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthNodeId {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A check or function whose health is evaluated.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthNode {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "Unique ID of the node within the graph.\n"]
        pub id: HealthNodeId,
        #[doc = "Owner of the node, which replaces or removes it as a whole.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Human-readable name of the node.\n"]
        pub label: ::std::option::Option<::std::string::String>,
        #[doc = "IDs of associated components; associations create no dependencies.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "How the node's answers are determined; omitted means an observed node without constraints.\n"]
        pub rule: ::std::option::Option<HealthRule>,
        #[doc = "Aggregate nodes that the node attaches to as an input.\n"]
        pub part_of: ::std::option::Option<::std::vec::Vec<Attachment>>,
        #[doc = "Further data that the specification does not define, with namespaced member names.\n"]
        pub annotations: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthNode {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::HealthNode" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < HealthNodeId > ,) . with_docs ("Unique ID of the node within the graph.\n") , :: sidex :: shape :: FieldShape :: new ("owner" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Owner of the node, which replaces or removes it as a whole.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("label" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Human-readable name of the node.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("IDs of associated components; associations create no dependencies.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("rule" , :: sidex :: shape :: shape_of :: < HealthRule > ,) . with_docs ("How the node's answers are determined; omitted means an observed node without constraints.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("part_of" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < Attachment , > > ,) . with_docs ("Aggregate nodes that the node attaches to as an input.\n") . optional () . json_name ("partOf") , :: sidex :: shape :: FieldShape :: new ("annotations" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further data that the specification does not define, with namespaced member names.\n") . optional ()]))) . with_docs ("A check or function whose health is evaluated.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthNode {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthNode {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                2usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .label
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .rule
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .part_of
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .annotations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthNode {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_5 = ::std::option::Option::<HealthRule>::None;
            let mut __field_6 = ::std::option::Option::<::std::vec::Vec<Attachment>>::None;
            let mut __field_7 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <HealthRule as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::vec::Vec<Attachment> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                id: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                owner: __field_2,
                label: __field_3,
                components: __field_4,
                rule: __field_5,
                part_of: __field_6,
                annotations: __field_7,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthNode {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthNode {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthNode {
        #[doc = "Creates a new [`HealthNode`]."]
        pub fn new(id: HealthNodeId) -> Self {
            Self {
                id: id,
                spec_version: ::std::default::Default::default(),
                owner: ::std::default::Default::default(),
                label: ::std::default::Default::default(),
                components: ::std::default::Default::default(),
                rule: ::std::default::Default::default(),
                part_of: ::std::default::Default::default(),
                annotations: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: HealthNodeId) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: HealthNodeId) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `label`."]
        pub fn set_label(
            &mut self,
            label: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.label = label;
            self
        }
        #[doc = "Sets the value of `label`."]
        pub fn with_label(mut self, label: ::std::option::Option<::std::string::String>) -> Self {
            self.label = label;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(
            mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `rule`."]
        pub fn set_rule(&mut self, rule: ::std::option::Option<HealthRule>) -> &mut Self {
            self.rule = rule;
            self
        }
        #[doc = "Sets the value of `rule`."]
        pub fn with_rule(mut self, rule: ::std::option::Option<HealthRule>) -> Self {
            self.rule = rule;
            self
        }
        #[doc = "Sets the value of `part_of`."]
        pub fn set_part_of(
            &mut self,
            part_of: ::std::option::Option<::std::vec::Vec<Attachment>>,
        ) -> &mut Self {
            self.part_of = part_of;
            self
        }
        #[doc = "Sets the value of `part_of`."]
        pub fn with_part_of(
            mut self,
            part_of: ::std::option::Option<::std::vec::Vec<Attachment>>,
        ) -> Self {
            self.part_of = part_of;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn set_annotations(
            &mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.annotations = annotations;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn with_annotations(
            mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.annotations = annotations;
            self
        }
    }
    #[doc = "How a node's answers are determined.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub enum HealthRule {
        #[doc = "The node takes its answers from a status record.\n"]
        Observed(ObservationRule),
        #[doc = "The node derives its answers from a metric sample.\n"]
        Threshold(ThresholdRule),
        #[doc = "The node derives its answers from other nodes.\n"]
        Aggregate(AggregationRule),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthRule {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthRule",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Observed",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ObservationRule>),
                    )
                    .with_docs("The node takes its answers from a status record.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Threshold",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ThresholdRule>),
                    )
                    .with_docs("The node derives its answers from a metric sample.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Aggregate",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<AggregationRule>),
                    )
                    .with_docs("The node derives its answers from other nodes.\n"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "type",
                    content: "content",
                }),
            ),
        )
        .with_docs("How a node's answers are determined.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tag",
                value: ::sidex::shape::AttrValue::String("type"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for HealthRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Observed(_) => 0usize,
                Self::Threshold(_) => 1usize,
                Self::Aggregate(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Observed(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::Threshold(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::Aggregate(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => {
                        Self::Observed(<ObservationRule as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => Self::Threshold(<ThresholdRule as ::sidex::Decode>::decode(decoder)?),
                    2usize => {
                        Self::Aggregate(<AggregationRule as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Constraints on the status records of an observed node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ObservationRule {
        #[doc = "Upper limit for the age of status records, in milliseconds; must be positive.\n"]
        pub max_age_ms: ::std::option::Option<u64>,
        #[doc = "Tag that status records need to carry to be fresh.\n"]
        pub expected_tag: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ObservationRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::ObservationRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Upper limit for the age of status records, in milliseconds; must be positive.\n") . optional () . json_name ("maxAgeMs") , :: sidex :: shape :: FieldShape :: new ("expected_tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Tag that status records need to carry to be fresh.\n") . optional () . json_name ("expectedTag")]))) . with_docs ("Constraints on the status records of an observed node.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ObservationRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ObservationRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .max_age_ms
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .expected_tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ObservationRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<u64>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                max_age_ms: __field_0,
                expected_tag: __field_1,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ObservationRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ObservationRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ObservationRule {
        #[doc = "Creates a new [`ObservationRule`]."]
        pub fn new() -> Self {
            Self {
                max_age_ms: ::std::default::Default::default(),
                expected_tag: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn set_max_age_ms(&mut self, max_age_ms: ::std::option::Option<u64>) -> &mut Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn with_max_age_ms(mut self, max_age_ms: ::std::option::Option<u64>) -> Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `expected_tag`."]
        pub fn set_expected_tag(
            &mut self,
            expected_tag: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.expected_tag = expected_tag;
            self
        }
        #[doc = "Sets the value of `expected_tag`."]
        pub fn with_expected_tag(
            mut self,
            expected_tag: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.expected_tag = expected_tag;
            self
        }
    }
    impl ::std::default::Default for ObservationRule {
        fn default() -> Self {
            Self::new()
        }
    }
    #[doc = "Ranges that a metric sample is compared with.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ThresholdRule {
        #[doc = "ID of the metric.\n"]
        pub metric: super::metric::MetricId,
        #[doc = "Unit that the ranges assume, as a UCUM code.\n"]
        pub unit: ::std::option::Option<::std::string::String>,
        #[doc = "Range in which the desired criteria are met; a subset of the usable range.\n"]
        pub healthy: super::metric::MetricRange,
        #[doc = "Range in which the minimum criteria are met.\n"]
        pub usable: super::metric::MetricRange,
        #[doc = "Upper limit for the age of samples, in milliseconds; must be positive.\n"]
        pub max_age_ms: ::std::option::Option<u64>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ThresholdRule {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::ThresholdRule",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "metric",
                    ::sidex::shape::shape_of::<super::metric::MetricId>,
                )
                .with_docs("ID of the metric.\n"),
                ::sidex::shape::FieldShape::new(
                    "unit",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Unit that the ranges assume, as a UCUM code.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "healthy",
                    ::sidex::shape::shape_of::<super::metric::MetricRange>,
                )
                .with_docs(
                    "Range in which the desired criteria are met; a subset of the usable range.\n",
                ),
                ::sidex::shape::FieldShape::new(
                    "usable",
                    ::sidex::shape::shape_of::<super::metric::MetricRange>,
                )
                .with_docs("Range in which the minimum criteria are met.\n"),
                ::sidex::shape::FieldShape::new("max_age_ms", ::sidex::shape::shape_of::<u64>)
                    .with_docs(
                        "Upper limit for the age of samples, in milliseconds; must be positive.\n",
                    )
                    .optional()
                    .json_name("maxAgeMs"),
            ])),
        )
        .with_docs("Ranges that a metric sample is compared with.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ThresholdRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ThresholdRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.metric)),
                1usize => self
                    .unit
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.healthy)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.usable)),
                4usize => self
                    .max_age_ms
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ThresholdRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::metric::MetricId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<super::metric::MetricRange>::None;
            let mut __field_3 = ::std::option::Option::<super::metric::MetricRange>::None;
            let mut __field_4 = ::std::option::Option::<u64>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::metric::MetricId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::metric::MetricRange as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <super::metric::MetricRange as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                metric: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `metric`"))?,
                unit: __field_1,
                healthy: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthy`"))?,
                usable: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `usable`"))?,
                max_age_ms: __field_4,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ThresholdRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ThresholdRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl ThresholdRule {
        #[doc = "Creates a new [`ThresholdRule`]."]
        pub fn new(
            metric: super::metric::MetricId,
            healthy: super::metric::MetricRange,
            usable: super::metric::MetricRange,
        ) -> Self {
            Self {
                metric: metric,
                healthy: healthy,
                usable: usable,
                unit: ::std::default::Default::default(),
                max_age_ms: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `metric`."]
        pub fn set_metric(&mut self, metric: super::metric::MetricId) -> &mut Self {
            self.metric = metric;
            self
        }
        #[doc = "Sets the value of `metric`."]
        pub fn with_metric(mut self, metric: super::metric::MetricId) -> Self {
            self.metric = metric;
            self
        }
        #[doc = "Sets the value of `unit`."]
        pub fn set_unit(
            &mut self,
            unit: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.unit = unit;
            self
        }
        #[doc = "Sets the value of `unit`."]
        pub fn with_unit(mut self, unit: ::std::option::Option<::std::string::String>) -> Self {
            self.unit = unit;
            self
        }
        #[doc = "Sets the value of `healthy`."]
        pub fn set_healthy(&mut self, healthy: super::metric::MetricRange) -> &mut Self {
            self.healthy = healthy;
            self
        }
        #[doc = "Sets the value of `healthy`."]
        pub fn with_healthy(mut self, healthy: super::metric::MetricRange) -> Self {
            self.healthy = healthy;
            self
        }
        #[doc = "Sets the value of `usable`."]
        pub fn set_usable(&mut self, usable: super::metric::MetricRange) -> &mut Self {
            self.usable = usable;
            self
        }
        #[doc = "Sets the value of `usable`."]
        pub fn with_usable(mut self, usable: super::metric::MetricRange) -> Self {
            self.usable = usable;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn set_max_age_ms(&mut self, max_age_ms: ::std::option::Option<u64>) -> &mut Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn with_max_age_ms(mut self, max_age_ms: ::std::option::Option<u64>) -> Self {
            self.max_age_ms = max_age_ms;
            self
        }
    }
    #[doc = "Inputs of an aggregate node and the counts they need to meet.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct AggregationRule {
        #[doc = "The node's inputs.\n"]
        pub inputs: ::std::option::Option<::std::vec::Vec<AggregationInput>>,
        #[doc = "Number of counted inputs that need to be usable; omitted means `min_healthy`.\n"]
        pub min_usable: ::std::option::Option<u64>,
        #[doc = "Number of counted inputs that need to be healthy; omitted means all counted inputs.\n"]
        pub min_healthy: ::std::option::Option<u64>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AggregationRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::AggregationRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("inputs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < AggregationInput , > > ,) . with_docs ("The node's inputs.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("min_usable" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Number of counted inputs that need to be usable; omitted means `min_healthy`.\n") . optional () . json_name ("minUsable") , :: sidex :: shape :: FieldShape :: new ("min_healthy" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Number of counted inputs that need to be healthy; omitted means all counted inputs.\n") . optional () . json_name ("minHealthy")]))) . with_docs ("Inputs of an aggregate node and the counts they need to meet.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for AggregationRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for AggregationRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .min_usable
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .min_healthy
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AggregationRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<AggregationInput>>::None;
            let mut __field_1 = ::std::option::Option::<u64>::None;
            let mut __field_2 = ::std::option::Option::<u64>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<AggregationInput> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                inputs: __field_0,
                min_usable: __field_1,
                min_healthy: __field_2,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for AggregationRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for AggregationRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl AggregationRule {
        #[doc = "Creates a new [`AggregationRule`]."]
        pub fn new() -> Self {
            Self {
                inputs: ::std::default::Default::default(),
                min_usable: ::std::default::Default::default(),
                min_healthy: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<::std::vec::Vec<AggregationInput>>,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<::std::vec::Vec<AggregationInput>>,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `min_usable`."]
        pub fn set_min_usable(&mut self, min_usable: ::std::option::Option<u64>) -> &mut Self {
            self.min_usable = min_usable;
            self
        }
        #[doc = "Sets the value of `min_usable`."]
        pub fn with_min_usable(mut self, min_usable: ::std::option::Option<u64>) -> Self {
            self.min_usable = min_usable;
            self
        }
        #[doc = "Sets the value of `min_healthy`."]
        pub fn set_min_healthy(&mut self, min_healthy: ::std::option::Option<u64>) -> &mut Self {
            self.min_healthy = min_healthy;
            self
        }
        #[doc = "Sets the value of `min_healthy`."]
        pub fn with_min_healthy(mut self, min_healthy: ::std::option::Option<u64>) -> Self {
            self.min_healthy = min_healthy;
            self
        }
    }
    impl ::std::default::Default for AggregationRule {
        fn default() -> Self {
            Self::new()
        }
    }
    #[doc = "One input of an aggregate node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct AggregationInput {
        #[doc = "ID of the input node.\n"]
        pub node: HealthNodeId,
        #[doc = "How the input affects the node's answers; omitted means `Counted`.\n"]
        pub mode: ::std::option::Option<InputMode>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AggregationInput {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::AggregationInput",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("node", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("ID of the input node.\n"),
                ::sidex::shape::FieldShape::new("mode", ::sidex::shape::shape_of::<InputMode>)
                    .with_docs(
                        "How the input affects the node's answers; omitted means `Counted`.\n",
                    )
                    .optional(),
            ])),
        )
        .with_docs("One input of an aggregate node.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for AggregationInput {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for AggregationInput {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.node)),
                1usize => self
                    .mode
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for AggregationInput {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_1 = ::std::option::Option::<InputMode>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <InputMode as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                node: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `node`"))?,
                mode: __field_1,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for AggregationInput {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for AggregationInput {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl AggregationInput {
        #[doc = "Creates a new [`AggregationInput`]."]
        pub fn new(node: HealthNodeId) -> Self {
            Self {
                node: node,
                mode: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `node`."]
        pub fn set_node(&mut self, node: HealthNodeId) -> &mut Self {
            self.node = node;
            self
        }
        #[doc = "Sets the value of `node`."]
        pub fn with_node(mut self, node: HealthNodeId) -> Self {
            self.node = node;
            self
        }
        #[doc = "Sets the value of `mode`."]
        pub fn set_mode(&mut self, mode: ::std::option::Option<InputMode>) -> &mut Self {
            self.mode = mode;
            self
        }
        #[doc = "Sets the value of `mode`."]
        pub fn with_mode(mut self, mode: ::std::option::Option<InputMode>) -> Self {
            self.mode = mode;
            self
        }
    }
    #[doc = "Attachment of a node to an aggregate node as an input.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct Attachment {
        #[doc = "ID of the aggregate node.\n"]
        pub node: HealthNodeId,
        #[doc = "Mode of the input; omitted means `Counted`.\n"]
        pub mode: ::std::option::Option<InputMode>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Attachment {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::Attachment",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("node", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("ID of the aggregate node.\n"),
                ::sidex::shape::FieldShape::new("mode", ::sidex::shape::shape_of::<InputMode>)
                    .with_docs("Mode of the input; omitted means `Counted`.\n")
                    .optional(),
            ])),
        )
        .with_docs("Attachment of a node to an aggregate node as an input.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for Attachment {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Attachment {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.node)),
                1usize => self
                    .mode
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Attachment {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_1 = ::std::option::Option::<InputMode>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <InputMode as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                node: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `node`"))?,
                mode: __field_1,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Attachment {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Attachment {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl Attachment {
        #[doc = "Creates a new [`Attachment`]."]
        pub fn new(node: HealthNodeId) -> Self {
            Self {
                node: node,
                mode: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `node`."]
        pub fn set_node(&mut self, node: HealthNodeId) -> &mut Self {
            self.node = node;
            self
        }
        #[doc = "Sets the value of `node`."]
        pub fn with_node(mut self, node: HealthNodeId) -> Self {
            self.node = node;
            self
        }
        #[doc = "Sets the value of `mode`."]
        pub fn set_mode(&mut self, mode: ::std::option::Option<InputMode>) -> &mut Self {
            self.mode = mode;
            self
        }
        #[doc = "Sets the value of `mode`."]
        pub fn with_mode(mut self, mode: ::std::option::Option<InputMode>) -> Self {
            self.mode = mode;
            self
        }
    }
    #[doc = "How an input affects the answers of an aggregate node.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum InputMode {
        #[doc = "The input counts toward `min_usable` and `min_healthy`.\n"]
        Counted,
        #[doc = "The input counts, and the node is only usable and healthy if the input is.\n"]
        Mandatory,
        #[doc = "The input does not count; the node is only healthy if the input is healthy.\n"]
        Optional,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for InputMode {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::InputMode" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Counted" , :: std :: option :: Option :: None) . with_docs ("The input counts toward `min_usable` and `min_healthy`.\n") , :: sidex :: shape :: CaseShape :: new ("Mandatory" , :: std :: option :: Option :: None) . with_docs ("The input counts, and the node is only usable and healthy if the input is.\n") , :: sidex :: shape :: CaseShape :: new ("Optional" , :: std :: option :: Option :: None) . with_docs ("The input does not count; the node is only healthy if the input is healthy.\n")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("How an input affects the answers of an aggregate node.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("Copy") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for InputMode {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for InputMode {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Counted => 0usize,
                Self::Mandatory => 1usize,
                Self::Optional => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Counted => ::std::option::Option::None,
                Self::Mandatory => ::std::option::Option::None,
                Self::Optional => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for InputMode {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Counted,
                    1usize => Self::Mandatory,
                    2usize => Self::Optional,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for InputMode {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for InputMode {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Result of a check, published while the system runs.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthStatus {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "ID of the observed node.\n"]
        pub id: HealthNodeId,
        #[doc = "Result of the check.\n"]
        pub state: HealthState,
        #[doc = "ID of the boot during which the check took place.\n"]
        pub boot_id: ::std::string::String,
        #[doc = "Boot-time clock value, in milliseconds, when the check took place.\n"]
        pub observed_at_ms: u64,
        #[doc = "Duration, in milliseconds, for which the status record is valid; must be positive.\n"]
        pub max_age_ms: u64,
        #[doc = "Owner of the status record.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Opaque label identifying the configuration or instance that took the evidence.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Explanation supplied by the writer.\n"]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = "Further information about the observation, for display.\n"]
        pub details: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
        #[doc = "Further data that the specification does not define, with namespaced member names.\n"]
        pub annotations: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthStatus {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::HealthStatus" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < HealthNodeId > ,) . with_docs ("ID of the observed node.\n") , :: sidex :: shape :: FieldShape :: new ("state" , :: sidex :: shape :: shape_of :: < HealthState > ,) . with_docs ("Result of the check.\n") , :: sidex :: shape :: FieldShape :: new ("boot_id" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("ID of the boot during which the check took place.\n") . json_name ("bootId") , :: sidex :: shape :: FieldShape :: new ("observed_at_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Boot-time clock value, in milliseconds, when the check took place.\n") . json_name ("observedAtMs") , :: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Duration, in milliseconds, for which the status record is valid; must be positive.\n") . json_name ("maxAgeMs") , :: sidex :: shape :: FieldShape :: new ("owner" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Owner of the status record.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Opaque label identifying the configuration or instance that took the evidence.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Explanation supplied by the writer.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("details" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further information about the observation, for display.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("annotations" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further data that the specification does not define, with namespaced member names.\n") . optional ()]))) . with_docs ("Result of a check, published while the system runs.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthStatus {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthStatus {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.state)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.boot_id)),
                4usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.observed_at_ms))
                }
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.max_age_ms)),
                6usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                8usize => self
                    .message
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                9usize => self
                    .details
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                10usize => self
                    .annotations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthStatus {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_2 = ::std::option::Option::<HealthState>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 = ::std::option::Option::<u64>::None;
            let mut __field_5 = ::std::option::Option::<u64>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_8 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_9 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            let mut __field_10 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <HealthState as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    9usize => {
                        __field_9 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    10usize => {
                        __field_10 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                id: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                state: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `state`"))?,
                boot_id: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bootId`"))?,
                observed_at_ms: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `observedAtMs`"))?,
                max_age_ms: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `maxAgeMs`"))?,
                owner: __field_6,
                tag: __field_7,
                message: __field_8,
                details: __field_9,
                annotations: __field_10,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthStatus {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthStatus {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthStatus {
        #[doc = "Creates a new [`HealthStatus`]."]
        pub fn new(
            id: HealthNodeId,
            state: HealthState,
            boot_id: impl ::std::convert::Into<::std::string::String>,
            observed_at_ms: u64,
            max_age_ms: u64,
        ) -> Self {
            Self {
                id: id,
                state: state,
                boot_id: boot_id.into(),
                observed_at_ms: observed_at_ms,
                max_age_ms: max_age_ms,
                spec_version: ::std::default::Default::default(),
                owner: ::std::default::Default::default(),
                tag: ::std::default::Default::default(),
                message: ::std::default::Default::default(),
                details: ::std::default::Default::default(),
                annotations: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: HealthNodeId) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: HealthNodeId) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `state`."]
        pub fn set_state(&mut self, state: HealthState) -> &mut Self {
            self.state = state;
            self
        }
        #[doc = "Sets the value of `state`."]
        pub fn with_state(mut self, state: HealthState) -> Self {
            self.state = state;
            self
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn set_boot_id(
            &mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn with_boot_id(
            mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `observed_at_ms`."]
        pub fn set_observed_at_ms(&mut self, observed_at_ms: u64) -> &mut Self {
            self.observed_at_ms = observed_at_ms;
            self
        }
        #[doc = "Sets the value of `observed_at_ms`."]
        pub fn with_observed_at_ms(mut self, observed_at_ms: u64) -> Self {
            self.observed_at_ms = observed_at_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn set_max_age_ms(&mut self, max_age_ms: u64) -> &mut Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn with_max_age_ms(mut self, max_age_ms: u64) -> Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn set_tag(&mut self, tag: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn with_tag(mut self, tag: ::std::option::Option<::std::string::String>) -> Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn set_message(
            &mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn with_message(
            mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn set_details(
            &mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.details = details;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn with_details(
            mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.details = details;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn set_annotations(
            &mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.annotations = annotations;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn with_annotations(
            mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.annotations = annotations;
            self
        }
    }
    #[doc = "Summary of the answers of a node.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum HealthState {
        #[doc = "The desired criteria are met.\n"]
        Healthy,
        #[doc = "The minimum criteria are met; whether the desired criteria are met is `No` or `Unknown`.\n"]
        Degraded,
        #[doc = "The minimum criteria are not met.\n"]
        Unhealthy,
        #[doc = "Whether the minimum criteria are met is unknown.\n"]
        Unknown,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthState {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::HealthState" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Healthy" , :: std :: option :: Option :: None) . with_docs ("The desired criteria are met.\n") , :: sidex :: shape :: CaseShape :: new ("Degraded" , :: std :: option :: Option :: None) . with_docs ("The minimum criteria are met; whether the desired criteria are met is `No` or `Unknown`.\n") , :: sidex :: shape :: CaseShape :: new ("Unhealthy" , :: std :: option :: Option :: None) . with_docs ("The minimum criteria are not met.\n") , :: sidex :: shape :: CaseShape :: new ("Unknown" , :: std :: option :: Option :: None) . with_docs ("Whether the minimum criteria are met is unknown.\n")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Summary of the answers of a node.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("Copy") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthState {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for HealthState {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Healthy => 0usize,
                Self::Degraded => 1usize,
                Self::Unhealthy => 2usize,
                Self::Unknown => 3usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Healthy => ::std::option::Option::None,
                Self::Degraded => ::std::option::Option::None,
                Self::Unhealthy => ::std::option::Option::None,
                Self::Unknown => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthState {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Healthy,
                    1usize => Self::Degraded,
                    2usize => Self::Unhealthy,
                    3usize => Self::Unknown,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthState {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthState {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Answer to whether criteria are met.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum Answer {
        #[doc = "The criteria are met.\n"]
        Yes,
        #[doc = "The criteria are not met.\n"]
        No,
        #[doc = "The available evidence does not determine whether the criteria are met.\n"]
        Unknown,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Answer {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::Answer" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Yes" , :: std :: option :: Option :: None) . with_docs ("The criteria are met.\n") , :: sidex :: shape :: CaseShape :: new ("No" , :: std :: option :: Option :: None) . with_docs ("The criteria are not met.\n") , :: sidex :: shape :: CaseShape :: new ("Unknown" , :: std :: option :: Option :: None) . with_docs ("The available evidence does not determine whether the criteria are met.\n")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Answer to whether criteria are met.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("Copy") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Answer {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Answer {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Yes => 0usize,
                Self::No => 1usize,
                Self::Unknown => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Yes => ::std::option::Option::None,
                Self::No => ::std::option::Option::None,
                Self::Unknown => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Answer {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Yes,
                    1usize => Self::No,
                    2usize => Self::Unknown,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Answer {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Answer {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Why a node's evidence was not used or its rule was not applied.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum Reason {
        #[doc = "The evidence or the metric definition does not exist.\n"]
        Missing,
        #[doc = "The evidence belongs to another boot.\n"]
        OtherBoot,
        #[doc = "The owner of the evidence differs.\n"]
        OwnerMismatch,
        #[doc = "The tag of the evidence differs.\n"]
        TagMismatch,
        #[doc = "The evidence was observed after the current time.\n"]
        Future,
        #[doc = "The evidence is too old.\n"]
        Expired,
        #[doc = "Records contradict each other.\n"]
        Conflict,
        #[doc = "The node is on a cycle.\n"]
        Cycle,
        #[doc = "A rule type, state, or input mode is unknown to the evaluator.\n"]
        Unsupported,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Reason {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::Reason",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Missing", ::std::option::Option::None)
                        .with_docs("The evidence or the metric definition does not exist.\n"),
                    ::sidex::shape::CaseShape::new("OtherBoot", ::std::option::Option::None)
                        .with_docs("The evidence belongs to another boot.\n"),
                    ::sidex::shape::CaseShape::new("OwnerMismatch", ::std::option::Option::None)
                        .with_docs("The owner of the evidence differs.\n"),
                    ::sidex::shape::CaseShape::new("TagMismatch", ::std::option::Option::None)
                        .with_docs("The tag of the evidence differs.\n"),
                    ::sidex::shape::CaseShape::new("Future", ::std::option::Option::None)
                        .with_docs("The evidence was observed after the current time.\n"),
                    ::sidex::shape::CaseShape::new("Expired", ::std::option::Option::None)
                        .with_docs("The evidence is too old.\n"),
                    ::sidex::shape::CaseShape::new("Conflict", ::std::option::Option::None)
                        .with_docs("Records contradict each other.\n"),
                    ::sidex::shape::CaseShape::new("Cycle", ::std::option::Option::None)
                        .with_docs("The node is on a cycle.\n"),
                    ::sidex::shape::CaseShape::new("Unsupported", ::std::option::Option::None)
                        .with_docs(
                            "A rule type, state, or input mode is unknown to the evaluator.\n",
                        ),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Why a node's evidence was not used or its rule was not applied.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("externally"),
                }],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Reason {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Reason {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Missing => 0usize,
                Self::OtherBoot => 1usize,
                Self::OwnerMismatch => 2usize,
                Self::TagMismatch => 3usize,
                Self::Future => 4usize,
                Self::Expired => 5usize,
                Self::Conflict => 6usize,
                Self::Cycle => 7usize,
                Self::Unsupported => 8usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Missing => ::std::option::Option::None,
                Self::OtherBoot => ::std::option::Option::None,
                Self::OwnerMismatch => ::std::option::Option::None,
                Self::TagMismatch => ::std::option::Option::None,
                Self::Future => ::std::option::Option::None,
                Self::Expired => ::std::option::Option::None,
                Self::Conflict => ::std::option::Option::None,
                Self::Cycle => ::std::option::Option::None,
                Self::Unsupported => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Reason {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Missing,
                    1usize => Self::OtherBoot,
                    2usize => Self::OwnerMismatch,
                    3usize => Self::TagMismatch,
                    4usize => Self::Future,
                    5usize => Self::Expired,
                    6usize => Self::Conflict,
                    7usize => Self::Cycle,
                    8usize => Self::Unsupported,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Reason {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Reason {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Boot and time in which evidence is evaluated.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthContext {
        #[doc = "Current boot ID.\n"]
        pub boot_id: ::std::string::String,
        #[doc = "Current boot-time clock value in milliseconds.\n"]
        pub now_ms: u64,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthContext {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthContext",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "boot_id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Current boot ID.\n")
                .json_name("bootId"),
                ::sidex::shape::FieldShape::new("now_ms", ::sidex::shape::shape_of::<u64>)
                    .with_docs("Current boot-time clock value in milliseconds.\n")
                    .json_name("nowMs"),
            ])),
        )
        .with_docs("Boot and time in which evidence is evaluated.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthContext {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthContext {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.boot_id)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.now_ms)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthContext {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<u64>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                boot_id: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bootId`"))?,
                now_ms: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `nowMs`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthContext {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthContext {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthContext {
        #[doc = "Creates a new [`HealthContext`]."]
        pub fn new(boot_id: impl ::std::convert::Into<::std::string::String>, now_ms: u64) -> Self {
            Self {
                boot_id: boot_id.into(),
                now_ms: now_ms,
            }
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn set_boot_id(
            &mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn with_boot_id(
            mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `now_ms`."]
        pub fn set_now_ms(&mut self, now_ms: u64) -> &mut Self {
            self.now_ms = now_ms;
            self
        }
        #[doc = "Sets the value of `now_ms`."]
        pub fn with_now_ms(mut self, now_ms: u64) -> Self {
            self.now_ms = now_ms;
            self
        }
    }
    #[doc = "Correlation of the boot-time clock with wall-clock time.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthClockSample {
        #[doc = "Boot ID and boot-time clock value at sampling.\n"]
        pub context: HealthContext,
        #[doc = "Wall-clock time at the same instant, in RFC 3339 format in UTC.\n"]
        pub timestamp: ::sidex::types::Timestamp,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthClockSample {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthClockSample",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "context",
                    ::sidex::shape::shape_of::<HealthContext>,
                )
                .with_docs("Boot ID and boot-time clock value at sampling.\n"),
                ::sidex::shape::FieldShape::new(
                    "timestamp",
                    ::sidex::shape::shape_of::<::sidex::types::Timestamp>,
                )
                .with_docs("Wall-clock time at the same instant, in RFC 3339 format in UTC.\n"),
            ])),
        )
        .with_docs("Correlation of the boot-time clock with wall-clock time.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthClockSample {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthClockSample {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.context)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.timestamp)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthClockSample {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<HealthContext>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Timestamp>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthContext as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Timestamp as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                context: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `context`"))?,
                timestamp: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `timestamp`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthClockSample {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthClockSample {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthClockSample {
        #[doc = "Creates a new [`HealthClockSample`]."]
        pub fn new(context: HealthContext, timestamp: ::sidex::types::Timestamp) -> Self {
            Self {
                context: context,
                timestamp: timestamp,
            }
        }
        #[doc = "Sets the value of `context`."]
        pub fn set_context(&mut self, context: HealthContext) -> &mut Self {
            self.context = context;
            self
        }
        #[doc = "Sets the value of `context`."]
        pub fn with_context(mut self, context: HealthContext) -> Self {
            self.context = context;
            self
        }
        #[doc = "Sets the value of `timestamp`."]
        pub fn set_timestamp(&mut self, timestamp: ::sidex::types::Timestamp) -> &mut Self {
            self.timestamp = timestamp;
            self
        }
        #[doc = "Sets the value of `timestamp`."]
        pub fn with_timestamp(mut self, timestamp: ::sidex::types::Timestamp) -> Self {
            self.timestamp = timestamp;
            self
        }
    }
    #[doc = "Result of evaluating health.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthReport {
        #[doc = "One result per node ID, in the order of the node definitions.\n"]
        pub nodes: ::std::vec::Vec<HealthNodeResult>,
        #[doc = "Diagnostics about health records and directory configurations, and those added during\nevaluation.\n"]
        pub diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthReport {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::HealthReport" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("nodes" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < HealthNodeResult , > > ,) . with_docs ("One result per node ID, in the order of the node definitions.\n") , :: sidex :: shape :: FieldShape :: new ("diagnostics" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < super :: common :: Diagnostic , > > ,) . with_docs ("Diagnostics about health records and directory configurations, and those added during\nevaluation.\n")]))) . with_docs ("Result of evaluating health.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthReport {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthReport {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.nodes)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.diagnostics)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthReport {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::vec::Vec<HealthNodeResult>>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<super::common::Diagnostic>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthNodeResult> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::common::Diagnostic,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                nodes: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `nodes`"))?,
                diagnostics: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `diagnostics`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthReport {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthReport {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthReport {
        #[doc = "Creates a new [`HealthReport`]."]
        pub fn new(
            nodes: ::std::vec::Vec<HealthNodeResult>,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> Self {
            Self {
                nodes: nodes,
                diagnostics: diagnostics,
            }
        }
        #[doc = "Sets the value of `nodes`."]
        pub fn set_nodes(&mut self, nodes: ::std::vec::Vec<HealthNodeResult>) -> &mut Self {
            self.nodes = nodes;
            self
        }
        #[doc = "Sets the value of `nodes`."]
        pub fn with_nodes(mut self, nodes: ::std::vec::Vec<HealthNodeResult>) -> Self {
            self.nodes = nodes;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn set_diagnostics(
            &mut self,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> &mut Self {
            self.diagnostics = diagnostics;
            self
        }
        #[doc = "Sets the value of `diagnostics`."]
        pub fn with_diagnostics(
            mut self,
            diagnostics: ::std::vec::Vec<super::common::Diagnostic>,
        ) -> Self {
            self.diagnostics = diagnostics;
            self
        }
    }
    #[doc = "Answers of one node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthNodeResult {
        #[doc = "ID of the node.\n"]
        pub id: HealthNodeId,
        #[doc = "Owner of the node.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Whether the minimum criteria are met.\n"]
        pub usable: Answer,
        #[doc = "Whether the desired criteria are met.\n"]
        pub healthy: Answer,
        #[doc = "IDs of the inputs of an aggregate node; empty for other nodes.\n"]
        pub inputs: ::std::vec::Vec<HealthNodeId>,
        #[doc = "Why the node's evidence was not used or its rule was not applied.\n"]
        pub reason: ::std::option::Option<Reason>,
        #[doc = "Message of the fresh evidence.\n"]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = "Details of the fresh evidence.\n"]
        pub details: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthNodeResult {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthNodeResult",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("id", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("ID of the node.\n"),
                ::sidex::shape::FieldShape::new(
                    "owner",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Owner of the node.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("usable", ::sidex::shape::shape_of::<Answer>)
                    .with_docs("Whether the minimum criteria are met.\n"),
                ::sidex::shape::FieldShape::new("healthy", ::sidex::shape::shape_of::<Answer>)
                    .with_docs("Whether the desired criteria are met.\n"),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<::std::vec::Vec<HealthNodeId>>,
                )
                .with_docs("IDs of the inputs of an aggregate node; empty for other nodes.\n"),
                ::sidex::shape::FieldShape::new("reason", ::sidex::shape::shape_of::<Reason>)
                    .with_docs(
                        "Why the node's evidence was not used or its rule was not applied.\n",
                    )
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "message",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Message of the fresh evidence.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "details",
                    ::sidex::shape::shape_of::<
                        ::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        >,
                    >,
                )
                .with_docs("Details of the fresh evidence.\n")
                .optional(),
            ])),
        )
        .with_docs("Answers of one node.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthNodeResult {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthNodeResult {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.usable)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.healthy)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.inputs)),
                5usize => self
                    .reason
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .message
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .details
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthNodeResult {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<Answer>::None;
            let mut __field_3 = ::std::option::Option::<Answer>::None;
            let mut __field_4 = ::std::option::Option::<::std::vec::Vec<HealthNodeId>>::None;
            let mut __field_5 = ::std::option::Option::<Reason>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_7 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <Answer as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <Answer as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthNodeId> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <Reason as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                owner: __field_1,
                usable: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `usable`"))?,
                healthy: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthy`"))?,
                inputs: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `inputs`"))?,
                reason: __field_5,
                message: __field_6,
                details: __field_7,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthNodeResult {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthNodeResult {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl HealthNodeResult {
        #[doc = "Creates a new [`HealthNodeResult`]."]
        pub fn new(
            id: HealthNodeId,
            usable: Answer,
            healthy: Answer,
            inputs: ::std::vec::Vec<HealthNodeId>,
        ) -> Self {
            Self {
                id: id,
                usable: usable,
                healthy: healthy,
                inputs: inputs,
                owner: ::std::default::Default::default(),
                reason: ::std::default::Default::default(),
                message: ::std::default::Default::default(),
                details: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: HealthNodeId) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: HealthNodeId) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `usable`."]
        pub fn set_usable(&mut self, usable: Answer) -> &mut Self {
            self.usable = usable;
            self
        }
        #[doc = "Sets the value of `usable`."]
        pub fn with_usable(mut self, usable: Answer) -> Self {
            self.usable = usable;
            self
        }
        #[doc = "Sets the value of `healthy`."]
        pub fn set_healthy(&mut self, healthy: Answer) -> &mut Self {
            self.healthy = healthy;
            self
        }
        #[doc = "Sets the value of `healthy`."]
        pub fn with_healthy(mut self, healthy: Answer) -> Self {
            self.healthy = healthy;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(&mut self, inputs: ::std::vec::Vec<HealthNodeId>) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(mut self, inputs: ::std::vec::Vec<HealthNodeId>) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `reason`."]
        pub fn set_reason(&mut self, reason: ::std::option::Option<Reason>) -> &mut Self {
            self.reason = reason;
            self
        }
        #[doc = "Sets the value of `reason`."]
        pub fn with_reason(mut self, reason: ::std::option::Option<Reason>) -> Self {
            self.reason = reason;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn set_message(
            &mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn with_message(
            mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn set_details(
            &mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.details = details;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn with_details(
            mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.details = details;
            self
        }
    }
}
pub mod metric {
    #![doc = "Metrics, their samples, and ranges.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "ID of a metric.\n"]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MetricId(pub(crate) ::std::string::String);
    impl ::std::convert::From<MetricId> for ::std::string::String {
        fn from(wrapped: MetricId) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MetricId {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::metric::MetricId",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<::std::string::String>),
        )
        .with_docs("ID of a metric.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("Eq"),
                    ::sidex::shape::Attr::Path("PartialOrd"),
                    ::sidex::shape::Attr::Path("Ord"),
                    ::sidex::shape::Attr::Path("Hash"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for MetricId {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for MetricId {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<::std::string::String as ::sidex::Decode>::decode(
                decoder,
            )?))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MetricId {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MetricId {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A named numeric measurement.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct MetricDefinition {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "Unique ID of the metric.\n"]
        pub id: MetricId,
        #[doc = "Owner of the definition, which replaces or removes it as a whole.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Human-readable name of the metric.\n"]
        pub label: ::std::option::Option<::std::string::String>,
        #[doc = "IDs of associated components; associations create no dependencies.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Unit of all samples and thresholds of the metric, as a UCUM code.\n"]
        pub unit: ::std::string::String,
        #[doc = "Upper limit for the age of samples, in milliseconds; must be positive.\n"]
        pub max_age_ms: ::std::option::Option<u64>,
        #[doc = "Tag that samples need to carry to be fresh.\n"]
        pub expected_tag: ::std::option::Option<::std::string::String>,
        #[doc = "Further data that the specification does not define, with namespaced member names.\n"]
        pub annotations: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MetricDefinition {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::metric::MetricDefinition" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < MetricId > ,) . with_docs ("Unique ID of the metric.\n") , :: sidex :: shape :: FieldShape :: new ("owner" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Owner of the definition, which replaces or removes it as a whole.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("label" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Human-readable name of the metric.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("IDs of associated components; associations create no dependencies.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("unit" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Unit of all samples and thresholds of the metric, as a UCUM code.\n") , :: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Upper limit for the age of samples, in milliseconds; must be positive.\n") . optional () . json_name ("maxAgeMs") , :: sidex :: shape :: FieldShape :: new ("expected_tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Tag that samples need to carry to be fresh.\n") . optional () . json_name ("expectedTag") , :: sidex :: shape :: FieldShape :: new ("annotations" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further data that the specification does not define, with namespaced member names.\n") . optional ()]))) . with_docs ("A named numeric measurement.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for MetricDefinition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for MetricDefinition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                2usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .label
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.unit)),
                6usize => self
                    .max_age_ms
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .expected_tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                8usize => self
                    .annotations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for MetricDefinition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<MetricId>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_5 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_6 = ::std::option::Option::<u64>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_8 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <MetricId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                id: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                owner: __field_2,
                label: __field_3,
                components: __field_4,
                unit: __field_5.ok_or_else(|| ::sidex::DecodeError::new("missing field `unit`"))?,
                max_age_ms: __field_6,
                expected_tag: __field_7,
                annotations: __field_8,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MetricDefinition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MetricDefinition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl MetricDefinition {
        #[doc = "Creates a new [`MetricDefinition`]."]
        pub fn new(id: MetricId, unit: impl ::std::convert::Into<::std::string::String>) -> Self {
            Self {
                id: id,
                unit: unit.into(),
                spec_version: ::std::default::Default::default(),
                owner: ::std::default::Default::default(),
                label: ::std::default::Default::default(),
                components: ::std::default::Default::default(),
                max_age_ms: ::std::default::Default::default(),
                expected_tag: ::std::default::Default::default(),
                annotations: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: MetricId) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: MetricId) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `label`."]
        pub fn set_label(
            &mut self,
            label: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.label = label;
            self
        }
        #[doc = "Sets the value of `label`."]
        pub fn with_label(mut self, label: ::std::option::Option<::std::string::String>) -> Self {
            self.label = label;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn set_components(
            &mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `components`."]
        pub fn with_components(
            mut self,
            components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.components = components;
            self
        }
        #[doc = "Sets the value of `unit`."]
        pub fn set_unit(
            &mut self,
            unit: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.unit = unit.into();
            self
        }
        #[doc = "Sets the value of `unit`."]
        pub fn with_unit(mut self, unit: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.unit = unit.into();
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn set_max_age_ms(&mut self, max_age_ms: ::std::option::Option<u64>) -> &mut Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn with_max_age_ms(mut self, max_age_ms: ::std::option::Option<u64>) -> Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `expected_tag`."]
        pub fn set_expected_tag(
            &mut self,
            expected_tag: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.expected_tag = expected_tag;
            self
        }
        #[doc = "Sets the value of `expected_tag`."]
        pub fn with_expected_tag(
            mut self,
            expected_tag: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.expected_tag = expected_tag;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn set_annotations(
            &mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.annotations = annotations;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn with_annotations(
            mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.annotations = annotations;
            self
        }
    }
    #[doc = "Current measurement of a metric, published while the system runs.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct MetricSample {
        #[doc = "Version of the specification that the record follows; omitted means `1.0`.\n"]
        pub spec_version: ::std::option::Option<::std::string::String>,
        #[doc = "ID of the metric.\n"]
        pub id: MetricId,
        #[doc = "Measured value in the metric's unit; must be finite.\n"]
        pub value: f64,
        #[doc = "ID of the boot during which the measurement took place.\n"]
        pub boot_id: ::std::string::String,
        #[doc = "Boot-time clock value, in milliseconds, when the measurement took place.\n"]
        pub observed_at_ms: u64,
        #[doc = "Duration, in milliseconds, for which the sample is valid; must be positive.\n"]
        pub max_age_ms: u64,
        #[doc = "Owner of the sample.\n"]
        pub owner: ::std::option::Option<::std::string::String>,
        #[doc = "Opaque label identifying the configuration or instance that took the evidence.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Explanation supplied by the writer.\n"]
        pub message: ::std::option::Option<::std::string::String>,
        #[doc = "Further information about the observation, for display.\n"]
        pub details: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
        #[doc = "Further data that the specification does not define, with namespaced member names.\n"]
        pub annotations: ::std::option::Option<
            ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MetricSample {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::metric::MetricSample" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("spec_version" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Version of the specification that the record follows; omitted means `1.0`.\n") . optional () . json_name ("specVersion") , :: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < MetricId > ,) . with_docs ("ID of the metric.\n") , :: sidex :: shape :: FieldShape :: new ("value" , :: sidex :: shape :: shape_of :: < f64 > ,) . with_docs ("Measured value in the metric's unit; must be finite.\n") , :: sidex :: shape :: FieldShape :: new ("boot_id" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("ID of the boot during which the measurement took place.\n") . json_name ("bootId") , :: sidex :: shape :: FieldShape :: new ("observed_at_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Boot-time clock value, in milliseconds, when the measurement took place.\n") . json_name ("observedAtMs") , :: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u64 > ,) . with_docs ("Duration, in milliseconds, for which the sample is valid; must be positive.\n") . json_name ("maxAgeMs") , :: sidex :: shape :: FieldShape :: new ("owner" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Owner of the sample.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Opaque label identifying the configuration or instance that took the evidence.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("message" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Explanation supplied by the writer.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("details" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further information about the observation, for display.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("annotations" , :: sidex :: shape :: shape_of :: < :: std :: collections :: BTreeMap < :: std :: string :: String , :: sidex :: types :: JsonValue , > > ,) . with_docs ("Further data that the specification does not define, with namespaced member names.\n") . optional ()]))) . with_docs ("Current measurement of a metric, published while the system runs.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for MetricSample {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for MetricSample {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .spec_version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.value)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.boot_id)),
                4usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.observed_at_ms))
                }
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.max_age_ms)),
                6usize => self
                    .owner
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                8usize => self
                    .message
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                9usize => self
                    .details
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                10usize => self
                    .annotations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for MetricSample {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_1 = ::std::option::Option::<MetricId>::None;
            let mut __field_2 = ::std::option::Option::<f64>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 = ::std::option::Option::<u64>::None;
            let mut __field_5 = ::std::option::Option::<u64>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_8 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_9 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            let mut __field_10 = ::std::option::Option::<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <MetricId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<f64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    9usize => {
                        __field_9 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    10usize => {
                        __field_10 = ::std::option::Option::Some(<::std::collections::BTreeMap<
                            ::std::string::String,
                            ::sidex::types::JsonValue,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                spec_version: __field_0,
                id: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                value: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `value`"))?,
                boot_id: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bootId`"))?,
                observed_at_ms: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `observedAtMs`"))?,
                max_age_ms: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `maxAgeMs`"))?,
                owner: __field_6,
                tag: __field_7,
                message: __field_8,
                details: __field_9,
                annotations: __field_10,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MetricSample {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MetricSample {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl MetricSample {
        #[doc = "Creates a new [`MetricSample`]."]
        pub fn new(
            id: MetricId,
            value: f64,
            boot_id: impl ::std::convert::Into<::std::string::String>,
            observed_at_ms: u64,
            max_age_ms: u64,
        ) -> Self {
            Self {
                id: id,
                value: value,
                boot_id: boot_id.into(),
                observed_at_ms: observed_at_ms,
                max_age_ms: max_age_ms,
                spec_version: ::std::default::Default::default(),
                owner: ::std::default::Default::default(),
                tag: ::std::default::Default::default(),
                message: ::std::default::Default::default(),
                details: ::std::default::Default::default(),
                annotations: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn set_spec_version(
            &mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `spec_version`."]
        pub fn with_spec_version(
            mut self,
            spec_version: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.spec_version = spec_version;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn set_id(&mut self, id: MetricId) -> &mut Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `id`."]
        pub fn with_id(mut self, id: MetricId) -> Self {
            self.id = id;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(&mut self, value: f64) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: f64) -> Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn set_boot_id(
            &mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> &mut Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `boot_id`."]
        pub fn with_boot_id(
            mut self,
            boot_id: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.boot_id = boot_id.into();
            self
        }
        #[doc = "Sets the value of `observed_at_ms`."]
        pub fn set_observed_at_ms(&mut self, observed_at_ms: u64) -> &mut Self {
            self.observed_at_ms = observed_at_ms;
            self
        }
        #[doc = "Sets the value of `observed_at_ms`."]
        pub fn with_observed_at_ms(mut self, observed_at_ms: u64) -> Self {
            self.observed_at_ms = observed_at_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn set_max_age_ms(&mut self, max_age_ms: u64) -> &mut Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `max_age_ms`."]
        pub fn with_max_age_ms(mut self, max_age_ms: u64) -> Self {
            self.max_age_ms = max_age_ms;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn set_owner(
            &mut self,
            owner: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `owner`."]
        pub fn with_owner(mut self, owner: ::std::option::Option<::std::string::String>) -> Self {
            self.owner = owner;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn set_tag(&mut self, tag: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn with_tag(mut self, tag: ::std::option::Option<::std::string::String>) -> Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn set_message(
            &mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `message`."]
        pub fn with_message(
            mut self,
            message: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.message = message;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn set_details(
            &mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.details = details;
            self
        }
        #[doc = "Sets the value of `details`."]
        pub fn with_details(
            mut self,
            details: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.details = details;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn set_annotations(
            &mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> &mut Self {
            self.annotations = annotations;
            self
        }
        #[doc = "Sets the value of `annotations`."]
        pub fn with_annotations(
            mut self,
            annotations: ::std::option::Option<
                ::std::collections::BTreeMap<::std::string::String, ::sidex::types::JsonValue>,
            >,
        ) -> Self {
            self.annotations = annotations;
            self
        }
    }
    #[doc = "One bound of a range, in the metric's unit.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct MetricBound {
        #[doc = "Finite value of the bound.\n"]
        pub value: f64,
        #[doc = "Whether the bound itself belongs to the range.\n"]
        pub inclusive: bool,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MetricBound {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::metric::MetricBound",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("value", ::sidex::shape::shape_of::<f64>)
                    .with_docs("Finite value of the bound.\n"),
                ::sidex::shape::FieldShape::new("inclusive", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Whether the bound itself belongs to the range.\n"),
            ])),
        )
        .with_docs("One bound of a range, in the metric's unit.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for MetricBound {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for MetricBound {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.inclusive)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for MetricBound {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<f64>::None;
            let mut __field_1 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<f64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                value: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `value`"))?,
                inclusive: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `inclusive`"))?,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MetricBound {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MetricBound {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl MetricBound {
        #[doc = "Creates a new [`MetricBound`]."]
        pub fn new(value: f64, inclusive: bool) -> Self {
            Self {
                value: value,
                inclusive: inclusive,
            }
        }
        #[doc = "Sets the value of `value`."]
        pub fn set_value(&mut self, value: f64) -> &mut Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `value`."]
        pub fn with_value(mut self, value: f64) -> Self {
            self.value = value;
            self
        }
        #[doc = "Sets the value of `inclusive`."]
        pub fn set_inclusive(&mut self, inclusive: bool) -> &mut Self {
            self.inclusive = inclusive;
            self
        }
        #[doc = "Sets the value of `inclusive`."]
        pub fn with_inclusive(mut self, inclusive: bool) -> Self {
            self.inclusive = inclusive;
            self
        }
    }
    #[doc = "A non-empty range; a missing bound leaves the range unbounded on that side.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct MetricRange {
        #[doc = "Lower bound.\n"]
        pub lower: ::std::option::Option<MetricBound>,
        #[doc = "Upper bound.\n"]
        pub upper: ::std::option::Option<MetricBound>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MetricRange {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::metric::MetricRange",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("lower", ::sidex::shape::shape_of::<MetricBound>)
                    .with_docs("Lower bound.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new("upper", ::sidex::shape::shape_of::<MetricBound>)
                    .with_docs("Upper bound.\n")
                    .optional(),
            ])),
        )
        .with_docs("A non-empty range; a missing bound leaves the range unbounded on that side.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for MetricRange {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for MetricRange {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .lower
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .upper
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for MetricRange {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<MetricBound>::None;
            let mut __field_1 = ::std::option::Option::<MetricBound>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <MetricBound as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <MetricBound as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                lower: __field_0,
                upper: __field_1,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MetricRange {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MetricRange {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    impl MetricRange {
        #[doc = "Creates a new [`MetricRange`]."]
        pub fn new() -> Self {
            Self {
                lower: ::std::default::Default::default(),
                upper: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `lower`."]
        pub fn set_lower(&mut self, lower: ::std::option::Option<MetricBound>) -> &mut Self {
            self.lower = lower;
            self
        }
        #[doc = "Sets the value of `lower`."]
        pub fn with_lower(mut self, lower: ::std::option::Option<MetricBound>) -> Self {
            self.lower = lower;
            self
        }
        #[doc = "Sets the value of `upper`."]
        pub fn set_upper(&mut self, upper: ::std::option::Option<MetricBound>) -> &mut Self {
            self.upper = upper;
            self
        }
        #[doc = "Sets the value of `upper`."]
        pub fn with_upper(mut self, upper: ::std::option::Option<MetricBound>) -> Self {
            self.upper = upper;
            self
        }
    }
    impl ::std::default::Default for MetricRange {
        fn default() -> Self {
            Self::new()
        }
    }
}
