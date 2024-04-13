import { invoke } from "@tauri-apps/api/tauri";
import { createApp, ref } from "vue";
import FileIndicator from './components/NavBar/FileIndicator.vue';
import { FileType, WildcardFile } from "./fileType.ts";
import { WildcardDocument } from "./ts/document/wildcardDocument.ts";
import { DocumentIndex } from "./ts/document/documentData.ts";
import { DocumentSpan } from "./ts/document/documentSpan.ts";

let item;

function addFileClickHandler(instance)
{
    instance.$el.addEventListener("mousedown", async function ()
    {
        var wildcardName = instance.$data.file.replace(/\.[^/.]+$/, "");
        const wildcard = ref();
        wildcard.value = await invoke("load_wildcard", { name: wildcardName });
        console.log(wildcard.value);
        

        var doc = new WildcardDocument(wildcard.value.content);
        item.innerHTML = '';
        item.appendChild(doc.render());

        var selected = document.querySelector('.file-entry.selected-entry');
        if (selected) selected.classList.remove('selected-entry');
        instance.$el.classList.add('selected-entry');
    });
}

function addIconToElement(type: FileType, element: HTMLElement)
{
    const icon = new WildcardFile(type).GetIconInstance();
    icon.$el.classList.add('file-icon');
    icon.$el.style = "height: 100%; margin-bottom: 5px;";

    element.prepend(icon.$el);
}

function createFileInstance(componentProperties)
{
    const tempDiv = document.createElement('div');
    return createApp(FileIndicator, componentProperties).mount(tempDiv);
}

export async function buildProjectExplorer()
{
    // Set destination item
    item = document.getElementById('text-editor-0')?.querySelector('.line-container')!;

    const files = ref();
    files.value = await invoke('load_wildcards');

    var hierarchy = document.getElementById('file-hierarchy')!;
    
    for (let i = 0; i < files.value.length; i++)
    {
        const instance = createFileInstance({ name: files.value[i].name });
        addIconToElement(FileType.WILDCARD_STD, instance.$el);
        addFileClickHandler(instance);
        
        hierarchy.appendChild(instance.$el);
    }
}