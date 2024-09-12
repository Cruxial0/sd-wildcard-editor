
export interface NodeConstructor
{
    new (props: Record<string, any>);
}

// Helper class to make sure classes are properly structured
export abstract class NodeRenderer
{
    constructor(protected props: Record<string, any>) {}
    public abstract render(): HTMLElement;
}