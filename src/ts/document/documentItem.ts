import { DocumentIndex } from "./documentData";
import { WildcardDocument } from "./wildcardDocument";

export class DocumentItem
{
    protected EVENT = new CustomEvent("invoke");
    protected parent : DocumentItem | WildcardDocument;
    protected idx: number = 0;
    protected self: HTMLElement;

    public get(): HTMLElement { return this.self }
    public update(): void { return }
    public select(): void { return }
    public index(idx: DocumentIndex): DocumentItem | string | null { return "Not Implemented: \n" + idx }
    public getFullIndex(idx: DocumentIndex): DocumentIndex { return idx; }
    public getTextAfterIndex(idx: DocumentIndex) : string { return idx.toString() }

    constructor(id: number, parent : DocumentItem | WildcardDocument)
    {
        this.idx = id;
        this.self = document.createElement('div');
        this.parent = parent;
    }

    public getParent() : DocumentItem | WildcardDocument
    {
        return this.parent;
    }
    
    public setIndex(index: number) {
        this.idx = index;
    }

    public shiftIndexBy(change: number)
    {
        this.idx = this.idx + change;
    }

    public getIndex()
    {
        return this.idx;
    } 

    protected getCursorPosition(): number | null
    {
        var sel = window.getSelection();
        return sel?.getRangeAt(sel.rangeCount - 1).endOffset!;
    }
}