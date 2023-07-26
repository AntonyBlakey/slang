#[macro_export]
macro_rules! slang_grammar {

    ( $( $($idents:ident)+ = $value:tt ; )* ) => {
        $(
            slang_grammar_top_level_clause! { $($idents)+ $value }
        )*

        pub trait GrammarConstructor {
            fn new() -> std::rc::Rc<$crate::Grammar>;
        }

        impl GrammarConstructor for $crate::Grammar {
            fn new() -> std::rc::Rc<$crate::Grammar> {
                let mut grammar = Self {
                    elements: std::collections::HashMap::new(),
                    // TODO: capture these from the macro
                    name: "Solidity".into(),
                    versions: [
                        "0.4.11" , "0.4.12" , "0.4.13" , "0.4.14" , "0.4.15" , "0.4.16" , "0.4.17" , "0.4.18" , "0.4.19" , "0.4.20" , "0.4.21" , "0.4.22" , "0.4.23" ,
                        "0.4.24" , "0.4.25" , "0.4.26" , "0.5.0" , "0.5.1" , "0.5.2" , "0.5.3" , "0.5.4" , "0.5.5" , "0.5.6" , "0.5.7" , "0.5.8" , "0.5.9" , "0.5.10" ,
                        "0.5.11" , "0.5.12" , "0.5.13" , "0.5.14" , "0.5.15" , "0.5.16" , "0.5.17" , "0.6.0" , "0.6.1" , "0.6.2" , "0.6.3" , "0.6.4" , "0.6.5" , "0.6.6" ,
                        "0.6.7" , "0.6.8" , "0.6.9" , "0.6.10" , "0.6.11" , "0.6.12" , "0.7.0" , "0.7.1" , "0.7.2" , "0.7.3" , "0.7.4" , "0.7.5" , "0.7.6" , "0.8.0" ,
                        "0.8.1" , "0.8.2" , "0.8.3" , "0.8.4" , "0.8.5" , "0.8.6" , "0.8.7" , "0.8.8" , "0.8.9" , "0.8.10" , "0.8.11" , "0.8.12" , "0.8.13" , "0.8.14" ,
                        "0.8.15" , "0.8.16" , "0.8.17" , "0.8.18" , "0.8.19"
                     ].iter().map(|s| semver::Version::parse(s).unwrap()).collect(),
                };
                $(
                    slang_grammar_register_definitions! { grammar $($idents)+ $value }
                )*
                std::rc::Rc::new(grammar)
            }
        }
    };

}
#[macro_export]
macro_rules! slang_grammar_top_level_clause {

    (config $value:tt) => { /* Not Yet Implemented */ };
    (scanner $name:ident $value:tt) => {
        slang_grammar_definition!{
            None
            ScannerDefinition
            ScannerDefinitionRef
            ScannerDefinitionNode
            slang_scanner
            None
            $name
            $value
        }
    };
    (lexical context $name:ident { $( $($idents:ident)+ = $value:tt ; )* } ) => {
        $(
            slang_grammar_lexical_context! {
                $name
                $($idents)+ $value
            }
        )*
    };
}

#[macro_export]
macro_rules! slang_grammar_lexical_context {
    ($context:ident config  $value:tt) => {
        /* Not Yet Implemented */
    };
    ($context:ident trivia parser $name:ident $value:tt) => {
        slang_grammar_definition! {
            $context
            TriviaParserDefinition
            TriviaParserDefinitionRef
            ParserDefinitionNode
            slang_parser
            None
            $name
            $value
        }
    };
    ($context:ident inline parser $name:ident $value:tt) => {
        slang_grammar_definition! {
            $context
            ParserDefinition
            ParserDefinitionRef
            ParserDefinitionNode
            slang_parser
            (true)
            $name
            $value
        }
    };
    ($context:ident parser $name:ident $value:tt) => {
        slang_grammar_definition! {
            $context
            ParserDefinition
            ParserDefinitionRef
            ParserDefinitionNode
            slang_parser
            (false)
            $name
            $value
        }
    };
    ($context:ident precedence parser $name:ident $value:tt) => {
        slang_grammar_definition! {
            $context
            PrecedenceParserDefinition
            PrecedenceParserDefinitionRef
            PrecedenceParserDefinitionNode
            slang_precedence_parser
            None
            $name
            $value
        }
    };
}

