import { ViewportItem } from "./viewportItem";

export class ViewportMergePatternEditor extends ViewportItem
{
    private id: number = -1;
    public getDataId(): number
    {
        return this.id;
    }

    public display(element: HTMLElement)
    {
        console.log(element.id);
        element.innerHTML = '';
        console.log(this.data);
        element.appendChild(this.data!);
        this.isLoaded = true;
    }
    
    constructor(data: HTMLElement, id: number, wildcard) {
        super(data);
        
        this.data = data;
        this.title = "Merge Patterns (" + wildcard.name + ")";
        this.id = id;
        console.log("DATA:");
        console.log(this.data);
    }
}