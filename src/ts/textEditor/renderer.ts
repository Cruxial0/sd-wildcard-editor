import { NodeConstructor, NodeRenderer } from "./nodeInstance";
import { ComponentNode, DocumentNode, TextNode } from "./nodes/baseNode";

// EXPERIMENTAL DOM BASED APPROACH. MIGHT REPLACE THE VUE COMPONENT BASED APPROACH IF NEEDED
export class Renderer
{
	constructor(private componentRegistry: Record<string, NodeConstructor>) { }
	
	renderLine(nodes: DocumentNode[]): HTMLElement
	{
		const lineElement = document.createElement('div');
		lineElement.className = 'editor-line';

		nodes.forEach(node =>
		{
			const nodeElement = this.renderNode(node);
			lineElement.appendChild(nodeElement);
		});

		return lineElement;
	}

	private renderNode(node: DocumentNode): HTMLElement
	{
		if (node.type === 'text')
		{
			return this.renderTextNode(node);
		}
		else if (node.type === 'component')
		{
			return this.renderComponentNode(node);
		}
		else
		{
			return document.createElement('span');
		}
	}

	private renderTextNode(node: TextNode): HTMLElement
	{
		const span = document.createElement('span');
		span.textContent = node.content;
		span.className = 'editor-text';
		return span;
	}

	private renderComponentNode(node: ComponentNode): HTMLElement
	{
		const ComponentClass = this.componentRegistry[node.componentType];
		if (!ComponentClass)
		{
			console.warn(`Unknown component type: ${node.componentType}`);
			return document.createElement('span');
		}

		const componentInstance = new ComponentClass(node.props);
		const wrapper = document.createElement('span');
		wrapper.className = 'editor-component';
		wrapper.appendChild(componentInstance.render());
		return wrapper;
	}
}

export class WildcardNode extends NodeRenderer
{
	public render(): HTMLElement
	{
		console.log(this.props);
		let element = document.createElement('div');
		element.innerHTML = this.props.content;
		element.style.color = this.props.color;
		return element;
	}
	
}

// const componentRegistry: Record<string, NodeConstructor> = {
// 	wildcard: WildcardNode
// }