#[macro_export]
macro_rules! slang_grammar_definition {
    ($context:ident $trait:ident $trait_ref:ident $node_type:ident $dsl_macro:ident $is_inline:tt $name:ident $value:tt) => {
        #[derive(Debug)]
        struct $name {
            node: once_cell::unsync::OnceCell<$crate::$node_type>,
        }

        impl $name {
            const SOURCE_LOCATION: $crate::SourceLocation = slang_location!();
            const NAME: &str = stringify!($name);
            const INSTANCE: once_cell::unsync::OnceCell<std::rc::Rc<Self>> =
                once_cell::unsync::OnceCell::new();
            fn instance() -> $crate::$trait_ref {
                Self::INSTANCE
                    .get_or_init(|| {
                        std::rc::Rc::new(Self {
                            node: once_cell::unsync::OnceCell::new(),
                        })
                    })
                    .clone()
            }
        }

        impl $crate::$trait for $name {
            fn name(&self) -> &'static str {
                Self::NAME
            }
            fn source_location(&self) -> $crate::SourceLocation {
                Self::SOURCE_LOCATION
            }
            fn node(&self) -> &$crate::$node_type {
                &self.node.get_or_init(|| $dsl_macro!($value))
            }
            slang_grammar_definition! { @context_method $context }
            slang_grammar_definition! { @is_inline_method $is_inline }
        }
    };

    (@context_method None ) => {};
    (@context_method $context:ident ) => {
        fn context(&self) -> &'static str {
            stringify!($context)
        }
    };

    (@is_inline_method None ) => {};
    (@is_inline_method ($is_inline:literal) ) => {
        fn is_inline(&self) -> bool {
            $is_inline
        }
    };
}

#[macro_export]
macro_rules! slang_grammar_register_definitions {

    ($grammar:ident lexical context $name:ident { $( $($idents:ident)+ = $value:tt ; )* } ) => {
        $(
            slang_grammar_register_definitions! { $grammar $($idents)+ $value }
        )*
    };
    ($grammar:ident config $value:tt) => {};
    ($grammar:ident $(inline)? $(trivia)? $(precedence)? parser $name:ident $value:tt) => {
        $grammar.register($name::instance());
    };
    ($grammar:ident scanner $name:ident $value:tt) => {
        $grammar.register($name::instance());
    };

}

#[macro_export]
macro_rules! slang_location {
    // Meaningless in macro_rules! because the locations are all wrong.
    () => {
        $crate::SourceLocation {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

#[macro_export]
macro_rules! slang_parser {

    ( $x:ident ) => {
        ($x::instance(), slang_location!()).into()
    };

    (( $x:tt ? )) => {
        $crate::ParserDefinitionNode::Optional(
            Box::new(slang_parser!($x)) ,
            slang_location!()
        )
    };
    (( $x:tt * )) => {
        $crate::ParserDefinitionNode::ZeroOrMore(
            Box::new(slang_parser!($x)) ,
            slang_location!()
        )
    };
    (( $x:tt + )) => {
        $crate::ParserDefinitionNode::OneOrMore(
            Box::new(slang_parser!($x)) ,
            slang_location!()
        )
    };

    (( $b:tt delimited_by $o:tt and $c:tt )) => {
        $crate::ParserDefinitionNode::DelimitedBy(
            Box::new(slang_parser!($o)),
            Box::new(slang_parser!($b)),
            Box::new(slang_parser!($c)),
            slang_location!()
        )
    };
    (( $b:tt terminated_by $t:tt )) => {
        $crate::ParserDefinitionNode::TerminatedBy(
            Box::new(slang_parser!($b)),
            Box::new(slang_parser!($t)),
            slang_location!()
        )
    };
    (( $b:tt separated_by $s:tt )) => {
        $crate::ParserDefinitionNode::SeparatedBy(
            Box::new(slang_parser!($b)),
            Box::new(slang_parser!($s)),
            slang_location!()
        )
    };

    (( $first:tt | $($rest:tt)|+ )) => {
        $crate::ParserDefinitionNode::Choice(
            vec![slang_parser!($first), $(slang_parser!($rest)),+],
            slang_location!()
        )
    };

    (( $x:tt )) => {
        slang_parser!($x)
    };

    (( $($rest:tt)+ )) => {
        $crate::ParserDefinitionNode::Sequence(
            vec![$(slang_parser!($rest)),+],
            slang_location!()
        )
    };

    ({ $($rest:tt)+ }) => {
        slang_dsl_versioned!([ ParserDefinitionNode slang_parser [] ] $($rest)+)
    };

}

#[macro_export]
macro_rules! slang_precedence_parser {

    (( [ $($operators:tt)+ ] with primary expression $primary:tt )) => {
        $crate::PrecedenceParserDefinitionNode {
            primary_expression: Box::new(slang_parser!($primary)),
            operators: slang_precedence_parser_operators!([ [] [] ] $($operators)+),
            source_location: slang_location!()
        }
    };

}

#[macro_export]
macro_rules! slang_precedence_parser_operators {

    ([ [ $($operator:expr),* ] $versions:tt ] right $operator_parser:ident as $name:ident $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                [
                    $($operator,)*
                    (
                        vec! $versions,
                        $crate::PrecedenceOperatorModel::BinaryRightAssociative,
                        stringify!($name),
                        $operator_parser::instance()
                    )
                ]
                []
            ]
            $($($rest)*)?
        )
    };
    ([ [ $($operator:expr),* ] $versions:tt ] left $operator_parser:ident as $name:ident $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                [
                    $($operator,)*
                    (
                        vec! $versions,
                        $crate::PrecedenceOperatorModel::BinaryLeftAssociative,
                        stringify!($name),
                        $operator_parser::instance()
                    )
                ]
                []
            ]
            $($($rest)*)?
        )
    };
    ([ [ $($operator:expr),* ] $versions:tt ] prefix $operator_parser:ident as $name:ident $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                [
                    $($operator,)*
                    (
                        vec! $versions,
                        $crate::PrecedenceOperatorModel::Prefix,
                        stringify!($name),
                        $operator_parser::instance()
                    )
                ]
                []
            ]
            $($($rest)*)?
        )
    };
    ([ [ $($operator:expr),* ] $versions:tt ] postfix $operator_parser:ident as $name:ident $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                [
                    $($operator,)*
                    (
                        vec! $versions,
                        $crate::PrecedenceOperatorModel::Postfix,
                        stringify!($name),
                        $operator_parser::instance()
                    )
                ]
                []
            ]
            $($($rest)*)?
        )
    };
    // These do just enough to handle solidity 0.4.11 .. 0.8.20
    // TODO: fully general with `... and` syntax as in `slang_parser!`
    ([ $operators:tt [ $($version:tt),* ] ] { enabled from $from:literal $mode:ident $operator:ident as $name:ident } $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                $operators
                [
                    $($version,)*
                    {
                        $crate::VersionQualityRange {
                            from: (semver::Version::parse($from).unwrap(), slang_location!()),
                            quality: ($crate::VersionQuality::Enabled, slang_location!())
                        }
                    }
                ]
            ]
            $mode $operator as $name $(, $($rest)*)?)
    };
    ([ $operators:tt [ $($version:tt),* ]]  { disabled from $from:literal $mode:ident $operator:ident as $name:ident } $(, $($rest:tt)*)?) => {
        slang_precedence_parser_operators!(
            [
                $operators
                [
                    $($version,)*
                    {
                        $crate::VersionQualityRange {
                            from: (semver::Version::parse($from).unwrap(), slang_location!()),
                            quality: ($crate::VersionQuality::Disabled, slang_location!())
                        }
                    }
                ]
            ]
            $mode $operator as $name
            $(, $($rest)*)?)
    };

    ([ $operators:tt [] ]) => { vec! $operators };

}

