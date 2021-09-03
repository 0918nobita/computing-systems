type Kind = In | Out | Wire

type Stmt = {
    From: Kind * uint8
    To: Kind * uint8
}

type ChipDef =
    {
        Name: string
        InPins: uint8
        OutPins: uint8
        Wires: uint8
        Chips: uint8
        Stmts: Stmt array
    }
    override this.ToString() = $"[%s{this.Name}]"

{ Name = "Not"; InPins = 2uy; OutPins = 2uy; Wires = 0uy; Chips = 0uy; Stmts = [| (* ... *) |] }
|> printfn "%O"
