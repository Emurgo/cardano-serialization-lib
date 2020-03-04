// We need to figure how out we handle integers as they can only be serialized as
// Unsigned or Negative. Do we do an enum for the int type?
// It's only used in transaction_metadata as one choice.
// How it has to be serialized (assuming number: i32):
// if number >= 0 {
//     serializer.write_unsigned_integer(number as u64)
// } else {
//     serializer.write_negative_integer(number as i64)
// }

use cddl::ast::*;

fn group_entry_to_field_name(entry: &GroupEntry, index: usize) -> String {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => match vmk.member_key.as_ref() {
            Some(member_key) => match member_key {
                MemberKey::Value(value) => format!("key_{}", value),
                MemberKey::Bareword(ident) => ("bw_".to_owned() + &ident.to_string()),
                MemberKey::Type1(_) => panic!("Encountered Type1 member key in multi-field map - not supported"),
            },
            None => format!("index_{}", index),
        },
        GroupEntry::TypeGroupname(tge) => {
            // This was before, but it makes more sense for what we've done so far
            // to have it be indexed. This may or may not be correct.
            //("tgn_".to_owned() + &tge.name.to_string()),
            format!("index_{}", index)
        },
        GroupEntry::InlineGroup(_) => panic!("not implemented (define a new struct for this!)"),
    }
}

// TODO: Can we do this, or do we need to be more explicit to match the schema?
fn convert_types(raw: &str) -> &str {
    match raw {
        "uint" => "u64",
        // Not sure on this one, I think they can be bigger than i64 can fit
        // but the cbor_event serialization takes the argument as an i64
        "nint" => "i64",
        // TODO: define enum or something as otherwise it can overflow i64
        // and also we can't define the serialization traits for types
        // that are defined outside of this crate (includes primitives)
        //"int" => "i64",
        "tstr" | "text" => "String",
        // TODO: Is this right to have it be Vec<u8>?
        // the serialization library for bytes takes type [u8]
        // so we'll have to put some logic in there I guess?
        // it might be necessary to put a wrapper type..
        "bstr" | "bytes" => "Bytes",
        // What about bingint/other stuff in the standard prelude?
        x => x,
    }
}

// Returns None if this is a fixed value that we should not be storing
fn rust_type_from_type2(type2: &Type2) -> Option<String> {
    match type2 {
        // ignoring IntValue/FloatValue/other primitives since they're not in the shelley spec
        // ie Type2::UintValue(value) => format!("uint<{}>", value),
        // generic args not in shelley.cddl
        // TODO: socket plugs (used in hash type)
        Type2::Typename((ident, _generic_arg)) => Some(convert_types(&(ident.0).0).to_owned()),
        // Map(group) not implemented as it's not in shelley.cddl
        Type2::Array(group) => {
            let mut s = String::new();
            for choice in &group.0 {
                // special case for homogenous arrays
                if let Some((entry, _has_comma)) = choice.0.first() {
                    let element_type = match entry {
                        GroupEntry::ValueMemberKey(vmk) => rust_type(&vmk.entry_type),
                        GroupEntry::TypeGroupname(tgn) => Some(tgn.name.to_string()),
                        _ => Some(format!("UNSUPPORTED_ARRAY_ELEMENT<{:?}>", entry)),
                    };
                    s.push_str(&format!("Array<{}>", element_type.unwrap()));
                } else {
                    // TODO: how do we handle this? tuples?
                    // or creating a struct definition and referring to it
                    // by name?
                }
                // TODO: handle group choices (enums?)
                break;
            }
            Some(s)
        },
        x => None,
    }
}

fn rust_type(t: &Type) -> Option<String> {
    for type1 in t.0.iter() {
        // ignoring range control operator here, only interested in Type2
        return rust_type_from_type2(&type1.type2);

        // TODO: how to handle type choices? define an enum for every option?
        //       deserializing would be more complicated since you'd
        //       have to test them until one matches.
    }
    panic!("rust_type() is broken for: '{}'", t)
}

