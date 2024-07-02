export enum ViewportType
{
    TextEditor,
    StyleEditor,
    Settings,
    Search
}

export function setViewport(viewportType: ViewportType)
{
    switch (viewportType)
    {
        case ViewportType.TextEditor: 
        case ViewportType.StyleEditor:
        case ViewportType.Settings:
        case ViewportType.Search:
    }
}