export abstract class ViewportItem
{
    public keepInCache: boolean = false;
    protected data: any;
    protected isLoaded: boolean = false;
    protected title: string | undefined;

    /**
     * Display this viewport element at the given element
     */
    public async display(element: HTMLElement)
    {
        // console.log(element.id);
        element.innerHTML = '';
        // console.log(this.data);
        element.appendChild(this.data!);
        this.isLoaded = true;
        return;
    }

    /**
     * Whether or not this element can be safely unloaded from memory
     */
    public canUnload(): boolean { throw new Error("Not Implemented") }

    public unload()
    {   
        this.data = undefined;
        this.isLoaded = false;
    }

    public async getName(): Promise<string | undefined>
    {
        return this.title;
    }

    /**
     * A method to retrieve the ID for the data contained within this viewport. Should always be overridden
     * @returns The ID for the corresponding data
     */
    public abstract getDataId(): number

    public constructor(_data: any){ }
}