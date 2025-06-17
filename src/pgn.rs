
/* pgn.rs
 * ---
 *
 * pgn format starts with a bunch of standard and optional key-value pairs
 * then the chess moves turn by turn in SAN
 *
 * # Seven Tag Roster
 *
 * there are seven required tags that must appear /before/ any other tags, in this order
 * [Event ""] ; Name of the tournament or match event
 * [Site ""] {location of event. in =City, Region COUNTRY= format. COUNTRY ::= three-letter IOC
 * code
 * example: New York City, NY USA}
 * [Date ""] ; start date of game, in YYYY.MM.DD form, ?? for unknown values
 * [Round ""] ; "playing round ordinal of the game within the event" whatever that means
 * [White ""] ; player of white pieces, =Lastname, Firstname= format
 * [Black ""] ; same as white
 * [Result ""] ; result of game, =whiteScore-blackScore= format, or * if ongoing or smn
 *
 *  # Optional tag pairs
 *  [Annotator ""] ; guy annotating the pgn
 *  [PlyCount ""] ; total ply's, or half-moves played (full move is both players moving)
 *  [TimeControl ""] ; "e.g. =40/7200:3600= (moves per seconds, sudden death seconds)
 *  [Time ""] ; time the game started, in =HH:MM:SS= form
 *  [Termination ""] {context about the end of the game. enum. possible values are abandoned,
 *  adjudication (result determined by third party), death, emergency, normal, rules infraction,
 *  time forfeit, or unterminated}
 *  [Mode ""] ; OTB (Over the board) or ICS (Internet Chess Server)
 *  [FEN ""] {initial board position in Forsyth-Edwards Notation. used to record partial games or
 *  variants}
 *  [SetUp ""] if FEN tag is used, SetUp must also appear and have value set to "1"
 */
