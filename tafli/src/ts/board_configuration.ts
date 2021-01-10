export class BoardConfiguration {
    fields: FieldState[][];
    turn: Player;

    constructor(fields: FieldState[][], turn: Player) {
        this.fields = fields;
        this.turn = turn;
    }
}

export enum FieldState {
    WhiteKing = "WhiteKing",
    WhitePiece = "WhitePiece",
    BlackPiece = "BlackPiece",
    Empty = "Empty"
}

export enum Player {
    White, Black
}