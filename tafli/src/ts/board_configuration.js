"use strict";
exports.__esModule = true;
exports.BoardSide = exports.Player = exports.player = exports.FieldState = exports.BoardConfiguration = void 0;
var BoardConfiguration = /** @class */ (function () {
    function BoardConfiguration(fields, turn) {
        this.fields = fields;
        this.turn = turn;
    }

    return BoardConfiguration;
}());
exports.BoardConfiguration = BoardConfiguration;
var FieldState;
(function (FieldState) {
    FieldState["WhiteKing"] = "WhiteKing";
    FieldState["WhitePiece"] = "WhitePiece";
    FieldState["BlackPiece"] = "BlackPiece";
    FieldState["Empty"] = "Empty";
})(FieldState = exports.FieldState || (exports.FieldState = {}));

function player(state) {
    if (state == FieldState.WhiteKing || state == FieldState.WhitePiece) {
        return Player.White;
    } else if (state == FieldState.BlackPiece) {
        return Player.Black;
    } else {
        return null;
    }
}

exports.player = player;
var Player;
(function (Player) {
    Player["White"] = "White";
    Player["Black"] = "Black";
})(Player = exports.Player || (exports.Player = {}));
var BoardSide;
(function (BoardSide) {
    BoardSide["White"] = "White";
    BoardSide["Black"] = "Black";
    BoardSide["Spectator"] = "Spectator";
})(BoardSide = exports.BoardSide || (exports.BoardSide = {}));
