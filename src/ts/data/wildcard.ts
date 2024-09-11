import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

export class Wildcard
{
    name: string;
    uuid: string;
    content: string[];

    constructor(json)
    {
        this.name = json.name;
        this.uuid = json.uuid;
        this.content = json.content;
    }
}

export async function LoadWildcard(uuid: string): Promise<Wildcard>
{
    const wildcard = ref<Wildcard>();
    wildcard.value = await invoke("load_wildcard", { uuid: uuid });
    return wildcard.value!;
}