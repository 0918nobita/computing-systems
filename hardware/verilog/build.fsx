open System.IO

let depFilesMap =
    [
        ("./not_test.v", ["./not.v"])
        ("./and_test.v", ["./and.v"; "./not.v"])
        ("./or_test.v", ["./or.v"; "./not.v"])
        ("./and16_test.v", ["./and16.v"; "./and.v"; "./not.v"])
        ("./or16_test.v", ["./or16.v"; "./or.v"; "./not.v"])
    ]
    |> List.map (fun (mainInputFile, depFiles) -> (Path.GetFullPath mainInputFile, List.map Path.GetFullPath depFiles))
    |> Map.ofList

type BuildStmt = {
    Rule: string
    InputFiles: string seq
    OutputFile: string
}

module BuildStmt =
    let compile (stmt: BuildStmt): string =
        let inputFiles = stmt.InputFiles |> String.concat " "
        $"build %s{stmt.OutputFile}: %s{stmt.Rule} %s{inputFiles}"

let buildStmts =
    seq {
        for path in Directory.EnumerateFiles("./", "*_test.v", SearchOption.AllDirectories) do
            let fullPath = Path.GetFullPath path
            let depFiles =
                depFilesMap
                |> Map.tryFind fullPath
                |> Option.defaultValue []
            let vvpFilePath = Path.ChangeExtension(fullPath, "vvp")
            let vcdFilePath = Path.ChangeExtension(fullPath, "vcd")
            yield! [
                { Rule = "compile"; InputFiles = [fullPath] @ depFiles; OutputFile = vvpFilePath }
                { Rule = "run_test"; InputFiles = [vvpFilePath]; OutputFile = vcdFilePath }
            ]
    }
    |> Seq.map BuildStmt.compile
    |> String.concat "\n"

let contents =
    $"""include rules.ninja

%s{buildStmts}
"""

File.WriteAllText ("build.ninja", contents)
