/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod component {
    #![doc = "Component declarations and compatibility reports.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "A participant in a system composition.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ComponentDeclaration {
        #[doc = "Unique component ID within the composition.\n"]
        pub id: ::std::string::String,
        #[doc = "Ownership group for replacement or removal, matched exactly.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Component version, also provided as an implicit capability.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Explicit capabilities.\n"]
        pub provides: ::std::option::Option<::std::vec::Vec<CapabilityDeclaration>>,
        #[doc = "Exclusive resource ownership.\n"]
        pub claims: ::std::option::Option<::std::vec::Vec<ClaimDeclaration>>,
        #[doc = "Selectors that must each match at least one capability.\n"]
        pub requires: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
        #[doc = "Selectors that must not match any capability, including this component's.\n"]
        pub conflicts: ::std::option::Option<::std::vec::Vec<CapabilitySelectorDeclaration>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ComponentDeclaration {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::ComponentDeclaration",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Unique component ID within the composition.\n"),
                ::sidex::shape::FieldShape::new(
                    "origin",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Ownership group for replacement or removal, matched exactly.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Component version, also provided as an implicit capability.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "provides",
                    ::sidex::shape::shape_of::<::std::vec::Vec<CapabilityDeclaration>>,
                )
                .with_docs("Explicit capabilities.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "claims",
                    ::sidex::shape::shape_of::<::std::vec::Vec<ClaimDeclaration>>,
                )
                .with_docs("Exclusive resource ownership.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "requires",
                    ::sidex::shape::shape_of::<::std::vec::Vec<CapabilitySelectorDeclaration>>,
                )
                .with_docs("Selectors that must each match at least one capability.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "conflicts",
                    ::sidex::shape::shape_of::<::std::vec::Vec<CapabilitySelectorDeclaration>>,
                )
                .with_docs(
                    "Selectors that must not match any capability, including this component's.\n",
                )
                .optional(),
            ])),
        )
        .with_docs("A participant in a system composition.\n");
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .version
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .provides
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .claims
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .requires
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .conflicts
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
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<CapabilityDeclaration>>::None;
            let mut __field_4 = ::std::option::Option::<::std::vec::Vec<ClaimDeclaration>>::None;
            let mut __field_5 =
                ::std::option::Option::<::std::vec::Vec<CapabilitySelectorDeclaration>>::None;
            let mut __field_6 =
                ::std::option::Option::<::std::vec::Vec<CapabilitySelectorDeclaration>>::None;
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
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilityDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::std::vec::Vec<ClaimDeclaration> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilitySelectorDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::std::vec::Vec<
                            CapabilitySelectorDeclaration,
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
                origin: __field_1,
                version: __field_2,
                provides: __field_3,
                claims: __field_4,
                requires: __field_5,
                conflicts: __field_6,
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
    #[doc = "An exclusive resource key.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClaimDeclaration {
        #[doc = "Resource ID.\n"]
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
                .with_docs("Resource ID.\n"),
            ])),
        )
        .with_docs("An exclusive resource key.\n");
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
    #[doc = "A fact supplied by a component.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CapabilityDeclaration {
        #[doc = "Capability ID.\n"]
        pub id: ::std::string::String,
        #[doc = "Version, mutually exclusive with value.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Exact string value, mutually exclusive with version.\n"]
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
                .with_docs("Capability ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Version, mutually exclusive with value.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Exact string value, mutually exclusive with version.\n")
                .optional(),
            ])),
        )
        .with_docs("A fact supplied by a component.\n");
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
    #[doc = "A selector over provided capabilities.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CapabilitySelectorDeclaration {
        #[doc = "Capability ID.\n"]
        pub id: ::std::string::String,
        #[doc = "Version requirement, mutually exclusive with value.\n"]
        pub version: ::std::option::Option<::std::string::String>,
        #[doc = "Exact value requirement, mutually exclusive with version.\n"]
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
                .with_docs("Capability ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "version",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Version requirement, mutually exclusive with value.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Exact value requirement, mutually exclusive with version.\n")
                .optional(),
            ])),
        )
        .with_docs("A selector over provided capabilities.\n");
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
    #[doc = "Result of evaluating the complete component set.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct CompatibilityReport {
        #[doc = "Whether all constraints are satisfied.\n"]
        pub consistent: bool,
        #[doc = "Violated constraints in deterministic evaluation order.\n"]
        pub problems: ::std::vec::Vec<CompatibilityProblem>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for CompatibilityReport {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::component::CompatibilityReport",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("consistent", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Whether all constraints are satisfied.\n"),
                ::sidex::shape::FieldShape::new(
                    "problems",
                    ::sidex::shape::shape_of::<::std::vec::Vec<CompatibilityProblem>>,
                )
                .with_docs("Violated constraints in deterministic evaluation order.\n"),
            ])),
        )
        .with_docs("Result of evaluating the complete component set.\n");
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
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                consistent: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `consistent`"))?,
                problems: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `problems`"))?,
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
    #[doc = "A violated composition constraint.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub enum CompatibilityProblem {
        #[doc = "Multiple components use one ID.\n"]
        DuplicateComponent(ComponentIdProblem),
        #[doc = "Multiple components own one resource.\n"]
        DuplicateClaim(ClaimProblem),
        #[doc = "No provided capability matches a requirement.\n"]
        UnsatisfiedRequirement(SelectorProblem),
        #[doc = "A provided capability matches a conflict.\n"]
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
                    .with_docs("Multiple components use one ID.\n")
                    .json_name("duplicateComponent"),
                    ::sidex::shape::CaseShape::new(
                        "DuplicateClaim",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ClaimProblem>),
                    )
                    .with_docs("Multiple components own one resource.\n")
                    .json_name("duplicateClaim"),
                    ::sidex::shape::CaseShape::new(
                        "UnsatisfiedRequirement",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<SelectorProblem>),
                    )
                    .with_docs("No provided capability matches a requirement.\n")
                    .json_name("unsatisfiedRequirement"),
                    ::sidex::shape::CaseShape::new(
                        "Conflict",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ConflictProblem>),
                    )
                    .with_docs("A provided capability matches a conflict.\n")
                    .json_name("conflict"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "problem",
                    content: "content",
                }),
            ),
        )
        .with_docs("A violated composition constraint.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("problem"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("camelCase"),
                },
            ],
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
    #[doc = "Duplicate component ID.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ComponentIdProblem {
        #[doc = "Component ID.\n"]
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
                .with_docs("Component ID.\n"),
            ])),
        )
        .with_docs("Duplicate component ID.\n");
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
    #[doc = "Duplicate exclusive ownership.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClaimProblem {
        #[doc = "Resource ID.\n"]
        pub id: ::std::string::String,
        #[doc = "IDs of owning components.\n"]
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
                .with_docs("Resource ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "components",
                    ::sidex::shape::shape_of::<::std::vec::Vec<::std::string::String>>,
                )
                .with_docs("IDs of owning components.\n"),
            ])),
        )
        .with_docs("Duplicate exclusive ownership.\n");
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
    #[doc = "Unsatisfied selector.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct SelectorProblem {
        #[doc = "Requiring component.\n"]
        pub component: ::std::string::String,
        #[doc = "Unsatisfied requirement.\n"]
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
                .with_docs("Requiring component.\n"),
                ::sidex::shape::FieldShape::new(
                    "selector",
                    ::sidex::shape::shape_of::<CapabilitySelectorDeclaration>,
                )
                .with_docs("Unsatisfied requirement.\n"),
            ])),
        )
        .with_docs("Unsatisfied selector.\n");
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
    #[doc = "A matching conflict.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ConflictProblem {
        #[doc = "Component declaring the conflict.\n"]
        pub component: ::std::string::String,
        #[doc = "Matching selector.\n"]
        pub selector: CapabilitySelectorDeclaration,
        #[doc = "Component providing the capability.\n"]
        pub provider: ::std::string::String,
        #[doc = "Matching capability.\n"]
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
                .with_docs("Component declaring the conflict.\n"),
                ::sidex::shape::FieldShape::new(
                    "selector",
                    ::sidex::shape::shape_of::<CapabilitySelectorDeclaration>,
                )
                .with_docs("Matching selector.\n"),
                ::sidex::shape::FieldShape::new(
                    "provider",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Component providing the capability.\n"),
                ::sidex::shape::FieldShape::new(
                    "capability",
                    ::sidex::shape::shape_of::<CapabilityDeclaration>,
                )
                .with_docs("Matching capability.\n"),
            ])),
        )
        .with_docs("A matching conflict.\n");
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
}
pub mod filesystem {
    #![doc = "Directory extensions and the collected runtime view.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Additional absolute directory paths supplied by packages or administrators.\nWhole `*` path segments match real directories in lexical order on each load.\nExplicit path segments may follow symlinks; `..` and partial wildcards are invalid.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct DirectoryConfig {
        #[doc = "Additional component declaration directories.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Additional health definition directories.\n"]
        pub health_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Additional health status directories.\n"]
        pub health_statuses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Additional value definition directories.\n"]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Additional value observation directories.\n"]
        pub value_observations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DirectoryConfig {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::filesystem::DirectoryConfig" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Additional component declaration directories.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("health_nodes" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Additional health definition directories.\n") . optional () . json_name ("healthNodes") , :: sidex :: shape :: FieldShape :: new ("health_statuses" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Additional health status directories.\n") . optional () . json_name ("healthStatuses") , :: sidex :: shape :: FieldShape :: new ("values" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Additional value definition directories.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("value_observations" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Additional value observation directories.\n") . optional () . json_name ("valueObservations")]))) . with_docs ("Additional absolute directory paths supplied by packages or administrators.\nWhole `*` path segments match real directories in lexical order on each load.\nExplicit path segments may follow symlinks; `..` and partial wildcards are invalid.\n") ;
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
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .health_nodes
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .health_statuses
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .values
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .value_observations
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
            let mut __field_0 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_2 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
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
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                components: __field_0,
                health_nodes: __field_1,
                health_statuses: __field_2,
                values: __field_3,
                value_observations: __field_4,
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
    #[doc = "Complete collection loaded from a filesystem or supplied directly.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct RuntimeSnapshot {
        #[doc = "Component declarations.\n"]
        pub components: ::std::vec::Vec<super::component::ComponentDeclaration>,
        #[doc = "Health definitions.\n"]
        pub health_nodes: ::std::vec::Vec<super::health::HealthNode>,
        #[doc = "Published health status records.\n"]
        pub health_statuses: ::std::vec::Vec<super::health::HealthStatus>,
        #[doc = "Numeric value definitions.\n"]
        pub values: ::std::option::Option<::std::vec::Vec<super::value::ValueDefinition>>,
        #[doc = "Published value observations.\n"]
        pub value_observations:
            ::std::option::Option<::std::vec::Vec<super::value::ValueObservation>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuntimeSnapshot {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::filesystem::RuntimeSnapshot",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "components",
                    ::sidex::shape::shape_of::<
                        ::std::vec::Vec<super::component::ComponentDeclaration>,
                    >,
                )
                .with_docs("Component declarations.\n"),
                ::sidex::shape::FieldShape::new(
                    "health_nodes",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::health::HealthNode>>,
                )
                .with_docs("Health definitions.\n")
                .json_name("healthNodes"),
                ::sidex::shape::FieldShape::new(
                    "health_statuses",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::health::HealthStatus>>,
                )
                .with_docs("Published health status records.\n")
                .json_name("healthStatuses"),
                ::sidex::shape::FieldShape::new(
                    "values",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::value::ValueDefinition>>,
                )
                .with_docs("Numeric value definitions.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value_observations",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::value::ValueObservation>>,
                )
                .with_docs("Published value observations.\n")
                .optional()
                .json_name("valueObservations"),
            ])),
        )
        .with_docs("Complete collection loaded from a filesystem or supplied directly.\n");
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.components)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.health_nodes)),
                2usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.health_statuses))
                }
                3usize => self
                    .values
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .value_observations
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
            let mut __field_0 = ::std::option::Option::<
                ::std::vec::Vec<super::component::ComponentDeclaration>,
            >::None;
            let mut __field_1 =
                ::std::option::Option::<::std::vec::Vec<super::health::HealthNode>>::None;
            let mut __field_2 =
                ::std::option::Option::<::std::vec::Vec<super::health::HealthStatus>>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<super::value::ValueDefinition>>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<super::value::ValueObservation>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::component::ComponentDeclaration,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::health::HealthNode,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::health::HealthStatus,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::value::ValueDefinition,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::value::ValueObservation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                components: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `components`"))?,
                health_nodes: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthNodes`"))?,
                health_statuses: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthStatuses`"))?,
                values: __field_3,
                value_observations: __field_4,
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
}
pub mod health {
    #![doc = "Named health observations, numeric thresholds, and aggregation rules.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Identifier of a health graph node, serialized as a string.\n"]
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
        .with_docs("Identifier of a health graph node, serialized as a string.\n")
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
    #[doc = "A health check or function in a directed acyclic graph.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthNode {
        #[doc = "Unique health node ID.\n"]
        pub id: HealthNodeId,
        #[doc = "Ownership group for replacement or removal, matched exactly.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Human-readable label.\n"]
        pub label: ::std::option::Option<::std::string::String>,
        #[doc = "Related component IDs; these associations do not imply health dependencies.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "How this node's health is determined.\n"]
        pub rule: HealthRule,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthNode {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthNode",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("id", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("Unique health node ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "origin",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Ownership group for replacement or removal, matched exactly.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "label",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Human-readable label.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "components",
                    ::sidex::shape::shape_of::<::std::vec::Vec<::std::string::String>>,
                )
                .with_docs(
                    "Related component IDs; these associations do not imply health dependencies.\n",
                )
                .optional(),
                ::sidex::shape::FieldShape::new("rule", ::sidex::shape::shape_of::<HealthRule>)
                    .with_docs("How this node's health is determined.\n"),
            ])),
        )
        .with_docs("A health check or function in a directed acyclic graph.\n");
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .label
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.rule)),
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
            let mut __field_0 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_4 = ::std::option::Option::<HealthRule>::None;
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
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <HealthRule as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                origin: __field_1,
                label: __field_2,
                components: __field_3,
                rule: __field_4.ok_or_else(|| ::sidex::DecodeError::new("missing field `rule`"))?,
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
    #[doc = "Definition of an observed, threshold, or aggregate node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub enum HealthRule {
        #[doc = "Health is supplied by an external status record.\n"]
        Observed(ObservationRule),
        #[doc = "Health is derived from a fresh numeric observation.\n"]
        Threshold(ThresholdRule),
        #[doc = "Health is evaluated from other nodes.\n"]
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
                    .with_docs("Health is supplied by an external status record.\n")
                    .json_name("observed"),
                    ::sidex::shape::CaseShape::new(
                        "Threshold",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ThresholdRule>),
                    )
                    .with_docs("Health is derived from a fresh numeric observation.\n")
                    .json_name("threshold"),
                    ::sidex::shape::CaseShape::new(
                        "Aggregate",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<AggregationRule>),
                    )
                    .with_docs("Health is evaluated from other nodes.\n")
                    .json_name("aggregate"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "rule",
                    content: "content",
                }),
            ),
        )
        .with_docs("Definition of an observed, threshold, or aggregate node.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("rule"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("lowercase"),
                },
            ],
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
    #[doc = "Constraints on externally supplied observations.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ObservationRule {
        #[doc = "Optional consumer limit on status age, in milliseconds.\n"]
        pub max_age_ms: ::std::option::Option<u32>,
        #[doc = "Optional tag that published status records must match exactly.\n"]
        pub expected_tag: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ObservationRule {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::ObservationRule",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("max_age_ms", ::sidex::shape::shape_of::<u32>)
                    .with_docs("Optional consumer limit on status age, in milliseconds.\n")
                    .optional()
                    .json_name("maxAgeMs"),
                ::sidex::shape::FieldShape::new(
                    "expected_tag",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Optional tag that published status records must match exactly.\n")
                .optional()
                .json_name("expectedTag"),
            ])),
        )
        .with_docs("Constraints on externally supplied observations.\n");
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
            let mut __field_0 = ::std::option::Option::<u32>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 =
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
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
    #[doc = "Desired and minimum ranges for one runtime value.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ThresholdRule {
        #[doc = "Value definition supplying the unit, identity, and freshness constraints.\n"]
        pub value: super::value::ValueId,
        #[doc = "Desired range; must be a subset of the usable range.\n"]
        pub healthy: super::value::ValueRange,
        #[doc = "Minimum range for usability.\n"]
        pub usable: super::value::ValueRange,
        #[doc = "Optional consumer limit on observation age, in milliseconds; must be positive.\n"]
        pub max_age_ms: ::std::option::Option<u32>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ThresholdRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::ThresholdRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("value" , :: sidex :: shape :: shape_of :: < super :: value :: ValueId > ,) . with_docs ("Value definition supplying the unit, identity, and freshness constraints.\n") , :: sidex :: shape :: FieldShape :: new ("healthy" , :: sidex :: shape :: shape_of :: < super :: value :: ValueRange > ,) . with_docs ("Desired range; must be a subset of the usable range.\n") , :: sidex :: shape :: FieldShape :: new ("usable" , :: sidex :: shape :: shape_of :: < super :: value :: ValueRange > ,) . with_docs ("Minimum range for usability.\n") , :: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u32 > ,) . with_docs ("Optional consumer limit on observation age, in milliseconds; must be positive.\n") . optional () . json_name ("maxAgeMs")]))) . with_docs ("Desired and minimum ranges for one runtime value.\n") ;
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.healthy)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.usable)),
                3usize => self
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
            let mut __field_0 = ::std::option::Option::<super::value::ValueId>::None;
            let mut __field_1 = ::std::option::Option::<super::value::ValueRange>::None;
            let mut __field_2 = ::std::option::Option::<super::value::ValueRange>::None;
            let mut __field_3 = ::std::option::Option::<u32>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::value::ValueId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::value::ValueRange as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::value::ValueRange as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                value: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `value`"))?,
                healthy: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthy`"))?,
                usable: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `usable`"))?,
                max_age_ms: __field_3,
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
    #[doc = "Minimum and desired health targets.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct AggregationRule {
        #[doc = "Dependencies with distinct node IDs.\n"]
        pub inputs: ::std::vec::Vec<AggregationInput>,
        #[doc = "Minimum healthy or degraded input count, including mandatory inputs.\n"]
        pub required: u32,
        #[doc = "Desired healthy input count, including mandatory inputs; required <= healthy <= inputs length.\n"]
        pub healthy: u32,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AggregationRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::AggregationRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("inputs" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < AggregationInput , > > ,) . with_docs ("Dependencies with distinct node IDs.\n") , :: sidex :: shape :: FieldShape :: new ("required" , :: sidex :: shape :: shape_of :: < u32 > ,) . with_docs ("Minimum healthy or degraded input count, including mandatory inputs.\n") , :: sidex :: shape :: FieldShape :: new ("healthy" , :: sidex :: shape :: shape_of :: < u32 > ,) . with_docs ("Desired healthy input count, including mandatory inputs; required <= healthy <= inputs length.\n")]))) . with_docs ("Minimum and desired health targets.\n") ;
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.inputs)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.required)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.healthy)),
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
            let mut __field_1 = ::std::option::Option::<u32>::None;
            let mut __field_2 = ::std::option::Option::<u32>::None;
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
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                inputs: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `inputs`"))?,
                required: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `required`"))?,
                healthy: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `healthy`"))?,
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
    #[doc = "One dependency of an aggregate health node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct AggregationInput {
        #[doc = "Referenced health node ID.\n"]
        pub node: HealthNodeId,
        #[doc = "Whether this dependency must meet minimum and desired criteria; omitted means false.\n"]
        pub required: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for AggregationInput {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::AggregationInput" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("node" , :: sidex :: shape :: shape_of :: < HealthNodeId > ,) . with_docs ("Referenced health node ID.\n") , :: sidex :: shape :: FieldShape :: new ("required" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Whether this dependency must meet minimum and desired criteria; omitted means false.\n") . optional ()]))) . with_docs ("One dependency of an aggregate health node.\n") ;
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
                    .required
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
            let mut __field_1 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthNodeId as ::sidex::Decode>::decode(decoder)?,
                        )
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
                node: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `node`"))?,
                required: __field_1,
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
    #[doc = "Reported status of one observed node within a boot.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthStatus {
        #[doc = "Observed node ID.\n"]
        pub id: HealthNodeId,
        #[doc = "Ownership group; must match the node's origin when one is specified.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Reported health.\n"]
        pub state: HealthState,
        #[doc = "Boot identifier defining the observation clock domain.\n"]
        pub boot_id: ::std::string::String,
        #[doc = "Milliseconds on the boot-time clock when health was observed.\n"]
        pub observed_at_ms: u64,
        #[doc = "Finite positive validity duration in milliseconds.\n"]
        pub max_age_ms: u32,
        #[doc = "Opaque label identifying the check configuration or instance.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Explanation supplied by the observer.\n"]
        pub message: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthStatus {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthStatus",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("id", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("Observed node ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "origin",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Ownership group; must match the node's origin when one is specified.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("state", ::sidex::shape::shape_of::<HealthState>)
                    .with_docs("Reported health.\n"),
                ::sidex::shape::FieldShape::new(
                    "boot_id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Boot identifier defining the observation clock domain.\n")
                .json_name("bootId"),
                ::sidex::shape::FieldShape::new("observed_at_ms", ::sidex::shape::shape_of::<u64>)
                    .with_docs("Milliseconds on the boot-time clock when health was observed.\n")
                    .json_name("observedAtMs"),
                ::sidex::shape::FieldShape::new("max_age_ms", ::sidex::shape::shape_of::<u32>)
                    .with_docs("Finite positive validity duration in milliseconds.\n")
                    .json_name("maxAgeMs"),
                ::sidex::shape::FieldShape::new(
                    "tag",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Opaque label identifying the check configuration or instance.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "message",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Explanation supplied by the observer.\n")
                .optional(),
            ])),
        )
        .with_docs("Reported status of one observed node within a boot.\n");
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
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.state)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.boot_id)),
                4usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.observed_at_ms))
                }
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.max_age_ms)),
                6usize => self
                    .tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .message
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
            let mut __field_0 = ::std::option::Option::<HealthNodeId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<HealthState>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 = ::std::option::Option::<u64>::None;
            let mut __field_5 = ::std::option::Option::<u32>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
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
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
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
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                origin: __field_1,
                state: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `state`"))?,
                boot_id: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bootId`"))?,
                observed_at_ms: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `observedAtMs`"))?,
                max_age_ms: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `maxAgeMs`"))?,
                tag: __field_6,
                message: __field_7,
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
    #[doc = "Health relative to a node's declared expectations.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum HealthState {
        #[doc = "Desired health criteria are confirmed.\n"]
        Healthy,
        #[doc = "Minimum criteria are confirmed; desired criteria are unconfirmed or unmet.\n"]
        Degraded,
        #[doc = "Minimum criteria are confirmed to be unmet.\n"]
        Unhealthy,
        #[doc = "Evidence does not determine whether minimum criteria are met.\n"]
        Unknown,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthState {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::health::HealthState" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Healthy" , :: std :: option :: Option :: None) . with_docs ("Desired health criteria are confirmed.\n") . json_name ("healthy") , :: sidex :: shape :: CaseShape :: new ("Degraded" , :: std :: option :: Option :: None) . with_docs ("Minimum criteria are confirmed; desired criteria are unconfirmed or unmet.\n") . json_name ("degraded") , :: sidex :: shape :: CaseShape :: new ("Unhealthy" , :: std :: option :: Option :: None) . with_docs ("Minimum criteria are confirmed to be unmet.\n") . json_name ("unhealthy") , :: sidex :: shape :: CaseShape :: new ("Unknown" , :: std :: option :: Option :: None) . with_docs ("Evidence does not determine whether minimum criteria are met.\n") . json_name ("unknown")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Health relative to a node's declared expectations.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") } , :: sidex :: shape :: Attr :: Assign { path : "tag" , value : :: sidex :: shape :: AttrValue :: String ("state") } , :: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("lowercase") }] } , :: sidex :: shape :: Attr :: List { path : "rust" , args : & [:: sidex :: shape :: Attr :: List { path : "derive" , args : & [:: sidex :: shape :: Attr :: Path ("Copy") , :: sidex :: shape :: Attr :: Path ("Eq")] }] }]) ;
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
    #[doc = "Clock context supplied by a consumer.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthContext {
        #[doc = "Current boot identifier.\n"]
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
                .with_docs("Current boot identifier.\n")
                .json_name("bootId"),
                ::sidex::shape::FieldShape::new("now_ms", ::sidex::shape::shape_of::<u64>)
                    .with_docs("Current boot-time clock value in milliseconds.\n")
                    .json_name("nowMs"),
            ])),
        )
        .with_docs("Clock context supplied by a consumer.\n");
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
    #[doc = "Paired wall-clock and boot-time sample from the system being observed.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthClockSample {
        #[doc = "Boot identifier and boot-time milliseconds at sampling.\n"]
        pub context: HealthContext,
        #[doc = "Wall-clock timestamp at the same instant, serialized as an RFC 3339 string.\n"]
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
                .with_docs("Boot identifier and boot-time milliseconds at sampling.\n"),
                ::sidex::shape::FieldShape::new(
                    "timestamp",
                    ::sidex::shape::shape_of::<::sidex::types::Timestamp>,
                )
                .with_docs(
                    "Wall-clock timestamp at the same instant, serialized as an RFC 3339 string.\n",
                ),
            ])),
        )
        .with_docs("Paired wall-clock and boot-time sample from the system being observed.\n");
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
    #[doc = "Transport view of a health graph and the evidence used to evaluate it.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthSnapshot {
        #[doc = "Clock correlation sampled after collecting the records.\n"]
        pub clock: HealthClockSample,
        #[doc = "Health definitions, including their origins.\n"]
        pub nodes: ::std::vec::Vec<HealthNode>,
        #[doc = "Original status records; observation times and validity are preserved.\n"]
        pub statuses: ::std::vec::Vec<HealthStatus>,
        #[doc = "Value definitions used by threshold nodes or displayed independently.\n"]
        pub values: ::std::option::Option<::std::vec::Vec<super::value::ValueDefinition>>,
        #[doc = "Original measurements; observation times and validity are preserved.\n"]
        pub value_observations:
            ::std::option::Option<::std::vec::Vec<super::value::ValueObservation>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthSnapshot {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthSnapshot",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "clock",
                    ::sidex::shape::shape_of::<HealthClockSample>,
                )
                .with_docs("Clock correlation sampled after collecting the records.\n"),
                ::sidex::shape::FieldShape::new(
                    "nodes",
                    ::sidex::shape::shape_of::<::std::vec::Vec<HealthNode>>,
                )
                .with_docs("Health definitions, including their origins.\n"),
                ::sidex::shape::FieldShape::new(
                    "statuses",
                    ::sidex::shape::shape_of::<::std::vec::Vec<HealthStatus>>,
                )
                .with_docs(
                    "Original status records; observation times and validity are preserved.\n",
                ),
                ::sidex::shape::FieldShape::new(
                    "values",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::value::ValueDefinition>>,
                )
                .with_docs(
                    "Value definitions used by threshold nodes or displayed independently.\n",
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "value_observations",
                    ::sidex::shape::shape_of::<::std::vec::Vec<super::value::ValueObservation>>,
                )
                .with_docs("Original measurements; observation times and validity are preserved.\n")
                .optional()
                .json_name("valueObservations"),
            ])),
        )
        .with_docs("Transport view of a health graph and the evidence used to evaluate it.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for HealthSnapshot {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for HealthSnapshot {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.clock)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.nodes)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.statuses)),
                3usize => self
                    .values
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .value_observations
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for HealthSnapshot {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<HealthClockSample>::None;
            let mut __field_1 = ::std::option::Option::<::std::vec::Vec<HealthNode>>::None;
            let mut __field_2 = ::std::option::Option::<::std::vec::Vec<HealthStatus>>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<super::value::ValueDefinition>>::None;
            let mut __field_4 =
                ::std::option::Option::<::std::vec::Vec<super::value::ValueObservation>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <HealthClockSample as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthNode> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthStatus> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::value::ValueDefinition,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::std::vec::Vec<
                            super::value::ValueObservation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                clock: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `clock`"))?,
                nodes: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `nodes`"))?,
                statuses: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `statuses`"))?,
                values: __field_3,
                value_observations: __field_4,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for HealthSnapshot {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for HealthSnapshot {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Evaluated graph, in declaration order.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthReport {
        #[doc = "Evaluated nodes.\n"]
        pub nodes: ::std::vec::Vec<HealthNodeResult>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthReport {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthReport",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "nodes",
                    ::sidex::shape::shape_of::<::std::vec::Vec<HealthNodeResult>>,
                )
                .with_docs("Evaluated nodes.\n"),
            ])),
        )
        .with_docs("Evaluated graph, in declaration order.\n");
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
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthNodeResult> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                nodes: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `nodes`"))?,
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
    #[doc = "Evaluated state and evidence for a node.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct HealthNodeResult {
        #[doc = "Node ID.\n"]
        pub id: HealthNodeId,
        #[doc = "Origin of the evaluated node.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Evaluated health.\n"]
        pub state: HealthState,
        #[doc = "Direct dependencies explaining an aggregate result.\n"]
        pub inputs: ::std::vec::Vec<HealthNodeId>,
        #[doc = "Observation message or freshness explanation.\n"]
        pub message: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for HealthNodeResult {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::health::HealthNodeResult",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("id", ::sidex::shape::shape_of::<HealthNodeId>)
                    .with_docs("Node ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "origin",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Origin of the evaluated node.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("state", ::sidex::shape::shape_of::<HealthState>)
                    .with_docs("Evaluated health.\n"),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<::std::vec::Vec<HealthNodeId>>,
                )
                .with_docs("Direct dependencies explaining an aggregate result.\n"),
                ::sidex::shape::FieldShape::new(
                    "message",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Observation message or freshness explanation.\n")
                .optional(),
            ])),
        )
        .with_docs("Evaluated state and evidence for a node.\n");
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
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.state)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.inputs)),
                4usize => self
                    .message
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
            let mut __field_2 = ::std::option::Option::<HealthState>::None;
            let mut __field_3 = ::std::option::Option::<::std::vec::Vec<HealthNodeId>>::None;
            let mut __field_4 = ::std::option::Option::<::std::string::String>::None;
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
                            <HealthState as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::std::vec::Vec<HealthNodeId> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                origin: __field_1,
                state: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `state`"))?,
                inputs: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `inputs`"))?,
                message: __field_4,
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
}
pub mod value {
    #![doc = "Numeric runtime values, their units, and threshold ranges.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Identifier of a runtime value, unique independently of health node IDs.\n"]
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValueId(pub(crate) ::std::string::String);
    impl ::std::convert::From<ValueId> for ::std::string::String {
        fn from(wrapped: ValueId) -> Self {
            wrapped.0
        }
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueId {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::value::ValueId",
            ::sidex::shape::ShapeKind::Wrapper(::sidex::shape::shape_of::<::std::string::String>),
        )
        .with_docs("Identifier of a runtime value, unique independently of health node IDs.\n")
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
    impl ::sidex::Encode for ValueId {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::Encode::encode(&self.0)
        }
    }
    #[automatically_derived]
    impl ::sidex::Decode for ValueId {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            ::std::result::Result::Ok(Self(<::std::string::String as ::sidex::Decode>::decode(
                decoder,
            )?))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ValueId {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueId {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A named measurement, independent of any health policy.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ValueDefinition {
        #[doc = "Unique value ID.\n"]
        pub id: ValueId,
        #[doc = "Ownership group for replacement or removal, matched exactly.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Human-readable label.\n"]
        pub label: ::std::option::Option<::std::string::String>,
        #[doc = "Related component IDs; these associations do not imply health dependencies.\n"]
        pub components: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Unit shared by observations and all thresholds referencing this value.\n"]
        pub unit: ValueUnit,
        #[doc = "Optional consumer limit on observation age, in milliseconds; must be positive.\n"]
        pub max_age_ms: ::std::option::Option<u32>,
        #[doc = "Optional tag that observations must match exactly.\n"]
        pub expected_tag: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueDefinition {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::system_runtime::value::ValueDefinition" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("id" , :: sidex :: shape :: shape_of :: < ValueId > ,) . with_docs ("Unique value ID.\n") , :: sidex :: shape :: FieldShape :: new ("origin" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Ownership group for replacement or removal, matched exactly.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("label" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Human-readable label.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("components" , :: sidex :: shape :: shape_of :: < :: std :: vec :: Vec < :: std :: string :: String , > > ,) . with_docs ("Related component IDs; these associations do not imply health dependencies.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("unit" , :: sidex :: shape :: shape_of :: < ValueUnit > ,) . with_docs ("Unit shared by observations and all thresholds referencing this value.\n") , :: sidex :: shape :: FieldShape :: new ("max_age_ms" , :: sidex :: shape :: shape_of :: < u32 > ,) . with_docs ("Optional consumer limit on observation age, in milliseconds; must be positive.\n") . optional () . json_name ("maxAgeMs") , :: sidex :: shape :: FieldShape :: new ("expected_tag" , :: sidex :: shape :: shape_of :: < :: std :: string :: String > ,) . with_docs ("Optional tag that observations must match exactly.\n") . optional () . json_name ("expectedTag")]))) . with_docs ("A named measurement, independent of any health policy.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ValueDefinition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ValueDefinition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .label
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .components
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.unit)),
                5usize => self
                    .max_age_ms
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .expected_tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ValueDefinition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<ValueId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_3 =
                ::std::option::Option::<::std::vec::Vec<::std::string::String>>::None;
            let mut __field_4 = ::std::option::Option::<ValueUnit>::None;
            let mut __field_5 = ::std::option::Option::<u32>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <ValueId as ::sidex::Decode>::decode(decoder)?,
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
                        __field_3 = ::std::option::Option::Some(<::std::vec::Vec<
                            ::std::string::String,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <ValueUnit as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                origin: __field_1,
                label: __field_2,
                components: __field_3,
                unit: __field_4.ok_or_else(|| ::sidex::DecodeError::new("missing field `unit`"))?,
                max_age_ms: __field_5,
                expected_tag: __field_6,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ValueDefinition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueDefinition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "One measurement within a boot; publishing it does not assert health.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ValueObservation {
        #[doc = "Observed value ID.\n"]
        pub id: ValueId,
        #[doc = "Ownership group; must match the definition's origin when specified.\n"]
        pub origin: ::std::option::Option<::std::string::String>,
        #[doc = "Finite measurement in the definition's unit.\n"]
        pub value: f64,
        #[doc = "Boot identifier defining the observation clock domain.\n"]
        pub boot_id: ::std::string::String,
        #[doc = "Milliseconds on the boot-time clock when the measurement was taken.\n"]
        pub observed_at_ms: u64,
        #[doc = "Positive validity duration in milliseconds.\n"]
        pub max_age_ms: u32,
        #[doc = "Opaque label identifying the measurement configuration or instance.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Explanation supplied by the observer.\n"]
        pub message: ::std::option::Option<::std::string::String>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueObservation {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::value::ValueObservation",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("id", ::sidex::shape::shape_of::<ValueId>)
                    .with_docs("Observed value ID.\n"),
                ::sidex::shape::FieldShape::new(
                    "origin",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Ownership group; must match the definition's origin when specified.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("value", ::sidex::shape::shape_of::<f64>)
                    .with_docs("Finite measurement in the definition's unit.\n"),
                ::sidex::shape::FieldShape::new(
                    "boot_id",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Boot identifier defining the observation clock domain.\n")
                .json_name("bootId"),
                ::sidex::shape::FieldShape::new("observed_at_ms", ::sidex::shape::shape_of::<u64>)
                    .with_docs(
                        "Milliseconds on the boot-time clock when the measurement was taken.\n",
                    )
                    .json_name("observedAtMs"),
                ::sidex::shape::FieldShape::new("max_age_ms", ::sidex::shape::shape_of::<u32>)
                    .with_docs("Positive validity duration in milliseconds.\n")
                    .json_name("maxAgeMs"),
                ::sidex::shape::FieldShape::new(
                    "tag",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Opaque label identifying the measurement configuration or instance.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "message",
                    ::sidex::shape::shape_of::<::std::string::String>,
                )
                .with_docs("Explanation supplied by the observer.\n")
                .optional(),
            ])),
        )
        .with_docs("One measurement within a boot; publishing it does not assert health.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ValueObservation {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ValueObservation {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.id)),
                1usize => self
                    .origin
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.value)),
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.boot_id)),
                4usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.observed_at_ms))
                }
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.max_age_ms)),
                6usize => self
                    .tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .message
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ValueObservation {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<ValueId>::None;
            let mut __field_1 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_2 = ::std::option::Option::<f64>::None;
            let mut __field_3 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_4 = ::std::option::Option::<u64>::None;
            let mut __field_5 = ::std::option::Option::<u32>::None;
            let mut __field_6 = ::std::option::Option::<::std::string::String>::None;
            let mut __field_7 = ::std::option::Option::<::std::string::String>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <ValueId as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::std::string::String as ::sidex::Decode>::decode(decoder)?,
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
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
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
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                id: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `id`"))?,
                origin: __field_1,
                value: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `value`"))?,
                boot_id: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `bootId`"))?,
                observed_at_ms: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `observedAtMs`"))?,
                max_age_ms: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `maxAgeMs`"))?,
                tag: __field_6,
                message: __field_7,
            })
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ValueObservation {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueObservation {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Semantic unit; display prefixes, precision, and conversions are consumer choices.\n"]
    #[derive(Clone, Debug, PartialEq, Copy, Eq)]
    pub enum ValueUnit {
        #[doc = "Degrees Celsius; consumers may convert for display.\n"]
        Celsius,
        #[doc = "Bytes; consumers may display decimal or binary prefixes.\n"]
        Bytes,
        #[doc = "Seconds; consumers may display subsecond units.\n"]
        Seconds,
        #[doc = "Dimensionless fraction, with 1 corresponding to 100 percent.\n"]
        Ratio,
        #[doc = "Number of items or events.\n"]
        Count,
        #[doc = "Bytes per second.\n"]
        BytesPerSecond,
        #[doc = "Cycles per second.\n"]
        Hertz,
        #[doc = "Electric potential in volts.\n"]
        Volts,
        #[doc = "Electric current in amperes.\n"]
        Amperes,
        #[doc = "Power in watts.\n"]
        Watts,
        #[doc = "Revolutions per minute.\n"]
        RevolutionsPerMinute,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueUnit {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::value::ValueUnit",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Celsius", ::std::option::Option::None)
                        .with_docs("Degrees Celsius; consumers may convert for display.\n")
                        .json_name("celsius"),
                    ::sidex::shape::CaseShape::new("Bytes", ::std::option::Option::None)
                        .with_docs("Bytes; consumers may display decimal or binary prefixes.\n")
                        .json_name("bytes"),
                    ::sidex::shape::CaseShape::new("Seconds", ::std::option::Option::None)
                        .with_docs("Seconds; consumers may display subsecond units.\n")
                        .json_name("seconds"),
                    ::sidex::shape::CaseShape::new("Ratio", ::std::option::Option::None)
                        .with_docs("Dimensionless fraction, with 1 corresponding to 100 percent.\n")
                        .json_name("ratio"),
                    ::sidex::shape::CaseShape::new("Count", ::std::option::Option::None)
                        .with_docs("Number of items or events.\n")
                        .json_name("count"),
                    ::sidex::shape::CaseShape::new("BytesPerSecond", ::std::option::Option::None)
                        .with_docs("Bytes per second.\n")
                        .json_name("bytes-per-second"),
                    ::sidex::shape::CaseShape::new("Hertz", ::std::option::Option::None)
                        .with_docs("Cycles per second.\n")
                        .json_name("hertz"),
                    ::sidex::shape::CaseShape::new("Volts", ::std::option::Option::None)
                        .with_docs("Electric potential in volts.\n")
                        .json_name("volts"),
                    ::sidex::shape::CaseShape::new("Amperes", ::std::option::Option::None)
                        .with_docs("Electric current in amperes.\n")
                        .json_name("amperes"),
                    ::sidex::shape::CaseShape::new("Watts", ::std::option::Option::None)
                        .with_docs("Power in watts.\n")
                        .json_name("watts"),
                    ::sidex::shape::CaseShape::new(
                        "RevolutionsPerMinute",
                        ::std::option::Option::None,
                    )
                    .with_docs("Revolutions per minute.\n")
                    .json_name("revolutions-per-minute"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs(
            "Semantic unit; display prefixes, precision, and conversions are consumer choices.\n",
        )
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "tagged",
                        value: ::sidex::shape::AttrValue::Path("externally"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "rename_all",
                        value: ::sidex::shape::AttrValue::String("kebab-case"),
                    },
                ],
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
    impl ::sidex::Encode for ValueUnit {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ValueUnit {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Celsius => 0usize,
                Self::Bytes => 1usize,
                Self::Seconds => 2usize,
                Self::Ratio => 3usize,
                Self::Count => 4usize,
                Self::BytesPerSecond => 5usize,
                Self::Hertz => 6usize,
                Self::Volts => 7usize,
                Self::Amperes => 8usize,
                Self::Watts => 9usize,
                Self::RevolutionsPerMinute => 10usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Celsius => ::std::option::Option::None,
                Self::Bytes => ::std::option::Option::None,
                Self::Seconds => ::std::option::Option::None,
                Self::Ratio => ::std::option::Option::None,
                Self::Count => ::std::option::Option::None,
                Self::BytesPerSecond => ::std::option::Option::None,
                Self::Hertz => ::std::option::Option::None,
                Self::Volts => ::std::option::Option::None,
                Self::Amperes => ::std::option::Option::None,
                Self::Watts => ::std::option::Option::None,
                Self::RevolutionsPerMinute => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ValueUnit {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Celsius,
                    1usize => Self::Bytes,
                    2usize => Self::Seconds,
                    3usize => Self::Ratio,
                    4usize => Self::Count,
                    5usize => Self::BytesPerSecond,
                    6usize => Self::Hertz,
                    7usize => Self::Volts,
                    8usize => Self::Amperes,
                    9usize => Self::Watts,
                    10usize => Self::RevolutionsPerMinute,
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
    impl ::serde::Serialize for ValueUnit {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueUnit {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "One endpoint of a numeric range, in the referenced value's unit.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ValueBound {
        #[doc = "Finite endpoint in the referenced value's unit.\n"]
        pub value: f64,
        #[doc = "Whether equality with this endpoint is inside the range.\n"]
        pub inclusive: bool,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueBound {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::value::ValueBound",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("value", ::sidex::shape::shape_of::<f64>)
                    .with_docs("Finite endpoint in the referenced value's unit.\n"),
                ::sidex::shape::FieldShape::new("inclusive", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Whether equality with this endpoint is inside the range.\n"),
            ])),
        )
        .with_docs("One endpoint of a numeric range, in the referenced value's unit.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ValueBound {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ValueBound {
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
    impl ::sidex::Decode for ValueBound {
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
    impl ::serde::Serialize for ValueBound {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueBound {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A nonempty interval; omitted endpoints are unbounded.\n"]
    #[derive(Clone, Debug, PartialEq)]
    pub struct ValueRange {
        #[doc = "Optional lower endpoint.\n"]
        pub lower: ::std::option::Option<ValueBound>,
        #[doc = "Optional upper endpoint.\n"]
        pub upper: ::std::option::Option<ValueBound>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ValueRange {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::system_runtime::value::ValueRange",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("lower", ::sidex::shape::shape_of::<ValueBound>)
                    .with_docs("Optional lower endpoint.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new("upper", ::sidex::shape::shape_of::<ValueBound>)
                    .with_docs("Optional upper endpoint.\n")
                    .optional(),
            ])),
        )
        .with_docs("A nonempty interval; omitted endpoints are unbounded.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ValueRange {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ValueRange {
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
    impl ::sidex::Decode for ValueRange {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<ValueBound>::None;
            let mut __field_1 = ::std::option::Option::<ValueBound>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <ValueBound as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <ValueBound as ::sidex::Decode>::decode(decoder)?,
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
    impl ::serde::Serialize for ValueRange {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ValueRange {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
