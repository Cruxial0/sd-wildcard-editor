import { DOMDirection, DocumentIndex, formatInput } from "./documentData";
import { DocumentItem } from "./documentItem";
import { DocumentLine } from "./documentLine";
import { WildcardDocument } from "./wildcardDocument";

export class DocumentSpan extends DocumentItem
{
    INTERACT_EVENT = 'mousedown';
    public getFullIndex(): DocumentIndex
    {      
        return new DocumentIndex((this.parent as DocumentLine).getIndex(), this.idx, this.getCursorPosition());
    }

    public index(idx: DocumentIndex): string | DocumentItem | null
    {
        if (idx.char == null && idx.span == null) return null;
        if (idx.char == null) return this;
        if (this.self.innerHTML.length < idx.char!) return null;
        return this.self.innerHTML[idx.char!];
    }

    public popTextFromIndex(idx: number, direction: DOMDirection)
    { 
        var text = this.getTextFromIndex(idx, direction);
        
        this.self.innerHTML = this.self.innerHTML.replace(text, '');
        return text;
    }

    public getTextAfterIndex(_idx: DocumentIndex): string {
        return this.getTextFromIndex(0, DOMDirection.FORWARD);
    }

    public getTextFromIndex(idx: number, direction: DOMDirection)
    {
        return this.self.innerHTML.substring(
            direction == DOMDirection.FORWARD ? idx : 0,
            direction == DOMDirection.FORWARD ? this.self.innerHTML.length : idx);
    }

    private updateSelectedIndex()
    {   
        ((this.parent as DocumentLine).getParent() as WildcardDocument).setIndex(this.getFullIndex(), this.updateSelectedIndex.name);
    }

    public select()
    {
        this.updateSelectedIndex();
    }

    constructor(id: number, text: string, parent : DocumentLine | WildcardDocument)
    {
        super(id, parent);
        this.self = document.createElement('span');
        this.self.innerHTML = formatInput(text);
        this.parent = parent;

        this.self.addEventListener("invoke", () =>
        {
            this.updateSelectedIndex();
        });
    }
}