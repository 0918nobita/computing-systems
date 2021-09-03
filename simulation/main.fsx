type Kind = In | Out | Wire

type ChipRef = ChipRef of string

type PinRef = PinRef of Kind * uint8

type ChipConn = ChipConn of ChipRef * PinRef seq * PinRef seq

type ChipDef =
    {
        Name: string
        InPins: uint8
        OutPins: uint8
        Wires: uint8
        Parts: ChipConn seq
    }
    override this.ToString() = $"[%s{this.Name}]"

{ Name = "Not"; InPins = 2uy; OutPins = 2uy; Wires = 0uy; Parts = [| (* ... *) |] }
|> printfn "%O"
