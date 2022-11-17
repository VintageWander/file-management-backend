use crate::{helper::into_string, response::file::FileResponse, Result};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::file::{check_dir, check_filename, check_full_filename, check_fullpath};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub owner: ObjectId,

    #[validate(custom = "check_filename")]
    pub filename: String,

    pub extension: Extension,

    pub visibility: Visibility,

    #[validate(custom = "check_full_filename")]
    pub full_filename: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    #[validate(custom = "check_fullpath")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Extension {
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "txt")]
    Txt,
    #[serde(rename = "mp3")]
    Mp3,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl From<File> for Document {
    fn from(f: File) -> Self {
        let extension = f.extension_to_str();
        let visibility = f.visibility_to_str();

        doc! {
            "owner": f.owner,
            "filename": f.filename.clone(),
            "extension": extension,
            "visibility": visibility,
            "fullFilename": f.full_filename,
            "position": f.position,
            "fullpath": f.fullpath,
            "createdAt": f.created_at,
            "updatedAt": f.updated_at
        }
    }
}

impl File {
    // This constructor does a lot of things behind the scenes to make things easier to create
    // Like taking the full filename and split it into the name and extension seperately
    // I didn't check each field one by one, though it is more performant,
    // But normally I cannot cover all of the cases, and I have to duplicate a lot of code
    // It's best to just construct an entire struct and validate it all in one go
    pub fn new(
        id: ObjectId,
        owner: &ObjectId,
        full_filename: &str,
        visibility: Visibility,
        position: &str,
        created_at: Option<i64>,
    ) -> Result<Self> {
        check_full_filename(full_filename).map_err(into_string)?;
        check_dir(position).map_err(into_string)?;
        let (filename, extension) = full_filename
            .split_once('.')
            .ok_or("Cannot extract filename and extension from the full filename")?;

        let extension = match extension {
            "jpeg" => Extension::Jpeg,
            "jpg" => Extension::Jpg,
            "png" => Extension::Png,
            "txt" => Extension::Txt,
            "mp3" => Extension::Mp3,
            _ => return Err("Invalid extension".into()),
        };

        let position = format!("{owner}/{position}");

        let file = File {
            id,
            owner: *owner,
            filename: filename.to_string(),
            extension,
            visibility,
            full_filename: full_filename.to_string(),
            position: position.to_string(),
            fullpath: format!("{position}{full_filename}"),
            created_at: created_at.unwrap_or_else(|| Utc::now().timestamp_millis()),
            updated_at: Utc::now().timestamp_millis(),
        };
        file.validate()?;
        Ok(file)
    }

    pub fn extension_to_str(&self) -> &str {
        match self.extension {
            Extension::Jpeg => "jpeg",
            Extension::Jpg => "jpg",
            Extension::Png => "png",
            Extension::Txt => "txt",
            Extension::Mp3 => "mp3",
        }
    }

    pub fn visibility_to_str(&self) -> &str {
        match self.visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        }
    }

    // pub fn id(&self) -> Result<ObjectId> {
    //     self.id
    //         .ok_or_else(|| "This file does not have an ID".into())
    // }

    // pub fn id_str(&self) -> Result<String> {
    //     Ok(self.id()?.to_string())
    // }

    pub fn into_response(self) -> Result<FileResponse> {
        FileResponse::from_file(self)
    }
}
