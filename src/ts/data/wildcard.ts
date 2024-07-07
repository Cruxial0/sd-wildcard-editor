import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

export class Wildcard
{
    name: string;
    id: number;
    content: string[];

    constructor(json)
    {
        this.name = json.name;
        this.id = parseInt(json.id);
        this.content = json.content;
    }
}

export async function LoadWildcard(id: number): Promise<Wildcard>
{
    const wildcard = ref<Wildcard>();
    wildcard.value = await invoke("load_wildcard", { id: id });
    return new Wildcard(wildcard.value!)
}