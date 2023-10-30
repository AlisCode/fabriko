use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{discouraged::Speculative, Parse},
    Expr, Ident,
};

pub(crate) fn do_create(item: TokenStream) -> TokenStream {
    let context: CreateDeclaration =
        syn::parse2(item).expect("Failed to parse create command. See usage.");
    dbg!(context);
    quote!()
}

impl Parse for CreateDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let context = input.parse()?;
        // let _comma = input.parse()?;
        let instructions = input.parse()?;
        Ok(CreateDeclaration {
            context,
            // _comma,
            instructions,
        })
    }
}

#[derive(Debug)]
struct CreateDeclaration {
    context: Ident,
    // _comma: syn::token::Comma,
    instructions: CreateInstructions,
}

impl Parse for CreateInstructions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut instructions = Vec::default();
        loop {
            match input.parse() {
                Ok(instr) => instructions.push(instr),
                Err(_e) => {
                    break;
                }
            }
        }
        Ok(CreateInstructions(instructions))
    }
}

#[derive(Debug)]
struct CreateInstructions(Vec<CreateInstruction>);

impl Parse for CreateInstruction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _dot_token = input.parse()?;
        let ident = input.parse()?;
        let kind = input.parse()?;
        Ok(CreateInstruction {
            _dot_token,
            ident,
            kind,
        })
    }
}

#[derive(Debug)]
struct CreateInstruction {
    _dot_token: syn::token::Dot,
    ident: Ident,
    kind: CreateInstructionKind,
}

impl Parse for CreateInstructionKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let forked = input.fork();
        let content;
        syn::braced!(content in forked);
        match content.parse::<CreateInstruction>() {
            Ok(nested) => {
                input.advance_to(&forked);
                return Ok(CreateInstructionKind::Nest(Box::new(nested)));
            }
            Err(_e) => {}
        }
        forked.parse().map(|expr: Expr| {
            input.advance_to(&forked);
            CreateInstructionKind::SetValue(expr)
        })
    }
}

#[derive(Debug)]
enum CreateInstructionKind {
    Nest(Box<CreateInstruction>),
    SetValue(Expr),
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_parse_create_macro() {
        let input: TokenStream = syn::parse_quote!(create!(context, user .name "Olivier"));
        let output = do_create(input);
        dbg!(output);
    }
}
