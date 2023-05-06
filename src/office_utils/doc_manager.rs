use std::collections::HashMap;


const DOC_SERV_FILLFORMS : [&str;2] = [".oform", ".docx"];
const DOC_SERV_VIEWED : [&str;4] = [".pdf", ".djvu", ".xps", ".oxps"] ; //# file extensions that can be viewed
const DOC_SERV_EDITED  : [&str;6]= [".docx", ".xlsx", ".csv", ".pptx", ".txt", ".docxf"];  //# file extensions that can be edited
const DOC_SERV_CONVERT : [&str;35] = [                                           //# file extensions that can be converted
    ".docm", ".doc", ".dotx", ".dotm", ".dot", ".odt",
    ".fodt", ".ott", ".xlsm", ".xlsb", ".xls", ".xltx", ".xltm",
    ".xlt", ".ods", ".fods", ".ots", ".pptm", ".ppt",
    ".ppsx", ".ppsm", ".pps", ".potx", ".potm", ".pot",
    ".odp", ".fodp", ".otp", ".rtf", ".mht", ".html", ".htm", ".xml", ".epub", ".fb2"
];
const DOCUMENT_SERVER_URL : &str = "http://localhost:8000/";
pub struct DocumentManager;

impl DocumentManager{
    fn isCanFillForms(&self,file_extension : &str) -> bool{
        DOC_SERV_FILLFORMS.contains(&file_extension)
    }

    fn isCanView(&self,file_extension : &str)-> bool{
        DOC_SERV_VIEWED.contains(&file_extension)
    }

    fn isCanEdit(&self,file_extension : &str) -> bool{
        DOC_SERV_EDITED.contains(&file_extension)
    }

    fn isCanConvert(&self,file_extension : &str) -> bool{
        DOC_SERV_CONVERT.contains(&file_extension)
    }

    fn isSupportedExtension(&self,file_extension : &str) -> bool{
        self.isCanFillForms(file_extension) || self.isCanView(file_extension) || self.isCanEdit(file_extension) || self.isCanConvert(file_extension)
    }

    fn getInternalExtension(&self,file_type : &str) -> String{
        let mapping = HashMap::from([
            ("word",".docx"),
            ("cell",".xlsx"),
            ("slide",".pptx"),
            ("docxf",".docxf")
        ]);

        mapping.get(file_type).map_or(".docx".to_owned(), |f| f.to_string())
    }

    fn getTemplateImageUrl(&self){
        todo!()
    }

    fn getCorrectName(&self,filename : &str){
        todo!()
    }

    fn getServerUrl(&self) -> String{
        DOCUMENT_SERVER_URL.to_owned()
    }
}