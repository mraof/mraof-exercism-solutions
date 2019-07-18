class QueenAttack {
    white: [number, number]
    black: [number, number]

    constructor(positioning: { white: [number, number], black: [number, number] }) {
        if (positioning.white[0] === positioning.black[0] && positioning.white[1] === positioning.black[1]) {
            throw "Queens cannot share the same space"
        }
        this.white = positioning.white
        this.black = positioning.black
    }

    canAttack() {
        return this.white[0] === this.black[0] || this.white[1] === this.black[1] || (Math.abs(this.white[0] - this.black[0]) === Math.abs(this.white[1] - this.black[1]))
    }

    toString() {
        let s = ""
        for (let y = 0; y < 8; y++) {
            for (let x = 0; x < 8; x++) {
                if (this.white[0] === y && this.white[1] === x) {
                    s += "W"
                } else if (this.black[0] === y && this.black[1] === x) {
                    s += "B"
                } else {
                    s += "_"
                }
                if (x < 7) {
                    s += " "
                }
            }
            s += "\n"
        }
        return s
    }
}

export default QueenAttack
