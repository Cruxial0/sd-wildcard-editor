import { insert } from "../helpers/stringHelpers";
import { DOMDirection, DocumentIndex, formatInput, formatOutput } from "./documentData";
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
        
        this.self.innerHTML = formatInput(text[1]);
        return text[0];
    }

    public getTextAfterIndex(_idx: DocumentIndex): string {
        return this.getTextFromIndex(0, DOMDirection.FORWARD)[0];
    }

    private updateSelectedIndex()
    {       
        ((this.parent as DocumentLine).getParent() as WildcardDocument).setIndex(this.getFullIndex(), false, this.updateSelectedIndex.name);
    }

    public updateVisualText()
    {
        this.visualText = formatOutput(this.self.innerHTML);
        if (this.visualText.endsWith('  ')) this.self.classList.add('warning');
        else if (this.self.classList.contains('warning')) this.self.classList.remove('warning');

        (this.parent as DocumentLine).updateText();
        
    }

    public select()
    {
        this.updateSelectedIndex();
    }

    public insertText(text: string, index: number)
    {
        this.self.innerHTML = insert(this.visualText, index, text);
        this.updateVisualText();
    }

    constructor(id: number, text: string, parent : DocumentLine | WildcardDocument)
    {
        super(id, parent);
        this.self = document.createElement('span');
        this.self.innerHTML = formatInput(text);   
        this.updateVisualText();

        if(id % 2 == 1) this.self.classList.add('gtk9')

        this.self.addEventListener("invoke", () =>
        {
            this.updateSelectedIndex();
            this.updateVisualText();
        });
    }
}