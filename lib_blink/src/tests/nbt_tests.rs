use std::{fs::File, io::BufReader, path::PathBuf};

use crate::nbt::{NBTLexer, NBTTag};

fn get_bigtest_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_data");
    path.push("bigtest");
    path
}

#[test]
pub fn test_lex_compound() {
    let path = get_bigtest_path();

    let file = File::open(path).expect("Unable to open bigtest nbt file!");

    let mut reader = BufReader::new(file);
    let result = NBTLexer::lex_tag(&mut reader);
    match result {
        Ok(compound_tag) => {
            println!("Result: {:?}", compound_tag);
            if let NBTTag::TagCompound(Some(nbt_compound)) = compound_tag {
                assert_eq!("Level", nbt_compound.name.unwrap());
            } else {
                assert!(false);
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            assert!(false);
        }
    }
}

#[test]
pub fn test_convert_be_vec_in_place() {}
