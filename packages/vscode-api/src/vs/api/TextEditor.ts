import type { EditorFile } from "@acode-app/types/src/editor/file";
import type * as vscode from "vscode";
import Range from "./Range";
import Selection from "./Selection";
import TextDocument from "./TextDocument";
import ViewColumn from "./ViewColumn";

export default class TextEditor implements vscode.TextEditor {
	document: TextDocument;
	get selection(): Selection {
		return Selection.from(this.document.into().session.getSelection());
	}

	public get selections(): readonly Selection[] {
		return [this.selection];
	}

	get visibleRanges(): readonly Range[] {
		const screenRow = this.document.into().session.getScreenLength();
		const screenColumn = this.document.into().session.getScreenWidth();
		return [
			Range.from(
				ace.Range.fromPoints(
					this.document.into().session.screenToDocumentPosition(0, 0),
					this.document
						.into()
						.session.screenToDocumentPosition(screenRow, screenColumn),
				),
			),
		];
	}

	get options(): vscode.TextEditorOptions {
		return {
			tabSize: this.document.into().session.getTabSize(),
		};
	}

	viewColumn: ViewColumn | undefined = ViewColumn.One;

	constructor(value: EditorFile) {
		this.document = new TextDocument(value);
	}

	edit(
		_callback: (editBuilder: vscode.TextEditorEdit) => void,
		_options?: {
			readonly undoStopBefore: boolean;
			readonly undoStopAfter: boolean;
		},
	): Thenable<boolean> {
		throw new Error("Method not implemented.");
	}

	insertSnippet(
		_snippet: vscode.SnippetString,
		_location?:
			| vscode.Position
			| vscode.Range
			| readonly vscode.Position[]
			| readonly vscode.Range[],
		_options?: {
			readonly undoStopBefore: boolean;
			readonly undoStopAfter: boolean;
		},
	): Thenable<boolean> {
		throw new Error("Method not implemented.");
	}

	setDecorations(
		_decorationType: vscode.TextEditorDecorationType,
		_rangesOrOptions:
			| readonly vscode.Range[]
			| readonly vscode.DecorationOptions[],
	): void {
		this.document.into().session.addGutterDecoration;
		throw new Error("Method not implemented.");
	}

	revealRange(
		_range: vscode.Range,
		_revealType?: vscode.TextEditorRevealType,
	): void {
		throw new Error("Method not implemented.");
	}

	show(_column?: vscode.ViewColumn): void {
		throw new Error("Method not implemented.");
	}

	hide(): void {
		throw new Error("Method not implemented.");
	}
}
