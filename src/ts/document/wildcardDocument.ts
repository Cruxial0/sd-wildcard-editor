import { DOMDirection, DocumentIndex, offsetFromText } from "./documentData";
import { DocumentItem } from "./documentItem";
import { DocumentLine } from "./documentLine";
import { DocumentSpan } from "./documentSpan";

export class WildcardDocument
{
    private element: HTMLElement;
    private margin: HTMLDivElement;
    private editor: HTMLDivElement;
    private prevIndex: DocumentIndex;
    
    private lines: DocumentLine[] = [];

    public render(): HTMLElement
    {
        this.renderLines();
        return this.element;
    }

    public index(idx: DocumentIndex): string | DocumentItem | null
    {
        if (idx.line == null) return null;
        if (this.lines.length < idx.line) return null;
        return this.lines[idx.line].index(idx);
    }

    public lineBreak(idx: DocumentIndex)
    {
        if (!idx.isFull())
        {
            console.log("Can't create a linebreak from an incomplete DocumentIndex object. " + idx.toString());
            return;
        };

        var span = this.index(idx.toSpan()) as DocumentSpan;
        var text = span.popTextFromIndex(idx.char!, DOMDirection.FORWARD);
        
        this.shiftLineIndicesDirectionally(1, idx.line! + 1, DOMDirection.FORWARD);

        var line = new DocumentLine(idx.line! + 1, this);
        line.insertCSV(text);
        this.lines.splice(idx.line! + 1, 0, line);

        this.margin.insertBefore(line.getIndexElement(), this.margin.childNodes[idx.line! + 1]);
        this.editor.insertBefore(line.get(), this.editor.childNodes[idx.line! + 1]);
        line.select();
    }

    public deleteLine(idx: DocumentIndex)
    {
        if (!idx.isFull())
        {
            console.log("Can't delete a line with an incomplete DocumentIndex object. " + idx.toString());
            return;
        };
        
        if (idx.line! - 1 < 0) return;
        var text = this.lines[idx.line!].getTextAfterIndex(idx);
        var offset = offsetFromText(text);
        console.log(text)
        console.log("offset: " + offset.minus(new DocumentIndex(1, 1, 1)));

        var newLine = this.lines[idx.line! - 1];
        newLine.appendText(text);
        this.editor.removeChild(this.lines[idx.line!].get());
        this.margin.removeChild(this.lines[idx.line!].getIndexElement())
        this.lines.splice(idx.line!, 1);
        this.shiftLineIndicesDirectionally(-1, idx.line!, DOMDirection.FORWARD);

        var lastSpan = newLine.getLast();
        console.log(lastSpan.get().innerHTML);
        var newIndex = new DocumentIndex(this.prevIndex.line! - 1, lastSpan.getIndex(), lastSpan.get().innerHTML.length - 1).minus(offset);
        console.log("newIdx: " + newIndex.minus(new DocumentIndex(1, 1, 1)));
        
        this.setIndex(newIndex, this.deleteLine.name);
        this.setCursorAtIndex(newIndex);
    }

    public setIndex(idx: DocumentIndex, caller)
    {
        this.prevIndex = idx;
        console.log(caller + "->" + idx.minus(new DocumentIndex(1, 1, 1)));
        
        document.querySelector('#coordinate-label')!.innerHTML = idx.toString();

        var sel = window.getSelection()!.getRangeAt(0);
        if (idx.char! != sel.endOffset! - 1)
        {
            console.log("INDEX MISMATCH: " + idx.char + " (expected: " + (sel.endOffset! - 1) + ")");
        }
    }

    private shiftLineIndicesDirectionally(change:number, idx: number, direction: DOMDirection)
    {
        for (let i = idx; direction == DOMDirection.BACK ? (i > 0) : (i < this.lines.length); direction == DOMDirection.BACK ? i-- : i++)
        {
            this.lines[i].shiftIndexBy(change);
            this.lines[i].update();
        }
    }

    private renderLines()
    {
        for (let i = 0; i < this.lines.length; i++) this.renderLine(i);
    }

