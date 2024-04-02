import { addSelectionListener, addSelectionListenerWithCallback } from "./selectionListeners";

export class WildcardDocument
{
    private element: HTMLElement;
    private margin: HTMLDivElement;
    private editor: HTMLDivElement;
    
    private lines: DocumentLine[] = [];

    public render(): HTMLElement
    {
        this.renderLines();
        return this.element;
    }

    private renderLines()
    {
        for (let i = 0; i < this.lines.length; i++)
        {
            this.margin.appendChild(this.lines[i].getIndexElement());
            this.editor.appendChild(this.lines[i].get());
        }
    }

    private format()
    {
        this.element.className = "row";
        this.margin.className = "column disableSelection";
        this.editor.className = "editor";

        this.element.style.height = "100%";

        this.editor.setAttribute('contenteditable', 'true');
        this.editor.setAttribute('spellcheck', 'false');
        this.editor.style.width = "100%";

        this.element.appendChild(this.margin);
        this.element.appendChild(this.editor);
    }

    constructor(text: string[])
    {
        this.element = document.createElement('div');
        this.margin = document.createElement('div');
        this.editor = document.createElement('div');
        for (let i = 0; i < text.length; i++)
        {
            var line = new DocumentLine(i);
            line.insertCSV(text[i]);
            this.lines.push(line);
        }
        this.format();
    }
}

class DocumentStyle
{
    public lineHeight = '18px';
}

class DocumentItem
{
    protected index: number = 0;
    protected self: HTMLElement;

    public get(): HTMLElement { return this.self }

    constructor(id: number)
    {
        this.index = id;
        this.self = document.createElement('div');
    }
    
    public setIndex(index: number) {
        this.index = index;
    }

    public getIndex()
    {
        return this.index;
    } 
}

class DocumentLine extends DocumentItem
{
    SPLIT_REGEX = /(,)/g;
    private spans: DocumentSpan[] = [];
    private content: HTMLDivElement;
    private margin: HTMLDivElement;
    public insertCSV(text: string)
    {
        var index = 0;
        text.split(this.SPLIT_REGEX).forEach((x) =>
        {
            this.spans.push(new DocumentSpan(index, x))
            index++;
        });

        this.spans.forEach((y) => this.content.appendChild(y.get()));
    }

    public getIndexElement(): HTMLDivElement
    {
        return this.margin;
    }

    private generateIndexElement(): HTMLDivElement
    {
        var index = document.createElement('div');
        index.className = "index";
        index.innerHTML = (this.index + 1).toString();
        index.addEventListener("click", () => this.select());
        return index;
    }

    public select()
    {
        this.self.dispatchEvent(new Event("click"));
        var range = new Range();
        range.setStart(this.spans[0].get(), 0);
        range.setEnd(this.spans[this.spans.length - 1].get(), 1)
        
        var sel = window.getSelection();
        sel?.removeAllRanges();
        sel?.addRange(range);
    }

    public updateSelf()
    {
        var sections = this.self.querySelectorAll('span');
        var text = "";
        sections.forEach((x) => text += x);
        this.content.innerHTML = '';
        this.spans = [];
        this.insertCSV(text);
    }

    private format()
    {
        this.self.id = "line-" + this.index;
        this.self.className = 'line';
        this.self.appendChild(this.content);
        this.content.className = 'content';

        this.self.addEventListener("click", () =>
        {
            var selectedLine = document.querySelector('.line.selected-line');
            if (selectedLine) selectedLine.classList.remove('selected-line');
            var selectedIndex = document.querySelector('.index.selected-line');
            if (selectedIndex) selectedIndex.classList.remove('selected-line');

            this.margin.classList.add('selected-line');
            this.self.classList.add('selected-line');
        });
    }

    constructor(id: number)
    {
        super(id);
        this.self = document.createElement('div');
        this.content = document.createElement('div');
        this.margin = this.generateIndexElement();
        this.format();
    }
}

class DocumentSpan extends DocumentItem
{
    constructor(id: number, text: string)
    {
        super(id);
        this.self = document.createElement('span');
        this.self.innerHTML = text;
    }
}