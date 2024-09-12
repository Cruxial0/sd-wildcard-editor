import { ref } from "vue";
import { LoadWildcard, Wildcard } from "../data/wildcard";
import { DocumentNode } from "./nodes/baseNode";
import { Parser } from "./parser";
import { Position } from "./types";
import { Tokenizer } from "./tokenizer/tokenizer";

export class DocumentModel {
	lines: DocumentNode[][] = [];
	private uuid: string;

	constructor(text: string)
	{
		this.uuid = text;
		console.log(text);
	}

	async load()
	{
		let wildcard = await LoadWildcard(this.uuid);
		const tokenizer = new Tokenizer();
		const parser = new Parser();

		wildcard.content.forEach(line =>
		{
			// let tokens = tokenizer.tokenize(line.replace(/(\r\n|\n|\r)/gm, ""));
			let tokens = tokenizer.tokenize(line.trim());
			let nodes = parser.parse(tokens);
			this.lines.push(nodes);
		});
	}

	positionToIndex(position: Position): number {
	let index = 0;
	for (let i = 0; i < position.lineIndex; i++) {
		index += this.getLineLength(i);
	}
	for (let i = 0; i < position.nodeIndex; i++) {
		index += this.getNodeLength(this.lines[position.lineIndex][i]);
	}
		return index + position.offset;
	}

	indexToPosition(index: number): Position {
	let remainingChars = index;
	for (let lineIndex = 0; lineIndex < this.lines.length; lineIndex++) {
		const lineLength = this.getLineLength(lineIndex);
		if (remainingChars < lineLength) {
		for (let nodeIndex = 0; nodeIndex < this.lines[lineIndex].length; nodeIndex++) {
			const nodeLength = this.getNodeLength(this.lines[lineIndex][nodeIndex]);
			if (remainingChars < nodeLength) {
				return { lineIndex, nodeIndex, offset: remainingChars };
			}
			remainingChars -= nodeLength;
		}
		}
		remainingChars -= lineLength;
	}
	throw new Error('Index out of bounds');
	}

	private getLineLength(lineIndex: number): number {
	return this.lines[lineIndex].reduce((sum, node) => sum + this.getNodeLength(node), 0);
	}

	private getNodeLength(node: DocumentNode): number {
	return node.type === 'text' ? node.content.length : 1; // Treat components as length 1
	}

	deleteText(position: Position, length: number)
	{
		throw new Error("Method not implemented.");
	}
	insertText(position: Position, text: string)
	{
		throw new Error("Method not implemented.");
	}
}