use serde::{Deserialize, Serialize};

use crate::groups::SimpleRestrictionModel;

// OpenAttrs is This type is extended by almost all schema types
//        to allow attributes from other namespaces to be
//        added to user schemas.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenAttrs {}

// Annotated is This type is extended by all types which allow annotation
//        other than <schema> itself
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Annotated {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(flatten)]
    pub open_attrs: OpenAttrs,
}

// Composition ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Composition {
    #[serde(rename = "xs:include")]
    pub xs_include: Include,
    #[serde(rename = "xs:import")]
    pub xs_import: Import,
    #[serde(rename = "xs:redefine")]
    pub xs_redefine: Redefine,
    #[serde(rename = "xs:override")]
    pub xs_override: Override,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// SchemaTop ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SchemaTop {
    #[serde(rename = "xs:element")]
    pub xs_element: TopLevelElement,
    #[serde(rename = "xs:attribute")]
    pub xs_attribute: Attribute,
    #[serde(rename = "xs:notation")]
    pub xs_notation: Notation,
    #[serde(rename = "xs:redefinable")]
    pub xs_redefinable: Redefinable,
}

// Redefinable ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Redefinable {
    #[serde(rename = "xs:simpleType")]
    pub xs_simple_type: SimpleType,
    #[serde(rename = "xs:complexType")]
    pub xs_complex_type: TopLevelComplexType,
    #[serde(rename = "xs:group")]
    pub xs_group: Group,
    #[serde(rename = "xs:attributeGroup")]
    pub xs_attribute_group: AttributeGroup,
}

// FormChoice is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FormChoice {
    #[serde(rename = "formChoice")]
    pub form_choice: String,
}

// ReducedDerivationControl is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReducedDerivationControl {
    #[serde(rename = "reducedDerivationControl")]
    pub reduced_derivation_control: String,
}

// DerivationSet is #all or (possibly empty) subset of {extension, restriction}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DerivationSet {
    #[serde(rename = "derivationSet")]
    pub derivation_set: Vec<String>,
}

// TypeDerivationControl is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TypeDerivationControl {
    #[serde(rename = "typeDerivationControl")]
    pub type_derivation_control: String,
}

// FullDerivationSet is #all or (possibly empty) subset of {extension, restriction, list, union}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FullDerivationSet {
    #[serde(rename = "fullDerivationSet")]
    pub full_derivation_set: Vec<String>,
}

// Schema ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    #[serde(rename = "targetNamespace")]
    pub target_namespace: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "finalDefault")]
    pub final_default: Option<FullDerivationSet>,
    #[serde(rename = "blockDefault")]
    pub block_default: Option<BlockSet>,
    #[serde(rename = "attributeFormDefault")]
    pub attribute_form_default: Option<String>,
    #[serde(rename = "elementFormDefault")]
    pub element_form_default: Option<String>,
    #[serde(rename = "defaultAttributes")]
    pub default_attributes: Option<String>,
    #[serde(rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<XpathDefaultNamespace>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "xml:lang")]
    pub xml_lang: Option<Lang>,
    #[serde(rename = "xs:composition")]
    pub xs_composition: Vec<Composition>,
    #[serde(rename = "xs:schemaTop")]
    pub xs_schema_top: SchemaTop,
    #[serde(rename = "xs:defaultOpenContent")]
    pub xs_default_open_content: String,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Vec<Annotation>,
    #[serde(flatten)]
    pub open_attrs: OpenAttrs,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AllNNI {
    #[serde(rename = "allNNI")]
    pub non_negative_integer: u32,
}

// Occurs is for all particles
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Occurs {
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<u32>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<AllNNI>,
}

// DefRef is for element, group and attributeGroup,
//    which both define and reference
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DefRef {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub ref_attr: Option<String>,
}

// TypeDefParticle ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TypeDefParticle {
    #[serde(rename = "group")]
    pub group: GroupRef,
    #[serde(rename = "xs:all")]
    pub xs_all: All,
    #[serde(rename = "xs:choice")]
    pub xs_choice: Choice,
    #[serde(rename = "xs:sequence")]
    pub xs_sequence: Sequence,
}

// NestedParticle ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NestedParticle {
    #[serde(rename = "element")]
    pub element: LocalElement,
    #[serde(rename = "group")]
    pub group: GroupRef,
    #[serde(rename = "xs:choice")]
    pub xs_choice: Choice,
    #[serde(rename = "xs:sequence")]
    pub xs_sequence: Sequence,
    #[serde(rename = "xs:any")]
    pub xs_any: Any,
}

