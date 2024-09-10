import { Wildcard } from "../data/wildcard";
import { DOMDirection, DocumentIndex, offsetFromText } from "./documentData";
import { DocumentItem } from "./documentItem";
import { DocumentLine } from "./documentLine";
import { DocumentSpan } from "./documentSpan";

const DELAY = 5000;

export class WildcardDocument
{
    private wildcard: Wildcard;
    private element: HTMLElement;
    private margin: HTMLDivElement;
    private editor: HTMLDivElement;
    private prevIndex: DocumentIndex;
    private saved: boolean;
    private timer;
    
    private lines: DocumentLine[] = [];

    public render(): HTMLElement
    {
        this.renderLines();
        return this.element;
    }

    public unload()
    {
        this.lines = [];
        this.margin.innerHTML = '';
        this.editor.innerHTML = '';
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
        }
        
        var text = (this.index(idx.toLine()) as DocumentLine).breakText(idx);

        this.shiftLineIndicesDirectionally(1, idx.line! + 1, DOMDirection.FORWARD);

        var line = new DocumentLine(idx.line! + 1, this);
        line.insertCSV(text);
        this.lines.splice(idx.line! + 1, 0, line);

        this.margin.insertBefore(line.getIndexElement(), this.margin.childNodes[idx.line! + 1]);
        this.editor.insertBefore(line.get(), this.editor.childNodes[idx.line! + 1]);
        line.select();
        this.setIndex(new DocumentIndex(line.getIndex(), 0, 0), true, this.lineBreak.name);
        line.updateText();
        (this.index(idx.toLine()) as DocumentLine).getLast().updateVisualText();

