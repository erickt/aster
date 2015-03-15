#![feature(rustc_private)]

extern crate aster;
extern crate syntax;

use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;

use aster::AstBuilder;

#[test]
fn test_path() {
    let builder = AstBuilder::new();
    let ty = builder.ty().isize();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyPath(None, builder.path().id("isize").build()),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_option() {
    let builder = AstBuilder::new();
    let ty = builder.ty().option().isize();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyPath(
                None,
                builder.path().global()
                    .id("std")
                    .id("option")
                    .segment("Option")
                        .with_ty(builder.ty().id("isize"))
                        .build()
                    .build()
            ),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_result() {
    let builder = AstBuilder::new();
    let ty = builder.ty().result().isize().isize();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyPath(
                None,
                builder.path().global()
                    .id("std")
                    .id("result")
                    .segment("Result")
                        .with_ty(builder.ty().id("isize"))
                        .with_ty(builder.ty().id("isize"))
                        .build()
                    .build()
            ),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_unit() {
    let builder = AstBuilder::new();
    let ty = builder.ty().unit();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyTup(vec![]),
            span: DUMMY_SP,
        })
    );

    let ty = builder.ty().tuple().build();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyTup(vec![]),
            span: DUMMY_SP,
        })
    );
}

#[test]
fn test_tuple() {
    let builder = AstBuilder::new();
    let ty = builder.ty()
        .tuple()
            .ty().isize()
            .ty().tuple()
                .ty().unit()
                .ty().isize()
            .build()
        .build();

    assert_eq!(
        ty,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyTup(vec![
                builder.ty().isize(),
                P(ast::Ty {
                    id: ast::DUMMY_NODE_ID,
                    node: ast::TyTup(vec![
                        builder.ty().unit(),
                        builder.ty().isize(),
                    ]),
                    span: DUMMY_SP,
                }),
            ]),
            span: DUMMY_SP,
        })
    );
}