// Particle ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Particle {
    #[serde(rename = "element")]
    pub element: LocalElement,
    #[serde(rename = "group")]
    pub group: GroupRef,
    #[serde(rename = "xs:all")]
    pub xs_all: All,
    #[serde(rename = "xs:choice")]
    pub xs_choice: Choice,
    #[serde(rename = "xs:sequence")]
    pub xs_sequence: Sequence,
    #[serde(rename = "xs:any")]
    pub xs_any: Any,
}

// Attribute ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Attribute {
    #[serde(rename = "xs:defRef")]
    pub xs_def_ref: Vec<DefRef>,
    #[serde(rename = "type")]
    pub type_attr: Option<String>,
    #[serde(rename = "use")]
    pub use_attr: Option<String>,
    #[serde(rename = "default")]
    pub default: Option<String>,
    #[serde(rename = "fixed")]
    pub fixed: Option<String>,
    #[serde(rename = "form")]
    pub form: Option<String>,
    #[serde(rename = "targetNamespace")]
    pub target_namespace: Option<String>,
    #[serde(rename = "inheritable")]
    pub inheritable: Option<bool>,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// TopLevelAttribute ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TopLevelAttribute {
    #[serde(rename = "ref")]
    pub ref_attr: Option<char>,
    #[serde(rename = "form")]
    pub form: Option<char>,
    #[serde(rename = "use")]
    pub use_attr: Option<char>,
    #[serde(rename = "targetNamespace")]
    pub target_namespace: Option<char>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "inheritable")]
    pub inheritable: Option<bool>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
}

// AttrDecls ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AttrDecls {
    #[serde(rename = "attribute")]
    pub attribute: Attribute,
    #[serde(rename = "attributeGroup")]
    pub attribute_group: AttributeGroupRef,
    #[serde(rename = "xs:anyAttribute")]
    pub xs_any_attribute: AnyAttribute,
}

// AnyAttribute ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnyAttribute {
    #[serde(rename = "notQName")]
    pub not_q_name: Option<QnameListA>,
    #[serde(flatten)]
    pub wildcard: Wildcard,
}

// Assertions ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Assertions {
    #[serde(rename = "assert")]
    pub assert: Assertion,
}

// Assertion ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Assertion {
    #[serde(rename = "test")]
    pub test: Option<String>,
    #[serde(rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<XpathDefaultNamespace>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// ComplexTypeModel ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplexTypeModel {
    #[serde(rename = "xs:simpleContent")]
    pub xs_simple_content: SimpleContent,
    #[serde(rename = "xs:complexContent")]
    pub xs_complex_content: ComplexContent,
    #[serde(rename = "xs:openContent")]
    pub xs_open_content: String,
    #[serde(rename = "xs:typeDefParticle")]
    pub xs_type_def_particle: TypeDefParticle,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
}

// ComplexType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplexType {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "mixed")]
    pub mixed: Option<bool>,
    #[serde(rename = "abstract")]
    pub abstract_attr: Option<bool>,
    #[serde(rename = "final")]
    pub final_attr: Option<DerivationSet>,
    #[serde(rename = "block")]
    pub block: Option<DerivationSet>,
    #[serde(rename = "defaultAttributesApply")]
    pub default_attributes_apply: Option<bool>,
    #[serde(rename = "xs:complexTypeModel")]
    pub xs_complex_type_model: ComplexTypeModel,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// TopLevelComplexType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TopLevelComplexType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "xs:complexTypeModel")]
    pub xs_complex_type_model: ComplexTypeModel,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// LocalComplexType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalComplexType {
    #[serde(rename = "name")]
    pub name: Option<char>,
    #[serde(rename = "abstract")]
    pub abstract_attr: Option<char>,
    #[serde(rename = "final")]
    pub final_attr: Option<char>,
    #[serde(rename = "block")]
    pub block: Option<char>,
    #[serde(rename = "xs:complexTypeModel")]
    pub xs_complex_type_model: ComplexTypeModel,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// RestrictionType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RestrictionType {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "xs:typeDefParticle")]
    pub xs_type_def_particle: TypeDefParticle,
    #[serde(rename = "xs:simpleRestrictionModel")]
    pub xs_simple_restriction_model: SimpleRestrictionModel,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
    #[serde(rename = "xs:openContent")]
    pub xs_open_content: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// ComplexRestrictionType is This choice is added simply to
//                    make this a valid restriction per the REC
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplexRestrictionType {
    #[serde(rename = "xs:typeDefParticle")]
    pub xs_type_def_particle: TypeDefParticle,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "xs:openContent")]
    pub xs_open_content: String,
}