    private renderLine(idx: number)
    {
        this.margin.appendChild(this.lines[idx].getIndexElement());
        this.editor.appendChild(this.lines[idx].get());
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

    private setCursorAtIndex(idx: DocumentIndex)
    {
        var range = document.createRange();
        var node = (this.index(idx.toSpan()) as DocumentItem);

        range.setStart(node.get().firstChild!, idx.char!);
        var sel = window.getSelection();
        range.collapse(true);
        sel?.removeAllRanges();
        sel?.addRange(range);
        var line = (node.getParent() as DocumentLine);
        line.select();

        node.get().focus();
    }

    private arrowKeyHorizontal(direction: DOMDirection)
    {
        
    }

    private arrowKeyVertical(direction: DOMDirection)
    {
        var modifier = direction == DOMDirection.BACK ? -1 : 1;

        var index = new DocumentIndex(null, null, null);
        if (direction == DOMDirection.BACK) index.line = (this.prevIndex.line! + modifier >= 0) ? this.prevIndex.line! + modifier : 0;
        else index.line = (this.prevIndex.line! + modifier < this.lines.length) ? this.prevIndex.line! + modifier : this.lines.length - 1;
        
        var line = this.lines[index.line];

        var placeAtEnd = false;
        if (line.get().childNodes.length > this.prevIndex.span!) index.span = this.prevIndex.span;
        else
        {
            index.span = line.get().childNodes.length - 1;
            placeAtEnd = true;
        }
        
        var span = this.index(index) as DocumentSpan;
        
        if (span.get().innerHTML.length > this.prevIndex.char! && !placeAtEnd) index.char = this.prevIndex.char!;
        else index.char = span.get().innerHTML.length;
        
        this.setIndex(index, this.arrowKeyVertical);
        this.setCursorAtIndex(index);
    }

    private setupKeybinds()
    {
        this.element.addEventListener("keyup", (e) =>
        {
            console.log(e.code);
            
            switch (e.code)
            {
                case 'Enter':
                    e.preventDefault();
                    break;
                case 'Space':
                    e.preventDefault();
                    break;
                case 'KeyC':
                    if (!e.ctrlKey) break;
                    e.preventDefault();
                    console.log(window.getSelection()?.getRangeAt(0));
                    break;
                case 'ArrowUp': break;
                case 'ArrowDown': break;
                case 'ArrowRight': break;
                case 'ArrowLeft': break;
                case 'Backspace': break;
                
                default:
                    this.prevIndex.char! += 1;
                    this.setIndex(this.prevIndex, "documentIncrement");
                    break;
            }
        });

        this.element.addEventListener("keydown", (e) =>
        {
            switch (e.code)
            {
                case 'Enter':
                    e.preventDefault();
                    this.lineBreak(this.prevIndex);
                    break;
                case 'Backspace':
                    if (this.prevIndex.char == 0 && this.prevIndex.span == 0)
                    {
                        e.preventDefault();
                        this.deleteLine(this.prevIndex);
                        return;
                    } else if (this.prevIndex.char == 0)
                    {
                        var adjacentSpan = this.index(new DocumentIndex(this.prevIndex.line, this.prevIndex.span! - 1, null)) as DocumentSpan;
                        var newIndex = new DocumentIndex(this.prevIndex.line, adjacentSpan.getIndex(), adjacentSpan.get().innerHTML.length - 1);
                        this.setIndex(newIndex, "Key_Backspace (char == 0)");
                        this.setCursorAtIndex(this.prevIndex);
                        return;
                    }
                    this.prevIndex.char! -= 1;
                    this.setIndex(this.prevIndex, "Key_Backspace (default)");
                    break;
                case 'ArrowUp':
                    e.preventDefault();
                    this.arrowKeyVertical(DOMDirection.BACK);
                    break;
                case 'ArrowDown':
                    e.preventDefault();
                    this.arrowKeyVertical(DOMDirection.FORWARD);
                    break;
                case 'ArrowRight':
                    e.preventDefault();
                    break;
                case 'ArrowLeft':
                    e.preventDefault();
                    break;
            }
        });

        this.element.addEventListener("input", (e) => {
            var data = ((e as InputEvent).data);
            if (!data) return;
            this.prevIndex.char! += data.length - 1;
            this.setIndex(this.prevIndex, "userInput");
        });

        this.element.addEventListener("click", (e) => {
            console.log("DOCUMENT TARGET: " + e.target);
        });
    }

    constructor(text: string[])
    {
        this.element = document.createElement('div');
        this.margin = document.createElement('div');
        this.editor = document.createElement('div');
        this.prevIndex = new DocumentIndex(null, null, null);
        for (let i = 0; i < text.length; i++)
        {
            var line = new DocumentLine(i, this);
            line.insertCSV(text[i]);
            this.lines.push(line);
        }
        this.format();
        this.setupKeybinds();
    }
}

