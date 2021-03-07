use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    Path, Type, TypePath,
};

#[derive(Debug)]
struct Entity {
    name: String,
    fields: Vec<EntityField>,
}

#[derive(Debug)]
struct EntityField {
    name: String,
    ty: String,
}

fn get_entity_field(field: &Field) -> Option<EntityField> {
    let ident = match &field.ident {
        Some(id) => Some(format!("{}", id)),
        None => {
            return None;
        }
    };

    let ty_ident = match &field.ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => segments.first().and_then(|s| Some(format!("{}", s.ident))),
        _ => {
            return None;
        }
    };
    let entity_field = EntityField {
        name: ident.unwrap(),
        ty: ty_ident.unwrap(),
    };
    Some(entity_field)
    
}

#[proc_macro_derive(Entity)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!("This is not supported.")
    };
    let entity = Entity {
        name: format!("{}", ident),
        fields: fields.iter().filter_map(|field| get_entity_field(field)).collect(),
    };
    let fields: Vec<String> = entity.fields.iter().map(|f| f.name.to_string()).collect();
    let entity_name = entity.name;

    let result = quote! {
        impl #ident {
            pub fn select() -> ::entity::Select {
                ::entity::Select { entity: #entity_name, columns: vec![#(#fields),*], ..::std::default::Default::default() }
            }
        }
    };
    result.into()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {
        #[derive(Entity)]
        struct Books {
            id: i32,
            title: String,
        }
        let mut query = Entity::select();
        query.set_limit(200).set_columns(vec!["id", "title"]).set_unique();
        assert_eq!(query.to_sql(), "selectdistinct id,title from Books limit 300;");
    }
}