fn group_entry_to_type_name(entry: &GroupEntry) -> Option<String> {
    match entry {
        GroupEntry::ValueMemberKey(vmk) => rust_type(&vmk.entry_type),//convert_types(&vmk.entry_type.to_string()).to_owned(),
        GroupEntry::TypeGroupname(tge) => Some(tge.name.to_string()),
        GroupEntry::InlineGroup(_) => panic!("not implemented"),
    }
}

fn codegen_group_as_map(scope: &mut codegen::Scope, group: &Group, name: &str) {
    scope.raw("#[wasm_bindgen]");
    let s = scope.new_struct(name);
    s.field("group", format!("groups::{}", name));
    let mut ser_impl = codegen::Impl::new(name);
    ser_impl.impl_trait("cbor_event::se::Serialize");
    let mut ser_func = make_serialization_function("serialize");
    let mut group_impl = codegen::Impl::new(name);
    let to_bytes = group_impl.new_fn("to_bytes")
        .ret("Vec<u8>")
        .arg_ref_self();
    ser_func.line("self.group.serialize_as_map(serializer)");
    to_bytes.line("let mut buf = Serializer::new_vec();");
    to_bytes.line("self.serialize(&mut buf).unwrap();");
    to_bytes.line("buf.finalize()");
    ser_impl.push_fn(ser_func);
    scope.push_impl(ser_impl);
    scope.raw("#[wasm_bindgen]");
    scope.push_impl(group_impl);
    // TODO: write accessors here? would be common with codegen_group_as_array
}

// Separate function for when we support multiple choices as an enum
fn codegen_group(scope: &mut codegen::Scope, group: &Group, name: &str) {
    if group.0.len() == 1 {
        codegen_group_choice(scope, group.0.first().unwrap(), name);
    } else {
        let mut e = codegen::Enum::new(name);
        let mut e_impl = codegen::Impl::new(name);
        // TODO: serialize map. this is an issue since the implementations might not exist.
        let mut ser_array = make_serialization_function("serialize_as_array");
        ser_array.vis("pub (super)");
        let mut match_block = codegen::Block::new("match self");
        for (i, group_choice) in group.0.iter().enumerate() {
            let variant_name = name.to_owned() + &i.to_string();
            e.push_variant(codegen::Variant::new(&format!("{}({})", variant_name, variant_name)));
            codegen_group_choice(scope, group_choice, &variant_name);
            match_block.line(format!("{}::{}(x) => x.serialize_as_array(serializer),", name, variant_name));
        }
        ser_array.push_block(match_block);
        e_impl.push_fn(ser_array);
        scope.push_enum(e);
        scope.push_impl(e_impl);
    }
}

