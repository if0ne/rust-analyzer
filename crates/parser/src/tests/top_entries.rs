use expect_test::expect;

use crate::TopEntryPoint;

#[test]
fn source_file() {
    check(
        TopEntryPoint::SourceFile,
        "",
        expect![[r#"
        SOURCE_FILE
    "#]],
    );

    check(
        TopEntryPoint::SourceFile,
        "struct S;",
        expect![[r#"
        SOURCE_FILE
          STRUCT
            STRUCT_KW "struct"
            WHITESPACE " "
            NAME
              IDENT "S"
            SEMICOLON ";"
    "#]],
    );

    check(
        TopEntryPoint::SourceFile,
        "@error@",
        expect![[r#"
        SOURCE_FILE
          ERROR
            AT "@"
          MACRO_CALL
            PATH
              PATH_SEGMENT
                NAME_REF
                  IDENT "error"
          ERROR
            AT "@"
        error 0: expected an item
        error 6: expected BANG
        error 6: expected `{`, `[`, `(`
        error 6: expected SEMICOLON
        error 6: expected an item
    "#]],
    );
}

#[test]
fn macro_stmt() {
    check(
        TopEntryPoint::MacroStmts,
        "#!/usr/bin/rust",
        expect![[r##"
            MACRO_STMTS
              ERROR
                SHEBANG "#!/usr/bin/rust"
            error 0: expected expression
        "##]],
    );
    check(
        TopEntryPoint::MacroStmts,
        "let x = 1 2 struct S;",
        expect![[r#"
            MACRO_STMTS
              LET_STMT
                LET_KW "let"
                WHITESPACE " "
                IDENT_PAT
                  NAME
                    IDENT "x"
                WHITESPACE " "
                EQ "="
                WHITESPACE " "
                LITERAL
                  INT_NUMBER "1"
              WHITESPACE " "
              EXPR_STMT
                LITERAL
                  INT_NUMBER "2"
              WHITESPACE " "
              STRUCT
                STRUCT_KW "struct"
                WHITESPACE " "
                NAME
                  IDENT "S"
                SEMICOLON ";"
        "#]],
    );
}

#[track_caller]
fn check(entry: TopEntryPoint, input: &str, expect: expect_test::Expect) {
    let (parsed, _errors) = super::parse(entry, input);
    expect.assert_eq(&parsed)
}
