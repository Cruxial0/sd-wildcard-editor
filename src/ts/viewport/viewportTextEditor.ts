import { ref } from "vue";
import { LoadWildcard } from "../data/wildcard";
import { ViewportItem } from "./viewportItem"
import { invoke } from "@tauri-apps/api";

export class ViewportTextEditor extends ViewportItem
{
    
    /**Used to load the wildcard document in case of app restart or if the tab was unloaded */
    private wildcardId: string;
    private viewportId: number;

    override canUnload(): boolean
    {
        return false
    }

    override unload(): void
    {
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

    constructor(data: HTMLElement, id: number, wildcardId: string)
    {
        super(data);
        this.data = data;
        this.viewportId = id;
        this.wildcardId = wildcardId;
        console.log('WILDCARD ID: ' + this.wildcardId);
    }
}