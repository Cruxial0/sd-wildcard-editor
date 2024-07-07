import { ref } from "vue";
import { LoadWildcard } from "../data/wildcard";
import { WildcardDocument } from "../document/wildcardDocument";
import { ViewportItem } from "./viewportItem"
import { invoke } from "@tauri-apps/api";

export class ViewportTextEditor extends ViewportItem
{
    
    /**Used to load the wildcard document in case of app restart or if the tab was unloaded */
    private wildcardId: number;
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

    override display(element: HTMLElement): void
    {
        if (!this.isLoaded || !this.document)
        {
            LoadWildcard(this.wildcardId)
                .then((x) =>
                {
                    this.document = new WildcardDocument(x);
                    this.renderDoc();
                    super.display(element);
                });
            
            return;
        }

        this.renderDoc();
        super.display(element);
    }

    override unload(): void
    {
        this.document = undefined;
        this.isLoaded = false;
    }


    public async getName(): Promise<string | undefined>
    {
        const name = ref<string>();
        name.value = await invoke("wildcard_name_from_id", { id: this.wildcardId });
        return name.value;
    }

    public getDataId(): number
    {
        return this.wildcardId;
    }

    private renderDoc()
    {
        console.log('loading');
        this.data?.querySelector('.line-container')!.appendChild(this.document!.render());
    }

    constructor(data: HTMLElement, wildcardId: number)
    {
        super(data);
        this.data = data;
        this.wildcardId = wildcardId;
        console.log('WILDCARD ID: ' + this.wildcardId);
    }
}