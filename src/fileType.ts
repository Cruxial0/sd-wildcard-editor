import FileIcon from "./components/Icons/FileIcon.vue";
import ComboIcon from "./components/Icons/ComboIcon.vue";
import FolderIcon from "./components/Icons/FolderIcon.vue";
import { createApp } from "vue";

export enum FileType
{
    DIRECTORY,
    WILDCARD_STD,
    WILDCARD_COMBO,
    SUBJECT
}

export class WildcardFile
{
    private icon;
    public GetIconInstance()
    {
        const tempDiv = document.createElement('div');
        return createApp(this.icon).mount(tempDiv);
    }
    constructor(fileType: FileType) {
        this.icon = getFileIcon(fileType);
    }
}

function getFileIcon(fileType: FileType)
{
    switch (fileType)
    {
        case FileType.DIRECTORY:
            return FolderIcon;
        case FileType.WILDCARD_STD:
            return FileIcon;
        case FileType.WILDCARD_COMBO:
            return ComboIcon;
        case FileType.SUBJECT:
            return;
        default:
            throw new Error("Invalid file type encountered.");
            
    }
}