//! Built-in macro definitions.

use crate::macros::pattern::*;
use crate::macros::*;

use crate::syntax::operator;



// =======================
// === Built-in macros ===
// =======================

/// All built-in macro definitions.
pub fn all() -> resolver::SegmentMap<'static> {
    let mut macro_map = resolver::SegmentMap::default();
    macro_map.register(if_then());
    macro_map.register(if_then_else());
    register_import_macros(&mut macro_map);
    register_export_macros(&mut macro_map);
    macro_map.register(group());
    macro_map.register(type_def());
    macro_map.register(lambda());
    macro_map.register(case());
    macro_map.register(array());
    macro_map.register(tuple());
    macro_map
}

fn register_import_macros(macros: &mut resolver::SegmentMap<'_>) {
    use crate::macro_definition;
    let defs = [
        macro_definition! {("import", everything()) import_body},
        macro_definition! {("import", everything(), "as", everything()) import_body},
        macro_definition! {("import", everything(), "hiding", everything()) import_body},
        macro_definition! {("polyglot", everything(), "import", everything()) import_body},
        macro_definition! {
        ("polyglot", everything(), "import", everything(), "as", everything()) import_body},
        macro_definition! {
        ("polyglot", everything(), "import", everything(), "hiding", everything()) import_body},
        macro_definition! {
        ("from", everything(), "import", everything(), "hiding", everything()) import_body},
        macro_definition! {
        ("from", everything(), "as", everything(), "import", everything()) import_body},
        macro_definition! {("from", everything(), "import", everything()) import_body},
    ];
    for def in defs {
        macros.register(def);
    }
}

fn import_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    use operator::resolve_operator_precedence_if_non_empty;
    let mut polyglot = None;
    let mut from = None;
    let mut from_as = None;
    let mut import = None;
    let mut import_as = None;
    let mut hiding = None;
    for segment in segments {
        let header = segment.header;
        let body = resolve_operator_precedence_if_non_empty(segment.result.tokens());
        let field = match header.code.as_ref() {
            "polyglot" => &mut polyglot,
            "from" => &mut from,
            "as" if import.is_none() => &mut from_as,
            "import" => &mut import,
            "as" => &mut import_as,
            "hiding" => &mut hiding,
            _ => unreachable!(),
        };
        *field = Some(syntax::tree::MultiSegmentAppSegment { header, body });
    }
    let import = import.unwrap();
    syntax::Tree::import(polyglot, from, from_as, import, import_as, hiding)
}

fn register_export_macros(macros: &mut resolver::SegmentMap<'_>) {
    use crate::macro_definition;
    let defs = [
        macro_definition! {("export", everything()) export_body},
        macro_definition! {("export", everything(), "as", everything()) export_body},
        macro_definition! {("from", everything(), "export", everything()) export_body},
        macro_definition! {
        ("from", everything(), "export", everything(), "hiding", everything()) export_body},
        macro_definition! {
        ("from", everything(), "as", everything(), "export", everything()) export_body},
    ];
    for def in defs {
        macros.register(def);
    }
}

fn export_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    use operator::resolve_operator_precedence_if_non_empty;
    let mut from = None;
    let mut from_as = None;
    let mut export = None;
    let mut export_as = None;
    let mut hiding = None;
    for segment in segments {
        let header = segment.header;
        let body = resolve_operator_precedence_if_non_empty(segment.result.tokens());
        let field = match header.code.as_ref() {
            "from" => &mut from,
            "as" if export.is_none() => &mut from_as,
            "export" => &mut export,
            "as" => &mut export_as,
            "hiding" => &mut hiding,
            _ => unreachable!(),
        };
        *field = Some(syntax::tree::MultiSegmentAppSegment { header, body });
    }
    let export = export.unwrap();
    syntax::Tree::export(from, from_as, export, export_as, hiding)
}

/// If-then-else macro definition.
pub fn if_then_else<'s>() -> Definition<'s> {
    crate::macro_definition! {("if", everything(), "then", everything(), "else", everything())}
}

/// If-then macro definition.
pub fn if_then<'s>() -> Definition<'s> {
    crate::macro_definition! {("if", everything(), "then", everything())}
}

/// Group macro definition.
pub fn group<'s>() -> Definition<'s> {
    crate::macro_definition! {("(", everything(), ")", nothing()) group_body}
}

fn group_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    use operator::resolve_operator_precedence_if_non_empty;
    use syntax::token;
    macro_rules! into_symbol {
        ($token:expr) => {{
            let token::Token { left_offset, code, .. } = $token;
            token::symbol(left_offset, code)
        }};
    }
    let (close, mut segments) = segments.pop();
    let close = into_symbol!(close.header);
    let segment = segments.pop().unwrap();
    let open = into_symbol!(segment.header);
    let body = segment.result.tokens();
    let body = resolve_operator_precedence_if_non_empty(body);
    syntax::Tree::group(open, body, close)
}

