// eslint-disable-next-line no-unused-vars
export class BoardData {
    fields: FieldState[][];
    turn: Player;

    constructor(fields: FieldState[][], turn: Player) {
        this.fields = fields;
        this.turn = turn;
    }
}

// eslint-disable-next-line no-unused-vars
export enum FieldState {
    WhiteKing,
    WhitePiece,
    BlackPiece,
    Empty
}

// eslint-disable-next-line no-unused-vars
export enum Player {
    White, Black
}