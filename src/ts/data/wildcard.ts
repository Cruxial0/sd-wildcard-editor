class WildcardData
{
    name: string;
    content: string[];
    abs_path: string;

    constructor(name: string, content: string[], path: string)
    {
        this.name = name;
        this.content = content;
        this.abs_path = path;
    }
}

export class Wildcard
{
    data: WildcardData;

    constructor(json)
    {
        this.data = json.data;
    }
}