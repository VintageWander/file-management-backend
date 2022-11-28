use crate::{
    validation::file::{check_dir, check_fullpath, check_version_folder, check_version_path},
    Result,
};

use super::into_string;

// This function make
// From: user/folder/hello.txt
// Into: user-version-db/folder/hello/123.txt
// Useful for getting a version path to store file as backups
pub fn convert_file_path_to_version_path(str: &str, version: i64) -> Result<String> {
    check_fullpath(str).map_err(into_string)?;
    // The process goes like this
    // Example:
    // 1) user/folder/nested/hello.txt
    // 2) user/hello.txt
    // And the version input is 123

    let (position, filename) = str
        .rsplit_once('/')
        .ok_or("Cannot split position and filename")?;
    // 1) position = user/folder/nested
    //    filename = hello.txt
    // 2) position = user
    // 	  filename = hello.txt

    let (name, extension) = filename
        .rsplit_once('.')
        .ok_or("Cannot split name and extension")?;
    // 1) and 2) name      = hello
    //    		 extension = txt

    // This applies in case 2)
    if !position.contains('/') {
        // user-version-db/hello/123.txt
        return Ok(format!(
            "{position}-version-db/{name}/{}.{extension}",
            version
        ));
    }

    // Else
    // 1) first = user
    //    rest  = folder/nested
    let (first, rest) = position
        .split_once('/')
        .ok_or("Cannot split first and rest")?;

    // position = user-version-db/folder/nested
    let position = format!("{first}-version-db/{rest}");

    // result = user-version-db/folder/nested/hello/123.txt
    let result = format!("{position}/{name}/{version}.{extension}");
    Ok(result)
}

// This function make
// From: user-version-db/folder/hello/123.txt
// Into: user/folder/hello.txt
// Useful for getting the original path for file restoration

pub fn convert_version_to_file_path(str: &str) -> Result<String> {
    check_version_path(str).map_err(into_string)?;
    // Let's start with a string like this
    // 1) user-version-db/folder/hello/123.txt
    // 2) user-version-db/hello/123.txt

    let (position, full_filename) = str
        .rsplit_once('/')
        .ok_or("Cannot split position and filename")?;
    // 1) position      = user-version-db/folder/hello
    //    full_filename = 123.txt
    // 2) position      = user-version-db/hello
    //    full_filename = 123.txt

    let (_, extension) = full_filename
        .rsplit_once('.')
        .ok_or("Cannot extract extension")?;
    // 1) and 2) _         = 123
    //           extension = txt

    let (rest, filename) = position.rsplit_once('/').ok_or("Cannot get filename")?;
    // 1) rest     = user-version-db/folder
    //    filename = hello
    // 2) rest     = user-version-db
    //    filename = hello

    // This only works in case 2)
    if !rest.contains('/') {
        let end_index = rest.len() - "-version-db".len();
        let rest = &rest[0..end_index];
        return Ok(format!("{rest}/{filename}.{extension}"));
    }

    let (first, rest) = rest.split_once('/').ok_or("Cannot get the prefix")?;
    // 1) first = user-version-db
    //    rest  = folder

    // This basically trims user-version-db into user
    let end_index = first.len() - "-version-db".len();
    let first = &first[0..end_index];
    // 1) first = user

    // user/folder/hello.txt
    Ok(format!("{first}/{rest}/{filename}.{extension}"))
}

// This function make
// From: user/folder/hello.txt
// Into: user-version-db/folder/hello/
//
// Useful for batch renaming
// Meaning that if the user changes the filename in the main folder from "hello" to "something"
// I can quickly call the rename folder function
// First get the original old version file path user/folder/hello.txt -> user-version-db/folder/hello/
// Then change the filename, the path will also changes: user/folder/something.txt
// Use this function to convert from user/folder/something.txt -> user-version-db/folder/something/
// rename_folder( user-version-db/folder/hello/ , user-version-db/folder/something/ )
//
// Useful for new file creation
// When the user creates a new file
// I can create both user/folder/hello.txt and user-version-db/folder/hello/ at the same time

pub fn convert_file_path_to_version_folder(str: &str) -> Result<String> {
    check_fullpath(str).map_err(into_string)?;
    let converted_path = convert_file_path_to_version_path(str, 1)?;
    let (result, _) = converted_path
        .rsplit_once('/')
        .ok_or("Cannot split off the version filename")?;
    let result = format!("{}/", result);
    Ok(result)
}

// This function make
// From: user-version-db/folder/
// Into: user/folder/
// Useful to quickly get the file position that is mapped to version db side

pub fn convert_version_folder_to_folder(str: &str) -> Result<String> {
    check_version_folder(str).map_err(into_string)?;
    let (first, rest) = str
        .split_once('/')
        .ok_or("Cannot split the first and the rest")?;
    let end_index = first.len() - "-version-db".len();
    let first = &first[0..end_index];
    Ok(format!("{first}/{rest}"))
}

// This function make
// From: user-version-db/folder/hello/123.txt
// Into: user/folder/
// Useful to get the file position on the original path side from the version path

pub fn convert_version_to_file_folder(str: &str) -> Result<String> {
    check_version_path(str).map_err(into_string)?;
    let converted_path = convert_version_to_file_path(str)?;
    println!("{converted_path}");
    let (result, _) = converted_path
        .rsplit_once('/')
        .ok_or("Cannot split off the filename")?;
    Ok(format!("{result}/"))
}

// This function make
// From: user/folder/
// Into: user-version-db/folder/

pub fn convert_folder_to_version(str: &str) -> Result<String> {
    check_dir(str).map_err(into_string)?;
    let (first, rest) = str
        .split_once('/')
        .ok_or("Cannot get the first folder name")?;
    Ok(format!("{first}-version-db/{rest}"))
}
