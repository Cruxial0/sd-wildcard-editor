import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

export async function getUUID(): Promise<string>
{
    let uuid = ref<string>('');
    uuid.value = await invoke('get_uuid');
    return uuid.value;
}

export async function getNameByUUID(uuid: String): Promise<string>
{
    let name = ref<string>('');
    console.log("loading name: " + uuid);
    name.value = await invoke('get_name_by_uuid', {uuid: uuid});
    return name.value;
}