fn table_domain_range(group_choice: &GroupChoice) -> Option<(&Type2, &Type)> {
    if group_choice.0.len() == 1 {
        if let Some((GroupEntry::ValueMemberKey(vmk), _)) = group_choice.0.first() {
            match &vmk.member_key {
                // TODO: Do we need to handle cuts for what we're doing?
                // Does the range control operator matter?
                Some(MemberKey::Type1(type1)) => Some((&type1.0.type2, &vmk.entry_type)),
                _ => None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn make_serialization_function(name: &str) -> codegen::Function {
    let mut f = codegen::Function::new(name);
    f
        .generic("'se, W: Write")
        .ret("cbor_event::Result<&'se mut Serializer<W>>")
        .arg_ref_self()
        .arg("serializer", "&'se mut Serializer<W>");
    f
}

fn codegen_group_choice(scope: &mut codegen:: Scope, group_choice: &GroupChoice, name: &str) {
    // handles ValueMemberKey only
    // TODO: TypeGroupname / InlinedGroup are not supported yet
    // TODO: handle non-integer keys (all keys in shelley.cddl are uint)

    let s = scope.new_struct(name);
    s.vis("pub (super)");
    let mut s_impl = codegen::Impl::new(name);
    // We could re-use this for arrays I guess and add a tag?

    // Here we test if this is a struct vs a table.
    // struct: { x: int, y: int }, etc
    // table: { * int => tstr }, etc
    // this assumes that all maps representing tables are homogenous
    // and contain no other fields. I am not sure if this is a guarantee in
    // cbor but I would hope that the cddl specs we are using follow this.

    // Is there a more concise/readable way of expressing this in rust?
    let table_types = table_domain_range(group_choice);
    match table_types {
        Some((domain, range)) => {
            s.field("table", format!("std::collections::BTreeMap<{}, {}>", rust_type_from_type2(domain).unwrap(), rust_type(range).unwrap()));
            let mut ser_map = make_serialization_function("serialize_as_map");
            ser_map
                .vis("pub (super)")
                .line("panic!(\"TODO: implement\");");
            s_impl.push_fn(ser_map);
        },
        None => {
            let mut ser_array = make_serialization_function("serialize_as_array");
            let mut ser_map = make_serialization_function("serialize_as_map");
            ser_array
                .vis("pub (super)")
                .line(format!("serializer.write_array(cbor_event::Len::Len({}u64))?;", group_choice.0.len()));
            ser_map
                .vis("pub (super)")
                .line(format!("serializer.write_array(cbor_event::Len::Len({}u64))?;", group_choice.0.len()));
            // If we have a group with entries that have no names, that's fine for arrays
            // but not for maps, so if we encounter one assume we should not generate
            // map-related functions.
            // In the future we could change this tool to only emit the array or map
            // functions when they are strictly necessary (wrapped in array or map elsewhere)
            // This would also reduce error checking here since we wouldn't hit certain cases
            let mut contains_entries_without_names = false;
            for (index, (group_entry, _has_comma)) in group_choice.0.iter().enumerate() {
                let field_name = group_entry_to_field_name(group_entry, index);
                // Unsupported types so far are fixed values, only have fields
                // for these.
                if let Some(type_name) = group_entry_to_type_name(group_entry) {
                    s.field(
                        &field_name,
                        format!("Option<{}>", type_name)
                    );
                    // TODO: support conditional members (100% necessary for heterogenous maps (not tables))
                    // TODO: proper support since this assumes all members implement the trait
                    //       maybe we could put a special case for primitives or Maps/Vecs?
                    // TODO: remove clone()? Without it String gets moved out.
                    ser_array.line(format!("self.{}.clone().unwrap().serialize(serializer)?;", field_name));
                    match group_entry {
                        GroupEntry::ValueMemberKey(vmk) => {
                            match vmk.member_key.as_ref() {
                                Some(member_key) => match member_key {
                                    MemberKey::Value(value) => match value {
                                        cddl::token::Value::UINT(x) => {
                                            ser_map.line(format!("serializer.write_unsigned_integer({})?;", x));
                                        },
                                        _ => panic!("unsupported map identifier(1): {:?}", value),
                                    },
                                    MemberKey::Bareword(ident) => {
                                        ser_map.line(format!("serializer.write_text(\"{}\")?;", ident.to_string()));
                                    },
                                    x => panic!("unsupported map identifier(2): {:?}", x),
                                },
                                None => {
                                    contains_entries_without_names = true;
                                },
                            }
                        },
                        // TODO: why are we hitting this?
                        // GroupEntry::TypeGroupname(tgn) => match tgn.name.to_string().as_ref() {
                        //     "uint" => format!("serializer.write_unsigned_integer({})?;", x),
                        //     x => panic!("TODO: serialize '{}'", x),
                        // },
                        x => {
                            //panic!("unsupported map identifier(3): {:?}", x),
                            // TODO: only generate map vs array stuff when needed to avoid this hack
                            contains_entries_without_names = true;
                        },
                    };
                    ser_map.line(format!("self.{}.clone().unwrap().serialize(serializer)?;", field_name));
                } else {
                    // TODO: do we need to support type choices here?!
                    match group_entry {
                        GroupEntry::ValueMemberKey(vmk) => match vmk.entry_type.0.first() {
                            Some(x) => match &x.type2 {
                                Type2::UintValue(x) => {
                                    ser_array.line(format!("serializer.write_unsigned_integer({})?;", x));
                                },
                                x => panic!("unsupported fixed type: {}", x),
                            },
                            None => unreachable!(),
                        },
                        _ => panic!("unsupported fixed type: {:?}", group_entry),
                    }
                }
            }
            ser_array.line("serializer.write_special(cbor_event::Special::Break)");
            ser_map.line("serializer.write_special(cbor_event::Special::Break)");
            s_impl.push_fn(ser_array);
            if !contains_entries_without_names {
                s_impl.push_fn(ser_map);
            }
        }
    }
    scope.push_impl(s_impl);
}


// struct Array<T>(Vec<T>);

// impl<T> std::ops::Deref for Array<T> {
//     type Target = Vec<T>;

//     fn deref(&self) -> &Vec<T> {
//         &self.0
//     }
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cddl_in = std::fs::read_to_string("supported.cddl").unwrap();
    let cddl = cddl::parser::cddl_from_str(&cddl_in)?;
    //println!("CDDL file: {}", cddl);
    let mut scope = codegen::Scope::new();
    // Can't generate groups of imports with codegen::Import so we just output this as raw text
    // since we don't need it to be dynamic so it's fine. codegen::Impl::new("a", "{z::b, z::c}")
    // does not work.
    scope.raw("use cbor_event::{self, de::{Deserialize, Deserializer}, se::{Serialize, Serializer}};");
    scope.import("std::io", "Write");
    scope.import("wasm_bindgen::prelude", "*");
    // We need wrapper types for arrays/bytes as we can't specialize Vec<T> to cbor_event's Serialize
    // as they come from different external crates.
    scope.new_struct("Array<T>(Vec<T>)");
    scope
        .new_impl("Array<T>")
        .generic("T")
        .impl_trait("std::ops::Deref")
        .associate_type("Target", "Vec<T>")
        .new_fn("deref")
        .arg_ref_self()
        .ret("&Vec<T>")
        .line("&self.0");
    scope
        .new_struct("Bytes(Vec<u8>)")
        .derive("Clone");
    scope
        .new_impl("Bytes")
        .impl_trait("Serialize")
        .new_fn("serialize")
        .arg_ref_self()
        .arg("serializer", "&'a mut Serializer<W>")
        .generic("'a, W: Write + Sized")
        .ret("cbor_event::Result<&'a mut Serializer<W>>")
        .line("serializer.write_bytes(&self.0[..])");
    let mut group_module = codegen::Module::new("groups");
    let group_scope = group_module.scope();
    group_scope.import("super", "*");
    for rule in cddl.rules {
        match rule {
            Rule::Type(tr) => {
                // (1) does not handle optioanl generic parameters
                // (2) does not handle ranges - I think they're the character position in the CDDL
                // (3) is_type_choice_alternate ignored since shelley cddl doesn't need it
                //     It's used, but used for no reason as it is the initial definition
                //     (which is also valid cddl), but it would be fine as = instead of /=
                //let mut s = scope.new_struct(tr.name.to_string().as_ref());
                // TODO: choices (as enums I guess?)
                for choice in &tr.value.0 {
                    // ignores control operators - only used in shelley spec to limit string length for application metadata
                    match &choice.type2 {
                        Type2::Typename((identifier, _generic_arg)) => {
                            scope.raw(format!("type {} = {};", tr.name, convert_types(&identifier.to_string())).as_ref());
                        },
                        // TODO: try to re-use/refactor this for arrays
                        Type2::Map(group) => {
                            let group_name = tr.name.to_string();
                            codegen_group(group_scope, group, group_name.as_ref());
                            codegen_group_as_map(&mut scope, group, group_name.as_ref());
                        },
                        x => {
                            println!("\nignored typename {} -> {:?}\n", tr.name, x);
                            // ignored
                        }
                    }
                    //println!("{} type2 = {:?}\n", tr.name, choice.type2);
                    //s.field("foo", "usize");
                    // remove and implement type choices
                    break;
                }
            },
            Rule::Group(group_rule) => {
                // Freely defined group - no need to generate anything outside of group module
                match &group_rule.entry {
                    GroupEntry::InlineGroup((_occur, inline_group)) => {
                        codegen_group(group_scope, inline_group, &group_rule.name.to_string());
                    },
                    x => panic!("Group rule with non-inline group? {:?}", x),
                }
            },
        }
    }
    scope.push_module(group_module);
    println!("{}", scope.to_string());

    Ok(())
}
