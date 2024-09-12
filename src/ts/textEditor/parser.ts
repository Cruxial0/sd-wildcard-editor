import { ComponentNode, DelimiterNode, DocumentNode, TextNode } from "./nodes/baseNode";
import { Token } from "./tokenizer/tokenizer";

type componentType = 'wildcard' | 'number' | 'other'

export class Parser {
    parse(tokens: Token[]): DocumentNode[]
    {
        const nodes: DocumentNode[] = [];
        let currentTextContent = '';

        const flushTextNode = () =>
        {
            if (currentTextContent)
            {
                nodes.push(this.createTextNode(currentTextContent));
                currentTextContent = '';
            }
        }

        tokens.forEach((token) =>
        {
            switch (token.type)
            {
                case "text":
                    currentTextContent += token.value;
                    break;
                case "component":
                    flushTextNode();
                    nodes.push(this.createComponentNode(token, 'other'));
                    break;
                case "wildcard":
                    flushTextNode();
                    nodes.push(this.createComponentNode(token, 'wildcard'));
                    break;
                case "delimeter":
                    flushTextNode();
                    nodes.push(this.createDelimiterNode(token));
                    break;
                case "number":
                    flushTextNode();
                    nodes.push(this.createComponentNode(token, 'number'));
                    break;
                case "unknown":
                    currentTextContent += token.value;
                    break;
                default:
                    console.warn(`Unknown token type: ${(token as Token).type}`);
            }
        });

        flushTextNode();

        return nodes;
    }

    private createTextNode(content: string): TextNode
    {
        return {
            type: 'text',
            content
        }
    }

    private createDelimiterNode(token: Token): DelimiterNode
    {
        return {
            type: 'delimiter',
            delimiter: token.value
        }
    }
    

    private createComponentNode(token: Token, type: componentType): ComponentNode
    {
        var props: Record<string, any> = {};
        switch (type)
        {
            case "number": props = { content: token.value }; break;
            case "wildcard": props = { content: token.value, color: 'red' }; break;
            case "other": props = { content: token.value }; break;
        }

        return {
            type: 'component',
            componentType: type,
            props: props
        }
    }
}