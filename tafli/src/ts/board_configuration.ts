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

export function player(state: FieldState) {
    if (state == FieldState.WhiteKing || state == FieldState.WhitePiece) {
        return Player.White;
    }else if (state == FieldState.BlackPiece) {
        return Player.Black;
    } else {
        return null;
    }
}

export enum Player {
    White = "White", Black = "Black"
}