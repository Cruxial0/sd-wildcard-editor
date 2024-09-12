export type NodeType = 'text' | 'component' | 'delimiter';

export interface BaseNode {
	type: NodeType;
}

export interface TextNode extends BaseNode {
	type: 'text';
	content: string;
}

export interface ComponentNode extends BaseNode {
	type: 'component';
	componentType: string;
	props: Record<string, any>;
}

export interface DelimiterNode extends BaseNode
{
	type: 'delimiter';
	delimiter: string;
}

export type DocumentNode = TextNode | ComponentNode | DelimiterNode;