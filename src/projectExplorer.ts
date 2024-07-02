import { invoke } from "@tauri-apps/api/tauri";
import { createApp, ref } from "vue";
import FileIndicator from './components/NavBar/FileIndicator.vue';
import { FileType, WildcardFile } from "./fileType.ts";
import { LoadWildcard, Wildcard } from "./ts/data/wildcard.ts";
import { AddViewportTab, AddViewportTextEditor, DisplayViewport } from "./ts/viewport/viewportHelper.ts";

let item;

async function addFileClickHandler(instance)
{
    const wildcard = ref<Wildcard>();
    wildcard.value = await LoadWildcard(instance.$data.id);
    

    instance.$el.addEventListener("mousedown", async function ()
    {
        let id = AddViewportTextEditor(wildcard.value!.id);
        await AddViewportTab(id);
        await DisplayViewport(id, item as HTMLElement);

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
    const instance = createFileInstance({ name: wildcard.name, wildcardId: wildcard.id });
    addIconToElement(FileType.WILDCARD_STD, instance.$el);
    addFileClickHandler(instance);
    return instance;
}

function createCompWildcard(compWildcard)
{
    const subject = createFileInstance({ name: compWildcard.name, wildcardId: compWildcard.id });
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
    // InitializeViewportHelper();
    item = document.getElementById('viewport-content')!;

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