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