// ExtensionType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExtensionType {
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "xs:typeDefParticle")]
    pub xs_type_def_particle: TypeDefParticle,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
    #[serde(rename = "xs:openContent")]
    pub xs_open_content: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// ComplexContent ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComplexContent {
    #[serde(rename = "mixed")]
    pub mixed: Option<bool>,
    #[serde(rename = "restriction")]
    pub restriction: ComplexRestrictionType,
    #[serde(rename = "extension")]
    pub extension: ExtensionType,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// OpenContent ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenContent {
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "openContent")]
    pub open_content: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// DefaultOpenContent ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DefaultOpenContent {
    #[serde(rename = "appliesToEmpty")]
    pub applies_to_empty: Option<bool>,
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "defaultOpenContent")]
    pub default_open_content: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// SimpleRestrictionType is This choice is added simply to
//                    make this a valid restriction per the REC
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleRestrictionType {
    #[serde(rename = "xs:simpleRestrictionModel")]
    pub xs_simple_restriction_model: SimpleRestrictionModel,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// SimpleExtensionType is No typeDefParticle group reference
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleExtensionType {
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:assertions")]
    pub xs_assertions: Assertions,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// SimpleContent ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleContent {
    #[serde(rename = "restriction")]
    pub restriction: SimpleRestrictionType,
    #[serde(rename = "extension")]
    pub extension: SimpleExtensionType,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// BlockSet is #all or (possibly empty) subset of {substitution, extension,
//     restriction}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockSet {
    #[serde(rename = "blockSet")]
    pub block_set: Vec<String>,
}

// Element is The element element can be used either
//    at the top level to define an element-type binding globally,
//    or within a content model to either reference a globally-defined
//    element or type or declare an element-type binding locally.
//    The ref form is not allowed at the top level.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Element {
    #[serde(rename = "xs:defRef")]
    pub xs_def_ref: Vec<DefRef>,
    #[serde(rename = "xs:occurs")]
    pub xs_occurs: Vec<Occurs>,
    #[serde(rename = "type")]
    pub type_attr: Option<String>,
    #[serde(rename = "substitutionGroup")]
    pub substitution_group: Option<String>,
    #[serde(rename = "default")]
    pub default: Option<String>,
    #[serde(rename = "fixed")]
    pub fixed: Option<String>,
    #[serde(rename = "nillable")]
    pub nillable: Option<bool>,
    #[serde(rename = "abstract")]
    pub abstract_attr: Option<bool>,
    #[serde(rename = "final")]
    pub final_attr: Option<DerivationSet>,
    #[serde(rename = "block")]
    pub block: Option<BlockSet>,
    #[serde(rename = "form")]
    pub form: Option<String>,
    #[serde(rename = "targetNamespace")]
    pub target_namespace: Option<String>,
    #[serde(rename = "xs:identityConstraint")]
    pub xs_identity_constraint: Vec<IdentityConstraint>,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(rename = "complexType")]
    pub complex_type: LocalComplexType,
    #[serde(rename = "alternative")]
    pub alternative: Vec<AltType>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// TopLevelElement ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TopLevelElement {
    #[serde(rename = "ref")]
    pub ref_attr: Option<char>,
    #[serde(rename = "form")]
    pub form: Option<char>,
    #[serde(rename = "targetNamespace")]
    pub target_namespace: Option<char>,
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<char>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<char>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "xs:identityConstraint")]
    pub xs_identity_constraint: Vec<IdentityConstraint>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(rename = "complexType")]
    pub complex_type: LocalComplexType,
    #[serde(rename = "alternative")]
    pub alternative: Vec<AltType>,
}

// LocalElement ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalElement {
    #[serde(rename = "substitutionGroup")]
    pub substitution_group: Option<char>,
    #[serde(rename = "final")]
    pub final_attr: Option<char>,
    #[serde(rename = "abstract")]
    pub abstract_attr: Option<char>,
    #[serde(rename = "xs:identityConstraint")]
    pub xs_identity_constraint: Vec<IdentityConstraint>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(rename = "complexType")]
    pub complex_type: LocalComplexType,
    #[serde(rename = "alternative")]
    pub alternative: Vec<AltType>,
}

// AltType is This type is used for 'alternative' elements.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AltType {
    #[serde(rename = "test")]
    pub test: Option<String>,
    #[serde(rename = "type")]
    pub type_attr: Option<String>,
    #[serde(rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<XpathDefaultNamespace>,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(rename = "complexType")]
    pub complex_type: LocalComplexType,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Group is group type for explicit groups, named top-level groups and
