use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
struct NestedField {
    name: String,
    ty: String,
    nested_fields: Option<Vec<NestedField>>,
}

#[derive(Debug)]
struct SchemaStructure {
    name: String,
    fields: Vec<NestedField>,
    derives: String,
}

#[proc_macro]
pub fn i18n_schema(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    match parse_and_generate(&input) {
        Ok(output) => output,
        Err(e) => {
            let error = format!("i18n_schema error: {}", e);
            quote! { compile_error!(#error); }.into()
        }
    }
}

fn parse_and_generate(input: &str) -> Result<TokenStream, String> {
    let parsed = parse_schema(input)?;
    let output = generate_all_structs(&parsed);
    Ok(TokenStream::from(output))
}

fn parse_schema(input: &str) -> Result<SchemaStructure, String> {
    let input = input.trim();

    // Extraer atributos y derives
    let mut derives = String::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.starts_with("#[") {
            lines.next();
            if line.contains("derive") {
                derives.push_str(line);
            }
        } else {
            break;
        }
    }

    // Buscar "pub struct NAME {"
    let struct_idx = input.find("pub struct").ok_or("Expected 'pub struct'")?;
    let after_struct = &input[struct_idx + 10..].trim_start();
    let name_end = after_struct
        .find(|c: char| c.is_whitespace())
        .ok_or("Expected struct name")?;
    let name = after_struct[..name_end].to_string();

    // Extraer contenido entre { }
    let brace_start = input.find('{').ok_or("Expected '{'")?;
    let brace_end = input.rfind('}').ok_or("Expected '}'")?;

    let fields_str = &input[brace_start + 1..brace_end];
    let fields = parse_fields(fields_str)?;

    Ok(SchemaStructure {
        name,
        fields,
        derives,
    })
}

fn parse_fields(fields_str: &str) -> Result<Vec<NestedField>, String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut depth = 0;

    for ch in fields_str.chars() {
        match ch {
            '{' => {
                depth += 1;
                current_field.push(ch);
            }
            '}' => {
                depth -= 1;
                current_field.push(ch);
            }
            ',' if depth == 0 => {
                if !current_field.trim().is_empty() {
                    fields.push(parse_single_field(current_field.trim())?);
                }
                current_field.clear();
            }
            _ => current_field.push(ch),
        }
    }

    if !current_field.trim().is_empty() {
        fields.push(parse_single_field(current_field.trim())?);
    }

    Ok(fields)
}

fn parse_single_field(field_str: &str) -> Result<NestedField, String> {
    let field_str = field_str.trim();

    if let Some(colon_pos) = field_str.find(':') {
        let name_part = field_str[..colon_pos].trim();
        let type_part = field_str[colon_pos + 1..].trim();

        let name = name_part
            .split_whitespace()
            .last()
            .unwrap_or("")
            .to_string();

        // Verificar si es un struct anidado
        if type_part.starts_with("struct ") {
            let struct_name = type_part
                .strip_prefix("struct ")
                .unwrap()
                .split('{')
                .next()
                .unwrap_or("")
                .trim()
                .to_string();

            if let Some(brace_start) = type_part.find('{')
                && let Some(brace_end) = type_part.rfind('}')
            {
                let nested_fields_str = &type_part[brace_start + 1..brace_end];
                let nested_fields = parse_fields(nested_fields_str)?;

                return Ok(NestedField {
                    name,
                    ty: struct_name,
                    nested_fields: Some(nested_fields),
                });
            }
        }

        Ok(NestedField {
            name,
            ty: type_part.to_string(),
            nested_fields: None,
        })
    } else {
        Err(format!("Invalid field syntax: {}", field_str))
    }
}

fn generate_all_structs(schema: &SchemaStructure) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    // Generar structs anidados recursivamente (de adentro hacia afuera)
    for field in &schema.fields {
        generate_nested_structs(field, &mut output);
    }

    // Generar la struct principal
    let main_name = proc_macro2::Ident::new(&schema.name, proc_macro2::Span::call_site());
    let main_fields = generate_main_fields(&schema.fields);

    let derives_tokens: proc_macro2::TokenStream = if schema.derives.is_empty() {
        quote! { #[derive(Debug, serde::Deserialize, Clone)] }
    } else {
        schema.derives.parse().unwrap_or_else(|_| {
            quote! { #[derive(Debug, serde::Deserialize, Clone)] }
        })
    };

    output.extend(quote! {
        #derives_tokens
        #[allow(dead_code)]
        pub struct #main_name {
            #main_fields
        }
    });

    output
}

fn generate_nested_structs(field: &NestedField, output: &mut proc_macro2::TokenStream) {
    if let Some(ref nested) = field.nested_fields {
        let struct_name = proc_macro2::Ident::new(&field.ty, proc_macro2::Span::call_site());
        let nested_fields = generate_main_fields(nested);

        // Generar structs anidados recursivamente (primero los más profundos)
        for nf in nested {
            generate_nested_structs(nf, output);
        }

        // Luego generar este struct
        output.extend(quote! {
            #[derive(Debug, serde::Deserialize, Clone)]
            pub struct #struct_name {
                #nested_fields
            }
        });
    }
}

fn generate_main_fields(fields: &[NestedField]) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    for field in fields {
        let field_name = proc_macro2::Ident::new(&field.name, proc_macro2::Span::call_site());

        if field.nested_fields.is_some() {
            let field_type = proc_macro2::Ident::new(&field.ty, proc_macro2::Span::call_site());
            output.extend(quote! {
                pub #field_name: #field_type,
            });
        } else {
            let field_type: proc_macro2::TokenStream = field.ty.parse().unwrap_or_else(|_| {
                quote! { String }
            });
            output.extend(quote! {
                pub #field_name: #field_type,
            });
        }
    }

    output
}
