use cddl::{ast::*, token::*};

fn verify_group(group: &Group, is_map: bool) -> Result<(), String> {
    for group_choice in group.group_choices.iter() {
        for (entry, _comma) in group_choice.group_entries.iter() {
            verify_group_entry(&entry, is_map).map_err(|e| format!("{}: {}", entry, e))?;
        }
    }
    Ok(())
}

fn verify_ident(ident: &Identifier, is_key: bool) -> Result<(), String> {
    match ident.ident.as_str() {
        // this can refer to valid standard prelude types
        "uint"       |
        "int"        |
        "nint"       |
        "text"       |
        "tstr"       |
        "bytes"      |
        "bstr"       => Ok(()),
        // these are non-standard types referring to the cddl-codgen tool
        "u32"        |
        "i32"        |
        "u64"        |
        "i64"        => Ok(()),
        // or invalid standard prelude types
        "bool"       |
        "float"      |
        "float16"    |
        "float32"    |
        "float64"    |
        "float16-32" |
        "float32-64" |
        "tdate"      |
        "time"       |
        "number"     |
        "biguint"    |
        "bignint"    |
        "bigint"     |
        "integer"    |
        "unsigned"   |
        "decfrac"    |
        "bigfloat"   |
        "eb64url"    |
        "eb64legacy" |
        "eb16"       |
        "encoded-cbor" |
        "uri"        |
        "b64url"     |
        "b64legacy"  |
        "regexp"     |
        "mime-message" |
        "cbor-any"   |
        "null"       |
        "nil"        |
        "undefined"  |
        "true"       |
        "false" => Err(format!("invalid standard prelude type: {}", ident)),
        // refers to user-defined type
        other => if is_key {
            verify_len(other.len())
        } else {
            Ok(())
        }
    }
}

fn verify_group_entry(entry: &GroupEntry, is_map: bool) -> Result<(), String> {
    match entry {
        GroupEntry::ValueMemberKey { ge, .. } => {
            // keys are only serialized in cddl maps, not array structs
            if is_map {
                match &ge.member_key {
                    Some(key) => match key {
                        MemberKey::Type1 { t1, .. } => verify_type2(&t1.type2)?,
                        MemberKey::Bareword { ident, .. } => verify_ident(&ident, true)?,
                        MemberKey::Value { value, .. } => match value {
                            Value::FLOAT(_) => return Err(String::from("floats not supported")),
                            Value::BYTE(bv) => match bv {
                                ByteValue::UTF8(bytes) => verify_len(bytes.len())?,
                                ByteValue::B16(bytes) => verify_len(bytes.len())?,
                                ByteValue::B64(bytes) => verify_len(bytes.len())?,
                            },
                            Value::TEXT(text) => verify_len(text.len())?,
                            _ => (),
                        }
                    },
                    None => (),
                }
            }
            verify_type(&ge.entry_type)
        },
        // verify type referred to here where it's defined instead
        GroupEntry::TypeGroupname { ge, .. } => verify_ident(&ge.name, false),
        GroupEntry::InlineGroup { group, .. } => verify_group(&group, true),
    }
}

fn verify_len(len: usize) -> Result<(), String> {
    if len <= 64 {
        Ok(())
    } else {
        Err(format!("literal len too big: {}, limit is 64", len))
    }
}

fn verify_type(ty: &Type) -> Result<(), String> {
    for type_choice in ty.type_choices.iter() {
        verify_type2(&type_choice.type2)?;
    }
    Ok(())
}

fn verify_type2(type2: &Type2) -> Result<(), String> {
    match type2 {
        Type2::UintValue { .. } => Ok(()),
        Type2::IntValue { .. } => Ok(()),
        Type2::TextValue { value, .. } => verify_len(value.len()),
        Type2::UTF8ByteString { value, .. } => verify_len(value.len()),
        Type2::B16ByteString { value, .. } => verify_len(value.len()),
        Type2::B64ByteString { value, .. } => verify_len(value.len()),
        Type2::Typename { ident, .. } => verify_ident(&ident, false),
        Type2::Map { group, .. } => verify_group(group, true),
        Type2::Array { group, .. } => verify_group(group, false),
        Type2::TaggedData { .. } => Err(String::from("no tagged data")),
        unsupported => Err(format!("unsupported type: {}", unsupported)),
    }
}

fn verify_rule(cddl_rule: &Rule) -> Result<(), String> {
    match cddl_rule {
        Rule::Type{ rule, .. } => {
            verify_type(&rule.value)
        },
        Rule::Group{ rule, .. } => {
            match &rule.entry {
                GroupEntry::InlineGroup{ group, .. } => {
                    // TODO: be less strict on array type keys for plain groups but this is probably ok
                    verify_group(&group, true)?;
                    Ok(())
                },
                x => panic!("Group rule with non-inline group? {:?}", x),
            }
        },
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cddl_in = std::fs::read_to_string("input.cddl").expect("input.cddl file not present or could not be opened");
    let cddl = cddl::parser::cddl_from_str(&cddl_in)?;
    for cddl_rule in &cddl.rules {
        verify_rule(cddl_rule).map_err(|e| format!("type {} not valid metadata: {}", cddl_rule.name(), e))?;
    }
    Ok(())
}
