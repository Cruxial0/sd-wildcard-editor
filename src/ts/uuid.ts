import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

export async function getUUID(): Promise<string>
{
    let uuid = ref<string>('');
    uuid.value = await invoke('get_uuid');
    return uuid.value;
}