#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Bar,
                    fields = (
                        field_1 = Required(Baz),
                        field_1 = Required(Baz),
                        field_3 = Required(Baz)
                    )
                ),
                Token(
                    name = Baz,
                    definitions = [TokenDefinition(scanner = Atom("baz"))]
                )
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
