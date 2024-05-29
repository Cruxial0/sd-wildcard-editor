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
        // var wildcardName = instance.$data.file.replace(/\.[^/.]+$/, "");
        const wildcard = ref<Wildcard>();
        wildcard.value = await invoke("load_wildcard", { id: instance.$data.id });

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
    icon.$el.style = "height: 100%; aspect-ratio: 1 / 1; margin-bottom: 5px;";

    element.querySelector("#file-entry")!.prepend(icon.$el);
}

function createFileInstance(componentProperties)
{
    const tempDiv = document.createElement('div');
    return createApp(FileIndicator, componentProperties).mount(tempDiv);
}

function createSingleWildcard(wildcard: Wildcard)
{
    const instance = createFileInstance({ name: wildcard.name, id: wildcard.id });
    addIconToElement(FileType.WILDCARD_STD, instance.$el);
    addFileClickHandler(instance);
    return instance;
}

function createCompWildcard(compWildcard)
{
    const subject = createFileInstance({ name: compWildcard.name, id: compWildcard.id });
    subject.$el.querySelector("#file-entry").classList.add("gtk1");
    addIconToElement(FileType.DIRECTORY, subject.$el);
    
    for (let i = 0; i < compWildcard.projects.length; i++)
    {
        var project = createCompWildcard(compWildcard.projects[i]);
        subject.$el.querySelector("#children").appendChild(project.$el);
    }

    for (let i = 0; i < compWildcard.wildcards.length; i++)
    {
        var wildcard = compWildcard.wildcards[i] as Wildcard;
        var item = createSingleWildcard(wildcard);
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
    // var writeBtn = document.querySelector("#writeBtn") as HTMLDivElement;
    // console.log(writeBtn);
    // const data = ref();
    
    // writeBtn.onmousedown = () =>
    // {
    //     console.log("loading from db");
    //     data.value = invoke('load_wildcard_db').then((x) => console.log(x));
    // }

    // Set destination item
    item = document.getElementById('text-editor-0')?.querySelector('.line-container')!;

    const files = ref();
    files.value = await invoke('load_workspace');
    console.log(files.value);

    if (!files.value)
    {
        console.log("Backend returned no wildcards");
        return;
    }
    
    buildSubject(files.value);
}