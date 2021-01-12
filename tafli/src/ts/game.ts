import {BoardConfiguration} from "./board_configuration";

export class Game {
    board: BoardConfiguration
    status: any

    constructor(board: BoardConfiguration, status: any) {
        this.board = board;
        this.status = status;
    }
}