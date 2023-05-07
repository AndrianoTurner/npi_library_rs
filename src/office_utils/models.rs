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
        
        DocumentUser { name: value.email, group: value.group.to_string(), id: value.id.to_string(), lastname: "".to_string() }
    }
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Editor{
    actionLink : Option<String>,
    callbackUrl : String,
    coEditing : Option<CoEditing>,
    createUrl : String,
    lang : Option<String>,
    location : Option<String>,
    recent : Option<Vec<Recent>>,
    region : Option<String>,
    templates : Option<Vec<Template>>,
    user : Option<DocumentUser>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Config{
    #[serde(rename = "type")]
    _type : String, 
    documentType : Option<String>,
    document : Option<Document>,
    editorConfig : Option<Editor>,
}