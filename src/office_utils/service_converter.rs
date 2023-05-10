#![allow(non_snake_case,unused,dead_code)]
use super::{models::CallbackData,file_utils};

use crate::error::Error;

type Result<T> = std::result::Result<T,Error>;

pub async fn get_converter_uri(
    doc_uri : &str,
    from_ext : &str,
    to_ext : &str,
    doc_key : &str,
) -> Result<String>{
    #[derive(serde::Serialize)]
    struct ConverterPayload{
        #[serde(rename = "async")]
        _async : bool,
        url : String,
        outputtype : String,
        filetype : String,
        title : String,
        key : String,
        password : Option<String>,
        region : Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct ConverterResponse{
        pub endConvert : bool,
        pub error : i32,
        pub fileUrl : Option<String>,
    }
    use crate::config::{DOCUMENT_SERVER_URL,DOC_SERV_CONVERTER_URL};
    let title = file_utils::get_file_name(doc_uri)?;
    let payload = ConverterPayload{
        _async : true,
        url : doc_uri.to_owned(),
        outputtype : to_ext.to_owned(),
        filetype : from_ext.to_owned(),
        title : title.clone(),
        key : doc_key.to_owned(),
        password : None,
        region : None,
    };
    let url_for_req = format!("{}{}",DOCUMENT_SERVER_URL,DOC_SERV_CONVERTER_URL);
    let parsed = reqwest::Url::parse(&url_for_req).map_err(|_| Error::ConverterError)?;
    let client = reqwest::Client::new();
    let resp = client.post(parsed)
        .json(&payload)
        .send()
        .await?
        .json::<ConverterResponse>()
        .await?;

    if resp.error != 0{
        return  Err(Error::ConverterError);
    }
    // Че тут делать ?
    Ok(resp.fileUrl.unwrap())

}
