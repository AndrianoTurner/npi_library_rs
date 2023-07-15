export default class Book{
    constructor(id,title,discipline,owner_id,filename,downloadUrl,callbackUrl,filetype,key,documentType){
        this.id = id;
        this.title = title
        this.discipline = discipline
        this.owner_id = owner_id
        this.filename = filename
        this.downloadUrl = downloadUrl
        this.callbackUrl = callbackUrl
        this.filetype = filetype
        this.key = key
        this.documentType = documentType 
    }

    link() {
        return `book/${this.id}`
    }
}
