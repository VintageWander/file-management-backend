use validator::ValidationError;

use super::check_with;

pub fn check_filename(filename: &str) -> Result<(), ValidationError> {
    // This regex matches hello,
    // Rejects hello.txt and .txt
    // This can also match hello_world or hello-world
    check_with(
        filename,
        r#"^[a-zA-Z0-9-_]{3,}$"#,
        "The name can only contain a-z A-Z 0-9 and within 3 to 20 characters in length",
    )
}

pub fn check_extension(extension: &str) -> Result<(), ValidationError> {
    check_with(
        extension,
        r#"^(txt|png|jpg|jpeg|mp3)$"#,
        "Supported extensions are txt, png, jpg, jpeg, mp3",
    )
}

pub fn check_visibility(visibility: &str) -> Result<(), ValidationError> {
    check_with(
        visibility,
        r#"^(public|private)$"#,
        "Visibility can only be public or private",
    )
}

pub fn check_full_filename(full_filename: &str) -> Result<(), ValidationError> {
    // This regex will match cases like hello.txt, hello or .txt or just txt will fail
    check_with(full_filename, r#"^[a-zA-Z0-9-_]{3,}\.(txt|mp3|png|jpg|jpeg)$"#, "The full filename must met filename requirements and can only have txt, mp3, png, jpeg or jpg extension")
}

pub fn check_dir(position: &str) -> Result<(), ValidationError> {
    // This regex will match for cases like user/, user/hello/, hello-world/user/, hello_world/something/
    // It will reject cases like user, /user, /user/, or user/hello
    // Basically it requires a slash must exists at the end
    check_with(
        position,
        r#"^([a-zA-Z0-9-_]{3,}[/])*$"#,
        "The dir input is in wrong format",
    )
}

pub fn check_fullpath(fullpath: &str) -> Result<(), ValidationError> {
    // This regex will match cases like user/hello.txt, hello.txt, or nested/something-deep/hello.txt
    // This will reject cases like hello/.txt, hello/world, or even hello/
    check_with(
        fullpath,
        r#"^(([a-zA-Z0-9-_]{3,}[/])*)[a-zA-Z0-9-_]{3,}\.(png|txt|jpg|jpeg|mp3)$"#,
        "The fullpath is incorrect",
    )
}

pub fn check_version_path(version_path: &str) -> Result<(), ValidationError> {
    // Matches
    // user-version-db/folder/hello/123.txt
    // user-version-db/hello/123.txt
    // Rejects:
    // user/folder/hello.txt
    // user-version-db/folder/hello.txt
    // user-version-db/123.txt
    check_with(
        version_path,
        r#"^[a-zA-Z0-9-_]{3,}(-version-db)[/](([a-zA-Z0-9-_]{3,}[/])*)[0-9]{3,}\.(png|txt|jpg|jpeg|mp3)$"#,
        "The version path is invalid",
    )
}

pub fn check_version_folder(version_folder: &str) -> Result<(), ValidationError> {
    check_with(
        version_folder,
        r#"^[a-zA-Z0-9-_]{3,}(-version-db)[/](([a-zA-Z0-9-_]{3,}[/])*)$"#,
        "The version folder is invalid",
    )
}

pub fn check_folder_name(folder_name: &str) -> Result<(), ValidationError> {
    check_filename(folder_name)
}