//    group references
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Group {
    #[serde(rename = "xs:defRef")]
    pub xs_def_ref: Vec<DefRef>,
    #[serde(rename = "xs:occurs")]
    pub xs_occurs: Vec<Occurs>,
    #[serde(rename = "xs:particle")]
    pub xs_particle: Vec<Particle>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// RealGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RealGroup {
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "xs:all")]
    pub xs_all: All,
    #[serde(rename = "xs:choice")]
    pub xs_choice: Choice,
    #[serde(rename = "xs:sequence")]
    pub xs_sequence: Sequence,
}

// All ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct All {
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<char>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<char>,
    #[serde(rename = "xs:allModel")]
    pub xs_all_model: AllModel,
}

// NamedGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamedGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ref")]
    pub ref_attr: Option<char>,
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<char>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<char>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(rename = "all")]
    pub all: All,
    #[serde(rename = "choice")]
    pub choice: SimpleExplicitGroup,
    #[serde(rename = "sequence")]
    pub sequence: SimpleExplicitGroup,
}

// GroupRef ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GroupRef {
    #[serde(rename = "ref")]
    pub ref_attr: String,
    #[serde(rename = "name")]
    pub name: Option<char>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// ExplicitGroup is group type for the three kinds of group
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExplicitGroup {
    #[serde(rename = "name")]
    pub name: Option<char>,
    #[serde(rename = "ref")]
    pub ref_attr: Option<char>,
    #[serde(rename = "xs:nestedParticle")]
    pub xs_nested_particle: Vec<NestedParticle>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// SimpleExplicitGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleExplicitGroup {
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<char>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<char>,
    #[serde(rename = "xs:nestedParticle")]
    pub xs_nested_particle: Vec<NestedParticle>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// AnyAttrGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnyAttrGroup {
    #[serde(rename = "namespace")]
    pub namespace: Option<NamespaceList>,
    #[serde(rename = "notNamespace")]
    pub not_namespace: Option<BasicNamespaceList>,
    #[serde(rename = "processContents")]
    pub process_contents: Option<String>,
}

// Wildcard ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Wildcard {
    #[serde(rename = "xs:anyAttrGroup")]
    pub xs_any_attr_group: Vec<AnyAttrGroup>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Any ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Any {
    #[serde(rename = "xs:occurs")]
    pub xs_occurs: Vec<Occurs>,
    #[serde(rename = "notQName")]
    pub not_q_name: Option<QnameList>,
    #[serde(flatten)]
    pub wildcard: Wildcard,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamespaceList {
    #[serde(rename = "namespaceList")]
    pub basic_namespace_list: BasicNamespaceList,
    #[serde(rename = "namespaceList")]
    pub special_namespace_list: SpecialNamespaceList,
}

// BasicNamespaceList is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BasicNamespaceList {
    #[serde(rename = "basicNamespaceList")]
    pub basic_namespace_list: Vec<String>,
}

// SpecialNamespaceList is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SpecialNamespaceList {
    #[serde(rename = "specialNamespaceList")]
    pub special_namespace_list: String,
}

// QnameList is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct QnameList {
    #[serde(rename = "qnameList")]
    pub qname_list: Vec<String>,
}

// QnameListA is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct QnameListA {
    #[serde(rename = "qnameListA")]
    pub qname_list_a: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct XpathDefaultNamespace {
    #[serde(rename = "xpathDefaultNamespace")]
    pub any_uri: String,
}

// AttributeGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AttributeGroup {
    #[serde(rename = "xs:defRef")]
    pub xs_def_ref: Vec<DefRef>,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// NamedAttributeGroup ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamedAttributeGroup {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ref")]
    pub ref_attr: Option<char>,
    #[serde(rename = "xs:attrDecls")]
    pub xs_attr_decls: AttrDecls,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// AttributeGroupRef ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AttributeGroupRef {
    #[serde(rename = "ref")]
    pub ref_attr: String,
    #[serde(rename = "name")]
    pub name: Option<char>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// Include ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Include {
    #[serde(rename = "schemaLocation")]
    pub schema_location: String,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Redefine ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Redefine {
    #[serde(rename = "schemaLocation")]
    pub schema_location: String,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "xs:redefinable")]
    pub xs_redefinable: Vec<Redefinable>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Vec<Annotation>,
    #[serde(flatten)]
    pub open_attrs: OpenAttrs,
}

// Override ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Override {
    #[serde(rename = "schemaLocation")]
    pub schema_location: String,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "xs:schemaTop")]
    pub xs_schema_top: Vec<SchemaTop>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
    #[serde(flatten)]
    pub open_attrs: OpenAttrs,
}

