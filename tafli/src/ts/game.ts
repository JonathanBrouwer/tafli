import {BoardConfiguration} from "./board_configuration";

export class Game {
    board: BoardConfiguration
    status: any
    prev_move_info: PreviousMoveInfo
    gameid: number
    player_white: PlayerInfo
    player_black: PlayerInfo
    time_control: number[]
}

export class PlayerInfo {
    userid: number
    name: String
}

export class PreviousMoveInfo {
    last_move: number[][]
    captures: number[][]
}