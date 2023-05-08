use serde::{Serialize,Deserialize};
use crate::database::models::{User,Groups};
#[derive(Debug,Serialize,Deserialize)]
pub enum UserFilePermissions{
    #[serde(rename = "Full Access")]
    FullAccess,
    #[serde(rename = "Read Only")]
    ReadOnly,
    #[serde(rename = "Deny Access")]
    DenyAccess,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct ReferenceData{
    fileKey : String,
    instanceId : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Document{
    fileType : String,
    key : String,
    referenceData : Option<ReferenceData>,
    title : String,
    url : String,
    info : Option<DocumentInfo>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct SharingSetting{
    isLink : Option<bool>,
    permissions : Option<UserFilePermissions>,
    user : String
}
#[derive(Deserialize,Serialize,Debug)]
pub struct DocumentInfo{
    author : Option<String>,
    uploaded : Option<String>,
    favorite : Option<bool>,
    folder : Option<String>,
    owner : Option<String>,
    sharingSettings : Option<Vec<SharingSetting>>
}
#[derive(Deserialize,Serialize,Debug)]
pub struct CoEditing{
    mode : String,
    change : bool,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Recent{
    folder : String,
    title : String,
    url : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Template{
    image : String,
    title : String,
    url : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct DocumentUser{
    name : String,
    group : String,
    id : String,
    lastname : String,
}

impl From<User> for DocumentUser{
    fn from(value: User) -> Self {
        DocumentUser { 
            name: value.email, 
            group: value.group.to_string(), 
            id: value.id.to_string(), 
            lastname: "".to_string() 
        }
    }
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Editor{
    pub actionLink : Option<String>,
    pub callbackUrl : String,
    pub coEditing : Option<CoEditing>,
    pub createUrl : String,
    pub lang : Option<String>,
    pub location : Option<String>,
    pub recent : Option<Vec<Recent>>,
    pub region : Option<String>,
    pub templates : Option<Vec<Template>>,
    pub user : Option<DocumentUser>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Config{
    #[serde(rename = "type")]
    pub _type : String, 
    pub documentType : Option<String>,
    pub document : Option<Document>,
    pub editorConfig : Option<Editor>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct ActionObject{
    #[serde(rename = "type")]
    pub _type : i32,
    pub userid : String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct ChangesObject{
    pub changes : String,
    pub serverVersion : String,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct CallbackData{
    pub actions : Option<Vec<ActionObject>>,
    pub changesurl : Option<String>,
    pub filetype : Option<String>,
    pub forcesavetype : Option<i32>,
    //pub history : Option<>,
    pub key : String,
    pub status : i32,
    pub url : Option<String>,
    pub userdata : Option<String>,
    pub users : Option<Vec<String>>,
}