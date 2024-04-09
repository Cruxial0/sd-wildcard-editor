export var SPLIT_REGEX = /(, )/g;

export class DocumentIndex
{
    public line: number | null;
    public span: number | null;
    public char: number | null;

    constructor(line: number | null, span: number | null, char: number | null)
    {
        this.line = line;
        this.span = span;
        this.char = char;
    }

    public isFull()
    {   
        return !(this.line == null || this.span == null || this.char == null);
    }

    public toLine()
    {
        return new DocumentIndex(this.line, null, null);
    }

    public toSpan()
    {
        return new DocumentIndex(this.line, this.span, null);
    }

    
    public minus(idx: DocumentIndex): DocumentIndex;
    public minus(idx: [number, number, number]): DocumentIndex;
    public minus(idx: DocumentIndex | [number, number, number]): DocumentIndex
    {
        console.log((idx as DocumentIndex).span!);
        var newIdx = new DocumentIndex(null, null, null);
        newIdx.line = this.handleSubtraction(this.line, idx instanceof DocumentIndex ? idx.line! : idx[0]);
        newIdx.span = this.handleSubtraction(this.span, idx instanceof DocumentIndex ? idx.span! : idx[1]);
        newIdx.char = this.handleSubtraction(this.char, idx instanceof DocumentIndex ? idx.char! : idx[2]);
        return newIdx;
    }

    public plus(idx: [number, number, number]): DocumentIndex;
    public plus(idx: DocumentIndex): DocumentIndex
    public plus(idx: DocumentIndex | [number, number, number]): DocumentIndex
    {
        var newIdx = new DocumentIndex(null, null, null);
        newIdx.line = this.handleAddition(this.line, idx instanceof DocumentIndex ? idx.line! : idx[0]);
        newIdx.span = this.handleAddition(this.span, idx instanceof DocumentIndex ? idx.span! : idx[1]);
        newIdx.char = this.handleAddition(this.char, idx instanceof DocumentIndex ? idx.char! : idx[2]);
        return newIdx;
    }

    public copy(): DocumentIndex
    {
        return new DocumentIndex(this.line, this.span, this.char);
    }

    private handleSubtraction(part1, part2): number
    {    
        if (part1 == null) part1 == 0;
        if (part2 == null) part2 == 0;
        return part1 - part2;
    }

    private handleAddition(part1, part2): number
    {
        if (part1 == null) part1 == 0;
        if (part2 == null) part2 == 0;
        return part1 + part2;
    }

    public toString(): string
    {
        return "(" + ((this.line != null) ? this.line + 1 : "_") + ", " + ((this.span != null) ? this.span + 1 : "_") + ", " + ((this.char != null) ? this.char + 1 : "_") + ")";
    }
}

// class DocumentStyle
// {
//     public lineHeight = '18px';
// }

export enum DOMDirection
{
    BACK = 0,
    FORWARD = 1
}

export function formatInput(input: string): string
{
    var output = "";
    output = input.replace(/ /g, '&nbsp;');
    return output;
}

export function formatOutput(output: string): string
{
    var input = "";
    input = output.replace(/&nbsp;/g, ' ').replace(/<br>/g, '');
    return input;
}

export function offsetFromText(text: string): DocumentIndex
{
    var spans = formatOutput(text).split(SPLIT_REGEX);
    var chars = spans.splice(spans.length - 1, 1)[0];
    console.log(chars);
    
    return new DocumentIndex(0, spans.length, chars.length);
}