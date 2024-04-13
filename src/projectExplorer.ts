import { invoke } from "@tauri-apps/api/tauri";
import { createApp, ref } from "vue";
import FileIndicator from './components/NavBar/FileIndicator.vue';
import { FileType, WildcardFile } from "./fileType.ts";
import { WildcardDocument } from "./ts/document/wildcardDocument.ts";
import { Wildcard } from "./ts/data/wildcard.ts";

let item;

function addFileClickHandler(instance)
{
    instance.$el.addEventListener("mousedown", async function ()
    {
        var wildcardName = instance.$data.file.replace(/\.[^/.]+$/, "");
        const wildcard = ref<Wildcard>();
        wildcard.value = await invoke("load_wildcard", { name: wildcardName });

        var doc = new WildcardDocument(wildcard.value!);
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

    element.querySelector("#file-entry")!.prepend(icon.$el);
}

function createFileInstance(componentProperties)
{
    const tempDiv = document.createElement('div');
    return createApp(FileIndicator, componentProperties).mount(tempDiv);
}

function createSingleWildcard(wildcard: Wildcard)
{
    const instance = createFileInstance({ name: wildcard.data.name });
    addIconToElement(FileType.WILDCARD_STD, instance.$el);
    addFileClickHandler(instance);
    return instance;
}

function createCompWildcard(compWildcard)
{
    const subject = createFileInstance({ name: compWildcard.data.name });
    subject.$el.querySelector("#file-entry").classList.add("gtk1");
    addIconToElement(FileType.DIRECTORY, subject.$el);
    
    for (let i = 0; i < compWildcard.children.length; i++)
    {
        var item;
        if (compWildcard.children[i].Simple)
        {
            var wildcard = compWildcard.children[i].Simple as Wildcard;
            item = createSingleWildcard(wildcard);
        }
        if (compWildcard.children[i].Compository)
        {
            item = createCompWildcard(compWildcard.children[i].Compository);
        }
        
        subject.$el.querySelector("#children").appendChild(item.$el);
    }
    return subject;
}

function buildSubject(compWildcard)
{
    var hierarchy = document.getElementById('file-hierarchy')!;

    //addFileClickHandler(instance);
    hierarchy.appendChild(createCompWildcard(compWildcard).$el);
    
}

export async function buildProjectExplorer()
{
    // Set destination item
    item = document.getElementById('text-editor-0')?.querySelector('.line-container')!;

    const files = ref();
    files.value = await invoke('load_comp_wildcard');
    console.log(files.value);

    if (!files.value)
    {
        console.log("Backend returned no wildcards");
        return;
    }
    
    buildSubject(files.value);
}