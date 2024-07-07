import { DOMDirection, DocumentIndex, SPLIT_REGEX, formatOutput } from "./documentData";
import { DocumentItem } from "./documentItem";
import { DocumentSpan } from "./documentSpan";
import { WildcardDocument } from "./wildcardDocument";

export class DocumentLine extends DocumentItem
{
    INTERACT_EVENT = 'click';
    private spans: DocumentSpan[] = [];
    private content: HTMLDivElement;
    private margin: HTMLDivElement;

    public insertCSV(text: string)
    {
        var index = 0;
        text.split(SPLIT_REGEX).forEach((x) =>
        {
            this.spans.push(new DocumentSpan(index, x, this))
            index++;
        });

        this.spans.forEach((y) => this.content.appendChild(y.get()));
        // console.log("Line initialized (ID: " + this.idx + "): " + this.spans.length + " children created");
        
    }

    public index(idx: DocumentIndex): string | DocumentItem | null {
        if (idx.line == null) return null;
        if (idx.span == null) return this;
        if (this.spans.length < idx.span!) return null;
        return this.spans[idx.span!].index(idx);
    }

    public getIndexElement(): HTMLDivElement
    {
        return this.margin;
    }

    private generateIndexElement(): HTMLDivElement
    {
        var index = document.createElement('div');
        var span = document.createElement('a');
        span.innerHTML = (this.idx + 1).toString();
        index.className = "index";
        index.appendChild(span);
        index.addEventListener(this.INTERACT_EVENT, () => this.select());
        return index;
    }

    public select()
    {
        var selectedLine = document.querySelector('.line.selected-line');
        if (selectedLine) selectedLine.classList.remove('selected-line');
        var selectedIndex = document.querySelector('.index.selected-line');
        if (selectedIndex) selectedIndex.classList.remove('selected-line');

        this.margin.classList.add('selected-line');
        this.self.classList.add('selected-line');
    }

    public selectWithMarkup()
    {
        this.self.dispatchEvent(new Event(this.INTERACT_EVENT));
        var range = new Range();
        range.setStart(this.spans[0].get(), 0);
        range.setEnd(this.spans[this.spans.length - 1].get(), 1);
        
        var sel = window.getSelection();
        sel?.removeAllRanges();
        sel?.addRange(range);
    }

    public update(): void {
        this.margin.innerHTML = (this.idx + 1).toString();
        this.self.id = 'line-' + this.idx;
    }
    
    public getTextAfterIndex(idx: DocumentIndex): string {
        var text = "";

        for (let i = idx.span!; i < this.spans.length; i++)
            text += this.spans[i].getTextFromIndex(0, DOMDirection.FORWARD);

        return text;
    }
    
    public breakText(idx: DocumentIndex): string
    {
        if (idx.char == null || idx.span == null) return "";

        console.log(this.spans);
        console.log(this.spans[idx.span]);
        
        var text = this.spans[idx.span].popTextFromIndex(idx.char, DOMDirection.FORWARD);
        var targets: DocumentSpan[] = this.spans.splice(idx.span + 1, (this.spans.length - 1) - idx.span + 1);
        targets.forEach((x) =>
        {
            var val = x.getTextFromIndex(0, DOMDirection.FORWARD)[0];
            text += val;
            this.content.removeChild(x.get());
        });
        console.log(text);
        
        return text;
    }

    public appendText(text: string)
    {
        var oldText = this.spans[this.spans.length - 1].get().innerHTML;
        text = oldText + text;
        
        this.spans[this.spans.length - 1].get().innerHTML = text;
        this.updateSelf();
    }

    public getLast(): DocumentSpan
    {
        return this.spans[this.spans.length - 1];
    }

    public getFirst(): DocumentSpan
    {
        return this.spans[0];
    }

    public count(): number
    {
        return this.spans.length;
    }

    public updateSelf()
    {
        var sections = this.self.querySelectorAll('span');
        var text = "";
        sections.forEach((x) => text += formatOutput(x.innerHTML));
        this.content.innerHTML = '';
        this.spans = [];
        this.insertCSV(text);
    }

    public updateText()
    {
        var text = "";
        this.spans.forEach((x) => { text += x.getText(); });
        this.visualText = text;
    }

    private format()
    {
        this.self.id = "line-" + this.idx;
        this.self.className = 'line';
        this.self.appendChild(this.content);
        this.content.className = 'content';

        this.self.addEventListener(this.INTERACT_EVENT, (e) =>
        {
            this.select();
            var selection = this.getCursorPosition()!;

            if ((e.target as HTMLElement).nodeName == 'DIV')
            {
                var span: DocumentSpan;
                if (selection > 0)
                {
                    span = this.spans[this.spans.length - 1];
                    (this.parent as WildcardDocument).setIndex(new DocumentIndex(this.idx, span.getIndex(), formatOutput(span.get().innerHTML).length), false, "lineClick");
                }
                if (selection == 0) (this.parent as WildcardDocument).setIndex(new DocumentIndex(this.idx, 0, 0), false, "lineClick");
            }
            else
            {
                e.target?.dispatchEvent(new CustomEvent("invoke"));
            }
            
        });
    }

    constructor(id: number, parent : DocumentItem | WildcardDocument)
    {
        super(id, parent);
        this.self = document.createElement('div');
        this.content = document.createElement('div');
        this.margin = this.generateIndexElement();
        this.format();
        this.updateText();
    }
}
