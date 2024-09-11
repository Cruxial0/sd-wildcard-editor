import { ref } from "vue";
import { LoadWildcard } from "../data/wildcard";
import { WildcardDocument } from "../document/wildcardDocument";
import { ViewportItem } from "./viewportItem"
import { invoke } from "@tauri-apps/api";

export class ViewportTextEditor extends ViewportItem
{
    
    /**Used to load the wildcard document in case of app restart or if the tab was unloaded */
    private wildcardId: string;
    private viewportId: number;
    private document: WildcardDocument | undefined;

    override canUnload(): boolean
    {
        if (this.document)
        {
            return this.document.isSaved();
        }
        else
        {
            return false
        }
    }

    override async display(element: HTMLElement): Promise<void>
    {
        if (!this.isLoaded || !this.document)
        {
            LoadWildcard(this.wildcardId).then(x =>
            {
                this.document = new WildcardDocument(x, this.wildcardId);
                this.renderDoc();
                super.display(element);
                console.log("wildcard post display");
            });
            
            this.test();
            return;
        }
        // this.renderDoc();
        // super.display(element);
    }

    override unload(): void
    {
        console.log("Unloading document")
        this.document = undefined;
        this.isLoaded = false;
    }


    public async getName(): Promise<string | undefined>
    {
        const name = ref<string>();
        console.log("name id: " + this.wildcardId);
        name.value = await invoke("wildcard_name_from_id", { uuid: this.wildcardId });
        return name.value;
    }

    public getDataId(): number
    {
        return this.viewportId;
    }

    public test()
    {
        console.log("Test:");
        console.log(this.data);
    }

    private renderDoc()
    {
        console.log('loading');
        this.data!.querySelector('.line-container')!.innerHTML = '';
        this.data!.querySelector('.line-container')!.appendChild(this.document!.render());
    }

    constructor(data: HTMLElement, id: number, wildcardId: string)
    {
        super(data);
        this.data = data;
        this.viewportId = id;
        this.wildcardId = wildcardId;
        console.log('WILDCARD ID: ' + this.wildcardId);
    }
}