#[macro_export]
macro_rules! slang_scanner {

    ( $x:literal ) => {
        $crate::ScannerDefinitionNode::Literal(
            $x.to_string(),
            slang_location!()
        )
    };
    ( $x:ident ) => {
        ($x::instance(), slang_location!()).into()
    };

    (( ! $x:literal )) => {
        $crate::ScannerDefinitionNode::NoneOf(
            $x.to_string(),
            slang_location!()
        )
    };
    (( $x:tt ? )) => {
        $crate::ScannerDefinitionNode::Optional(
            Box::new(slang_scanner!($x)) ,
            slang_location!()
        )
    };
    (( $x:tt * )) => {
        $crate::ScannerDefinitionNode::ZeroOrMore(
            Box::new(slang_scanner!($x)) ,
            slang_location!()
        )
    };
    (( $x:tt + )) => {
        $crate::ScannerDefinitionNode::OneOrMore(
            Box::new(slang_scanner!($x)) ,
            slang_location!()
        )
    };

    (( $x:literal .. $y:literal )) => {
        $crate::ScannerDefinitionNode::CharRange(
            $x, $y,
            slang_location!()
        )
    };

    (( $first:tt | $($rest:tt)|+ )) => {
        $crate::ScannerDefinitionNode::Choice(
            vec![slang_scanner!($first), $(slang_scanner!($rest)),+],
            slang_location!()
        )
    };

    (( $x:tt )) => {
        slang_scanner!($x)
    };

    (( $($rest:tt)+ )) => {
        $crate::ScannerDefinitionNode::Sequence(
            vec![$(slang_scanner!($rest)),+],
            slang_location!()
        )
    };

    ({ $($rest:tt)+ }) => {
        slang_dsl_versioned!([ ScannerDefinitionNode slang_scanner [] ] $($rest)+)
    };

}

#[macro_export]
macro_rules! slang_dsl_versioned {

    // The qualities are explicit to avoid ambiguity and hence give better error messages
    ($args:tt enabled $($rest:tt)+) => {
        slang_dsl_versioned!($args Enabled $($rest)+)
    };
    ($args:tt disabled $($rest:tt)+) => {
        slang_dsl_versioned!($args Disabled $($rest)+)
    };

    ($args:tt and $($rest:tt)+) => {
        slang_dsl_versioned!($args $($rest)+)
    };

    ([ $node_type:ident $dsl_macro:ident [ $($accum:tt),* ] ] $quality:ident from $from:literal $($rest:tt)+) => {
        slang_dsl_versioned!(
            [
                $node_type
                $dsl_macro
                [
                    $($accum,)*
                    {
                        $crate::VersionQualityRange {
                            from: (semver::Version::parse($from).unwrap(), slang_location!()),
                            quality: ($crate::VersionQuality::$quality, slang_location!())
                        }
                    }
                ]
            ]
            $($rest)+
        )
    };
    ([ $node_type:ident $dsl_macro:ident $accum:tt ] $($rest:tt)+) => {
        $crate::$node_type::Versioned(
            Box::new($dsl_macro!($($rest)+)),
            vec! $accum,
            slang_location!()
        )
    };

}