        this.saved = false;
    }

    public deleteLine(idx: DocumentIndex)
    {
        if (!idx.isFull())
        {
            console.log("Can't delete a line with an incomplete DocumentIndex object. " + idx.toString());
            return;
        };
        
        if (idx.line! - 1 < 0) return;
        var text = this.lines[idx.line!].getText();
        var offset = offsetFromText(text);
        if (offset.span! > 0) offset.char = 0;
        
        console.log("LINE TEXT: " + text);
        //console.log("offset: " + offset.minus(new DocumentIndex(1, 1, 1)));

        var newLine = this.lines[idx.line! - 1];
        newLine.appendText(text);
        
        this.shiftLineIndicesDirectionally(-1, idx.line!, DOMDirection.FORWARD);

        var lastSpan = newLine.getLast();
        console.log("oldIdx: " + new DocumentIndex(newLine.getIndex(), lastSpan.getIndex(), lastSpan.getText().length));
        console.log(offset);
        
        var newIndex = new DocumentIndex(this.prevIndex.line! - 1, lastSpan.getIndex(), lastSpan.getText().length).minus(offset);
        if (newIndex.char! < 0) newIndex.char = 0;
        //console.log("newIdx: " + newIndex.minus(new DocumentIndex(1, 1, 1)));
        
        this.editor.removeChild(this.lines[idx.line!].get());
        this.margin.removeChild(this.lines[idx.line!].getIndexElement());
        this.lines.splice(idx.line!, 1);

        this.setIndex(newIndex, true, this.deleteLine.name);

        this.saved = false;
    }

    private addTag(idx: DocumentIndex)
    {
        var line = this.index(idx.toLine()) as DocumentLine;
        (line.index(idx.toSpan()) as DocumentSpan).insertText(', ', idx.char!);
        line.updateSelf();
        if (idx.char! == line.getLast().getText().length)
        {
            var span = document.createElement('span');
            line.get().children[0].appendChild(span);
        }
        
        var idx = this.prevIndex.plus([0, 2, 1]);
        idx.char = 0;
        this.setIndex(idx, true, "KeyComma");
    }

    public setIndex(idx: DocumentIndex, setCursor: boolean, caller)
    {
        console.log("AAA:" + (idx.span! == this.prevIndex.span!) + " (" + idx.span! + ", " + this.prevIndex.span! + ")");
        
        if (idx.span! != this.prevIndex.span! || idx.line! != this.prevIndex.line!)
        {
            
            if (this.prevIndex.char != null && this.prevIndex.line != null)
            {
                if(idx.line != this.prevIndex.line) (this.index(this.prevIndex.toLine()) as DocumentLine).updateSelf();
                
                var span = (this.index(this.prevIndex.toSpan()) as DocumentSpan).get();
                if(span.classList.contains('selected-span')) span.classList.remove('selected-span');
            }
                
            (this.index(idx.toSpan()) as DocumentSpan).get().classList.add('selected-span');
        }

        this.prevIndex = idx;
        console.log(caller + "->" + idx.minus(new DocumentIndex(1, 1, 1)));
        
        document.querySelector('#coordinate-label')!.innerHTML = idx.toString();

        var sel = window.getSelection()!.getRangeAt(0);
        if (idx.char! != sel.endOffset!)
        {
            console.log("INDEX MISMATCH: " + idx.char + " (expected: " + (sel.endOffset!) + ")");
        }
        if (setCursor) this.setCursorAtIndex(idx);
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
        let options = {
            root: document.querySelector(".line"),
            rootMargin: "0px",
            threshold: 1.0
        }
        let observer = new IntersectionObserver((_) => {}, options)
        observer.observe(this.editor);
        for (let i = 0; i < this.lines.length; i++) this.renderLine(i);
    }

    public isSaved(): boolean
    {
        return this.saved;
    }

    public getWildcard(): Wildcard
    {
        return this.wildcard;
    }

    private renderLine(idx: number)
    {
        this.margin.appendChild(this.lines[idx].getIndexElement());
        this.editor.appendChild(this.lines[idx].get());
    }

    private format()
    {
        this.element.className = "row";
        this.margin.className = "indexMargin column disableSelection";
        this.editor.className = "editor";

        this.element.style.height = "100%";

        this.editor.setAttribute('contenteditable', 'true');
        this.editor.setAttribute('spellcheck', 'false');

        this.element.appendChild(this.margin);
        this.element.appendChild(this.editor);
    }

    private setCursorAtIndex(idx: DocumentIndex)
    {
        var range = document.createRange();
        var node = (this.index(idx.toSpan()) as DocumentItem);
        console.log(node);

        if (!node.get().firstChild)
        {
            node.get().innerHTML = ' ';
        }

        range.setStart(node.get().firstChild ? node.get().firstChild! : node.get()!, idx.char!);
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
        var modifier = direction == DOMDirection.BACK ? -1 : 1;
        var idx = this.prevIndex.copy();
        var currSpan = this.index(idx.toSpan()) as DocumentSpan;
        var charLen = currSpan.getText().length;
        var spanLen = (currSpan.getParent() as DocumentLine).count();

        // Traverse span
        if ((idx.char! + modifier <= 0 || idx.char! + modifier > charLen) && (idx.span! + modifier >= 0 && idx.span! + modifier < spanLen))
        {
            idx.span! = idx.span! + modifier;
            var newSpan = (this.index(idx.toSpan()) as DocumentSpan);
            idx.char = direction == DOMDirection.BACK ? newSpan.getText().length : 1;
        }
        // Traverse line
        else if ((idx.char! + modifier < 0 || idx.char! + modifier > charLen) && (idx.span! + modifier <= 0 || idx.span! + modifier >= spanLen) && (idx.line! + modifier >= 0 && idx.line! + modifier <= this.lines.length - 1))
        {
            idx.line! = idx.line! + modifier;
            var newLine = (this.index(idx.toLine()) as DocumentLine);
            idx.span = direction == DOMDirection.BACK ? newLine.getLast().getIndex() : 0;
            idx.char = direction == DOMDirection.BACK ? newLine.getLast().getText().length : 0;
        }
        // Traverse char
        else if (idx.char! + modifier >= 0 && idx.char! + modifier <= charLen) idx.char! = idx.char! + modifier;

        this.setIndex(idx, true, this.arrowKeyHorizontal.name);
    }

    private arrowKeyVertical(direction: DOMDirection)
    {
        var modifier = direction == DOMDirection.BACK ? -1 : 1;

        var index = new DocumentIndex(null, null, null);
        var prev = this.prevIndex.copy();
        if (direction == DOMDirection.BACK) index.line = (prev.line! + modifier >= 0) ? prev.line! + modifier : 0;
        else index.line = (prev.line! + modifier < this.lines.length) ? prev.line! + modifier : this.lines.length - 1;
        
        var line = this.lines[index.line];

        var placeAtEnd = false;
        if (line.get().childNodes.length > prev.span!) index.span = prev.span;
        else
        {
            index.span = line.get().childNodes.length - 1;
            placeAtEnd = true;
        }
        
        var span = this.index(index) as DocumentSpan;
        
        if (span.get().innerHTML.length > prev.char! && !placeAtEnd) index.char = prev.char!;
        else index.char = span.get().innerHTML.length;
        
        this.setIndex(index, true, this.arrowKeyVertical.name);
    }

    private setupKeybinds()
    {
        var jumpSpan = false;
        this.element.addEventListener("keyup", (e) =>
        {
            console.log(e.code);
            
            switch (e.code)
            {
                case 'Enter':
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
                case 'Backspace':
                    (this.index(this.prevIndex.toSpan()) as DocumentSpan).updateVisualText();
                    if (jumpSpan)
                    {
                        var adjacentSpan = this.index(this.prevIndex.minus([0, 1, 0]).toSpan()) as DocumentSpan;
                        var newIndex = adjacentSpan.getFullIndex();
                        newIndex.char = adjacentSpan.getText().length;
                        this.setIndex(newIndex, true, "Key_Backspace (char == 0)");
                        jumpSpan = false;
                    }
                    break;
            }
        });

        this.element.addEventListener("keypress", (e) =>
        {
            console.log("PRESS:" + e.code);
            var span = this.index(this.prevIndex.toSpan()) as DocumentSpan;

            switch (e.code)
            {
                case 'Space':
                    e.preventDefault();              
                    span.insertText('&nbsp;', this.prevIndex.char!);
                    this.setIndex(this.prevIndex.plus([0, 0, 1]), true, "KeySpace");
                    this.saved = false;
                    this.resetTimer();
                    break;
                case 'Comma':
                    e.preventDefault();
                    this.addTag(this.prevIndex);
                    this.saved = false;
                    this.resetTimer();
                    break;
            }
        })

        this.element.addEventListener("keydown", (e) =>
        {
            switch (e.code)
            {
                case 'Enter':
                    e.preventDefault();
                    this.lineBreak(this.prevIndex);
                    this.resetTimer();
                    break;
                case 'Backspace':
                    if (this.prevIndex.char == 0 && this.prevIndex.span == 0)
                    {
                        e.preventDefault();
                        this.deleteLine(this.prevIndex);
                        return;
                    } else if (this.prevIndex.char == 1 && this.prevIndex.span != 0)
                    {
                        jumpSpan = true;
                        this.saved = false;
                        return;
                    }
                    var idx = this.prevIndex;
                    idx.char! -= 1;
                    this.setIndex(idx, false, "Key_Backspace (default)");
                    this.resetTimer();
                    break;
                case 'ArrowUp':
                    e.preventDefault();
                    this.arrowKeyVertical(DOMDirection.BACK);
                    break;
                case 'ArrowDown':
                    e.preventDefault();
                    this.arrowKeyVertical(DOMDirection.FORWARD);
                    break;
                case 'ArrowLeft':
                    e.preventDefault();
                    this.arrowKeyHorizontal(DOMDirection.BACK);
                    break;
                case 'ArrowRight':
                    e.preventDefault();
                    this.arrowKeyHorizontal(DOMDirection.FORWARD);
                    break; 
                    
            }
        });

        this.editor.addEventListener("input", (e) => {
            var data = ((e as InputEvent).data);         
            if (!data) return;
            var span = (this.index(this.prevIndex.toSpan()) as DocumentSpan);
            var idx = this.prevIndex.copy();
            idx.char! += data.length;
            span.updateVisualText();
            this.setIndex(idx, true, "userInput");
            this.saved = false;
            this.resetTimer();
        });

        this.element.addEventListener("click", (e) => {
            console.log("DOCUMENT TARGET: " + e.target);
        });

        this.editor.addEventListener("scroll", () =>
        {
            this.margin.style.overflowY = "scroll";
            this.margin.scrollTop = this.editor.scrollTop;
            this.margin.style.overflowY = "hidden";
        });
    }

    private resetTimer()
    {
        clearTimeout(this.timer);
        this.timer = setTimeout(this.save, DELAY);
    }

    private save()
    {
        this.saved = true;
        console.log("Saving document...");
    }

    private getLines(): string[]
    {
        var lines: string[] = []
        this.lines.forEach((x) =>
        {
            x.updateText();
            lines.push(x.getText());
        });
        console.log(lines);
        
        return lines;
    }

    private addButtonHandlers()
    { 
        // var writeBtn = this.element.querySelector("#writeBtn") as HTMLDivElement;
        // const data = ref();
        
        // writeBtn.onmousedown = () =>
        // {
        //     console.log("wrote text");
        //     this.wildcard.content = this.getLines();
        //     invoke('write_wildcard', { wildcard: this.wildcard });
        //     console.log("loading from db");
        //     data.value = invoke('load_wildcard_db').then((x) => console.log(x));
        // }
    }

    constructor(wildcard: Wildcard)
    {
        this.wildcard = wildcard;
        this.element = document.createElement('div');
        this.margin = document.createElement('div');
        this.editor = document.createElement('div');
        this.prevIndex = new DocumentIndex(null, null, null);
        this.saved = true;

        for (let i = 0; i < wildcard.content.length; i++)
        {
            var line = new DocumentLine(i, this);
            line.insertCSV(wildcard.content[i]);
            this.lines.push(line);
        }

        this.format();
        this.addButtonHandlers();
        this.setupKeybinds();
    }
}