/// New type definition macro definition.
pub fn type_def<'s>() -> Definition<'s> {
    use pattern::*;
    #[rustfmt::skip]
    let pattern = 
        identifier() / "name" % "type name" >>
        many(identifier() % "type parameter" / "param") % "type parameters" >>
        block(
            everything() / "statements"
        ) % "type definition body";
    crate::macro_definition! {
        ("type", pattern)
        type_def_body
    }
}

fn type_def_body(matched_segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    // FIXME: This implementation of parsing constructors works for correct inputs, but doesn't
    //  handle incorrect syntax ideally. Issue: #182745069
    let segment = matched_segments.pop().0;
    let match_tree = segment.result.into_var_map();
    let mut v = match_tree.view();
    let name = v.query("name").map(|name| name[0].clone()).unwrap_or_default();
    let name = operator::resolve_operator_precedence_if_non_empty(name);
    let no_params = [];
    let params = v.nested().query("param").unwrap_or(&no_params);
    let params = params
        .iter()
        .map(|tokens| {
            operator::resolve_operator_precedence_if_non_empty(tokens.iter().cloned()).unwrap()
        })
        .collect_vec();
    let mut constructors = default();
    let mut body = default();
    if let Some(items) = v.query("statements") {
        let items = items[0].iter().cloned();
        let mut builder = TypeDefBodyBuilder::default();
        for syntax::tree::block::Line { newline, expression } in syntax::tree::block::lines(items) {
            builder.line(newline, expression);
        }
        let (constructors_, body_) = builder.finish();
        constructors = constructors_;
        body = body_;
    }
    match name {
        Some(name) => syntax::Tree::type_def(segment.header, name, params, constructors, body),
        None => {
            let name = syntax::Tree::ident(syntax::token::ident("", "", false, 0, false));
            let result = syntax::Tree::type_def(segment.header, name, params, constructors, body);
            result.with_error("Expected identifier after `type` keyword.")
        }
    }
}

#[derive(Default)]
struct TypeDefBodyBuilder<'s> {
    constructors: Vec<syntax::tree::TypeConstructorLine<'s>>,
    body:         Vec<syntax::tree::block::Line<'s>>,
}

impl<'s> TypeDefBodyBuilder<'s> {
    /// Apply the line to the state.
    pub fn line(
        &mut self,
        newline: syntax::token::Newline<'s>,
        expression: Option<syntax::Tree<'s>>,
    ) {
        if self.body.is_empty() {
            if let Some(expression) = expression {
                match Self::to_constructor_line(expression) {
                    Ok(expression) => {
                        let expression = Some(expression);
                        let line = syntax::tree::TypeConstructorLine { newline, expression };
                        self.constructors.push(line);
                    }
                    Err(expression) => {
                        let expression = crate::expression_to_statement(expression);
                        let expression = Some(expression);
                        self.body.push(syntax::tree::block::Line { newline, expression });
                    }
                }
            } else {
                self.constructors.push(newline.into());
            }
        } else {
            let expression = expression.map(crate::expression_to_statement);
            self.body.push(syntax::tree::block::Line { newline, expression });
        }
    }

    /// Return the constructor/body sequences.
    pub fn finish(
        self,
    ) -> (Vec<syntax::tree::TypeConstructorLine<'s>>, Vec<syntax::tree::block::Line<'s>>) {
        (self.constructors, self.body)
    }

    /// Interpret the given expression as an `TypeConstructorDef`, if its syntax is compatible.
    fn to_constructor_line(
        expression: syntax::Tree<'_>,
    ) -> Result<syntax::tree::TypeConstructorDef<'_>, syntax::Tree<'_>> {
        use syntax::tree::*;
        if let Tree {
            variant:
                box Variant::ArgumentBlockApplication(ArgumentBlockApplication {
                    lhs: Some(Tree { variant: box Variant::Ident(ident), span: span_ }),
                    arguments,
                }),
            span,
        } = expression
        {
            let mut constructor = ident.token;
            let mut left_offset = span.left_offset;
            left_offset += &span_.left_offset;
            left_offset += constructor.left_offset;
            constructor.left_offset = left_offset;
            let block = arguments;
            let arguments = default();
            return Ok(TypeConstructorDef { constructor, arguments, block });
        }
        let mut arguments = vec![];
        let mut lhs = &expression;
        let mut left_offset = crate::source::span::Offset::default();
        while let Tree { variant: box Variant::App(App { func, arg }), span } = lhs {
            left_offset += &span.left_offset;
            lhs = func;
            arguments.push(arg.clone());
        }
        if let Tree { variant: box Variant::Ident(Ident { token }), span } = lhs {
            let mut constructor = token.clone();
            left_offset += &span.left_offset;
            left_offset += constructor.left_offset;
            constructor.left_offset = left_offset;
            arguments.reverse();
            let block = default();
            return Ok(TypeConstructorDef { constructor, arguments, block });
        }
        Err(expression)
    }
}

