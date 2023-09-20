// slang_nanopass! {

//     ( extends super::nanopass_regalloc )

//     ( struct foo (x Int), (y RegisterData) )
//     ( enum bar A (B Bool) (C Option(Vec(Int))) )

//     ( remove IfStatement "*Definition" (Model field) )
//     ( upsert ("*Type" register_data) (FunctionDefinition register_data) = RegisterData )

// }

#[macro_export]
macro_rules! slang_nanopass {

    (
        ( extends $incoming_nanopass:ident )
        $( ( $($clause:tt)* ) )*
    ) => {
        #[allow(unused_imports)]
        use $crate::type_model::{TypeModel, NamedType, AnonymousType::*};

        use lazy_static::lazy_static;

        lazy_static! {
            static ref TYPE_MODEL: TypeModel = {
                let mut model = $incoming_nanopass::TYPE_MODEL.clone();
                $( slang_nanopass!{ @clause model $($clause)* } );*
                model
            };
        }
    };

    // ToString

    (@to_string $x:ident) => { stringify!($x) };
    (@to_string $x:literal) => { $x };

    // Clause

    ( @clause $model:ident struct $name:ident $( ( $field_name:ident $field_type:expr ) )* ) => {
        $model.upsert_type(stringify!($name), NamedType::Struct(&[
            $( (stringify!($field_name), $field_type) ),*
        ]));
    };

    ( @clause $model:ident enum $name:ident $( ( $field_name:ident $($field_type:expr)? ) )* ) => {
        $model.upsert_type(stringify!($name), NamedType::Enum(&[
            $( (stringify!($field_name), slang_nanopass! { @clause_optional_field_type $($field_type)? }) ),*
        ]));
    };
    (@clause_optional_field_type) => { Void };
    (@clause_optional_field_type $field_type:expr) => { $field_type };

    ( @clause $model:ident type $name:ident = $type:expr ) => {
        model.upsert_type("", NamedType::Alias($type));
    };

    ( @clause $model:ident upsert $($rest:tt)* ) => {
        slang_nanopass! { @upsert $model $($rest)* }
    };

    ( @clause $model:ident remove $($rest:tt)* ) => {
        slang_nanopass! { @remove $model $($rest)* }
    };

    // Upsert

    ( @upsert_get_type = $type:expr ) => { $type };
    ( @upsert_get_type $head:tt $($rest:tt)* ) => { slang_nanopass! { @upsert_get_type $($rest)* } };

    ( @upsert_doit $model:ident $type:expr ; = $type_:expr ) => {};
    ( @upsert_doit $model:ident $type:expr ; ( $name:tt $field_name:tt ) $($rest:tt)* ) => {
        $model.upsert_field(
            slang_nanopass! { @to_string $name },
            slang_nanopass! { @to_string $field_name },
            $type
        );
        slang_nanopass! { @upsert_doit $model $type ; $($rest)* }
    };

    ( @upsert_doit $model:ident $type:expr ; $name:tt $($rest:tt)* ) => {
        $model.upsert_type(
            slang_nanopass!{ @to_string $name },
            $type
        );
        slang_nanopass! { @upsert_doit $model $type ; $($rest)* }
    };
    ( @upsert $model:ident $($rest:tt)* ) => {
        slang_nanopass! { @upsert_doit $model slang_nanopass! { @upsert_get_type $($rest)* } ; $($rest)* }
    } ;

    // Remove

    ( @remove $model:ident ( $name:tt $field_name:tt ) $($rest:tt)* ) => {
        $model.remove_field(
            slang_nanopass! { @to_string $name },
            slang_nanopass! { @to_string $field_name },
        );
        slang_nanopass! { @remove $model $($rest)* }
    };

    ( @remove $model:ident $name:tt $($rest:tt)* ) => {
        $model.remove_type(slang_nanopass! { @to_string $name });
        slang_nanopass! { @remove $model $($rest)* }
    };

    ( @remove $model:ident ) => {};
}

struct RegisterData {
    allocs: u8,
}

slang_nanopass! {

    ( extends nanopass_regalloc )

    ( struct Foo (x Int) (y External("RegisterData")) )
    ( enum Bar (a) (b Bool) (c Option(Vec(Int))) )

    ( remove IfStatement "*Definition" (Model field) )
    ( upsert ("*Type" register_data) (FunctionDefinition register_data) = External("RegisterData") )

}

mod nanopass_regalloc {
    use crate::type_model::TypeModel;

    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref TYPE_MODEL: TypeModel = TypeModel::default();
    }
}