// Import ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Import {
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "schemaLocation")]
    pub schema_location: Option<String>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Selector ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Selector {
    #[serde(rename = "xpath")]
    pub xpath: String,
    #[serde(rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<XpathDefaultNamespace>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Field ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Field {
    #[serde(rename = "xpath")]
    pub xpath: String,
    #[serde(rename = "xpathDefaultNamespace")]
    pub xpath_default_namespace: Option<XpathDefaultNamespace>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Keybase ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Keybase {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub ref_attr: Option<String>,
    #[serde(rename = "xs:selector")]
    pub xs_selector: Selector,
    #[serde(rename = "xs:field")]
    pub xs_field: Vec<Field>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Keyref ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Keyref {
    #[serde(rename = "refer")]
    pub refer: Option<String>,
    #[serde(flatten)]
    pub keybase: Keybase,
}

// Notation ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Notation {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public")]
    pub public: Option<String>,
    #[serde(rename = "system")]
    pub system: Option<String>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Public is A public identifier, per ISO 8879
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Public {
    #[serde(rename = "public")]
    pub public: String,
}

// Appinfo ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Appinfo {
    #[serde(rename = "source")]
    pub source: Option<String>,
}

// Documentation ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Documentation {
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(rename = "xml:lang")]
    pub xml_lang: Option<Lang>,
}

// Annotation ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Annotation {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "xs:appinfo")]
    pub xs_appinfo: Vec<Appinfo>,
    #[serde(rename = "xs:documentation")]
    pub xs_documentation: Vec<Documentation>,
    #[serde(flatten)]
    pub open_attrs: OpenAttrs,
}

// AnyType is Not the real urType, but as close an approximation as we can
//    get in the XML representation
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnyType {}

// DerivationControl is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DerivationControl {
    #[serde(rename = "derivationControl")]
    pub derivation_control: String,
}

// SimpleDerivationSet is A utility type, not for public use
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleDerivationSet {
    #[serde(rename = "simpleDerivationSet")]
    pub simple_derivation_set: Vec<String>,
}

// SimpleType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SimpleType {
    #[serde(rename = "final")]
    pub final_attr: Option<SimpleDerivationSet>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "xs:simpleDerivation")]
    pub xs_simple_derivation: SimpleDerivation,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// TopLevelSimpleType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TopLevelSimpleType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "xs:simpleDerivation")]
    pub xs_simple_derivation: SimpleDerivation,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// LocalSimpleType ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalSimpleType {
    #[serde(rename = "name")]
    pub name: Option<char>,
    #[serde(rename = "final")]
    pub final_attr: Option<char>,
    #[serde(rename = "xs:simpleDerivation")]
    pub xs_simple_derivation: SimpleDerivation,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// facet is An abstract element, representing facets in general.
//         The facets defined by this spec are substitutable for
//         this element, and implementation-defined facets should
//         also name this as a substitution-group head.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct facet {
    #[serde(rename = "facet")]
    pub facet: Facet,
}

// Restriction is base attribute and simpleType child are mutually
//           exclusive, but one or other is required
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Restriction {
    #[serde(rename = "base")]
    pub base: Option<String>,
    #[serde(rename = "xs:simpleRestrictionModel")]
    pub xs_simple_restriction_model: SimpleRestrictionModel,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// List is itemType attribute and simpleType child are mutually
//           exclusive, but one or other is required
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct List {
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
    #[serde(rename = "simpleType")]
    pub simple_type: LocalSimpleType,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// Union is memberTypes attribute must be non-empty or there must be
//           at least one simpleType child
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Union {
    #[serde(rename = "memberTypes")]
    pub member_types: Option<String>,
    #[serde(rename = "simpleType")]
    pub simple_type: Vec<LocalSimpleType>,
    #[serde(flatten)]
    pub annotated: Annotated,
}

// NoFixedFacet ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NoFixedFacet {
    #[serde(rename = "fixed")]
    pub fixed: Option<char>,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// NumFacet ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NumFacet {
    #[serde(rename = "value")]
    pub value: u32,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// IntFacet ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IntFacet {
    #[serde(rename = "value")]
    pub value: i32,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// TotalDigits ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TotalDigits {
    #[serde(rename = "value")]
    pub value: u32,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// WhiteSpace ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct WhiteSpace {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// Pattern ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Pattern {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}

// ExplicitTimezone ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExplicitTimezone {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "xs:annotation")]
    pub xs_annotation: Annotation,
}