/// Lambda expression.
///
/// The lambda operator `\` is similar to a unary operator, but is implemented as a macro because it
/// doesn't follow the whitespace precedence rules.
pub fn lambda<'s>() -> Definition<'s> {
    crate::macro_definition! {("\\", everything()) lambda_body}
}

fn lambda_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    use operator::resolve_operator_precedence_if_non_empty;
    let (segment, _) = segments.pop();
    let operator = segment.header;
    let syntax::token::Token { left_offset, code, .. } = operator;
    let properties = syntax::token::OperatorProperties::default();
    let operator = syntax::token::operator(left_offset, code, properties);
    let arrow = segment.result.tokens();
    let arrow = resolve_operator_precedence_if_non_empty(arrow);
    syntax::Tree::lambda(operator, arrow)
}

/// Case expression.
pub fn case<'s>() -> Definition<'s> {
    crate::macro_definition! {("case", everything(), "of", everything()) case_body}
}

fn case_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    use operator::resolve_operator_precedence_if_non_empty;
    use syntax::token;
    use syntax::tree::*;
    let into_ident = |token| {
        let token::Token { left_offset, code, .. } = token;
        token::ident(left_offset, code, false, 0, false)
    };
    let (of, mut rest) = segments.pop();
    let case = rest.pop().unwrap();
    let case_ = into_ident(case.header);
    let expression = case.result.tokens();
    let expression = resolve_operator_precedence_if_non_empty(expression);
    let of_ = into_ident(of.header);
    let body = of.result.tokens();
    let body = resolve_operator_precedence_if_non_empty(body);
    let mut initial = None;
    let mut lines = vec![];
    if let Some(body) = body {
        match body.variant {
            box Variant::ArgumentBlockApplication(ArgumentBlockApplication { lhs, arguments }) => {
                initial = lhs;
                lines = arguments;
                let mut left_offset = body.span.left_offset;
                if let Some(initial) = initial.as_mut() {
                    left_offset += mem::take(&mut initial.span.left_offset);
                    initial.span.left_offset = left_offset;
                } else if let Some(first) = lines.first_mut() {
                    left_offset += mem::take(&mut first.newline.left_offset);
                    first.newline.left_offset = left_offset;
                }
            }
            _ => initial = Some(body),
        }
    }
    Tree::case(case_, expression, of_, initial, lines)
}

/// Array literal.
pub fn array<'s>() -> Definition<'s> {
    crate::macro_definition! {("[", everything(), "]", nothing()) array_body}
}

fn array_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    let GroupedSequence { left, first, rest, right } = grouped_sequence(segments);
    syntax::tree::Tree::array(left, first, rest, right)
}

/// Tuple literal.
pub fn tuple<'s>() -> Definition<'s> {
    crate::macro_definition! {("{", everything(), "}", nothing()) tuple_body}
}

fn tuple_body(segments: NonEmptyVec<MatchedSegment>) -> syntax::Tree {
    let GroupedSequence { left, first, rest, right } = grouped_sequence(segments);
    syntax::tree::Tree::tuple(left, first, rest, right)
}

struct GroupedSequence<'s> {
    left:  syntax::token::Symbol<'s>,
    first: Option<syntax::Tree<'s>>,
    rest:  Vec<syntax::tree::OperatorDelimitedTree<'s>>,
    right: syntax::token::Symbol<'s>,
}

fn grouped_sequence(segments: NonEmptyVec<MatchedSegment>) -> GroupedSequence {
    use operator::resolve_operator_precedence_if_non_empty;
    use syntax::token;
    use syntax::tree::*;
    let into_symbol = |token| {
        let token::Token { left_offset, code, .. } = token;
        token::symbol(left_offset, code)
    };
    let (right, mut rest) = segments.pop();
    let right_ = into_symbol(right.header);
    let left = rest.pop().unwrap();
    let left_ = into_symbol(left.header);
    let expression = left.result.tokens();
    let expression = resolve_operator_precedence_if_non_empty(expression);
    let mut rest = vec![];
    let mut lhs_ = &expression;
    while let Some(Tree {
                       variant: box Variant::OprApp(OprApp { lhs, opr: Ok(opr), rhs: Some(rhs) }), ..
                   }) = lhs_ && opr.properties.is_sequence() {
        lhs_ = lhs;
        let operator = opr.clone();
        let body = rhs.clone();
        rest.push(OperatorDelimitedTree { operator, body });
    }
    let first = lhs_.clone();
    GroupedSequence { left: left_, first, rest, right: right_ }
}
