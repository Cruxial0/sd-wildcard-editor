import { ComponentNode, DelimiterNode, DocumentNode, TextNode } from "./nodes/baseNode";
import { Token } from "./tokenizer";

type componentType = 'wildcard' | 'other'

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
        if (type == 'other')
        {
            return {
                type: 'component',
                componentType: type,
                props: {content: token.value}
            }
        }
        else
        {
            return {
                type: 'component',
                componentType: type,
                props: {content: token.value, color: 'red'}
            }
        }
    }
}