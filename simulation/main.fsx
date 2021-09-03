type Kind = In | Out | Wire

type ChipRef = ChipRef of string

type PinRef = PinRef of Kind * uint8

type ChipDef =
    {
        Name: string
        InPins: uint8
        OutPins: uint8
        Wires: uint8
        Parts: ChipRef * PinRef array
    }
    override this.ToString() = $"[%s{this.Name}]"

{ Name = "Not"; InPins = 2uy; OutPins = 2uy; Wires = 0uy; Chips = 0uy; Stmts = [| (* ... *) |] }
|> printfn "%O"
