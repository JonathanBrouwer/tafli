import {BoardConfiguration} from "./board_configuration";

export class Game {
    board: BoardConfiguration
    status: any
    prev_move_info: PreviousMoveInfo
}

export class PreviousMoveInfo {
    last_move: number[][]
    captures: number[][]
}