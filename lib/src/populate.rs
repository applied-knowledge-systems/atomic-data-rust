use crate::{
    datatype::DataType,
    errors::AtomicResult,
    schema::{Class, Property},
    urls, Storelike,
};

/// Populates a store with some of the most fundamental Properties and Classes needed to bootstrap the whole.
pub fn populate_base_models(store: &impl Storelike) -> AtomicResult<()> {
    // Start with adding the most fundamental properties - the properties for Properties

    let shortname = Property {
        class_type: None,
        data_type: DataType::Slug,
        shortname: "shortname".into(),
        description: "A short name of something. It can only contain letters, numbers and dashes `-`. Use dashes to denote spaces between words. Not case sensitive - lowercase only. Useful in programming contexts where the user should be able to type something short to identify a specific thing.".into(),
        subject: urls::SHORTNAME.into(),
    }.to_resource()?;
    store.add_resource_unsafe(&shortname)?;

    let description = Property {
        class_type: None,
        data_type: DataType::String,
        shortname: "description".into(),
        description: "A textual description of something. When making a description, make sure that the first few words tell the most important part. Give examples. Since the text supports markdown, you're free to use links and more.".into(),
        subject: urls::DESCRIPTION.into(),
    }.to_resource()?;
    store.add_resource_unsafe(&description)?;

    let is_a = Property {
        class_type: Some(urls::CLASS.into()),
        data_type: DataType::ResourceArray,
        shortname: "is-a".into(),
        description: "A list of Classes of which the thing is an instance of. The Classes of a Resource determine which Properties are recommended and required.".into(),
        subject: urls::IS_A.into(),
    }.to_resource()?;
    store.add_resource_unsafe(&is_a)?;

    let datatype = Property {
        class_type: Some(urls::DATATYPE_CLASS.into()),
        data_type: DataType::AtomicUrl,
        shortname: "datatype".into(),
        description: "The Datatype of a property, such as String or Timestamp.".into(),
        subject: urls::DATATYPE_PROP.into(),
    }
    .to_resource()?;
    store.add_resource_unsafe(&datatype)?;

    let classtype = Property {
        class_type: Some(urls::CLASS.into()),
        data_type: DataType::AtomicUrl,
        shortname: "classtype".into(),
        description:
            "The class-type indicates that the Atomic URL should be an instance of this class."
                .into(),
        subject: urls::CLASSTYPE_PROP.into(),
    }
    .to_resource()?;
    store.add_resource_unsafe(&classtype)?;

    let property = Class {
        requires: vec![urls::SHORTNAME.into()],
        recommends: vec![],
        shortname: "property".into(),
        description: "A Property is a single field in a Class. It's the thing that a property field in an Atom points to. An example is `birthdate`. An instance of Property requires various Properties, most notably a `datatype` (e.g. `string` or `integer`), a human readable `description` (such as the thing you're reading), and a `shortname`.".into(),
        subject: urls::PROPERTY.into(),
    }
    .to_resource()?;
    store.add_resource_unsafe(&property)?;

    let class = Class {
        requires: vec![urls::SHORTNAME.into(), urls::DESCRIPTION.into()],
        recommends: vec![urls::RECOMMENDS.into(), urls::REQUIRES.into()],
        shortname: "class".into(),
        description: "A Class describes an abstract concept, such as 'Person' or 'Blogpost'. It describes the data shape of data and explains what the thing represents. It is convention to use Uppercase in its URL. Note that in Atomic Data, a Resource can have several Classes - not just a single one.".into(),
        subject: urls::CLASS.into(),
    }
    .to_resource()?;
    store.add_resource_unsafe(&class)?;

    Ok(())
}

/// Imports items from default_store.ad3
/// Might get stuck in infinite loop if populate_base_models is not yet run.
pub fn populate_default(store: &impl Storelike) -> AtomicResult<()> {
    let ad3 = include_str!("../defaults/default_store.ad3");
    let atoms = crate::parse::parse_ad3(&String::from(ad3))?;
    store.add_atoms(atoms)?;
    Ok(())
}

/// Generates some nice collections for a store, such as `/agents` and `/collections`
/// Might get stuck in infinite loop if populate_base_models is not yet run.
pub fn populate_collections(store: &impl Storelike) -> AtomicResult<()> {
    use crate::collections::CollectionBuilder;

    let classes = CollectionBuilder {
        subject: format!("{}classes", store.get_base_url()),
        property: Some(urls::IS_A.into()),
        value: Some(urls::CLASS.into()),
        sort_by: None,
        sort_desc: false,
        page_size: 1000,
        current_page: 0,
    };
    store.add_resource_unsafe(&classes.to_resource(store)?)?;

    let properties = CollectionBuilder {
        subject: format!("{}properties", store.get_base_url()),
        property: Some(urls::IS_A.into()),
        value: Some(urls::PROPERTY.into()),
        sort_by: None,
        sort_desc: false,
        page_size: 1000,
        current_page: 0,
    };
    store.add_resource_unsafe(&properties.to_resource(store)?)?;

    let commits = CollectionBuilder {
        subject: format!("{}commits", store.get_base_url()),
        property: Some(urls::IS_A.into()),
        value: Some(urls::COMMIT.into()),
        sort_by: None,
        sort_desc: false,
        page_size: 1000,
        current_page: 0,
    };
    store.add_resource_unsafe(&commits.to_resource(store)?)?;

    let agents = CollectionBuilder {
        subject: format!("{}agents", store.get_base_url()),
        property: Some(urls::IS_A.into()),
        value: Some(urls::AGENT.into()),
        sort_by: None,
        sort_desc: false,
        page_size: 1000,
        current_page: 0,
    };
    store.add_resource_unsafe(&agents.to_resource(store)?)?;

    let collections = CollectionBuilder {
        subject: format!("{}collections", store.get_base_url()),
        property: Some(urls::IS_A.into()),
        value: Some(urls::COLLECTION.into()),
        sort_by: None,
        sort_desc: false,
        page_size: 1000,
        current_page: 0,
    };
    store.add_resource_unsafe(&collections.to_resource(store)?)?;

    Ok(())
}
